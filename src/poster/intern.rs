use super::*;

impl<'a> Poster<'a> {
    /// Fill the minimum internship duration dropdown for internship postings.
    pub(super) fn fill_intern_months(&mut self, job: &JobRecord) -> BResult<()> {
        let raw = job.实习时长.trim();
        if !Self::has_excel_value(raw) {
            return Ok(());
        }

        let target_text = Self::intern_month_text(raw);
        if self.choose_row_select_option("实习要求", 0, &target_text)? {
            log::info!("  [√] 实习月数: {}", target_text);
            return Ok(());
        }

        Err(BossError::element(format!(
            "实习月数选项未找到: {}",
            target_text
        )))
    }

    /// Fill the minimum weekly attendance-days dropdown for internship postings.
    pub(super) fn fill_work_days(&mut self, job: &JobRecord) -> BResult<()> {
        let raw = job.其他说明.trim();
        if !Self::has_excel_value(raw) {
            return Ok(());
        }

        let target_text = Self::work_day_text(raw);
        if self.choose_row_select_option("实习要求", 1, &target_text)? {
            log::info!("  [√] 到岗天数: {}", target_text);
            return Ok(());
        }

        Err(BossError::element(format!(
            "周到岗天数选项未找到: {}",
            target_text
        )))
    }

    /// Normalize Excel internship month values to BOSS option labels.
    fn intern_month_text(raw: &str) -> String {
        let num = raw.chars().filter(|c| c.is_ascii_digit()).collect::<String>();
        if raw.contains('月') || num.is_empty() {
            raw.to_string()
        } else {
            format!("{}个月", num)
        }
    }

    /// Normalize Excel weekly work-day values to BOSS option labels.
    fn work_day_text(raw: &str) -> String {
        let num = raw.chars().filter(|c| c.is_ascii_digit()).collect::<String>();
        if raw.contains('天') || num.is_empty() {
            raw.to_string()
        } else {
            format!("{}天", num)
        }
    }
}
