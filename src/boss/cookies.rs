use super::*;
use rust_drission::page::Cookie;
use std::fs;
use std::time::{Duration, Instant};

use crate::error::{BossError, Result as BResult};
use crate::utils::sleep_random_ms;

impl BossClient {
    /// Return the local JSON file used for saved BOSS cookies.
    pub(super) fn cookie_file_path() -> PathBuf {
        dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("boss_auto")
            .join("cookies.json")
    }

    /// Persist the current browser cookies after verifying a login cookie exists.
    pub fn save_cookies(&self) -> BResult<()> {
        let cookies = self.fetch_boss_cookies()?;
        if !Self::has_auth_cookie_in_list(&cookies) {
            return Err(BossError::PostFailed(
                "未检测到BOSS登录Cookie，请扫码登录成功后再继续。".to_string(),
            ));
        }

        let entries: Vec<CookieEntry> = cookies
            .into_iter()
            .map(|c| CookieEntry {
                name: c.name,
                value: c.value,
                domain: c.domain.unwrap_or_else(|| "zhipin.com".to_string()),
                path: c.path,
            })
            .collect();

        let store = CookieStore {
            cookies: entries,
            saved_at: chrono::Local::now().to_rfc3339(),
            account_hint: "scanned".to_string(),
        };

        if let Some(parent) = self.cookie_path.parent() {
            fs::create_dir_all(parent)
                .map_err(BossError::map_config("无法创建配置目录"))?;
        }

        let json = serde_json::to_string_pretty(&store)
            .map_err(BossError::map_config("Cookie序列化失败"))?;

        fs::write(&self.cookie_path, json)
            .map_err(BossError::map_config("写入Cookie文件失败"))?;

        log::info!("Cookie已保存到: {:?}", self.cookie_path);
        Ok(())
    }

    /// Load the saved cookie file and confirm the browser still has auth cookies.
    pub(super) fn try_load_cookies(&self) -> BResult<()> {
        let store: CookieStore = serde_json::from_str(
            &fs::read_to_string(&self.cookie_path).map_err(|_| BossError::CookieExpired)?,
        )
        .map_err(|_| BossError::CookieExpired)?;

        if !Self::has_auth_cookie_in_store(&store) || !self.has_login_cookie()? {
            return Err(BossError::CookieExpired);
        }

        log::info!("Cookie验证成功，已登录BOSS");
        Ok(())
    }

    /// Wait until Chrome exposes one of the known BOSS login cookies.
    pub(super) fn wait_for_login_cookie(&self, timeout: Duration) -> BResult<bool> {
        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            if self.has_login_cookie()? {
                return Ok(true);
            }
            sleep_random_ms(800, 1100);
        }
        Ok(false)
    }

    /// Read BOSS-domain cookies from the current browser context.
    fn fetch_boss_cookies(&self) -> BResult<Vec<Cookie>> {
        let urls: Vec<String> = BOSS_COOKIE_URLS.iter().map(|u| u.to_string()).collect();
        self.page
            .cookies(Some(&urls))
            .map_err(BossError::map_cdp("读取BOSS Cookie失败"))
    }

    /// Return true if the cookie name is one of the known BOSS auth keys.
    fn is_auth_cookie_name(name: &str) -> bool {
        BOSS_AUTH_COOKIE_NAMES
            .iter()
            .any(|candidate| candidate.eq_ignore_ascii_case(name))
    }

    /// Return true when a browser cookie list contains a non-empty auth cookie.
    fn has_auth_cookie_in_list(cookies: &[Cookie]) -> bool {
        cookies
            .iter()
            .any(|c| Self::is_auth_cookie_name(&c.name) && !c.value.trim().is_empty())
    }

    /// Return true when the saved cookie store contains a non-empty auth cookie.
    fn has_auth_cookie_in_store(store: &CookieStore) -> bool {
        store
            .cookies
            .iter()
            .any(|c| Self::is_auth_cookie_name(&c.name) && !c.value.trim().is_empty())
    }

    /// Check the current browser context for a valid BOSS login cookie.
    fn has_login_cookie(&self) -> BResult<bool> {
        Ok(Self::has_auth_cookie_in_list(&self.fetch_boss_cookies()?))
    }
}
