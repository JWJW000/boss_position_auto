use rust_drission::chromium_page::ChromiumPage;
use std::fs;
use std::path::PathBuf;

use crate::error::{BossError, Result as BResult};

/// Stores fallback selector lists for each BOSS publishing form field.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct SelectorMap {
    pub job_title: Vec<String>,
    pub job_desc: Vec<String>,
    pub salary_range: Vec<String>,
    pub city: Vec<String>,
    pub education: Vec<String>,
    pub experience: Vec<String>,
    pub tags: Vec<String>,
    pub benefits: Vec<String>,
    pub submit_btn: Vec<String>,
    pub urgent_checkbox: Vec<String>,
    pub deadline: Vec<String>,
    pub district: Vec<String>,
    pub job_type: Vec<String>,
    pub overseas: Vec<String>,
}

impl SelectorMap {
    /// Build the default BOSS desktop selectors used when no config file exists.
    pub fn default_boss_pc() -> Self {
        Self {
            job_title: vec![
                "css:input[name='jobName']".to_string(),
                "css:.jobName".to_string(),
                "css:input.ipt[placeholder*='职位名称']".to_string(),
                "xpath://input[@name='jobName']".to_string(),
            ],
            job_desc: vec![
                "css:textarea".to_string(),
                "css:textarea.ipt".to_string(),
                "xpath://textarea[contains(@placeholder,'请勿填写QQ')]".to_string(),
            ],
            salary_range: vec![
                "css:.salary-type-container".to_string(),
                "css:.part-time-job-salary".to_string(),
                "css:.publish-content".to_string(),
            ],
            city: vec![
                "css:input[placeholder='选择工作地点']".to_string(),
                "xpath://input[@placeholder='选择工作地点']".to_string(),
                "css:input.ipt[readonly]".to_string(),
            ],
            education: vec![
                "xpath://div[contains(@class,'form-item')][.//*[contains(text(),'学历') or contains(@placeholder,'学历')]]//div[contains(@class,'ui-select')]".to_string(),
                "xpath://div[contains(@class,'job-form-item')][.//*[contains(text(),'学历') or contains(@placeholder,'学历')]]//div[contains(@class,'ui-select')]".to_string(),
                "xpath://div[contains(@class,'ui-select-inner')][contains(@placeholder,'学历')]".to_string(),
                "xpath://div[@tabindex and .//span[contains(@placeholder,'学历')]]".to_string(),
                "css:.ui-select-inner".to_string(),
            ],
            experience: vec![
                "xpath://div[contains(@class,'publish-edit-form-row')][.//*[contains(text(),'经验') or contains(@placeholder,'经验')]]//div[contains(@class,'ui-select')]".to_string(),
                "xpath://div[contains(@class,'ui-select')][.//span[contains(text(),'请选择经验要求')]]".to_string(),
                "xpath://div[@tabindex and .//span[contains(text(),'请选择经验要求')]]".to_string(),
            ],
            tags: vec![
                "css:.add-skill".to_string(),
                "css:.skill-input".to_string(),
                "xpath://div[contains(@class,'add-skill')]".to_string(),
            ],
            benefits: vec![
                "css:.optional-tag".to_string(),
                "xpath://span[contains(@class,'optional-tag')]".to_string(),
            ],
            submit_btn: vec![
                "css:.publish-btn".to_string(),
                "xpath://button[contains(@class,'publish')]".to_string(),
                "xpath://button[contains(text(),'发布')]".to_string(),
                "xpath://a[contains(text(),'立即发布')]".to_string(),
                "css:button.btn".to_string(),
            ],
            urgent_checkbox: vec![
                "css:.urgent-check".to_string(),
                "xpath://input[@type='checkbox'][contains(@class,'urgent')]".to_string(),
            ],
            deadline: vec![
                "css:input[placeholder='选择招聘截止时间']".to_string(),
                "xpath://input[@placeholder='选择招聘截止时间']".to_string(),
                "css:#deadline".to_string(),
            ],
            district: vec![
                "css:.district-select".to_string(),
                "css:#district".to_string(),
            ],
            job_type: vec![
                "css:.recruitment-type-content".to_string(),
                "css:.job-type-item".to_string(),
                "xpath://p[contains(@class,'job-type-item')]".to_string(),
            ],
            overseas: vec![
                "css:.overseas-entry-container".to_string(),
                "xpath://div[contains(@class,'overseas-entry-container')]".to_string(),
                "xpath://div[contains(@class,'publish-edit-form-row')][.//*[contains(text(),'是否驻外')]]".to_string(),
            ],
        }
    }

    /// 加载保存的选择器
    /// Load selector overrides from the user's local data directory.
    pub fn load() -> Option<Self> {
        let path = Self::path();
        serde_json::from_str(&fs::read_to_string(&path).ok()?).ok()
    }

    /// 保存选择器到文件
    /// Save selector overrides to the user's local data directory.
    pub fn save(&self) -> BResult<()> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(BossError::map_config("创建配置目录失败"))?;
        }
        let json = serde_json::to_string_pretty(self)
            .map_err(BossError::map_config("序列化选择器失败"))?;
        fs::write(&path, json)
            .map_err(BossError::map_config("写入选择器失败"))?;
        log::info!("选择器已保存到: {:?}", path);
        Ok(())
    }

    /// Return the selector configuration path.
    fn path() -> PathBuf {
        dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("boss_auto")
            .join("selectors.json")
    }

    /// 尝试用多个选择器找元素，返回第一个成功的
    /// Return the first DOM element found by any selector in the provided list.
    pub(super) fn find_first(page: &ChromiumPage, selectors: &[String]) -> Option<rust_drission::Element> {
        for sel in selectors {
            if let Ok(Some(el)) = page.ele(sel) {
                return Some(el);
            }
        }
        None
    }
}

