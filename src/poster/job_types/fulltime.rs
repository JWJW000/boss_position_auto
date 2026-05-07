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
        if !Self::has_excel_value(&job.经验) {
            log::warn!("  [跳过] 经验字段为空");
            return Ok(());
        }

        let target_value = job.经验.trim();

        let row_selectors = [
            ".requirements-info-content .publish-edit-form-row",
            ".publish-component .publish-edit-form-row",
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

                let label_text = match label_el.text_content() {
                    Ok(text) => text.trim().to_string(),
                    Err(_) => continue,
                };

                if label_text != "经验" {
                    continue;
                }

                log::info!("  [找到] 经验字段");

                let select_inner = form_row
                    .element(".ui-select-inner")
                    .map_err(BossError::map_element("未找到经验下拉框"))?
                    .ok_or_else(|| BossError::element("经验下拉框为空"))?;

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

        for item in select_items.iter().rev() {
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

    /// 社招全职 - 填写学历要求
    fn fill_full_time_education(&mut self, job: &JobRecord) -> BResult<()> {
        if !Self::has_excel_value(&job.学历) {
            log::warn!("  [跳过] 学历字段为空");
            return Ok(());
        }

        let target_value = job.学历.trim();

        let row_selectors = [
            ".requirements-info-content .publish-edit-form-row",
            ".publish-component .publish-edit-form-row",
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

                let label_text = match label_el.text_content() {
                    Ok(text) => text.trim().to_string(),
                    Err(_) => continue,
                };

                if label_text != "学历" {
                    continue;
                }

                log::info!("  [找到] 学历字段");

                let select_inner = form_row
                    .element(".ui-select-inner")
                    .map_err(BossError::map_element("未找到学历下拉框"))?
                    .ok_or_else(|| BossError::element("学历下拉框为空"))?;

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

        for item in select_items.iter().rev() {
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

    /// 社招全职 - 填写薪资范围
    fn fill_full_time_salary(&mut self, job: &JobRecord) -> BResult<()> {
        if !Self::has_excel_value(&job.薪资低) {
            log::warn!("  [跳过] 薪资字段为空");
            return Ok(());
        }

        log::info!("  [开始] 填写薪资");

        let row_selectors = [
            ".requirements-info-content .publish-edit-form-row",
            ".publish-component .publish-edit-form-row",
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

                let label_text = match label_el.text_content() {
                    Ok(text) => text.trim().to_string(),
                    Err(_) => continue,
                };

                if label_text != "薪资范围" {
                    continue;
                }

                log::info!("  [找到] 薪资范围字段");

                // 薪资范围在不同页面结构下不一定 3 个下拉都在同一个 form_row 内。
                // 有的页面 form_row 内只能找到第一个下拉，所以这里先取当前行，数量不足时再回退到页面全局下拉顺序。
                let mut salary_selects = form_row
                    .elements(".ui-select-selection")
                    .map_err(BossError::map_element("未找到薪资下拉框"))?;

                if salary_selects.len() < 3 {
                    let all_selects = self
                        .page
                        .eles(".ui-select-selection")
                        .map_err(BossError::map_element("未找到页面下拉框"))?;

                    // 当前流程顺序是：经验、学历、薪资低、薪资高、薪资单位。
                    // 因此当全局下拉数量足够时，跳过前两个，取后面三个作为薪资下拉。
                    if all_selects.len() >= 5 {
                        salary_selects = all_selects.into_iter().skip(2).take(3).collect();
                    } else if all_selects.len() >= 3 {
                        // 兜底：如果页面只有 3 个下拉，则直接取最后 3 个。
                        salary_selects = all_selects.into_iter().rev().take(3).collect();
                        salary_selects.reverse();
                    }
                }

                if salary_selects.len() < 3 {
                    return Err(BossError::element(format!(
                        "薪资下拉框数量不足，期望3个，实际{}个",
                        salary_selects.len()
                    )));
                }

                // 选择最低月薪
                let target_min_salary = job.薪资低.trim();
                salary_selects[0]
                    .click()
                    .map_err(BossError::map_element("点击最低月薪下拉框失败"))?;

                sleep_random_ms(300, 500);

                let min_salary_items = self
                    .page
                    .eles(".ui-select-item")
                    .map_err(BossError::map_element("未找到最低月薪选项"))?;

                let mut selected_min_salary = false;
                for item in min_salary_items.iter().rev() {
                    let item_text = item
                        .text_content()
                        .map_err(BossError::map_element("无法读取最低月薪选项文本"))?;

                    if item_text.trim() == target_min_salary {
                        item.click()
                            .map_err(BossError::map_element("点击最低月薪失败"))?;
                        selected_min_salary = true;
                        log::info!("  [√] 选择最低月薪: {}", target_min_salary);
                        break;
                    }
                }

                if !selected_min_salary {
                    return Err(BossError::element(format!(
                        "未找到最低月薪: {}",
                        target_min_salary
                    )));
                }

                sleep_random_ms(300, 500);

                // 选择最高月薪
                let target_max_salary = job.薪资高.trim();
                salary_selects[1]
                    .click()
                    .map_err(BossError::map_element("点击最高月薪下拉框失败"))?;

                sleep_random_ms(300, 500);

                let max_salary_items = self
                    .page
                    .eles(".ui-select-item")
                    .map_err(BossError::map_element("未找到最高月薪选项"))?;

                let mut selected_max_salary = false;
                for item in max_salary_items.iter().rev() {
                    let item_text = item
                        .text_content()
                        .map_err(BossError::map_element("无法读取最高月薪选项文本"))?;

                    if item_text.trim() == target_max_salary {
                        item.click()
                            .map_err(BossError::map_element("点击最高月薪失败"))?;
                        selected_max_salary = true;
                        log::info!("  [√] 选择最高月薪: {}", target_max_salary);
                        break;
                    }
                }

                if !selected_max_salary {
                    return Err(BossError::element(format!(
                        "未找到最高月薪: {}",
                        target_max_salary
                    )));
                }

                sleep_random_ms(300, 500);

                // 选择薪资单位
                let target_salary_unit = job.薪资单位.trim();
                salary_selects[2]
                    .click()
                    .map_err(BossError::map_element("点击薪资单位下拉框失败"))?;

                sleep_random_ms(300, 500);

                let salary_unit_items = self
                    .page
                    .eles(".ui-select-item")
                    .map_err(BossError::map_element("未找到薪资单位选项"))?;

                let mut selected_salary_unit = false;
                for item in salary_unit_items.iter().rev() {
                    let item_text = item
                        .text_content()
                        .map_err(BossError::map_element("无法读取薪资单位选项文本"))?;

                    if item_text.trim() == target_salary_unit {
                        item.click()
                            .map_err(BossError::map_element("点击薪资单位失败"))?;
                        selected_salary_unit = true;
                        log::info!("  [√] 选择薪资单位: {}", target_salary_unit);
                        break;
                    }
                }

                if !selected_salary_unit {
                    return Err(BossError::element(format!(
                        "未找到薪资单位: {}",
                        target_salary_unit
                    )));
                }

                sleep_random_ms(400, 500);
                return Ok(());
            }
        }

        Err(BossError::element("未找到薪资范围字段"))
    }

    /// 社招全职 - 填写是否驻外
    fn fill_full_time_overseas(&mut self, job: &JobRecord) -> BResult<()> {
        if !Self::has_excel_value(&job.是否驻外) {
            log::warn!("  [跳过] 是否驻外字段为空");
            return Ok(());
        }

        log::info!("  [开始] 填写是否驻外");

        let row_selectors = [
            ".publish-edit-form-row.overseas-entry-container",
            ".requirements-info-content .publish-edit-form-row",
            ".publish-component .publish-edit-form-row",
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

                let label_text = match label_el.text_content() {
                    Ok(text) => text.trim().to_string(),
                    Err(_) => continue,
                };

                if label_text != "是否驻外" {
                    continue;
                }

                log::info!("  [找到] 是否驻外字段");

                let options = match form_row.elements(".entry-content .chose-item") {
                    Ok(opts) if !opts.is_empty() => opts,
                    _ => match form_row.elements(".chose-item") {
                        Ok(opts) => opts,
                        Err(_) => {
                            log::warn!("  [跳过] 未找到是否驻外选项");
                            return Ok(());
                        }
                    },
                };

                let target_value = job.是否驻外.trim();
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
        }

        log::warn!("  [跳过] 未找到是否驻外字段，该字段可能不存在");
        Ok(())
    }

    /// 社招全职 - 填写职位关键词
    fn fill_full_time_tags(&mut self, job: &JobRecord) -> BResult<()> {
        if !Self::has_excel_value(&job.关键词) {
            log::warn!("  [跳过] 职位关键词字段为空");
            return Ok(());
        }

        log::info!("  [开始] 填写职位关键词");

        let keywords: Vec<&str> = job
            .关键词
            .split(|c: char| c.is_whitespace() || c == ',' || c == '，')
            .filter(|s| !s.trim().is_empty())
            .collect();

        if keywords.is_empty() {
            log::warn!("  [跳过] 关键词分割后为空");
            return Ok(());
        }

        let tag_input = SelectorMap::find_first(self.page, &self.selectors.tags);
        if tag_input.is_none() {
            log::warn!("  [跳过] 未找到职位关键词输入框");
            return Ok(());
        }

        let tag_input = tag_input.unwrap();

        for (i, keyword) in keywords.iter().enumerate() {
            let keyword = keyword.trim();
            if keyword.is_empty() {
                continue;
            }

            log::info!("  [输入] 关键词 {}: {}", i + 1, keyword);

            tag_input
                .click()
                .map_err(BossError::map_post("点击关键词输入框失败"))?;
            sleep_random_ms(200, 300);

            tag_input
                .input(keyword)
                .map_err(BossError::map_post("输入关键词失败"))?;
            sleep_random_ms(300, 500);

            tag_input
                .input("\n")
                .map_err(BossError::map_post("确认关键词失败"))?;
            sleep_random_ms(400, 600);
        }

        log::info!("  [√] 职位关键词: 已输入 {} 个", keywords.len());
        Ok(())
    }
}
