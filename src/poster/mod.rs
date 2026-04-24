//! Job posting automation module.
//!
//! This module is intentionally split by page area so each file stays small
//! enough to inspect while debugging BOSS page changes.

mod address;
mod basic_fields;
mod category;
mod dom;
mod dropdown;
mod education;
mod experience;
mod flow;
mod intern;
mod job_type;
mod misc;
mod requirements;
mod salary;
mod selectors;
mod tags;

use rust_drission::chromium_page::ChromiumPage;
use std::time::Duration;

use crate::boss::BossClient;
use crate::error::{BossError, Result as BResult};
use crate::excel::JobRecord;
use crate::utils::sleep_random_ms;

pub use selectors::SelectorMap;

/// Drives the BOSS job publishing form in one browser tab.
pub struct Poster<'a> {
    pub(super) page: &'a mut ChromiumPage,
    pub(super) selectors: SelectorMap,
}

pub(super) const STEP_SETTLE_MS: u64 = 900;

/// Recruitment type selected at the top of the BOSS publish form.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RecruitmentKind {
    FullTime,
    Campus,
    Intern,
    PartTime,
}

impl RecruitmentKind {
    /// Parse the Excel recruitment type into a stable internal branch.
    pub(super) fn parse(value: &str) -> BResult<Self> {
        let clean = value.replace(char::is_whitespace, "");
        if clean.contains("实习") {
            Ok(Self::Intern)
        } else if clean.contains("应届") || clean.contains("校园") {
            Ok(Self::Campus)
        } else if clean.contains("兼职") {
            Ok(Self::PartTime)
        } else if clean.contains("社招") || clean.contains("全职") {
            Ok(Self::FullTime)
        } else {
            Err(BossError::PostFailed(format!("无法识别招聘类型: {}", value)))
        }
    }

    /// Return the visible BOSS label for this recruitment type.
    pub(super) fn label(self) -> &'static str {
        match self {
            Self::FullTime => "社招全职",
            Self::Campus => "应届校园招聘",
            Self::Intern => "实习生招聘",
            Self::PartTime => "兼职招聘",
        }
    }
}

impl<'a> Poster<'a> {
    /// Create a poster bound to the already logged-in browser page.
    pub fn new(client: &'a mut BossClient) -> Self {
        let selectors = SelectorMap::load().unwrap_or_else(SelectorMap::default_boss_pc);
        Self {
            page: client.page(),
            selectors,
        }
    }
}
