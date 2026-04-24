use super::*;

impl<'a> Poster<'a> {
    /// Fill the job-requirements section based on the selected recruitment type.
    pub(super) fn fill_requirements_by_type(&mut self, job: &JobRecord) -> BResult<()> {
        let kind = RecruitmentKind::parse(&job.招聘类型)?;
        log::info!("  [DEBUG] 职位要求分支: {}", kind.label());

        match kind {
            RecruitmentKind::FullTime => self.fill_full_time_requirements(job, kind),
            RecruitmentKind::Campus => self.fill_campus_requirements(job, kind),
            RecruitmentKind::Intern => self.fill_intern_requirements(job, kind),
            RecruitmentKind::PartTime => self.fill_part_time_requirements(job, kind),
        }?;

        self.run_step("福利", |s| s.fill_benefits(job))?;
        Ok(())
    }

    /// Fill select-row fields with the same row-scanning strategy used by full-time posting.
    fn fill_select_fields_with_full_time_strategy(
        &mut self,
        select_fields: &[(&str, String)],
    ) -> BResult<()> {
        let requirements_form_item_eles = self
            .page
            .eles(".requirements-info-content .publish-edit-form-row")
            .map_err(BossError::map_element("元素未找到"))?;

        for requirements_form_item_ele in requirements_form_item_eles {
            let label = requirements_form_item_ele
                .element(".publish-title")
                .map_err(BossError::map_element("元素未找到"))?;
            let label_text = label
                .ok_or_else(|| BossError::element("元素未找到: .publish-title"))?
                .text_content()
                .map_err(BossError::map_element("元素未找到"))?
                .trim()
                .to_string();
            log::info!("选项：{}", label_text);

            let Some((_, target_option)) =
                select_fields.iter().find(|(label, _)| label_text == *label)
            else {
                continue;
            };

            if !Self::has_excel_value(target_option) {
                continue;
            }

            let select_inner_ele = requirements_form_item_ele
                .element(".ui-select-inner")
                .map_err(BossError::map_element("元素未找到"))?;
            let select_inner_ele = select_inner_ele
                .ok_or_else(|| BossError::element(format!("元素未找到: {} 下拉框", label_text)))?;
            select_inner_ele
                .click()
                .map_err(BossError::map_element("元素未找到"))?;

            sleep_random_ms(300, 500);
            log::info!("目标选项：{}", target_option);

            let select_item_eles = self
                .page
                .eles(".ui-select-item")
                .map_err(BossError::map_element("元素未找到"))?;

            let mut selected = false;
            for select_item_ele in select_item_eles {
                let select_item_label = select_item_ele
                    .text_content()
                    .map_err(BossError::map_element("元素未找到"))?
                    .trim()
                    .to_string();
                if select_item_label == *target_option {
                    select_item_ele
                        .click()
                        .map_err(BossError::map_element("元素未找到"))?;
                    selected = true;
                    break;
                }
            }

            if !selected {
                return Err(BossError::element(format!(
                    "{} 目标选项未找到: {}",
                    label_text, target_option
                )));
            }

            sleep_random_ms(400, 500);
        }

        Ok(())
    }

    /// Fill requirements shown for social full-time postings.
    fn fill_full_time_requirements(
        &mut self,
        job: &JobRecord,
        _kind: RecruitmentKind,
    ) -> BResult<()> {
        let select_fields = vec![
            ("经验", job.经验.clone()),
            ("学历", job.学历.clone()),
            ("薪资范围", job.薪资低.clone()),
        ];
        self.fill_select_fields_with_full_time_strategy(&select_fields)?;
        self.run_step("职位关键词", |s| s.fill_tags(job))?;
        Ok(())
    }

    /// Fill requirements shown for campus postings.
    fn fill_campus_requirements(&mut self, job: &JobRecord, _kind: RecruitmentKind) -> BResult<()> {
        let select_fields = vec![
            ("经验", job.经验.clone()),
            ("学历", job.学历.clone()),
            ("薪资范围", job.薪资低.clone()),
        ];
        self.fill_select_fields_with_full_time_strategy(&select_fields)?;
        self.run_step("职位关键词", |s| s.fill_tags(job))?;
        self.run_step("工作地址", |s| s.fill_city(job))?;

        self.run_step("毕业时间", |s| s.fill_graduate_time(job))?;
        self.run_step("招聘截止时间", |s| s.fill_deadline(job))?;
        Ok(())
    }

    /// Fill requirements shown for internship postings.
    fn fill_intern_requirements(&mut self, job: &JobRecord, kind: RecruitmentKind) -> BResult<()> {
        let select_fields = vec![("经验", job.经验.clone()), ("学历", job.学历.clone())];
        self.fill_select_fields_with_full_time_strategy(&select_fields)?;
        self.run_step("薪资范围", |s| s.fill_salary(job, kind))?;
        self.run_step("职位关键词", |s| s.fill_tags(job))?;
        self.run_step("工作地址", |s| s.fill_city(job))?;
        self.run_step("实习月数", |s| s.fill_intern_months(job))?;
        self.run_step("到岗天数", |s| s.fill_work_days(job))?;
        Ok(())
    }

    /// Fill requirements shown for part-time postings.
    fn fill_part_time_requirements(
        &mut self,
        job: &JobRecord,
        kind: RecruitmentKind,
    ) -> BResult<()> {
        let select_fields = vec![("经验", job.经验.clone()), ("学历", job.学历.clone())];
        self.fill_select_fields_with_full_time_strategy(&select_fields)?;
        self.run_step("结算方式", |s| s.fill_settlement(job))?;
        self.run_step("薪资范围", |s| s.fill_salary(job, kind))?;
        self.run_step("职位关键词", |s| s.fill_tags(job))?;
        self.run_step("招聘截止时间", |s| s.fill_deadline(job))?;
        Ok(())
    }
}
