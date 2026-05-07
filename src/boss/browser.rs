use super::*;
use rust_drission::browser::BrowserConfig;
use rust_drission::stealth;
use std::fs;

use crate::config::AppConfig;
use crate::error::{BossError, Result as BResult};

impl BossClient {
    /// Connect to an existing debug Chrome, or launch one with a stable data dir.
    pub(super) fn connect_or_launch_browser() -> BResult<ChromiumPage> {
        // 加载配置文件
        let app_config = AppConfig::load()
            .map_err(|e| BossError::Config(format!("加载配置文件失败: {}", e)))?;

        log::info!("使用浏览器类型: {:?}", app_config.browser.browser_type);

        // 尝试连接已有浏览器
        for port in [app_config.browser.port, 9222_u16, 19533, 9333] {
            let config = BrowserConfig::new()
                .set_address(format!("127.0.0.1:{}", port))
                .existing_only(true);
            if let Ok(mut page) = ChromiumPage::new(config) {
                Self::open_visible_tab(&mut page)?;
                stealth::inject(page.tab())
                    .map_err(BossError::map_cdp("注入stealth反检测脚本失败"))?;
                log::info!("已连接到现有浏览器调试端口: {} 并注入stealth", port);
                return Ok(page);
            }
        }

        // 启动新浏览器
        let data_dir = std::env::temp_dir().join("boss_auto_chrome");
        fs::create_dir_all(&data_dir)
            .map_err(BossError::map_config("创建Chrome数据目录失败"))?;

        let mut config = BrowserConfig::new()
            .set_local_port(app_config.browser.port)
            .user_data_dir(data_dir.to_string_lossy().to_string())
            .headless(app_config.browser.headless)
            .set_argument("--window-size", Some("1920,1080"))
            .set_argument("--force-device-scale-factor", Some("1"));

        // 设置浏览器路径
        if let Some(browser_path) = app_config.get_browser_path() {
            log::info!("使用浏览器路径: {}", browser_path);
            config = config.chrome_path(browser_path);
        } else {
            log::warn!("未检测到浏览器路径，将使用系统默认浏览器");
        }

        let mut page = ChromiumPage::new(config)
            .map_err(BossError::map_cdp("通过rust_drission启动浏览器失败"))?;
        Self::open_visible_tab(&mut page)?;
        stealth::inject(page.tab())
            .map_err(BossError::map_cdp("注入stealth反检测脚本失败"))?;
        log::info!("已启动新浏览器并注入stealth");
        Ok(page)
    }

    /// Bring the current browser tab to the foreground before user-visible steps.
    pub(super) fn activate_page(page: &ChromiumPage) -> BResult<()> {
        page.browser()
            .activate_tab(page.tab().tab_id())
            .map_err(BossError::map_cdp("activate browser tab failed"))
    }

    /// Open a blank visible tab so CDP actions are bound to a real page.
    fn open_visible_tab(page: &mut ChromiumPage) -> BResult<()> {
        page.new_tab_as_current(Some("about:blank"))
            .map_err(BossError::map_cdp("open visible browser tab failed"))
    }
}
