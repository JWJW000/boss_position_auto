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
        let content =
            toml::to_string_pretty(self).map_err(|e| ConfigError::WriteError(e.to_string()))?;
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

    /// 获取可尝试启动的浏览器路径列表。配置项优先，随后自动兜底到常见 Chromium 内核浏览器。
    pub fn get_browser_path_candidates(&self) -> Vec<String> {
        let mut candidates = Vec::new();

        match &self.browser.browser_type {
            BrowserType::Custom => {
                if let Some(path) = &self.browser.custom_path {
                    push_unique(&mut candidates, path.clone());
                }
            }
            BrowserType::Chrome => extend_detected(&mut candidates, detect_chrome_paths()),
            BrowserType::Edge => extend_detected(&mut candidates, detect_edge_paths()),
            BrowserType::Chromium => extend_detected(&mut candidates, detect_chromium_paths()),
        }

        extend_detected(&mut candidates, detect_chrome_paths());
        extend_detected(&mut candidates, detect_edge_paths());
        extend_detected(&mut candidates, detect_chromium_paths());
        extend_detected(&mut candidates, detect_brave_paths());

        candidates
    }
}

fn push_unique(candidates: &mut Vec<String>, path: String) {
    if !path.trim().is_empty() && !candidates.iter().any(|item| item == &path) {
        candidates.push(path);
    }
}

fn push_existing(candidates: &mut Vec<String>, path: impl Into<PathBuf>) {
    let path = path.into();
    if path.exists() {
        push_unique(candidates, path.to_string_lossy().to_string());
    }
}

fn extend_detected(candidates: &mut Vec<String>, paths: Vec<String>) {
    for path in paths {
        push_unique(candidates, path);
    }
}

/// 检测 Chrome 浏览器路径
fn detect_chrome_path() -> Option<String> {
    detect_chrome_paths().into_iter().next()
}

fn detect_chrome_paths() -> Vec<String> {
    let mut candidates = Vec::new();

    #[cfg(target_os = "windows")]
    {
        push_existing(
            &mut candidates,
            r"C:\Program Files\Google\Chrome\Application\chrome.exe",
        );
        push_existing(
            &mut candidates,
            r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
        );
        if let Some(local_app_data) = std::env::var_os("LOCALAPPDATA") {
            push_existing(
                &mut candidates,
                PathBuf::from(local_app_data)
                    .join("Google")
                    .join("Chrome")
                    .join("Application")
                    .join("chrome.exe"),
            );
        }

        // 尝试从注册表读取
        if let Ok(chrome_path) = read_chrome_path_from_registry() {
            push_existing(&mut candidates, chrome_path);
        }

        push_path_exe(&mut candidates, "chrome.exe");
    }

    #[cfg(target_os = "macos")]
    {
        let path = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";
        push_existing(&mut candidates, path);
    }

    #[cfg(target_os = "linux")]
    {
        for path in [
            "/usr/bin/google-chrome",
            "/usr/bin/google-chrome-stable",
            "/usr/bin/chromium",
            "/usr/bin/chromium-browser",
        ] {
            push_existing(&mut candidates, path);
        }
        push_path_exe(&mut candidates, "google-chrome");
        push_path_exe(&mut candidates, "google-chrome-stable");
    }

    candidates
}

/// 检测 Edge 浏览器路径
fn detect_edge_path() -> Option<String> {
    detect_edge_paths().into_iter().next()
}

fn detect_edge_paths() -> Vec<String> {
    let mut candidates = Vec::new();

    #[cfg(target_os = "windows")]
    {
        push_existing(
            &mut candidates,
            r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
        );
        push_existing(
            &mut candidates,
            r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
        );
        if let Some(local_app_data) = std::env::var_os("LOCALAPPDATA") {
            push_existing(
                &mut candidates,
                PathBuf::from(local_app_data)
                    .join("Microsoft")
                    .join("Edge")
                    .join("Application")
                    .join("msedge.exe"),
            );
        }
        if let Ok(edge_path) = read_app_path_from_registry("msedge.exe") {
            push_existing(&mut candidates, edge_path);
        }
        push_path_exe(&mut candidates, "msedge.exe");
    }

    #[cfg(target_os = "macos")]
    {
        let path = "/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge";
        push_existing(&mut candidates, path);
    }

    #[cfg(target_os = "linux")]
    {
        push_existing(&mut candidates, "/usr/bin/microsoft-edge");
        push_existing(&mut candidates, "/usr/bin/microsoft-edge-stable");
        push_path_exe(&mut candidates, "microsoft-edge");
        push_path_exe(&mut candidates, "microsoft-edge-stable");
    }

    candidates
}

/// 检测 Chromium 浏览器路径
fn detect_chromium_path() -> Option<String> {
    detect_chromium_paths().into_iter().next()
}

fn detect_chromium_paths() -> Vec<String> {
    let mut candidates = Vec::new();

    #[cfg(target_os = "windows")]
    {
        push_existing(
            &mut candidates,
            r"C:\Program Files\Chromium\Application\chrome.exe",
        );
        push_existing(
            &mut candidates,
            r"C:\Program Files (x86)\Chromium\Application\chrome.exe",
        );
        push_path_exe(&mut candidates, "chromium.exe");
    }

    #[cfg(target_os = "linux")]
    {
        push_existing(&mut candidates, "/usr/bin/chromium");
        push_existing(&mut candidates, "/usr/bin/chromium-browser");
        push_path_exe(&mut candidates, "chromium");
        push_path_exe(&mut candidates, "chromium-browser");
    }

    candidates
}

fn detect_brave_paths() -> Vec<String> {
    let mut candidates = Vec::new();

    #[cfg(target_os = "windows")]
    {
        push_existing(
            &mut candidates,
            r"C:\Program Files\BraveSoftware\Brave-Browser\Application\brave.exe",
        );
        push_existing(
            &mut candidates,
            r"C:\Program Files (x86)\BraveSoftware\Brave-Browser\Application\brave.exe",
        );
        if let Some(local_app_data) = std::env::var_os("LOCALAPPDATA") {
            push_existing(
                &mut candidates,
                PathBuf::from(local_app_data)
                    .join("BraveSoftware")
                    .join("Brave-Browser")
                    .join("Application")
                    .join("brave.exe"),
            );
        }
        push_path_exe(&mut candidates, "brave.exe");
    }

    #[cfg(target_os = "macos")]
    {
        push_existing(
            &mut candidates,
            "/Applications/Brave Browser.app/Contents/MacOS/Brave Browser",
        );
    }

    #[cfg(target_os = "linux")]
    {
        push_existing(&mut candidates, "/usr/bin/brave-browser");
        push_path_exe(&mut candidates, "brave-browser");
    }

    candidates
}

fn push_path_exe(candidates: &mut Vec<String>, exe_name: &str) {
    if let Some(path_var) = std::env::var_os("PATH") {
        for dir in std::env::split_paths(&path_var) {
            push_existing(candidates, dir.join(exe_name));
        }
    }
}

#[cfg(target_os = "windows")]
fn read_chrome_path_from_registry() -> Result<String, Box<dyn std::error::Error>> {
    read_app_path_from_registry("chrome.exe")
}

#[cfg(target_os = "windows")]
fn read_app_path_from_registry(exe_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    use winreg::enums::*;
    use winreg::RegKey;

    let key_path = format!(
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths\{}",
        exe_name
    );
    for root in [HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE] {
        let key = RegKey::predef(root);
        if let Ok(app_key) = key.open_subkey(&key_path) {
            let path: String = app_key.get_value("")?;
            return Ok(path);
        }
    }
    Err(format!("未在注册表 App Paths 中找到 {}", exe_name).into())
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
