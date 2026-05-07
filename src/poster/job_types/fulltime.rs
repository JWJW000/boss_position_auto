use super::*;

impl<'a> Poster<'a> {
    /// 社招全职岗位发布流程
    /// 从上到下按照页面顺序填写所有字段
    pub(in crate::poster) fn fill_full_time_requirements(
        &mut self,
        job: &JobRecord,
        _kind: RecruitmentKind,
    ) -> BResult<()> {
        log::info!("  [开始] 社招全职岗位要求填写");

        // ==================== 第一步：经验要求 ====================
        log::info!("  [步骤1] 填写经验要求");
        self.fill_full_time_experience(job)?;

        // ==================== 第二步：学历要求 ====================
        log::info!("  [步骤2] 填写学历要求");
        self.fill_full_time_education(job)?;

        // ==================== 第三步：薪资范围 ====================
        log::info!("  [步骤3] 填写薪资范围");
        self.fill_full_time_salary(job)?;

      
        // ==================== 第四步：职位关键词 ====================
        log::info!("  [步骤4] 填写职位关键词");
        self.fill_tags(job)?;
          // ==================== 第五步：是否驻外 ====================
        log::info!("  [步骤5] 填写是否驻外");
        self.fill_full_time_overseas(job)?;

        // ==================== 第六步：工作地址 ====================
        log::info!("  [步骤6] 填写工作地址");
        self.fill_city()?;

        log::info!("  [完成] 社招全职岗位要求填写完成");
        Ok(())
    }

    /// 社招全职 - 填写经验要求
    fn fill_full_time_experience(&mut self, job: &JobRecord) -> BResult<()> {
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

    /// 社招全职 - 填写学历要求
    fn fill_full_time_education(&mut self, job: &JobRecord) -> BResult<()> {
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

    /// 社招全职 - 填写薪资范围
    fn fill_full_time_salary(&mut self, job: &JobRecord) -> BResult<()> {
    if !Self::has_excel_value(&job.薪资低) {
        log::warn!("  [跳过] 薪资字段为空");
        return Ok(());
    }

    log::info!("  [开始] 填写薪资");

    // 1. 查找"薪资范围"所在的表单行
    let requirements_form_rows = self
        .page
        .eles(".requirements-info-content .publish-edit-form-row")
        .map_err(BossError::map_element("未找到表单行"))?;

    for form_row in requirements_form_rows {
        // 2. 读取表单行的标题（如"薪资范围"）
        let label_el = form_row
            .element(".publish-title")
            .map_err(BossError::map_element("未找到标题元素"))?; 

        let label_el = label_el.ok_or_else(|| BossError::element("标题元素为空"))?;
        let label_text = label_el
            .text_content()
            .map_err(BossError::map_element("无法读取标题文本"))?
            .trim()
            .to_string();

        // 3. 判断是否是"薪资范围"字段
        if label_text != "薪资范围" {
            continue;
        }

        log::info!("  [找到] 薪资范围字段");

        // 4. 点击最低月薪下拉框
        let min_salary_select = form_row
            .element(".ui-select-selection")
            .map_err(BossError::map_element("未找到最低月薪下拉框"))?
            .ok_or_else(|| BossError::element("下拉框未找到"))?;
        min_salary_select.click().map_err(BossError::map_element("点击最低月薪下拉框失败"))?;

        sleep_random_ms(300, 500);

        // 5. 选择最低月薪
        let min_salary_items = self
            .page
            .eles(".ui-select-item")
            .map_err(BossError::map_element("未找到薪资选项"))?;
        let target_min_salary = job.薪资低.trim();
        let mut selected_min_salary = false;

        // 选择项从后往前选择
        for item in min_salary_items.iter().rev() {
            let item_text = item.text_content().map_err(BossError::map_element("无法读取选项文本"))?;
            if item_text.trim() == target_min_salary {
                item.click().map_err(BossError::map_element("点击最低月薪失败"))?;
                selected_min_salary = true;
                log::info!("  [√] 选择最低月薪: {}", target_min_salary);
                break;
            }
        }

        if !selected_min_salary {
            return Err(BossError::element(format!("未找到最低月薪: {}", target_min_salary)));
        }

        // 6. 点击最高月薪下拉框
        let max_salary_select = form_row
            .element(".ui-select-selection")
            .map_err(BossError::map_element("未找到最高月薪下拉框"))?
            .ok_or_else(|| BossError::element("最高月薪下拉框未找到"))?;
        max_salary_select.click().map_err(BossError::map_element("点击最高月薪下拉框失败"))?;

        sleep_random_ms(300, 500);

        // 7. 选择最高月薪
        let max_salary_items = self
            .page
            .eles(".ui-select-item")
            .map_err(BossError::map_element("未找到薪资选项"))?;
        let target_max_salary = job.薪资高.trim();
        let mut selected_max_salary = false;

        // 选择项从后往前选择
        for item in max_salary_items.iter().rev() {
            let item_text = item.text_content().map_err(BossError::map_element("无法读取选项文本"))?;
            if item_text.trim() == target_max_salary {
                item.click().map_err(BossError::map_element("点击最高月薪失败"))?;
                selected_max_salary = true;
                log::info!("  [√] 选择最高月薪: {}", target_max_salary);
                break;
            }
        }

        if !selected_max_salary {
            return Err(BossError::element(format!("未找到最高月薪: {}", target_max_salary)));
        }

        // 8. 点击薪资单位下拉框
        let salary_unit_select = form_row
            .element(".ui-select-selection")
            .map_err(BossError::map_element("未找到薪资单位下拉框"))?
            .ok_or_else(|| BossError::element("薪资单位下拉框未找到"))?;
        salary_unit_select.click().map_err(BossError::map_element("点击薪资单位下拉框失败"))?;

        sleep_random_ms(300, 500);

        // 9. 选择薪资单位
        let salary_unit_items = self
            .page
            .eles(".ui-select-item")
            .map_err(BossError::map_element("未找到薪资单位选项"))?;
        let target_salary_unit = job.薪资单位.trim();
        let mut selected_salary_unit = false;

        // 选择项从后往前选择
        for item in salary_unit_items.iter().rev() {
            let item_text = item.text_content().map_err(BossError::map_element("无法读取选项文本"))?;
            if item_text.trim() == target_salary_unit {
                item.click().map_err(BossError::map_element("点击薪资单位失败"))?;
                selected_salary_unit = true;
                log::info!("  [√] 选择薪资单位: {}", target_salary_unit);
                break;
            }
        }

        if !selected_salary_unit {
            return Err(BossError::element(format!("未找到薪资单位: {}", target_salary_unit)));
        }

        sleep_random_ms(400, 500);
        return Ok(());
    }

    Err(BossError::element("未找到薪资范围字段"))
}

    /// 社招全职 - 填写是否驻外
    fn fill_full_time_overseas(&mut self, job: &JobRecord) -> BResult<()> {
    // 检查 Excel 中是否有驻外值
    if !Self::has_excel_value(&job.是否驻外) {
        log::warn!("  [跳过] 是否驻外字段为空");
        return Ok(());
    }

    log::info!("  [开始] 填写是否驻外");

    // 1. 查找"是否驻外"所在的表单行
    let requirements_form_rows = match self
        .page
        .eles(".publish-edit-form-row.overseas-entry-container") {
        Ok(rows) => rows,
        Err(_) => {
            log::warn!("  [跳过] 未找到是否驻外表单行，该字段可能不存在");
            return Ok(());
        }
    };

    for form_row in requirements_form_rows {
        // 2. 读取表单行的标题（如"是否驻外"）
        let label_el = match form_row.element(".publish-title") {
            Ok(Some(el)) => el,
            _ => continue,
        };

        let label_text = match label_el.text_content() {
            Ok(text) => text.trim().to_string(),
            Err(_) => continue,
        };

        // 3. 判断是否是"是否驻外"字段
        if label_text != "是否驻外" {
            continue;
        }

        log::info!("  [找到] 是否驻外字段");

        // 4. 查找所有选项
        let options = match form_row.elements(".entry-content .chose-item") {
            Ok(opts) => opts,
            Err(_) => {
                log::warn!("  [跳过] 未找到是否驻外选项");
                return Ok(());
            }
        };

        // 5. 获取目标选项
        let target_value = job.是否驻外.trim(); // 获取Excel中的值

        let mut selected = false;

        for option in options {
            if let Ok(option_text) = option.text_content() {
                if option_text.trim() == target_value {
                    if option.click().is_ok() {
                        selected = true;
                        log::info!("  [√] 选择是否驻外: {}", target_value);
                        break;
                    }
                }
            }
        }

        if !selected {
            log::warn!("  [跳过] 未找到是否驻外选项: {}", target_value);
        }

        sleep_random_ms(400, 500);
        return Ok(());
    }

    log::warn!("  [跳过] 未找到是否驻外字段，该字段可能不存在");
    Ok(())
}

    /// 社招全职 - 填写职位关键词
    fn fill_full_time_tags(&mut self, job: &JobRecord) -> BResult<()> {
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
}
