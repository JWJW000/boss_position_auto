use super::*;

fn parse_date(date_str: &str) -> BResult<(i32, i32, i32)> {
    // 尝试按 '-' 分割
    if date_str.contains('-') {
        let parts: Vec<&str> = date_str.split('-').collect();
        if parts.len() == 3 {
            let year = parts[0]
                .parse::<i32>()
                .map_err(|_| BossError::Config("年份解析失败".to_string()))?;
            let month = parts[1]
                .parse::<i32>()
                .map_err(|_| BossError::Config("月份解析失败".to_string()))?;
            let day = parts[2]
                .parse::<i32>()
                .map_err(|_| BossError::Config("日期解析失败".to_string()))?;
            if (1..=12).contains(&month) && (1..=31).contains(&day) {
                return Ok((year, month, day));
            }
        }
    }
    // 尝试按 '/' 分割
    if date_str.contains('/') {
        let parts: Vec<&str> = date_str.split('/').collect();
        if parts.len() == 3 {
            let year = parts[0]
                .parse::<i32>()
                .map_err(|_| BossError::Config("年份解析失败".to_string()))?;
            let month = parts[1]
                .parse::<i32>()
                .map_err(|_| BossError::Config("月份解析失败".to_string()))?;
            let day = parts[2]
                .parse::<i32>()
                .map_err(|_| BossError::Config("日期解析失败".to_string()))?;
            if (1..=12).contains(&month) && (1..=31).contains(&day) {
                return Ok((year, month, day));
            }
        }
    }
    // 尝试按中文分割
    if date_str.contains('年') && date_str.contains('月') && date_str.contains('日') {
        let parts: Vec<&str> = date_str.split(&['年', '月', '日'][..]).collect();
        if parts.len() >= 3 {
            let year = parts[0]
                .parse::<i32>()
                .map_err(|_| BossError::Config("年份解析失败".to_string()))?;
            let month = parts[1]
                .parse::<i32>()
                .map_err(|_| BossError::Config("月份解析失败".to_string()))?;
            let day = parts[2]
                .parse::<i32>()
                .map_err(|_| BossError::Config("日期解析失败".to_string()))?;
            if (1..=12).contains(&month) && (1..=31).contains(&day) {
                return Ok((year, month, day));
            }
        }
    }
    Err(BossError::Config(format!("无法解析日期格式: {}", date_str)))
}

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
        self.fill_campus_tags(job)?;

        // ==================== 第五步：工作地址 ====================
        log::info!("  [步骤5] 填写工作地址");
        self.fill_city(job)?;

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
        if !Self::has_excel_value(&job.经验) {
            log::warn!("  [跳过] 经验字段为空");
            return Ok(());
        }

        let target_value = job.经验.trim();

        let row_selectors = [
            ".requirements-info-content .publish-edit-form-row",
            ".publish-edit-form-row",
            ".form-row.job-experience-row",
            ".form-row",
        ];

        let mut clicked = false;

        for row_selector in row_selectors {
            let form_rows = match self.page.eles(row_selector) {
                Ok(rows) => rows,
                Err(_) => continue,
            };

            for form_row in form_rows {
                let label_el = match form_row.element(".publish-title") {
                    Ok(Some(el)) => el,
                    _ => match form_row.element(".title") {
                        Ok(Some(el)) => el,
                        _ => continue,
                    },
                };

                let label_text = label_el
                    .text_content()
                    .map_err(BossError::map_element("无法读取标题文本"))?
                    .trim()
                    .to_string();

                if label_text != "经验" {
                    continue;
                }

                log::info!("  [找到] 经验字段");

                let select_inner = form_row
                    .element(".ui-select-inner")
                    .map_err(BossError::map_element("未找到经验下拉框"))?;

                let select_inner =
                    select_inner.ok_or_else(|| BossError::element("经验下拉框为空"))?;

                select_inner
                    .click()
                    .map_err(BossError::map_element("点击经验下拉框失败"))?;

                clicked = true;
                break;
            }

            if clicked {
                break;
            }
        }

        if !clicked {
            return Err(BossError::element("未找到经验字段"));
        }

        sleep_random_ms(300, 500);

        let select_items = self
            .page
            .eles(".ui-select-item")
            .map_err(BossError::map_element("未找到经验下拉选项"))?;

        for item in select_items {
            let item_text = item
                .text_content()
                .map_err(BossError::map_element("无法读取经验选项文本"))?
                .trim()
                .to_string();

            if item_text == target_value {
                item.click()
                    .map_err(BossError::map_element("点击经验选项失败"))?;

                log::info!("  [√] 经验: {}", target_value);
                sleep_random_ms(400, 500);
                return Ok(());
            }
        }

        Err(BossError::element(format!(
            "未找到经验选项: {}",
            target_value
        )))
    }

    /// 应届生校园招聘 - 填写学历要求
    fn fill_campus_education(&mut self, job: &JobRecord) -> BResult<()> {
        if !Self::has_excel_value(&job.学历) {
            log::warn!("  [跳过] 学历字段为空");
            return Ok(());
        }

        let target_value = job.学历.trim();

        let row_selectors = [
            ".requirements-info-content .publish-edit-form-row",
            ".publish-edit-form-row",
            ".form-row.job-experience-row",
            ".form-row",
        ];

        let mut clicked = false;

        for row_selector in row_selectors {
            let form_rows = match self.page.eles(row_selector) {
                Ok(rows) => rows,
                Err(_) => continue,
            };

            for form_row in form_rows {
                let label_el = match form_row.element(".publish-title") {
                    Ok(Some(el)) => el,
                    _ => match form_row.element(".title") {
                        Ok(Some(el)) => el,
                        _ => continue,
                    },
                };

                let label_text = label_el
                    .text_content()
                    .map_err(BossError::map_element("无法读取标题文本"))?
                    .trim()
                    .to_string();

                if label_text != "学历" {
                    continue;
                }

                log::info!("  [找到] 学历字段");

                let select_inner = form_row
                    .element(".ui-select-inner")
                    .map_err(BossError::map_element("未找到学历下拉框"))?;

                let select_inner =
                    select_inner.ok_or_else(|| BossError::element("学历下拉框为空"))?;

                select_inner
                    .click()
                    .map_err(BossError::map_element("点击学历下拉框失败"))?;

                clicked = true;
                break;
            }

            if clicked {
                break;
            }
        }

        if !clicked {
            return Err(BossError::element("未找到学历字段"));
        }

        sleep_random_ms(300, 500);

        let select_items = self
            .page
            .eles(".ui-select-item")
            .map_err(BossError::map_element("未找到学历下拉选项"))?;

        for item in select_items {
            let item_text = item
                .text_content()
                .map_err(BossError::map_element("无法读取学历选项文本"))?
                .trim()
                .to_string();

            if item_text == target_value {
                item.click()
                    .map_err(BossError::map_element("点击学历选项失败"))?;

                log::info!("  [√] 学历: {}", target_value);
                sleep_random_ms(400, 500);
                return Ok(());
            }
        }

        Err(BossError::element(format!(
            "未找到学历选项: {}",
            target_value
        )))
    }

    /// 应届生校园招聘 - 填写薪资范围
    fn fill_campus_salary(&mut self, job: &JobRecord) -> BResult<()> {
        if !Self::has_excel_value(&job.薪资低) {
            log::warn!("  [跳过] 薪资字段为空");
            return Ok(());
        }

        log::info!("  [开始] 填写薪资");

        let row_selectors = [
            ".requirements-info-content .publish-edit-form-row",
            ".publish-edit-form-row",
            ".form-row.job-experience-row",
            ".form-row",
        ];

        let mut salary_row_found = false;
        let mut salary_selects = Vec::new();

        for row_selector in row_selectors {
            let form_rows = match self.page.eles(row_selector) {
                Ok(rows) => rows,
                Err(_) => continue,
            };

            for form_row in form_rows {
                let label_el = match form_row.element(".publish-title") {
                    Ok(Some(el)) => el,
                    _ => match form_row.element(".title") {
                        Ok(Some(el)) => el,
                        _ => continue,
                    },
                };

                let label_text = label_el
                    .text_content()
                    .map_err(BossError::map_element("无法读取标题文本"))?
                    .trim()
                    .to_string();

                if label_text != "薪资范围" && label_text != "薪资" {
                    continue;
                }

                log::info!("  [找到] 薪资范围字段");
                salary_row_found = true;

                if let Ok(selects) = form_row.elements(".ui-select-selection") {
                    salary_selects = selects;
                }

                break;
            }

            if salary_row_found {
                break;
            }
        }

        if !salary_row_found {
            return Err(BossError::element("未找到薪资范围字段"));
        }

        // 有些 form-row 版本薪资行里只能取到 1 个下拉，另外两个在相邻节点或全局结构里。
        // 因此不足 2 个时退回页面全局下拉列表；后面通过“点击后是否存在目标选项”来识别正确下拉。
        if salary_selects.len() < 2 {
            salary_selects = self
                .page
                .eles(".ui-select-selection")
                .map_err(BossError::map_element("未找到页面下拉框"))?;
        }

        if salary_selects.is_empty() {
            return Err(BossError::element("未找到薪资下拉框"));
        }

        let target_min_salary = job.薪资低.trim();
        let target_max_salary = job.薪资高.trim();
        let mut min_index: Option<usize> = None;

        // 选择最低月薪：从所有候选下拉中逐个尝试，直到打开后能看到目标薪资选项。
        for (idx, select) in salary_selects.iter().enumerate() {
            if select.click().is_err() {
                continue;
            }
            sleep_random_ms(300, 500);

            let items = match self.page.eles(".ui-select-item") {
                Ok(items) => items,
                Err(_) => continue,
            };

            let mut selected = false;
            for item in items.iter().rev() {
                let item_text = item
                    .text_content()
                    .map_err(BossError::map_element("无法读取最低月薪选项文本"))?;

                if item_text.trim() == target_min_salary {
                    item.click()
                        .map_err(BossError::map_element("点击最低月薪失败"))?;
                    selected = true;
                    min_index = Some(idx);
                    log::info!("  [√] 选择最低月薪: {}", target_min_salary);
                    break;
                }
            }

            if selected {
                break;
            }
        }

        let min_index = min_index
            .ok_or_else(|| BossError::element(format!("未找到最低月薪: {}", target_min_salary)))?;

        let mut max_index: Option<usize> = None;

        // 选择最高月薪：从最低月薪之后的下拉继续尝试，避免重复点回最低月薪。
        for (idx, select) in salary_selects.iter().enumerate().skip(min_index + 1) {
            if select.click().is_err() {
                continue;
            }
            sleep_random_ms(300, 500);

            let items = match self.page.eles(".ui-select-item") {
                Ok(items) => items,
                Err(_) => continue,
            };

            let mut selected = false;
            for item in items.iter().rev() {
                let item_text = item
                    .text_content()
                    .map_err(BossError::map_element("无法读取最高月薪选项文本"))?;

                if item_text.trim() == target_max_salary {
                    item.click()
                        .map_err(BossError::map_element("点击最高月薪失败"))?;
                    selected = true;
                    max_index = Some(idx);
                    log::info!("  [√] 选择最高月薪: {}", target_max_salary);
                    break;
                }
            }

            if selected {
                break;
            }
        }

        let max_index = max_index
            .ok_or_else(|| BossError::element(format!("未找到最高月薪: {}", target_max_salary)))?;

        if !Self::has_excel_value(&job.薪资单位) {
            log::info!("  [跳过] 薪资单位字段为空");
            sleep_random_ms(400, 500);
            return Ok(());
        }

        let target_salary_unit = job.薪资单位.trim();
        let mut selected_salary_unit = false;

        // 选择薪资单位：通常是最高月薪之后的下拉；找不到时仅警告跳过。
        for (_idx, select) in salary_selects.iter().enumerate().skip(max_index + 1) {
            if select.click().is_err() {
                continue;
            }
            sleep_random_ms(300, 500);

            let items = match self.page.eles(".ui-select-item") {
                Ok(items) => items,
                Err(_) => continue,
            };

            for item in items.iter().rev() {
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

            if selected_salary_unit {
                break;
            }
        }

        if !selected_salary_unit {
            log::warn!("  [跳过] 未找到薪资单位选项: {}", target_salary_unit);
        }

        sleep_random_ms(400, 500);
        Ok(())
    }

    /// 应届生校园招聘 - 填写职位关键词
    fn fill_campus_tags(&mut self, job: &JobRecord) -> BResult<()> {
        self.fill_tags(job)
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
        if !Self::has_excel_value(&job.届别) {
            log::warn!("  [跳过] 届别字段为空");
            return Ok(());
        }

        let target_year = job.届别.trim();
        let direct_selectors = [
            ".publish-edit-form-row.graduation-time-wrap .ui-select-selection",
            ".form-row.graduation-time-wrap .ui-select-selection",
            ".graduation-time-wrap .ui-select-selection",
        ];

        let mut clicked = false;

        // 1. 优先使用带 graduation-time-wrap 的直接选择器
        for selector in direct_selectors {
            match self.page.ele(selector) {
                Ok(Some(el)) => {
                    el.click()
                        .map_err(BossError::map_element("点击毕业时间下拉框失败"))?;
                    clicked = true;
                    break;
                }
                _ => continue,
            }
        }

        // 2. 兼容另一种 form-row/title 风格：通过标题“毕业时间/届别”找到所在行
        if !clicked {
            let row_selectors = [
                ".requirements-info-content .publish-edit-form-row",
                ".publish-edit-form-row",
                ".form-row.job-experience-row",
                ".form-row",
            ];

            for row_selector in row_selectors {
                let form_rows = match self.page.eles(row_selector) {
                    Ok(rows) => rows,
                    Err(_) => continue,
                };

                for form_row in form_rows {
                    let label_el = match form_row.element(".publish-title") {
                        Ok(Some(el)) => el,
                        _ => match form_row.element(".title") {
                            Ok(Some(el)) => el,
                            _ => continue,
                        },
                    };

                    let label_text = label_el
                        .text_content()
                        .map_err(BossError::map_element("无法读取标题文本"))?
                        .trim()
                        .to_string();

                    if label_text != "毕业时间" && label_text != "届别" {
                        continue;
                    }

                    let graduation_select = form_row
                        .element(".ui-select-selection")
                        .map_err(BossError::map_element("未找到毕业时间下拉框"))?;

                    let graduation_select = graduation_select
                        .ok_or_else(|| BossError::element("毕业时间下拉框为空"))?;

                    graduation_select
                        .click()
                        .map_err(BossError::map_element("点击毕业时间下拉框失败"))?;

                    clicked = true;
                    break;
                }

                if clicked {
                    break;
                }
            }
        }

        if !clicked {
            return Err(BossError::element("毕业时间下拉按钮不存在"));
        }

        sleep_random_ms(300, 500);

        let items = self
            .page
            .eles(".ui-select-item")
            .map_err(BossError::map_element("未找到毕业时间选项"))?;

        for item in items {
            let text = item
                .text_content()
                .map_err(BossError::map_element("无法读取毕业时间选项文本"))?;

            if text.trim() == target_year {
                item.click()
                    .map_err(BossError::map_element("点击目标毕业年份失败"))?;
                log::info!("  [√] 毕业时间: {}", target_year);
                sleep_random_ms(400, 500);
                return Ok(());
            }
        }

        Err(BossError::element(format!(
            "未找到目标毕业年份: {}",
            target_year
        )))
    }

    fn click_campus_deadline_trigger(&mut self) -> BResult<Option<rust_drission::Element>> {
        if let Some(el) = SelectorMap::find_first(self.page, &self.selectors.deadline) {
            el.click()
                .map_err(BossError::map_element("点击招聘截止时间控件失败"))?;
            sleep_random_ms(300, 500);
            log::info!("  [Debug] 已通过截止时间选择器点击日期控件");
            return Ok(Some(el));
        }

        let direct_selectors = [
            ".publish-edit-form-row.deadline-wrap .ui-select-selection",
            ".publish-edit-form-row.deadline-wrap input",
            ".form-row.deadline-wrap .ui-select-selection",
            ".form-row.deadline-wrap input",
            ".deadline-wrap .ui-select-selection",
            ".deadline-wrap input",
            "[class*='deadline'] .ui-select-selection",
            "[class*='deadline'] input",
            "[class*='date'] .ui-select-selection",
            "[class*='date'] input",
        ];

        for selector in direct_selectors {
            if let Ok(Some(el)) = self.page.ele(selector) {
                el.click()
                    .map_err(BossError::map_element("点击招聘截止时间控件失败"))?;
                sleep_random_ms(300, 500);
                log::info!("  [Debug] 已通过直接候选选择器点击日期控件: {}", selector);
                return Ok(Some(el));
            }
        }

        let row_selectors = [
            ".requirements-info-content .publish-edit-form-row",
            ".publish-component",
            ".publish-edit-form-row",
            ".form-row",
            ".job-form-item",
            ".ui-form-item",
            ".form-item",
            "li",
        ];
        let label_selectors = [
            ".publish-title",
            ".title",
            ".label",
            ".form-label",
            ".item-label",
            ".name",
            "label",
            "span",
            "div",
        ];
        let clickable_selectors = [
            ".ui-select-selection",
            ".ui-date-editor",
            ".date-picker",
            ".datepicker",
            ".input-wrap",
            ".ipt-wrap",
            ".ui-select-inner",
            "input",
            "[tabindex]",
            "[class*='date']",
            "[class*='time']",
        ];

        for row_selector in row_selectors {
            let rows = match self.page.eles(row_selector) {
                Ok(rows) => rows,
                Err(_) => continue,
            };

            for row in rows {
                let mut is_deadline_row = false;
                for label_selector in label_selectors {
                    let labels = match row.elements(label_selector) {
                        Ok(labels) => labels,
                        Err(_) => continue,
                    };
                    for label in labels {
                        let text = label.text_content().unwrap_or_default();
                        let text = Self::clean_text(&text);
                        if text.contains("招聘截止时间")
                            || text.contains("招聘截止日期")
                            || text.contains("截止时间")
                            || text.contains("截止日期")
                        {
                            is_deadline_row = true;
                            break;
                        }
                    }
                    if is_deadline_row {
                        break;
                    }
                }

                if !is_deadline_row {
                    let text = row.text_content().unwrap_or_default();
                    let text = Self::clean_text(&text);
                    is_deadline_row = text.contains("招聘截止时间")
                        || text.contains("招聘截止日期")
                        || text.contains("截止时间")
                        || text.contains("截止日期");
                }

                if !is_deadline_row {
                    continue;
                }

                for clickable_selector in clickable_selectors {
                    if let Ok(Some(el)) = row.element(clickable_selector) {
                        el.click()
                            .map_err(BossError::map_element("点击招聘截止时间控件失败"))?;
                        sleep_random_ms(300, 500);
                        log::info!(
                            "  [Debug] 已通过表单行点击日期控件: row={}, control={}",
                            row_selector,
                            clickable_selector
                        );
                        return Ok(Some(el));
                    }
                }
            }
        }

        Ok(None)
    }

    fn fill_campus_deadline(&mut self, job: &JobRecord) -> BResult<()> {
        if !Self::has_excel_value(&job.截止日期) {
            log::warn!("  [跳过] 截止日期字段为空");
            return Ok(());
        }

        log::info!("  [开始] 填写招聘截止时间: {}", job.截止日期);

        // 解析目标日期 (年, 月, 日，月份为 1-indexed)
        let target_date_str = job.截止日期.trim();
        let (target_year, target_month, target_day) = parse_date(target_date_str)?;

        if let Some(deadline_input) = self.click_campus_deadline_trigger()? {
            let element_js = format!(
                r#"
                return (async function(input) {{
                    const targetYear = {year};
                    const targetMonth = {month};
                    const targetDay = {day};
                    const sleep = ms => new Promise(r => setTimeout(r, ms));
                    const doc = input.ownerDocument || document;

                    input.value = '';
                    input.click();
                    await sleep(700);

                    function visible(el) {{
                        if (!el) return false;
                        const style = window.getComputedStyle(el);
                        return style.display !== 'none' && style.visibility !== 'hidden';
                    }}
                    function clean(text) {{
                        return String(text || '').replace(/\s+/g, '');
                    }}

                    let panels = [...doc.querySelectorAll('.ui-datepicker-panel')];
                    let datePanel = panels.find(p => visible(p) && p.querySelector('.ui-datepicker-tb'));
                    if (!datePanel) {{
                        return {{
                            ok: false,
                            msg: 'date panel not found from element context',
                            panelCount: panels.length,
                            bodyText: clean(doc.body?.innerText || '').slice(0, 800),
                            ownerDocumentUrl: doc.URL || ''
                        }};
                    }}

                    function getCurrentYearMonth(panel) {{
                        const btn = panel.querySelector('.ui-datepicker-btn');
                        if (!btn) return null;
                        const text = btn.innerText.trim();
                        const y = text.match(/(\d{{4}})年/);
                        const m = text.match(/(\d{{1,2}})月/);
                        if (!y || !m) return null;
                        return {{ year: parseInt(y[1]), month: parseInt(m[1]) }};
                    }}

                    for (let i = 0; i < 30; i++) {{
                        let cur = getCurrentYearMonth(datePanel);
                        if (!cur) break;
                        if (cur.year === targetYear && cur.month === targetMonth) break;
                        const nextBtn = datePanel.querySelector('.ui-datepicker-next');
                        const prevBtn = datePanel.querySelector('.ui-datepicker-prev');
                        if (cur.year < targetYear || (cur.year === targetYear && cur.month < targetMonth)) {{
                            if (nextBtn) nextBtn.click();
                        }} else {{
                            if (prevBtn) prevBtn.click();
                        }}
                        await sleep(300);
                    }}

                    let clicked = false;
                    const tds = datePanel.querySelectorAll('tbody td');
                    for (let td of tds) {{
                        const span = td.querySelector('span');
                        if (span && parseInt(span.innerText) === targetDay && td.classList.contains('z-existed') && !td.classList.contains('z-invalid')) {{
                            td.click();
                            clicked = true;
                            break;
                        }}
                    }}
                    if (!clicked) {{
                        return {{ ok: false, msg: 'day not found', panelText: clean(datePanel.innerText || '').slice(0, 500) }};
                    }}

                    await sleep(500);
                    panels = [...doc.querySelectorAll('.ui-datepicker-panel')];
                    const timePanel = panels.find(p => visible(p) && p.querySelector('.ui-datepicker-tb2'));
                    if (timePanel) {{
                        const firstTimeCell = timePanel.querySelector('.z-existed span');
                        if (firstTimeCell) firstTimeCell.click();
                        const confirm = timePanel.querySelector('.btn-sure, .btn-primary, .confirm');
                        if (confirm) confirm.click();
                        await sleep(300);
                    }}

                    const finalValue = `${{targetYear}}-${{String(targetMonth).padStart(2,'0')}}-${{String(targetDay).padStart(2,'0')}}`;
                    input.value = finalValue;
                    input.dispatchEvent(new Event('input', {{ bubbles: true }}));
                    input.dispatchEvent(new Event('change', {{ bubbles: true }}));
                    input.dispatchEvent(new Event('blur', {{ bubbles: true }}));
                    return {{ ok: true, value: input.value }};
                }})(this);
                "#,
                year = target_year,
                month = target_month,
                day = target_day
            );
            let ret = deadline_input
                .run_js_await(&element_js)
                .map_err(BossError::map_cdp("元素上下文选择日期失败"))?;
            log::info!("  [Debug] 日期选择结果: {:?}", ret);

            let ok = ret
                .get("value")
                .and_then(|v| v.get("ok"))
                .and_then(|v| v.as_bool())
                .or_else(|| ret.get("ok").and_then(|v| v.as_bool()))
                .unwrap_or(false);

            if ok {
                log::info!("  [√] 招聘截止时间已选择: {}", job.截止日期);
                return Ok(());
            }

            return Err(BossError::element(format!(
                "招聘截止时间日期面板点击选择失败: {:?}",
                ret
            )));
        }

        // 通过真实点击日期控件选择，不直接写入值。日期面板渲染依赖事件循环，必须异步等待。
        let js_script = format!(
            r#"
        new Promise(resolve => {{
            const targetYear = {year};
            const targetMonth = {month};
            const targetDay = {day};

            function visible(el) {{
                if (!el) return false;
                const rect = el.getBoundingClientRect();
                const style = window.getComputedStyle(el);
                return rect.width > 0 && rect.height > 0 && style.display !== 'none' && style.visibility !== 'hidden';
            }}
            function clean(text) {{
                return String(text || '').replace(/\s+/g, '');
            }}
            function isDeadlineText(text) {{
                const t = clean(text);
                return t.includes('招聘截止时间') || t.includes('招聘截止日期') || t.includes('截止时间') || t.includes('截止日期');
            }}
            function findDeadlineTrigger() {{
                const directSelectors = [
                    'input[placeholder="选择招聘截止时间"]',
                    'input[placeholder*="招聘截止"]',
                    'input[placeholder*="截止时间"]',
                    'input[placeholder*="截止日期"]',
                    'input[placeholder*="选择时间"]',
                    'input[placeholder*="选择日期"]',
                    'input[name*="deadline"]',
                    'input[id*="deadline"]'
                ];
                for (const selector of directSelectors) {{
                    const input = Array.from(document.querySelectorAll(selector)).find(visible);
                    if (input) return input;
                }}

                const rowSelectors = [
                    '.requirements-info-content',
                    '.publish-component',
                    '.publish-edit-form-row',
                    '.form-row',
                    '.job-form-item',
                    '.ui-form-item',
                    '.form-item',
                    '[class*="deadline"]',
                    '[class*="date"]'
                ];
                for (const row of Array.from(document.querySelectorAll(rowSelectors.join(',')))) {{
                    const text = clean(row.innerText || row.textContent || '');
                    if (!isDeadlineText(text)) continue;
                    const input = Array.from(row.querySelectorAll('input')).find(visible);
                    if (input) return input;
                    const clickable = Array.from(row.querySelectorAll(
                        '.ui-select-selection, .ui-date-editor, .date-picker, .datepicker, .input-wrap, .ipt-wrap, .ui-select-inner, [tabindex], button, span, div'
                    ))
                        .filter(visible)
                        .sort((a, b) => {{
                            const ar = a.getBoundingClientRect();
                            const br = b.getBoundingClientRect();
                            return (ar.width * ar.height) - (br.width * br.height);
                        }})
                        .find(el => {{
                            const elText = clean(el.innerText || el.textContent || '');
                            return !isDeadlineText(elText) || elText.includes('选择');
                        }});
                    if (clickable) return clickable;
                }}

                const labels = Array.from(document.querySelectorAll('label, span, div, p'))
                    .filter(el => visible(el) && isDeadlineText(el.innerText || el.textContent || ''))
                    .sort((a, b) => {{
                        const ar = a.getBoundingClientRect();
                        const br = b.getBoundingClientRect();
                        return (ar.width * ar.height) - (br.width * br.height);
                    }});
                for (const label of labels) {{
                    let row = label;
                    for (let i = 0; i < 6 && row; i++, row = row.parentElement) {{
                        const input = Array.from(row.querySelectorAll('input')).find(visible);
                        if (input) return input;
                        const clickables = Array.from(row.querySelectorAll('.ui-select-selection, .ui-date-editor, .date-picker, .datepicker, .input-wrap, .ipt-wrap, .ui-select-inner, [tabindex], button, span, div'))
                            .filter(el => visible(el) && el !== label)
                            .sort((a, b) => {{
                                const ar = a.getBoundingClientRect();
                                const br = b.getBoundingClientRect();
                                return (ar.width * ar.height) - (br.width * br.height);
                            }});
                        const candidate = clickables.find(el => {{
                            const r = el.getBoundingClientRect();
                            const lr = label.getBoundingClientRect();
                            return r.left >= lr.left && r.top >= lr.top - 20;
                        }}) || clickables[0];
                        if (candidate) return candidate;
                    }}
                }}
                return null;
            }}
            function clickLikeUser(el) {{
                el.scrollIntoView({{ block: 'center', inline: 'center' }});
                if (el.focus) el.focus();
                ['pointerdown', 'mousedown', 'mouseup', 'click'].forEach(type => {{
                    const EventCtor = type.startsWith('pointer') ? PointerEvent : MouseEvent;
                    el.dispatchEvent(new EventCtor(type, {{ bubbles: true, cancelable: true, view: window }}));
                }});
            }}
            function findDatePanel() {{
                const panels = document.querySelectorAll('.ui-datepicker-panel, .datepicker-panel, .date-picker-panel, [class*="datepicker"], [class*="date-picker"], [class*="calendar"]');
                for (let p of panels) {{
                    if (visible(p) && (p.querySelector('.ui-datepicker-tb') || p.querySelector('tbody td') || p.innerText.includes('今天'))) {{
                        return p;
                    }}
                }}
                return null;
            }}

            const trigger = findDeadlineTrigger();
            if (!trigger) {{
                resolve({{
                ok: false,
                msg: 'deadline trigger not found',
                inputs: Array.from(document.querySelectorAll('input')).map(input => ({{
                    placeholder: input.getAttribute('placeholder') || '',
                    name: input.getAttribute('name') || '',
                    id: input.id || '',
                    value: input.value || ''
                }})).slice(0, 80),
                deadlineRows: Array.from(document.querySelectorAll('.publish-component, .publish-edit-form-row, .form-row, .job-form-item, .ui-form-item, .form-item'))
                    .map(row => clean(row.innerText || row.textContent || '').slice(0, 120))
                    .filter(text => text.includes('截止') || text.includes('招聘'))
                    .slice(0, 20),
                bodyText: clean(document.body?.innerText || '').slice(0, 800)
                }});
                return;
            }}
            if (trigger) clickLikeUser(trigger);

            function getCurrentYearMonth(panel) {{
                const btn = panel.querySelector('.ui-datepicker-btn');
                if (!btn) return null;
                const text = btn.innerText.trim();
                const yMatch = text.match(/(\d{{4}})年/);
                const mMatch = text.match(/(\d{{1,2}})月/);
                if (!yMatch || !mMatch) return null;
                return {{ year: parseInt(yMatch[1]), month: parseInt(mMatch[1]) }};
            }}
            function clickDatePanel(panel) {{
                let maxAttempts = 30;
                for (let i = 0; i < maxAttempts; i++) {{
                    let cur = getCurrentYearMonth(panel);
                    if (!cur) break;
                    if (cur.year === targetYear && cur.month === targetMonth) break;
                    const nextBtn = panel.querySelector('.ui-datepicker-next');
                    const prevBtn = panel.querySelector('.ui-datepicker-prev');
                    if (cur.year < targetYear || (cur.year === targetYear && cur.month < targetMonth)) {{
                        if (nextBtn) nextBtn.click();
                    }} else {{
                        if (prevBtn) prevBtn.click();
                    }}
                }}

                const tds = panel.querySelectorAll('tbody td');
                for (let td of tds) {{
                    const span = td.querySelector('span') || td;
                    if (parseInt(span.innerText) === targetDay && !td.classList.contains('z-invalid') && !td.classList.contains('disabled')) {{
                        clickLikeUser(td);
                        return true;
                    }}
                }}
                return false;
            }}

            const started = Date.now();
            const panelTimer = setInterval(() => {{
                const datePanel = findDatePanel();
                if (datePanel) {{
                    clearInterval(panelTimer);
                    if (!clickDatePanel(datePanel)) {{
                        resolve({{ ok: false, msg: 'day not found', panelText: clean(datePanel.innerText || '').slice(0, 500) }});
                        return;
                    }}
                    setTimeout(() => {{
                        const timePanel = Array.from(document.querySelectorAll('.ui-datepicker-panel, .datepicker-panel, .date-picker-panel, [class*="datepicker"], [class*="date-picker"], [class*="calendar"]'))
                            .find(p => visible(p) && (p.querySelector('.ui-datepicker-tb2') || clean(p.innerText || '').includes('请选择时间')));
                        if (timePanel) {{
                            const timeCells = timePanel.querySelectorAll('.z-existed span, tbody td span, li, span');
                            const firstTime = Array.from(timeCells).find(visible);
                            if (firstTime) clickLikeUser(firstTime);
                            const confirmBtn = Array.from(timePanel.querySelectorAll('.btn-sure, .btn-primary, .confirm, button, span'))
                                .find(el => visible(el) && ['确定', '确认', '完成'].some(text => clean(el.innerText || el.textContent || '') === text));
                            if (confirmBtn) clickLikeUser(confirmBtn);
                        }}
                        resolve({{
                            ok: true,
                            triggerText: trigger ? clean(trigger.innerText || trigger.textContent || '').slice(0, 120) : 'clicked by rust',
                            waitedMs: Date.now() - started
                        }});
                    }}, 700);
                    return;
                }}
                if (Date.now() - started > 5000) {{
                    clearInterval(panelTimer);
                    resolve({{
                        ok: false,
                        msg: 'date panel not found',
                        triggerText: trigger ? clean(trigger.innerText || trigger.textContent || '').slice(0, 120) : 'clicked by rust',
                        visiblePanels: Array.from(document.querySelectorAll('[class*="date"], [class*="calendar"]'))
                            .filter(visible)
                            .map(el => ({{ className: String(el.className || ''), text: clean(el.innerText || '').slice(0, 120) }}))
                            .slice(0, 30)
                    }});
                }}
            }}, 250);
        }})
        "#,
            year = target_year,
            month = target_month,
            day = target_day
        );

        let ret = self
            .page
            .run_js_await(&js_script)
            .map_err(BossError::map_cdp("JS选择日期失败"))?;
        log::info!("  [Debug] 日期选择结果: {:?}", ret);

        let ok = ret
            .get("value")
            .and_then(|v| v.get("ok"))
            .and_then(|v| v.as_bool())
            .or_else(|| ret.get("ok").and_then(|v| v.as_bool()))
            .unwrap_or(false);

        if ok {
            log::info!("  [√] 招聘截止时间已选择: {}", job.截止日期);
            Ok(())
        } else {
            Err(BossError::element(format!(
                "招聘截止时间日期面板点击选择失败: {:?}",
                ret
            )))
        }
    }
}
