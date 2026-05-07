use super::*;

impl<'a> Poster<'a> {
    /// 应届生校园招聘岗位发布流程
    /// 从上到下按照页面顺序填写所有字段
    pub(in crate::poster) fn fill_campus_requirements(
        &mut self,
        job: &JobRecord,
        _kind: RecruitmentKind,
    ) -> BResult<()> {
        log::info!("  [开始] 应届生校园招聘岗位要求填写");

        // ==================== 第一步：经验要求 ====================
        // log::info!("  [步骤1] 填写经验要求");
        // self.fill_campus_experience(job)?;

        // ==================== 第二步：学历要求 ====================
        log::info!("  [步骤2] 填写学历要求");
        self.fill_campus_education(job)?;

        // ==================== 第三步：薪资范围 ====================
        log::info!("  [步骤3] 填写薪资范围");
        self.fill_campus_salary(job)?;

        // ==================== 第四步：职位关键词 ====================
        log::info!("  [步骤4] 填写职位关键词");
        self.fill_tags(job)?;

        // ==================== 第五步：工作地址 ====================
        log::info!("  [步骤5] 填写工作地址");
        self.fill_city()?;

        // ==================== 第六步：毕业时间 ====================
        log::info!("  [步骤6] 填写毕业时间");
        self.fill_campus_graduate_time(job)?;

        // ==================== 第七步：招聘截止时间 ====================
        log::info!("  [步骤7] 填写招聘截止时间");
        self.fill_campus_deadline(job)?;

        log::info!("  [完成] 应届生校园招聘岗位要求填写完成");
        Ok(())
    }

    /// 应届生校园招聘 - 填写经验要求
    fn fill_campus_experience(&mut self, job: &JobRecord) -> BResult<()> {
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

    /// 应届生校园招聘 - 填写学历要求
    fn fill_campus_education(&mut self, job: &JobRecord) -> BResult<()> {
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

    /// 应届生校园招聘 - 填写薪资范围
    fn fill_campus_salary(&mut self, job: &JobRecord) -> BResult<()> {
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

        // 8. 点击薪资单位下拉框（可选字段）
        if !Self::has_excel_value(&job.薪资单位) {
            log::info!("  [跳过] 薪资单位字段为空");
            sleep_random_ms(400, 500);
            return Ok(());
        }

        let salary_unit_select = match form_row.element(".ui-select-selection") {
            Ok(Some(el)) => el,
            _ => {
                log::warn!("  [跳过] 未找到薪资单位下拉框，该字段可能不存在");
                sleep_random_ms(400, 500);
                return Ok(());
            }
        };

        if salary_unit_select.click().is_err() {
            log::warn!("  [跳过] 点击薪资单位下拉框失败");
            sleep_random_ms(400, 500);
            return Ok(());
        }

        sleep_random_ms(300, 500);

        // 9. 选择薪资单位
        let salary_unit_items = match self.page.eles(".ui-select-item") {
            Ok(items) => items,
            Err(_) => {
                log::warn!("  [跳过] 未找到薪资单位选项");
                sleep_random_ms(400, 500);
                return Ok(());
            }
        };

        let target_salary_unit = job.薪资单位.trim();
        let mut selected_salary_unit = false;

        // 选择项从后往前选择
        for item in salary_unit_items.iter().rev() {
            if let Ok(item_text) = item.text_content() {
                if item_text.trim() == target_salary_unit {
                    if item.click().is_ok() {
                        selected_salary_unit = true;
                        log::info!("  [√] 选择薪资单位: {}", target_salary_unit);
                        break;
                    }
                }
            }
        }

        if !selected_salary_unit {
            log::warn!("  [跳过] 未找到薪资单位选项: {}", target_salary_unit);
        }

        sleep_random_ms(400, 500);
        return Ok(());
    }

    Err(BossError::element("未找到薪资范围字段"))
}

    /// 应届生校园招聘 - 填写职位关键词
    fn fill_campus_tags(&mut self, job: &JobRecord) -> BResult<()> {
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

    /// 应届生校园招聘 - 填写工作地址
    fn fill_campus_city(&mut self, job: &JobRecord) -> BResult<()> {
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

    /// 应届生校园招聘 - 填写毕业时间（届别）
    fn fill_campus_graduate_time(&mut self, job: &JobRecord) -> BResult<()> {
        // 检查 Excel 中是否有届别值
       // 获取目标毕业年份（例如："2026"）
    let target_year = &job.届别; // 假设是从 job 中获取目标年份

    // 1. 定位并点击毕业时间下拉框
    let graduation_select = self.page
        .ele(".publish-edit-form-row.graduation-time-wrap .ui-select-selection")?
        .ok_or_else(|| BossError::element("毕业时间下拉按钮不存在"))?;
    graduation_select.click().map_err(BossError::map_element("点击下拉框失败"))?;


    // 3. 查找目标年份并点击
    let items = self.page.eles(".publish-edit-form-row.graduation-time-wrap .ui-select-item")?;

    let mut selected = false;
    for item in items {
        let text = item.text()?;
        if text.trim() == target_year {
            item.click().map_err(BossError::map_element("点击目标毕业年份失败"))?;
            selected = true;
            break;
        }
    }

    if !selected {
        return Err(BossError::element(format!("未找到目标毕业年份: {}", target_year)));
    }

    Ok(())
    }

    /// 应届生校园招聘 - 填写招聘截止时间
    fn fill_campus_deadline(&mut self, job: &JobRecord) -> BResult<()> {
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
