use super::*;

impl<'a> Poster<'a> {
    /// 实习生招聘岗位发布流程
    /// 从上到下按照页面顺序填写所有字段
    pub(in crate::poster) fn fill_intern_requirements(
        &mut self,
        job: &JobRecord,
        kind: RecruitmentKind,
    ) -> BResult<()> {
        log::info!("  [开始] 实习生招聘岗位要求填写");

        // ==================== 第一步：经验要求 ====================
        log::info!("  [步骤1] 填写经验要求");
        self.fill_intern_experience(job)?;

        // ==================== 第二步：学历要求 ====================
        log::info!("  [步骤2] 填写学历要求");
        self.fill_intern_education(job)?;

        // ==================== 第三步：薪资范围 ====================
        log::info!("  [步骤3] 填写薪资范围");
        self.fill_intern_salary(job, kind)?;

        // ==================== 第四步：职位关键词 ====================
        log::info!("  [步骤4] 填写职位关键词");
        self.fill_tags(job)?;

        // ==================== 第五步：工作地址 ====================
        log::info!("  [步骤5] 填写工作地址");
        self.fill_city()?;

        // ==================== 第六步：实习月数 ====================
        log::info!("  [步骤6] 填写实习月数");
        self.fill_intern_months(job)?;

        // ==================== 第七步：到岗天数 ====================
        log::info!("  [步骤7] 填写到岗天数");
        self.fill_intern_days(job)?;

        log::info!("  [完成] 实习生招聘岗位要求填写完成");
        Ok(())
    }

    /// 实习生招聘 - 填写经验要求
    fn fill_intern_experience(&mut self, job: &JobRecord) -> BResult<()> {
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

    /// 实习生招聘 - 填写学历要求
    fn fill_intern_education(&mut self, job: &JobRecord) -> BResult<()> {
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

    /// 实习生招聘 - 填写薪资范围（日薪）
    fn fill_intern_salary(&mut self, job: &JobRecord, _kind: RecruitmentKind) -> BResult<()> {
        // 检查 Excel 中是否有薪资值
         let low = &job.薪资低;
         let high = &job.薪资高;

        if Self::has_excel_value(&low) {
            let start = self.page
                .ele(".margin-r-15 .ui-select-selection")
                .map_err(BossError::map_element("未找到起始薪资下拉框按钮"))?
                .ok_or_else(|| BossError::element("下拉框不存在"))?;
            start.click()
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
            let end = self.page
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

    /// 实习生招聘 - 填写职位关键词
    fn fill_intern_tags(&mut self, job: &JobRecord) -> BResult<()> {
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

    /// 实习生招聘 - 填写工作地址
    fn fill_intern_city(&mut self, job: &JobRecord) -> BResult<()> {
        // 检查 Excel 中是否有城市值
        if !Self::has_excel_value(&job.城市) {
            log::warn!("  [跳过] 城市字段为空");
            return Ok(());
        }

        log::info!("  [开始] 填写工作地址");

        // 1. 查找城市输入框
        let city_input = SelectorMap::find_first(self.page, &self.selectors.city);
        if city_input.is_none() {
            log::warn!("  [跳过] 未找到城市输入框");
            return Ok(());
        }

        let city_input = city_input.unwrap();

        // 2. 点击城市输入框
        city_input
            .click()
            .map_err(BossError::map_post("点击城市输入框失败"))?;
        sleep_random_ms(300, 500);

        // 3. 输入城市名称
        city_input
            .input(&job.城市)
            .map_err(BossError::map_post("输入城市失败"))?;
        sleep_random_ms(500, 800);

        // 4. 等待下拉列表出现并选择第一个匹配项
        let city_items = self
            .page
            .eles(".city-suggest-list .city-item")
            .or_else(|_| self.page.eles(".suggest-list .suggest-item"));

        if let Ok(items) = city_items {
            if !items.is_empty() {
                // 点击第一个匹配的城市
                items[0]
                    .click()
                    .map_err(BossError::map_post("点击城市选项失败"))?;
                sleep_random_ms(300, 500);
                log::info!("  [√] 工作地址: {}", job.城市);
            } else {
                log::warn!("  [警告] 未找到城市匹配项");
            }
        } else {
            log::warn!("  [警告] 未找到城市下拉列表");
        }

        Ok(())
    }

    fn fill_intern_months(&mut self, job: &JobRecord) -> BResult<()> {
        if !Self::has_excel_value(&job.最少实习月数) {
            log::warn!("skip intern months field");
            return Ok(());
        }

        log::info!("filling intern months");

        let rows = self.page.eles(".publish-edit-form-row")?;

        let mut target_row = None;
        for row in rows {
            if let Ok(Some(title_el)) = row.element(".publish-title") {
                if let Ok(text) = title_el.text() {
                    if text.contains("实习要求") {
                        target_row = Some(row);
                        break;
                    }
                }
            }
        }

        let row = target_row.ok_or_else(|| BossError::element("intern requirement row not found"))?;

        let month_select = row
            .element(".margin-r-15 .ui-select-selection")?
            .ok_or_else(|| BossError::element("month select not found"))?;

        month_select.click().map_err(BossError::map_element("click month select failed"))?;
        sleep_random_ms(300, 500);

        let target_text = &job.最少实习月数;
        let items = self.page.eles(".ui-select-item")?;
        let mut selected = false;

        for item in items {
            let text = item.text()?;
            if text.trim() == target_text {
                item.click().map_err(BossError::map_element("click month option failed"))?;
                selected = true;
                log::info!("selected intern months: {}", target_text);
                break;
            }
        }

        if !selected {
            return Err(BossError::element(format!("month option not found: {}", target_text)));
        }

        sleep_random_ms(400, 500);
        Ok(())
    }

    fn fill_intern_days(&mut self, job: &JobRecord) -> BResult<()> {
        if !Self::has_excel_value(&job.最少周到岗天数) {
            log::warn!("skip intern days field");
            return Ok(());
        }

        log::info!("filling intern days");

        let rows = self.page.eles(".publish-edit-form-row")?;

        let mut target_row = None;
        for row in rows {
            if let Ok(Some(title_el)) = row.element(".publish-title") {
                if let Ok(text) = title_el.text() {
                    if text.contains("实习要求") {
                        target_row = Some(row);
                        break;
                    }
                }
            }
        }

        let row = target_row.ok_or_else(|| BossError::element("intern requirement row not found"))?;

        let days_select = row
            .element(".ui-select-single .ui-select-selection")?
            .ok_or_else(|| BossError::element("days select not found"))?;

        days_select.click().map_err(BossError::map_element("click days select failed"))?;
        sleep_random_ms(300, 500);

        let target_text = &job.最少周到岗天数;
        let items = self.page.eles(".ui-select-item")?;
        let mut selected = false;

        for item in items {
            let text = item.text()?;
            if text.trim() == target_text {
                item.click().map_err(BossError::map_element("click days option failed"))?;
                selected = true;
                log::info!("selected intern days: {}", target_text);
                break;
            }
        }

        if !selected {
            return Err(BossError::element(format!("days option not found: {}", target_text)));
        }

        sleep_random_ms(400, 500);
        Ok(())
    }
}
