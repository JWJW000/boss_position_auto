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
        // log::info!("  [步骤1] 填写经验要求");
        // self.fill_intern_experience(job)?;

        // ==================== 第二步：学历要求 ====================
        log::info!("  [步骤2] 填写学历要求");
        self.fill_intern_education(job)?;

        // ==================== 第三步：薪资范围 ====================
        log::info!("  [步骤3] 填写薪资范围");
        self.fill_intern_salary(job, kind)?;

        // ==================== 第四步：职位关键词 ====================
        log::info!("  [步骤4] 填写职位关键词");
        self.fill_intern_tags(job)?;

        // ==================== 第五步：工作地址 ====================
        log::info!("  [步骤5] 填写工作地址");
        self.fill_city(job)?;

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

        let target_value = job.经验.trim();

        // 兼容两种页面结构：publish-edit-form-row / form-row + publish-title / title
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

    fn fill_intern_education(&mut self, job: &JobRecord) -> BResult<()> {
        // 检查 Excel 中是否有学历值
        if !Self::has_excel_value(&job.学历) {
            log::warn!("  [跳过] 学历字段为空");
            return Ok(());
        }

        let target_value = job.学历.trim();

        // 兼容两种页面结构：publish-edit-form-row / form-row + publish-title / title
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

    fn fill_intern_salary(&mut self, job: &JobRecord, _kind: RecruitmentKind) -> BResult<()> {
        let low = job.薪资低.trim();
        let high = job.薪资高.trim();

        if !Self::has_excel_value(&job.薪资低) && !Self::has_excel_value(&job.薪资高) {
            log::warn!("  [跳过] 薪资字段为空");
            return Ok(());
        }

        log::info!("  [开始] 填写实习薪资范围");

        // 先尝试在“薪资范围/薪资”表单行中找下拉；不足 2 个时再回退到全页面下拉。
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
            log::warn!("  [警告] 未通过标题找到薪资范围字段，尝试全局薪资下拉");
        }

        if salary_selects.len() < 2 {
            salary_selects = self
                .page
                .eles(".ui-select-selection")
                .map_err(BossError::map_element("未找到页面下拉框"))?;
        }

        if salary_selects.is_empty() {
            return Err(BossError::element("未找到薪资下拉框"));
        }

        let mut low_index: Option<usize> = None;

        if Self::has_excel_value(&job.薪资低) {
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
                    let text = item
                        .text_content()
                        .map_err(BossError::map_element("无法读取起始薪资选项文本"))?;

                    if text.trim() == low {
                        item.click()
                            .map_err(BossError::map_element("点击起始薪资选项失败"))?;
                        selected = true;
                        low_index = Some(idx);
                        log::info!("  [√] 起始薪资: {}", low);
                        break;
                    }
                }

                if selected {
                    break;
                }
            }

            if low_index.is_none() {
                return Err(BossError::element(format!("未找到起始薪资选项: {}", low)));
            }
        }

        if Self::has_excel_value(&job.薪资高) {
            let start_idx = low_index.map(|idx| idx + 1).unwrap_or(0);
            let mut selected_high = false;

            for select in salary_selects.iter().skip(start_idx) {
                if select.click().is_err() {
                    continue;
                }
                sleep_random_ms(300, 500);

                let items = match self.page.eles(".ui-select-item") {
                    Ok(items) => items,
                    Err(_) => continue,
                };

                for item in items.iter().rev() {
                    let text = item
                        .text_content()
                        .map_err(BossError::map_element("无法读取截止薪资选项文本"))?;

                    if text.trim() == high {
                        item.click()
                            .map_err(BossError::map_element("点击截止薪资选项失败"))?;
                        selected_high = true;
                        log::info!("  [√] 截止薪资: {}", high);
                        break;
                    }
                }

                if selected_high {
                    break;
                }
            }

            if !selected_high {
                return Err(BossError::element(format!("未找到截止薪资选项: {}", high)));
            }
        }

        sleep_random_ms(400, 500);
        Ok(())
    }

    fn fill_intern_tags(&mut self, job: &JobRecord) -> BResult<()> {
        // 检查 Excel 中是否有关键词
        if !Self::has_excel_value(&job.关键词) {
            log::warn!("  [跳过] 职位关键词字段为空");
            return Ok(());
        }

        log::info!("  [开始] 填写职位关键词");

        let keywords: Vec<&str> = job
            .关键词
            .split(|c: char| c.is_whitespace() || c == ',' || c == '，' || c == ';' || c == '；')
            .filter(|s| !s.trim().is_empty() && s.trim() != "无")
            .collect();

        if keywords.is_empty() {
            log::warn!("  [跳过] 关键词分割后为空");
            return Ok(());
        }

        // 方式一：新版 / form-row 页面，直接找输入框。
        let direct_selectors = [
            ".job-keyword input",
            ".job-keywords input",
            ".job-tags input",
            ".tag-input input",
            ".tags-input input",
            ".keyword-input input",
            ".publish-content input[placeholder*='关键词']",
            ".publish-content input[placeholder*='标签']",
            ".content input[placeholder*='关键词']",
            ".content input[placeholder*='标签']",
            "input[placeholder*='关键词']",
            "input[placeholder*='职位关键词']",
            "input[placeholder*='标签']",
        ];

        for selector in direct_selectors {
            if let Ok(Some(tag_input)) = self.page.ele(selector) {
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

                log::info!(
                    "  [√] 职位关键词: 已通过直接输入方式填写 {} 个",
                    keywords.len()
                );
                return Ok(());
            }
        }

        // 方式二：旧版页面，复用全局 fill_tags 弹窗逻辑。
        log::info!("  [提示] 未找到直接关键词输入框，尝试旧版关键词弹窗逻辑");
        self.fill_tags(job)
    }

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

        let row_selectors = [
            ".requirements-info-content .publish-edit-form-row",
            ".publish-edit-form-row",
            ".form-row.job-experience-row",
            ".form-row",
        ];

        let mut target_row = None;

        for row_selector in row_selectors {
            let rows = match self.page.eles(row_selector) {
                Ok(rows) => rows,
                Err(_) => continue,
            };

            for row in rows {
                let title_el = match row.element(".publish-title") {
                    Ok(Some(el)) => el,
                    _ => match row.element(".title") {
                        Ok(Some(el)) => el,
                        _ => continue,
                    },
                };

                if let Ok(text) = title_el.text() {
                    if text.contains("实习要求") {
                        target_row = Some(row);
                        break;
                    }
                }
            }

            if target_row.is_some() {
                break;
            }
        }

        let row =
            target_row.ok_or_else(|| BossError::element("intern requirement row not found"))?;

        let selects = row
            .elements(".ui-select-selection")
            .map_err(BossError::map_element(
                "intern requirement selects not found",
            ))?;

        let month_select = if let Some(select) = selects.get(0) {
            select
        } else {
            return Err(BossError::element("month select not found"));
        };

        month_select
            .click()
            .map_err(BossError::map_element("click month select failed"))?;
        sleep_random_ms(300, 500);

        let target_text = job.最少实习月数.trim();
        let items = self.page.eles(".ui-select-item")?;
        let mut selected = false;

        for item in items {
            let text = item.text()?;
            if text.trim() == target_text {
                item.click()
                    .map_err(BossError::map_element("click month option failed"))?;
                selected = true;
                log::info!("selected intern months: {}", target_text);
                break;
            }
        }

        if !selected {
            return Err(BossError::element(format!(
                "month option not found: {}",
                target_text
            )));
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

        let row_selectors = [
            ".requirements-info-content .publish-edit-form-row",
            ".publish-edit-form-row",
            ".form-row.job-experience-row",
            ".form-row",
        ];

        let mut target_row = None;

        for row_selector in row_selectors {
            let rows = match self.page.eles(row_selector) {
                Ok(rows) => rows,
                Err(_) => continue,
            };

            for row in rows {
                let title_el = match row.element(".publish-title") {
                    Ok(Some(el)) => el,
                    _ => match row.element(".title") {
                        Ok(Some(el)) => el,
                        _ => continue,
                    },
                };

                if let Ok(text) = title_el.text() {
                    if text.contains("实习要求") {
                        target_row = Some(row);
                        break;
                    }
                }
            }

            if target_row.is_some() {
                break;
            }
        }

        let row =
            target_row.ok_or_else(|| BossError::element("intern requirement row not found"))?;

        let selects = row
            .elements(".ui-select-selection")
            .map_err(BossError::map_element(
                "intern requirement selects not found",
            ))?;

        // “实习要求”通常有两个下拉：第一个是实习月数，第二个是周到岗天数。
        let days_select = if let Some(select) = selects.get(1) {
            select
        } else if let Some(select) = selects.get(0) {
            select
        } else {
            return Err(BossError::element("days select not found"));
        };

        days_select
            .click()
            .map_err(BossError::map_element("click days select failed"))?;
        sleep_random_ms(300, 500);

        let target_text = job.最少周到岗天数.trim();
        let items = self.page.eles(".ui-select-item")?;
        let mut selected = false;

        for item in items {
            let text = item.text()?;
            if text.trim() == target_text {
                item.click()
                    .map_err(BossError::map_element("click days option failed"))?;
                selected = true;
                log::info!("selected intern days: {}", target_text);
                break;
            }
        }

        if !selected {
            return Err(BossError::element(format!(
                "days option not found: {}",
                target_text
            )));
        }

        sleep_random_ms(400, 500);
        Ok(())
    }
}
