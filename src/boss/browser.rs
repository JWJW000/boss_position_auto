use super::*;
use rust_drission::browser::BrowserConfig;
use rust_drission::stealth;
use std::fs;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

use crate::config::AppConfig;
use crate::error::{BossError, Result as BResult};

impl BossClient {
    /// Connect to an existing debug Chrome, or launch one with a stable data dir.
    pub(super) fn connect_or_launch_browser() -> BResult<ChromiumPage> {
        // 加载配置文件
        let app_config =
            AppConfig::load().map_err(|e| BossError::Config(format!("加载配置文件失败: {}", e)))?;

        log::info!("使用浏览器类型: {:?}", app_config.browser.browser_type);

        // 尝试连接已有浏览器。先用短 TCP 探测过滤未开启端口，避免每个端口都等 CDP 超时。
        for port in [app_config.browser.port, 9222_u16, 19533, 9333] {
            if !is_debug_port_open(port) {
                log::debug!("浏览器调试端口 {} 未开放，跳过连接尝试", port);
                continue;
            }
            let config = BrowserConfig::new()
                .set_address(format!("127.0.0.1:{}", port))
                .existing_only(true);
            if let Ok(page) = ChromiumPage::new(config) {
                let page = Self::replace_with_new_tab(page)?;
                log::info!("已连接到现有浏览器调试端口: {} 并注入stealth", port);
                return Ok(page);
            }
        }

        // 启动新浏览器
        let data_dir = std::env::temp_dir().join("boss_auto_chrome");
        fs::create_dir_all(&data_dir).map_err(BossError::map_config("创建Chrome数据目录失败"))?;

        let base_config = BrowserConfig::new()
            .set_local_port(app_config.browser.port)
            .user_data_dir(data_dir.to_string_lossy().to_string())
            .headless(app_config.browser.headless)
            .set_argument("--window-size", Some("1920,1080"))
            .set_argument("--force-device-scale-factor", Some("1"));

        let mut browser_paths = app_config.get_browser_path_candidates();
        if browser_paths.is_empty() {
            log::warn!("未检测到浏览器路径，将尝试系统默认浏览器命令");
        } else {
            log::info!("检测到 {} 个可尝试浏览器路径", browser_paths.len());
        }
        browser_paths.push(String::new());

        let mut last_error = None;
        for browser_path in browser_paths {
            let mut config = base_config.clone();
            if browser_path.is_empty() {
                log::info!("尝试使用系统默认浏览器命令启动");
            } else {
                log::info!("尝试使用浏览器路径: {}", browser_path);
                config = config.chrome_path(browser_path.clone());
            }

            match ChromiumPage::new(config) {
                Ok(page) => {
                    let page = Self::replace_with_new_tab(page)?;
                    log::info!("已启动新浏览器并注入stealth");
                    return Ok(page);
                }
                Err(err) => {
                    let label = if browser_path.is_empty() {
                        "系统默认浏览器命令".to_string()
                    } else {
                        browser_path
                    };
                    log::warn!("浏览器启动失败，尝试下一个兜底项: {} ({})", label, err);
                    last_error = Some(err.to_string());
                }
            }
        }

        Err(BossError::PostFailed(format!(
            "通过rust_drission启动浏览器失败，已尝试配置路径和常见兜底浏览器。最后错误: {}",
            last_error.unwrap_or_else(|| "无可用浏览器候选".to_string())
        )))
    }

    /// Bring the current browser tab to the foreground before user-visible steps.
    pub(super) fn activate_page(page: &ChromiumPage) -> BResult<()> {
        page.browser()
            .activate_tab(page.tab().tab_id())
            .map_err(BossError::map_cdp("activate browser tab failed"))
    }

    /// Replace the current tab with a new blank tab and inject stealth.
    fn replace_with_new_tab(page: ChromiumPage) -> BResult<ChromiumPage> {
        let (browser, old_page) = page.into_parts();

        // Create new tab with about:blank
        let new_tab = browser
            .new_tab()
            .map_err(BossError::map_cdp("create new tab failed"))?;
        new_tab
            .goto("about:blank")
            .map_err(BossError::map_cdp("navigate to about:blank failed"))?;

        // Inject stealth into the new tab
        stealth::inject(&new_tab).map_err(BossError::map_cdp("inject stealth script failed"))?;

        // Activate the new tab
        browser
            .activate_tab(new_tab.tab_id())
            .map_err(BossError::map_cdp("activate new tab failed"))?;

        // Close the old tab if it's not the only one
        let _ = old_page.close();

        // Reconstruct ChromiumPage with the new tab
        Ok(unsafe { std::mem::transmute((browser, new_tab)) })
    }
}

fn is_debug_port_open(port: u16) -> bool {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    TcpStream::connect_timeout(&addr, Duration::from_millis(180)).is_ok()
}
