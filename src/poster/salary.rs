use super::*;

impl<'a> Poster<'a> {
    /// Fill salary controls according to the selected recruitment type.
    /// 这个方法仅用于向后兼容，实际的薪资填写逻辑已经在各个独立的招聘类型文件中实现
    pub(super) fn fill_salary(&mut self, job: &JobRecord, kind: RecruitmentKind) -> BResult<()> {
        match kind {
            RecruitmentKind::FullTime | RecruitmentKind::Campus => self.fill_month_salary(job),
            RecruitmentKind::Intern => {
                // 实习生薪资填写已在 internship.rs 中的 fill_intern_salary 实现
                // 这里保留空实现以避免编译错误
                Ok(())
            }
            RecruitmentKind::PartTime => {
                // 兼职薪资填写已在 parttime.rs 中的 fill_part_time_salary 实现
                // 这里保留空实现以避免编译错误
                Ok(())
            }
        }
    }

    /// Fill the full-time or campus minimum monthly salary dropdown.
    fn fill_month_salary(&mut self, job: &JobRecord) -> BResult<()> {
        let low = Self::salary_number(&job.薪资低);
        let high = Self::salary_number(&job.薪资高);

        if Self::has_excel_value(&low) {
            let start = self
                .page
                .ele(".margin-r-15 .ui-select-selection")
                .map_err(BossError::map_element("未找到起始薪资下拉框按钮"))?
                .ok_or_else(|| BossError::element("下拉框不存在"))?;
            start
                .click()
                .map_err(BossError::map_element("点击起始薪资下拉框失败"))?;

            let items = self.page.eles(".ui-select-item")?;
            for item in items {
                let text = item.text()?;
                if text.trim() == low {
                    item.click()?;
                    break;
                }
            }
        }

        if Self::has_excel_value(&high) {
            let end = self
                .page
                .ele(".salary-select .ui-select-selection")
                .map_err(BossError::map_element("未找到截止薪资下拉框按钮"))?
                .ok_or_else(|| BossError::element("下拉框不存在"))?;
            end.click()
                .map_err(BossError::map_element("点击截止薪资下拉框失败"))?;

            let items = self.page.eles(".ui-select-item")?;
            for item in items {
                let text = item.text()?;
                if text.trim() == high {
                    item.click()?;
                    break;
                }
            }
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
            candidates.extend([
                "月结".to_string(),
                "月结（可预支）".to_string(),
                "月结(可预支)".to_string(),
            ]);
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
}
