//! BOSS直聘登录与Cookie管理模块。
//!
//! 入口文件只保留公开数据结构和客户端的基础构造逻辑，具体流程拆在
//! `boss/` 子模块里，方便按问题域查看。

mod auth;
mod browser;
mod cookies;

use rust_drission::chromium_page::ChromiumPage;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::error::Result as BResult;

pub(super) const BOSS_LOGIN: &str = "https://www.zhipin.com/web/user/?ka=login";
pub(super) const BOSS_COOKIE_URLS: [&str; 2] =
    ["https://www.zhipin.com", "https://www.zhipin.com/web/user/"];
pub(super) const BOSS_AUTH_COOKIE_NAMES: [&str; 3] = ["wt2", "zp_at", "zp_token"];

/// Serializable store for the BOSS login cookies saved on disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CookieStore {
    pub cookies: Vec<CookieEntry>,
    pub saved_at: String,
    /// Marks which account produced this cookie file.
    pub account_hint: String,
}

/// Single browser cookie entry persisted in the cookie store.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CookieEntry {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: Option<String>,
}

/// BOSS client that owns the browser page and login-cookie state.
pub struct BossClient {
    page: ChromiumPage,
    cookie_path: PathBuf,
    forced_relogin: bool,
}

impl BossClient {
    /// Create a client by attaching to Chrome or launching a debug-enabled tab.
    pub fn new(force_relogin: bool) -> BResult<Self> {
        let cookie_path = Self::cookie_file_path();
        let page = Self::connect_or_launch_browser()?;
        Self::activate_page(&page)?;

        log::info!("已通过rust_drission连接到Chrome");

        Ok(Self {
            page,
            cookie_path,
            forced_relogin: force_relogin,
        })
    }

    /// Return the mutable page handle used by the poster.
    pub fn page(&mut self) -> &mut ChromiumPage {
        &mut self.page
    }

    /// Decide whether cookie reuse should be skipped.
    pub fn needs_login(&self) -> bool {
        self.forced_relogin || !self.cookie_path.exists()
    }
}
