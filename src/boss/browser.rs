use super::*;
use rust_drission::browser::BrowserConfig;
use std::fs;

use crate::error::{BossError, Result as BResult};

impl BossClient {
    /// Connect to an existing debug Chrome, or launch one with a stable data dir.
    pub(super) fn connect_or_launch_browser() -> BResult<ChromiumPage> {
        for port in [9222_u16, 19533, 9333] {
            let config = BrowserConfig::new()
                .set_address(format!("127.0.0.1:{}", port))
                .existing_only(true);
            if let Ok(mut page) = ChromiumPage::new(config) {
                Self::open_visible_tab(&mut page)?;
                log::info!("已连接到现有Chrome调试端口: {}", port);
                return Ok(page);
            }
        }

        let data_dir = std::env::temp_dir().join("boss_auto_chrome");
        fs::create_dir_all(&data_dir)
            .map_err(BossError::map_config("创建Chrome数据目录失败"))?;

        let config = BrowserConfig::new()
            .set_local_port(9222)
            .user_data_dir(data_dir.to_string_lossy().to_string());

        let mut page = ChromiumPage::new(config)
            .map_err(BossError::map_cdp("通过rust_drission启动Chrome失败"))?;
        Self::open_visible_tab(&mut page)?;
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
