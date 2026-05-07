//! 配置文件管理模块
//!
//! 支持从 exe 同目录的 config.toml 读取配置
//! 如果配置文件不存在，会自动创建默认配置

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("读取配置文件失败: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("解析配置文件失败: {0}")]
    ParseError(#[from] toml::de::Error),
    #[error("写入配置文件失败: {0}")]
    WriteError(String),
}

/// 浏览器类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BrowserType {
    Chrome,
    Edge,
    Chromium,
    Custom,
}

impl Default for BrowserType {
    fn default() -> Self {
        Self::Chrome
    }
}

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// 浏览器配置
    #[serde(default)]
    pub browser: BrowserConfig,

    /// 调试配置
    #[serde(default)]
    pub debug: DebugConfig,
}

/// 浏览器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserConfig {
    /// 浏览器类型 (chrome, edge, chromium, custom)
    #[serde(default)]
    pub browser_type: BrowserType,

    /// 自定义浏览器路径（当 browser_type = custom 时使用）
    pub custom_path: Option<String>,

    /// 调试端口
    #[serde(default = "default_port")]
    pub port: u16,

    /// 是否无头模式
    #[serde(default)]
    pub headless: bool,
}

impl Default for BrowserConfig {
    fn default() -> Self {
        Self {
            browser_type: BrowserType::Chrome,
            custom_path: None,
            port: 9222,
            headless: false,
        }
    }
}

fn default_port() -> u16 {
    9222
}

/// 调试配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugConfig {
    /// 是否启用详细日志
    #[serde(default)]
    pub verbose: bool,

    /// 每步操作后的额外等待时间（毫秒）
    #[serde(default)]
    pub extra_delay_ms: u64,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            verbose: false,
            extra_delay_ms: 0,
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            browser: BrowserConfig::default(),
            debug: DebugConfig::default(),
        }
    }
}

impl AppConfig {
    /// 从 exe 同目录加载配置文件，如果不存在则创建默认配置
    pub fn load() -> Result<Self, ConfigError> {
        let config_path = Self::config_path();

        if !config_path.exists() {
            log::info!("配置文件不存在，创建默认配置: {:?}", config_path);
            let default_config = Self::default();
            default_config.save(&config_path)?;
            return Ok(default_config);
        }

        log::info!("加载配置文件: {:?}", config_path);
        let content = fs::read_to_string(&config_path)?;
        let config: AppConfig = toml::from_str(&content)?;
        Ok(config)
    }

    /// 保存配置到文件
    pub fn save(&self, path: &Path) -> Result<(), ConfigError> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| ConfigError::WriteError(e.to_string()))?;
        fs::write(path, content)?;
        log::info!("配置文件已保存: {:?}", path);
        Ok(())
    }

    /// 获取配置文件路径（exe 同目录下的 config.toml）
    pub fn config_path() -> PathBuf {
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.join("config.toml")))
            .unwrap_or_else(|| PathBuf::from("config.toml"))
    }

    /// 获取浏览器可执行文件路径
    pub fn get_browser_path(&self) -> Option<String> {
        match &self.browser.browser_type {
            BrowserType::Custom => self.browser.custom_path.clone(),
            BrowserType::Chrome => detect_chrome_path(),
            BrowserType::Edge => detect_edge_path(),
            BrowserType::Chromium => detect_chromium_path(),
        }
    }
}

/// 检测 Chrome 浏览器路径
fn detect_chrome_path() -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        let paths = vec![
            r"C:\Program Files\Google\Chrome\Application\chrome.exe",
            r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
        ];

        for path in paths {
            if Path::new(path).exists() {
                return Some(path.to_string());
            }
        }

        // 尝试从注册表读取
        if let Ok(chrome_path) = read_chrome_path_from_registry() {
            return Some(chrome_path);
        }
    }

    #[cfg(target_os = "macos")]
    {
        let path = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";
        if Path::new(path).exists() {
            return Some(path.to_string());
        }
    }

    #[cfg(target_os = "linux")]
    {
        let paths = vec![
            "/usr/bin/google-chrome",
            "/usr/bin/google-chrome-stable",
            "/usr/bin/chromium",
            "/usr/bin/chromium-browser",
        ];

        for path in paths {
            if Path::new(path).exists() {
                return Some(path.to_string());
            }
        }
    }

    None
}

/// 检测 Edge 浏览器路径
fn detect_edge_path() -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        let paths = vec![
            r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
            r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
        ];

        for path in paths {
            if Path::new(path).exists() {
                return Some(path.to_string());
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        let path = "/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge";
        if Path::new(path).exists() {
            return Some(path.to_string());
        }
    }

    None
}

/// 检测 Chromium 浏览器路径
fn detect_chromium_path() -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        let paths = vec![
            r"C:\Program Files\Chromium\Application\chrome.exe",
            r"C:\Program Files (x86)\Chromium\Application\chrome.exe",
        ];

        for path in paths {
            if Path::new(path).exists() {
                return Some(path.to_string());
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        let paths = vec![
            "/usr/bin/chromium",
            "/usr/bin/chromium-browser",
        ];

        for path in paths {
            if Path::new(path).exists() {
                return Some(path.to_string());
            }
        }
    }

    None
}

#[cfg(target_os = "windows")]
fn read_chrome_path_from_registry() -> Result<String, Box<dyn std::error::Error>> {
    use winreg::enums::*;
    use winreg::RegKey;

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let chrome_key = hklm.open_subkey(r"SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths\chrome.exe")?;
    let path: String = chrome_key.get_value("")?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.browser.browser_type, BrowserType::Chrome);
        assert_eq!(config.browser.port, 9222);
        assert!(!config.browser.headless);
    }

    #[test]
    fn test_serialize_config() {
        let config = AppConfig::default();
        let toml_str = toml::to_string_pretty(&config).unwrap();
        assert!(toml_str.contains("browser_type"));
        assert!(toml_str.contains("chrome"));
    }
}
