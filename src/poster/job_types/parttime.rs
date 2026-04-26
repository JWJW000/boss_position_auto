use super::*;

impl<'a> Poster<'a> {
    /// 兼职招聘岗位发布流程
    /// 从上到下按照页面顺序填写所有字段
    pub(in crate::poster) fn fill_part_time_requirements(
        &mut self,
        job: &JobRecord,
        kind: RecruitmentKind,
    ) -> BResult<()> {
        log::info!("  [开始] 兼职招聘岗位要求填写");

        // ==================== 第一步：经验要求 ====================
        log::info!("  [步骤1] 填写经验要求");
        self.fill_part_time_experience(job)?;

        // ==================== 第二步：学历要求 ====================
        log::info!("  [步骤2] 填写学历要求");
        self.fill_part_time_education(job)?;

        // ==================== 第三步：结算方式 ====================
        log::info!("  [步骤3] 填写结算方式");
        self.fill_part_time_settlement(job)?;

        // ==================== 第四步：薪资范围 ====================
        log::info!("  [步骤4] 填写薪资范围");
        self.fill_part_time_salary(job, kind)?;

        // ==================== 第五步：职位关键词 ====================
        log::info!("  [步骤5] 填写职位关键词");
        self.fill_part_time_tags(job)?;

        // ==================== 第六步：招聘截止时间 ====================
        log::info!("  [步骤6] 填写招聘截止时间");
        self.fill_part_time_deadline(job)?;

        log::info!("  [完成] 兼职招聘岗位要求填写完成");
        Ok(())
    }

    /// 兼职招聘 - 填写经验要求
    fn fill_part_time_experience(&mut self, job: &JobRecord) -> BResult<()> {
        // 检查 Excel 中是否有经验值
        if !Self::has_excel_value(&job.经验) {
            log::warn!("  [跳过] 经验字段为空");
            return Ok(());
        }

        // 1. 查找"经验"所在的表单行
        let requirements_form_rows = self
            .page
            .eles(".requirements-info-content .publish-edit-form-row")
            .map_err(BossError::map_element("未找到表单行"))?;

        for form_row in requirements_form_rows {
            // 2. 读取表单行的标题（如"经验"）
            let label_el = form_row
                .element(".publish-title")
                .map_err(BossError::map_element("未找到标题元素"))?;

            let label_el = label_el.ok_or_else(|| BossError::element("标题元素为空"))?;

            let label_text = label_el
                .text_content()
                .map_err(BossError::map_element("无法读取标题文本"))?
                .trim()
                .to_string();

            // 3. 判断是否是"经验"字段
            if label_text != "经验" {
                continue;
            }

            log::info!("  [找到] 经验字段");

            // 4. 点击下拉框
            let select_inner = form_row
                .element(".ui-select-inner")
                .map_err(BossError::map_element("未找到下拉框"))?;

            let select_inner = select_inner.ok_or_else(|| BossError::element("下拉框为空"))?;

            select_inner
                .click()
                .map_err(BossError::map_element("点击下拉框失败"))?;

            sleep_random_ms(300, 500);

            // 5. 查找所有下拉选项
            let select_items = self
                .page
                .eles(".ui-select-item")
                .map_err(BossError::map_element("未找到下拉选项"))?;

            // 6. 遍历选项，找到匹配的并点击
            let target_value = job.经验.trim();
            let mut selected = false;

            for item in select_items {
                let item_text = item
                    .text_content()
                    .map_err(BossError::map_element("无法读取选项文本"))?
                    .trim()
                    .to_string();

                if item_text == target_value {
                    item.click()
                        .map_err(BossError::map_element("点击选项失败"))?;
                    selected = true;
                    log::info!("  [√] 经验: {}", target_value);
                    break;
                }
            }

            if !selected {
                return Err(BossError::element(format!("未找到经验选项: {}", target_value)));
            }

            sleep_random_ms(400, 500);
            return Ok(());
        }

        Err(BossError::element("未找到经验字段"))
    }

    /// 兼职招聘 - 填写学历要求
    fn fill_part_time_education(&mut self, job: &JobRecord) -> BResult<()> {
        // 检查 Excel 中是否有学历值
        if !Self::has_excel_value(&job.学历) {
            log::warn!("  [跳过] 学历字段为空");
            return Ok(());
        }

        // 1. 查找"学历"所在的表单行
        let requirements_form_rows = self
            .page
            .eles(".requirements-info-content .publish-edit-form-row")
            .map_err(BossError::map_element("未找到表单行"))?;

        for form_row in requirements_form_rows {
            // 2. 读取表单行的标题（如"学历"）
            let label_el = form_row
                .element(".publish-title")
                .map_err(BossError::map_element("未找到标题元素"))?;

            let label_el = label_el.ok_or_else(|| BossError::element("标题元素为空"))?;

            let label_text = label_el
                .text_content()
                .map_err(BossError::map_element("无法读取标题文本"))?
                .trim()
                .to_string();

            // 3. 判断是否是"学历"字段
            if label_text != "学历" {
                continue;
            }

            log::info!("  [找到] 学历字段");

            // 4. 点击下拉框
            let select_inner = form_row
                .element(".ui-select-inner")
                .map_err(BossError::map_element("未找到下拉框"))?;

            let select_inner = select_inner.ok_or_else(|| BossError::element("下拉框为空"))?;

            select_inner
                .click()
                .map_err(BossError::map_element("点击下拉框失败"))?;

            sleep_random_ms(300, 500);

            // 5. 查找所有下拉选项
            let select_items = self
                .page
                .eles(".ui-select-item")
                .map_err(BossError::map_element("未找到下拉选项"))?;

            // 6. 遍历选项，找到匹配的并点击
            let target_value = job.学历.trim();
            let mut selected = false;

            for item in select_items {
                let item_text = item
                    .text_content()
                    .map_err(BossError::map_element("无法读取选项文本"))?
                    .trim()
                    .to_string();

                if item_text == target_value {
                    item.click()
                        .map_err(BossError::map_element("点击选项失败"))?;
                    selected = true;
                    log::info!("  [√] 学历: {}", target_value);
                    break;
                }
            }

            if !selected {
                return Err(BossError::element(format!("未找到学历选项: {}", target_value)));
            }

            sleep_random_ms(400, 500);
            return Ok(());
        }

        Err(BossError::element("未找到学历字段"))
    }

    /// 兼职招聘 - 填写结算方式
    fn fill_part_time_settlement(&mut self, job: &JobRecord) -> BResult<()> {
        // 检查 Excel 中是否有结算方式值
        if !Self::has_excel_value(&job.结算方式) {
            log::warn!("  [跳过] 结算方式字段为空");
            return Ok(());
        }

        log::info!("  [开始] 填写结算方式");

        // 1. 查找"结算方式"所在的表单行
        let requirements_form_rows = self
            .page
            .eles(".requirements-info-content .publish-edit-form-row")
            .map_err(BossError::map_element("未找到表单行"))?;

        for form_row in requirements_form_rows {
            // 2. 读取表单行的标题（如"结算方式"）
            let label_el = form_row
                .element(".publish-title")
                .map_err(BossError::map_element("未找到标题元素"))?;

            let label_el = label_el.ok_or_else(|| BossError::element("标题元素为空"))?;

            let label_text = label_el
                .text_content()
                .map_err(BossError::map_element("无法读取标题文本"))?
                .trim()
                .to_string();

            // 3. 判断是否是"结算方式"字段
            if label_text != "结算方式" {
                continue;
            }

            log::info!("  [找到] 结算方式字段");

            // 4. 点击下拉框
            let select_inner = form_row
                .element(".ui-select-inner")
                .map_err(BossError::map_element("未找到下拉框"))?;

            let select_inner = select_inner.ok_or_else(|| BossError::element("下拉框为空"))?;

            select_inner
                .click()
                .map_err(BossError::map_element("点击下拉框失败"))?;

            sleep_random_ms(300, 500);

            // 5. 查找所有下拉选项（日结、周结、月结等）
            let select_items = self
                .page
                .eles(".ui-select-item")
                .map_err(BossError::map_element("未找到下拉选项"))?;

            // 6. 遍历选项，找到匹配的并点击
            let target_value = job.结算方式.trim();
            let mut selected = false;

            for item in select_items {
                let item_text = item
                    .text_content()
                    .map_err(BossError::map_element("无法读取选项文本"))?
                    .trim()
                    .to_string();

                // 支持模糊匹配：如"日结"可以匹配"按日结算"
                if item_text.contains(target_value) || target_value.contains(&item_text) {
                    item.click()
                        .map_err(BossError::map_element("点击选项失败"))?;
                    selected = true;
                    log::info!("  [√] 结算方式: {}", target_value);
                    break;
                }
            }

            if !selected {
                return Err(BossError::element(format!("未找到结算方式选项: {}", target_value)));
            }

            sleep_random_ms(400, 500);
            return Ok(());
        }

        Err(BossError::element("未找到结算方式字段"))
    }

    /// 兼职招聘 - 填写薪资范围
    fn fill_part_time_salary(&mut self, job: &JobRecord, _kind: RecruitmentKind) -> BResult<()> {
        // 检查 Excel 中是否有薪资值
        if !Self::has_excel_value(&job.薪资低) {
            log::warn!("  [跳过] 薪资字段为空");
            return Ok(());
        }

        log::info!("  [开始] 填写薪资范围（兼职）");

        // 1. 查找薪资范围容器
        let salary_container = self
            .page
            .ele(".salary-range-container")
            .or_else(|_| self.page.ele(".part-time-salary-container"))
            .map_err(BossError::map_element("未找到薪资范围容器"))?;

        if salary_container.is_none() {
            log::warn!("  [跳过] 未找到薪资范围容器");
            return Ok(());
        }

        // 2. 查找薪资单位选择器（元/小时、元/天、元/月等）
        if Self::has_excel_value(&job.薪资单位) {
            log::info!("  [开始] 选择薪资单位");

            let salary_unit_selectors = vec![
                "css:.salary-unit-select".to_string(),
                "xpath://div[contains(@class,'salary-unit')]//div[contains(@class,'ui-select')]".to_string(),
            ];

            let salary_unit_select = SelectorMap::find_first(self.page, &salary_unit_selectors);
            if let Some(unit_select) = salary_unit_select {
                // 3. 点击薪资单位下拉框
                unit_select
                    .click()
                    .map_err(BossError::map_post("点击薪资单位下拉框失败"))?;
                sleep_random_ms(300, 500);

                // 4. 查找并选择匹配的单位
                let unit_items = self
                    .page
                    .eles(".ui-select-item")
                    .map_err(BossError::map_element("未找到薪资单位选项"))?;

                let target_unit = job.薪资单位.trim();
                let mut unit_selected = false;

                for item in unit_items {
                    let item_text = item
                        .text_content()
                        .map_err(BossError::map_element("无法读取单位文本"))?
                        .trim()
                        .to_string();

                    // 支持模糊匹配：如"小时"可以匹配"元/小时"
                    if item_text.contains(target_unit) || target_unit.contains(&item_text) {
                        item.click()
                            .map_err(BossError::map_element("点击单位选项失败"))?;
                        unit_selected = true;
                        log::info!("  [√] 薪资单位: {}", target_unit);
                        break;
                    }
                }

                if !unit_selected {
                    log::warn!("  [警告] 未找到薪资单位选项: {}", target_unit);
                }

                sleep_random_ms(400, 500);
            }
        }

        // 5. 查找薪资下限输入框
        let salary_low_input = self
            .page
            .ele("css:input[placeholder*='最低']")
            .or_else(|_| self.page.ele("css:input[name='salaryLow']"))
            .map_err(BossError::map_element("未找到薪资下限输入框"))?;

        if let Some(input) = salary_low_input {
            // 6. 点击并输入薪资下限
            input
                .click()
                .map_err(BossError::map_post("点击薪资下限输入框失败"))?;
            sleep_random_ms(200, 300);

            input
                .input(&job.薪资低)
                .map_err(BossError::map_post("输入薪资下限失败"))?;
            sleep_random_ms(300, 500);

            log::info!("  [√] 薪资下限: {}", job.薪资低);
        }

        // 7. 查找薪资上限输入框
        if Self::has_excel_value(&job.薪资高) {
            let salary_high_input = self
                .page
                .ele("css:input[placeholder*='最高']")
                .or_else(|_| self.page.ele("css:input[name='salaryHigh']"))
                .map_err(BossError::map_element("未找到薪资上限输入框"))?;

            if let Some(input) = salary_high_input {
                // 8. 点击并输入薪资上限
                input
                    .click()
                    .map_err(BossError::map_post("点击薪资上限输入框失败"))?;
                sleep_random_ms(200, 300);

                input
                    .input(&job.薪资高)
                    .map_err(BossError::map_post("输入薪资上限失败"))?;
                sleep_random_ms(300, 500);

                log::info!("  [√] 薪资上限: {}", job.薪资高);
            }
        }

        log::info!("  [完成] 薪资范围填写完成");
        Ok(())
    }

    /// 兼职招聘 - 填写职位关键词
    fn fill_part_time_tags(&mut self, job: &JobRecord) -> BResult<()> {
        // 检查 Excel 中是否有关键词
        if !Self::has_excel_value(&job.关键词) {
            log::warn!("  [跳过] 职位关键词字段为空");
            return Ok(());
        }

        log::info!("  [开始] 填写职位关键词");

        // 1. 将关键词按空格或逗号分割
        let keywords: Vec<&str> = job.关键词
            .split(|c: char| c.is_whitespace() || c == ',' || c == '，')
            .filter(|s| !s.trim().is_empty())
            .collect();

        if keywords.is_empty() {
            log::warn!("  [跳过] 关键词分割后为空");
            return Ok(());
        }

        // 2. 查找关键词输入框
        let tag_input = SelectorMap::find_first(self.page, &self.selectors.tags);
        if tag_input.is_none() {
            log::warn!("  [跳过] 未找到职位关键词输入框");
            return Ok(());
        }

        let tag_input = tag_input.unwrap();

        // 3. 逐个输入关键词
        for (i, keyword) in keywords.iter().enumerate() {
            let keyword = keyword.trim();
            if keyword.is_empty() {
                continue;
            }

            log::info!("  [输入] 关键词 {}: {}", i + 1, keyword);

            // 4. 点击输入框
            tag_input
                .click()
                .map_err(BossError::map_post("点击关键词输入框失败"))?;
            sleep_random_ms(200, 300);

            // 5. 输入关键词
            tag_input
                .input(keyword)
                .map_err(BossError::map_post("输入关键词失败"))?;
            sleep_random_ms(300, 500);

            // 6. 按回车确认
            tag_input
                .input("\n")
                .map_err(BossError::map_post("确认关键词失败"))?;
            sleep_random_ms(400, 600);
        }

        log::info!("  [√] 职位关键词: 已输入 {} 个", keywords.len());
        Ok(())
    }

    /// 兼职招聘 - 填写招聘截止时间
    fn fill_part_time_deadline(&mut self, job: &JobRecord) -> BResult<()> {
        // 检查 Excel 中是否有截止日期值
        if !Self::has_excel_value(&job.截止日期) {
            log::warn!("  [跳过] 截止日期字段为空");
            return Ok(());
        }

        log::info!("  [开始] 填写招聘截止时间");

        // 1. 查找截止时间输入框
        let deadline_input = SelectorMap::find_first(self.page, &self.selectors.deadline);
        if deadline_input.is_none() {
            log::warn!("  [跳过] 未找到截止时间输入框");
            return Ok(());
        }

        let deadline_input = deadline_input.unwrap();

        // 2. 点击输入框
        deadline_input
            .click()
            .map_err(BossError::map_post("点击截止时间输入框失败"))?;
        sleep_random_ms(300, 500);

        // 3. 使用 JavaScript 设置日期值（更可靠）
        let deadline_json = serde_json::to_string(&job.截止日期)
            .map_err(BossError::map_config("截止日期序列化失败"))?;

        let script = format!(
            "this.value = {}; this.dispatchEvent(new Event('input', {{bubbles:true}})); this.dispatchEvent(new Event('change', {{bubbles:true}})); true;",
            deadline_json
        );

        if let Err(js_err) = deadline_input.run_js(&script) {
            log::warn!("  [WARN] 截止日期JS填写失败，尝试直接输入: {}", js_err);

            // 4. 如果 JS 失败，尝试直接输入
            deadline_input
                .input(&job.截止日期)
                .map_err(BossError::map_post("填写截止日期失败"))?;
        }

        sleep_random_ms(300, 500);
        log::info!("  [√] 招聘截止时间: {}", job.截止日期);
        Ok(())
    }
}
