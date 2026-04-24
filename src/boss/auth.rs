use super::*;
use std::io::{self, Write};
use std::time::Duration;

use crate::error::{BossError, Result as BResult};
use crate::utils::sleep_random_ms;

impl BossClient {
    /// Open the login page and wait for the user to finish login manually.
    pub fn qr_login(&mut self) -> BResult<()> {
        log::info!("正在打开BOSS登录页: {}", BOSS_LOGIN);
        Self::activate_page(&self.page)?;
        self.page
            .get(BOSS_LOGIN)
            .map_err(BossError::map_cdp("打开登录页失败"))?;

        println!("\n========================================");
        println!("  请在打开的Chrome窗口中完成BOSS登录");
        println!("========================================\n");
        print!("  登录完成后按 Enter 继续...");
        std::io::stdout().flush().ok();
        let _ = io::stdin().read_line(&mut String::new());

        sleep_random_ms(1200, 1800);
        if !self.wait_for_login_cookie(Duration::from_secs(30))? {
            return Err(BossError::PostFailed(
                "未检测到BOSS登录Cookie，登录没有成功，已停止进入下一环节。请确认登录成功后再按 Enter。"
                    .to_string(),
            ));
        }

        self.save_cookies()?;
        log::info!("登录流程完成");
        Ok(())
    }

    /// Reuse saved cookies when valid; otherwise run the manual login flow.
    pub fn login_if_needed(&mut self) -> BResult<()> {
        if !self.needs_login() {
            match self.try_load_cookies() {
                Ok(_) => {
                    log::info!("使用已保存的Cookie，无需重新登录");
                    return Ok(());
                }
                Err(BossError::CookieExpired) => {
                    log::info!("Cookie已过期，需要重新扫码登录");
                }
                Err(e) => {
                    log::warn!("Cookie验证失败: {}，需要重新登录", e);
                }
            }
        }

        self.qr_login()
    }
}
