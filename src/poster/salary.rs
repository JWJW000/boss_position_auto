use super::*;

impl<'a> Poster<'a> {
    /// Fill salary controls according to the selected recruitment type.
    pub(super) fn fill_salary(&mut self, job: &JobRecord, kind: RecruitmentKind) -> BResult<()> {
        match kind {
            RecruitmentKind::FullTime | RecruitmentKind::Campus => self.fill_month_salary(job),
            RecruitmentKind::Intern => self.fill_intern_salary(job),
            RecruitmentKind::PartTime => self.fill_part_time_salary(job),
        }
    }

    /// Fill the full-time or campus minimum monthly salary dropdown.
    fn fill_month_salary(&mut self, job: &JobRecord) -> BResult<()> {
        let low = Self::salary_number(&job.薪资低);
        if !Self::has_excel_value(&low) {
            return Ok(());
        }
        Self::log_ignored_salary_unit(&job.薪资单位, "社招/校招薪资默认按月");
        self.choose_salary_candidates(0, &Self::monthly_salary_candidates(&low), "最低月薪")?;
        log::info!("  [√] 最低月薪: {}k", low);
        Ok(())
    }

    /// Fill the internship daily salary range dropdowns.
    fn fill_intern_salary(&mut self, job: &JobRecord) -> BResult<()> {
        let low = Self::salary_number(&job.薪资低);
        let high = Self::salary_number(&job.薪资高);
        if Self::has_excel_value(&low) {
            self.choose_salary_candidates(0, &Self::daily_salary_candidates(&low), "实习薪资下限")?;
        }
        if Self::has_excel_value(&high) {
            self.choose_salary_candidates(1, &Self::daily_salary_candidates(&high), "实习薪资上限")?;
        }
        Self::log_ignored_salary_unit(&job.薪资单位, "实习薪资默认按天");
        if Self::has_excel_value(&low) || Self::has_excel_value(&high) {
            log::info!("  [√] 实习薪资: {}-{}元/天", low, high);
        }
        Ok(())
    }

    /// Fill part-time salary range and optional salary unit.
    fn fill_part_time_salary(&mut self, job: &JobRecord) -> BResult<()> {
        let low = Self::salary_number(&job.薪资低);
        let high = Self::salary_number(&job.薪资高);
        if Self::has_excel_value(&low) {
            self.choose_salary_candidates(0, &Self::daily_salary_candidates(&low), "兼职薪资下限")?;
        }
        if Self::has_excel_value(&high) {
            self.choose_salary_candidates(1, &Self::daily_salary_candidates(&high), "兼职薪资上限")?;
        }
        if Self::has_excel_value(&job.薪资单位) {
            self.choose_salary_candidates(2, &[job.薪资单位.trim().to_string()], "兼职薪资单位")?;
        }
        if Self::has_excel_value(&low) || Self::has_excel_value(&high) {
            log::info!("  [√] 兼职薪资: {}-{}{}", low, high, job.薪资单位.trim());
        }
        Ok(())
    }

    /// Fill the part-time settlement dropdown.
    pub(super) fn fill_settlement(&mut self, job: &JobRecord) -> BResult<()> {
        let settlement = job.结算方式.trim();
        if !Self::has_excel_value(settlement) {
            return Ok(());
        }
        let candidates = Self::settlement_candidates(settlement);
        for candidate in &candidates {
            if self.choose_row_select_option("结算方式", 0, candidate)? {
                log::info!("  [√] 结算方式: {}", candidate);
                return Ok(());
            }
        }
        Err(BossError::element(format!(
            "结算方式选项未找到: {}",
            candidates.join("/")
        )))
    }

    /// Try multiple salary option labels against the same dropdown index.
    fn choose_salary_candidates(&mut self, index: usize, candidates: &[String], label: &str) -> BResult<()> {
        for candidate in candidates {
            if self.choose_row_select_option("薪资范围", index, candidate)? {
                return Ok(());
            }
        }
        Err(BossError::element(format!(
            "{}选项未找到: {}",
            label,
            candidates.join("/")
        )))
    }

    /// Remove salary unit suffixes so Excel can contain either `10` or `10k`.
    fn salary_number(value: &str) -> String {
        value
            .replace('K', "")
            .replace('k', "")
            .replace("元/月", "")
            .replace("元/天", "")
            .replace("元/时", "")
            .replace("元/周", "")
            .trim()
            .to_string()
    }

    /// Build possible labels for monthly salary dropdowns.
    fn monthly_salary_candidates(value: &str) -> Vec<String> {
        vec![format!("{}k", value), value.to_string()]
    }

    /// Build candidate labels used by intern/part-time daily salary dropdowns.
    fn daily_salary_candidates(value: &str) -> Vec<String> {
        let raw = value.trim();
        if raw.is_empty() {
            return Vec::new();
        }
        let candidates = vec![
            raw.to_string(),
            format!("{}元", raw),
            format!("{}元/天", raw),
            format!("{}元/时", raw),
            format!("{}元/小时", raw),
            format!("{}元/月", raw),
            format!("{}k", raw),
        ];
        Self::dedup_candidates(candidates)
    }

    /// Build candidate labels for settlement dropdown values.
    fn settlement_candidates(value: &str) -> Vec<String> {
        let raw = value.trim();
        let clean = Self::clean_text(raw);
        let mut candidates = vec![raw.to_string(), clean.clone()];
        if clean.contains("日结") {
            candidates.extend([
                "日结".to_string(),
                "次结".to_string(),
                "日结/次结".to_string(),
                "日结（可预支）".to_string(),
                "日结(可预支)".to_string(),
            ]);
        }
        if clean.contains("周结") {
            candidates.extend([
                "周结".to_string(),
                "周结（可预支）".to_string(),
                "周结(可预支)".to_string(),
            ]);
        }
        if clean.contains("月结") {
            candidates.extend(["月结".to_string(), "月结（可预支）".to_string(), "月结(可预支)".to_string()]);
        }
        if clean.contains("完工") {
            candidates.extend(["完工结".to_string(), "完工结算".to_string()]);
        }
        Self::dedup_candidates(candidates)
    }

    /// Deduplicate candidate labels by normalized text.
    fn dedup_candidates(values: Vec<String>) -> Vec<String> {
        let mut out = Vec::new();
        for value in values {
            if !Self::has_excel_value(&value) {
                continue;
            }
            let normalized = Self::clean_text(&value);
            if out
                .iter()
                .any(|x: &String| Self::clean_text(x) == normalized)
            {
                continue;
            }
            out.push(value);
        }
        out
    }

    /// Log salary-unit values that are intentionally ignored outside part-time postings.
    fn log_ignored_salary_unit(unit: &str, reason: &str) {
        if Self::has_excel_value(unit) {
            log::info!("  [DEBUG] {}，忽略Excel薪资单位: {}", reason, unit.trim());
        }
    }
}
