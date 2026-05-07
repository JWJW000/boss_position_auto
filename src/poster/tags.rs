use super::*;

impl<'a> Poster<'a> {
    /// Fill the job keyword tags.
    ///
    /// 兼容两种页面：
    /// 1. 旧版：点击 .add-skill 后出现自定义关键词弹窗 / 标签题目组。
    /// 2. 新版：页面上直接有关键词输入框，不依赖弹窗。
    pub(super) fn fill_tags(&mut self, job: &JobRecord) -> BResult<()> {
        let mut tags = Self::split_tag_items(&job.关键词);
        if tags.is_empty() {
            return Ok(());
        }

        log::info!("  [DEBUG] 职位标签拆分: {:?}", tags);

        // ==================== 方式一：新版页面，直接输入关键词 ====================
        // form-row 风格页面通常没有旧版关键词弹窗，优先尝试直接找输入框。
        if self.try_fill_tags_by_direct_input(&tags)? {
            log::info!("  [√] 职位关键词: 已通过直接输入方式填写 {} 个", tags.len());
            return Ok(());
        }

        // ==================== 方式二：旧版页面，打开关键词弹窗 ====================
        let opened = self.open_custom_tag_dialog()?;
        if !opened {
            log::warn!("  [跳过] 未找到职位关键词入口，也未找到直接输入框");
            return Ok(());
        }

        sleep_random_ms(300, 500);

        // 弹窗打开后，再尝试一次直接输入。部分页面点击入口后出现的是输入框，不是 .question-item。
        if self.try_fill_tags_by_direct_input(&tags)? {
            log::info!("  [√] 职位关键词: 已通过弹出输入框方式填写 {} 个", tags.len());
            self.try_click_tag_final_confirm()?;
            return Ok(());
        }

        // 旧版问题组选项逻辑。
        let question_items = match self.page.eles(".question-item") {
            Ok(items) => items,
            Err(_) => Vec::new(),
        };

        if question_items.is_empty() {
            log::warn!("  [跳过] 未找到标签题目组，可能当前页面不需要填写关键词弹窗");
            return Ok(());
        }

        for question in question_items {
            // 必须放在点击 li 之前，避免点击后 Vue 重渲染导致 question 节点失效
            let is_required = question
                .element(".question-title .required")
                .map(|x| x.is_some())
                .unwrap_or(false);

            let li_eles = question
                .elements(".question-option li")
                .map_err(BossError::map_element("未找到标签选项"))?;

            if li_eles.is_empty() {
                continue;
            }

            let mut selected_count = 0;

            for li_ele in li_eles.iter() {
                let li_content = li_ele
                    .text_content()
                    .map_err(BossError::map_element("读取标签文本失败"))?
                    .trim()
                    .to_string();

                if tags.contains(&li_content) {
                    li_ele
                        .click()
                        .map_err(BossError::map_element("点击标签失败"))?;

                    tags.retain(|x| x != &li_content);
                    selected_count += 1;

                    log::info!("[Info] 已选择匹配标签: {}", li_content);
                    sleep_random_ms(80, 150);
                }
            }

            if selected_count == 0 {
                if is_required {
                    let first_li = li_eles
                        .first()
                        .ok_or_else(|| BossError::element("必填标签组没有可选项"))?;

                    let first_text = first_li
                        .text_content()
                        .unwrap_or_else(|_| "<读取失败>".to_string());

                    log::info!(
                        "[Info] 必填标签组未匹配到关键词，默认选择第一个: {}",
                        first_text
                    );

                    first_li
                        .click()
                        .map_err(BossError::map_element("点击必填标签组默认选项失败"))?;
                } else {
                    log::info!("[Info] 非必填标签组未匹配到关键词，跳过默认选择");
                }
            }

            sleep_random_ms(100, 200);
        }

        log::info!("[Debug] 需要自定义新增的关键字: {:?}", tags);

        for tag in tags {
            let add_skill_btn = match self.page.ele(".add-skill") {
                Ok(Some(ele)) => ele,
                _ => {
                    log::warn!("[Warn] 未找到自定义招聘偏好入口，跳过自定义关键词: {}", tag);
                    continue;
                }
            };

            add_skill_btn
                .click()
                .map_err(BossError::map_element("点击自定义招聘偏好入口失败"))?;

            sleep_random_ms(300, 500);

            if !self.try_input_custom_tag_by_js(&tag)? {
                log::warn!("[Warn] 未找到自定义关键词输入框，跳过: {}", tag);
                continue;
            }

            sleep_random_ms(500, 800);

            if self.try_click_custom_tag_confirm()? {
                log::info!("[Info] 已新增自定义关键词: {}", tag);
            } else {
                log::warn!("[Warn] 未找到自定义关键词确认按钮，跳过确认: {}", tag);
            }

            sleep_random_ms(500, 800);
        }

        self.try_click_tag_final_confirm()?;

        Ok(())
    }

    /// 新版 / form-row 页面：直接在关键词输入框中输入关键词。
    fn try_fill_tags_by_direct_input(&mut self, tags: &[String]) -> BResult<bool> {
        let selectors = [
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

        let mut input_ele = None;
        for selector in selectors {
            if let Ok(Some(ele)) = self.page.ele(selector) {
                input_ele = Some(ele);
                break;
            }
        }

        let Some(input_ele) = input_ele else {
            return Ok(false);
        };

        for tag in tags {
            let tag = tag.trim();
            if tag.is_empty() {
                continue;
            }

            input_ele
                .click()
                .map_err(BossError::map_element("点击关键词输入框失败"))?;
            sleep_random_ms(150, 250);

            input_ele
                .input(tag)
                .map_err(BossError::map_element("输入关键词失败"))?;
            sleep_random_ms(200, 350);

            input_ele
                .input("\n")
                .map_err(BossError::map_element("确认关键词失败"))?;
            sleep_random_ms(300, 500);

            log::info!("[Info] 已输入关键词: {}", tag);
        }

        Ok(true)
    }

    /// 兼容多种弹窗输入框结构。
    fn try_input_custom_tag_by_js(&mut self, tag: &str) -> BResult<bool> {
        let input_script = format!(
            r#"
            (function() {{
                const selectors = [
                    ".job-skill-add-container input.ipt",
                    ".job-skill-add-dialog input.ipt",
                    ".job-skill-add-dialog input",
                    ".job-skill-add-container input",
                    ".add-skill-dialog input",
                    ".ui-dialog input.ipt",
                    ".ui-dialog input",
                    ".dialog input.ipt",
                    ".dialog input",
                    "input[placeholder*='关键词']",
                    "input[placeholder*='标签']"
                ];

                let input = null;
                for (const selector of selectors) {{
                    input = document.querySelector(selector);
                    if (input) break;
                }}

                if (!input) return {{ ok: false, msg: "input not found" }};

                input.focus();

                const value = {tag:?};
                const setter = Object.getOwnPropertyDescriptor(
                    window.HTMLInputElement.prototype,
                    "value"
                ).set;

                setter.call(input, value);
                input.dispatchEvent(new Event("input", {{ bubbles: true }}));
                input.dispatchEvent(new Event("change", {{ bubbles: true }}));

                return {{
                    ok: true,
                    value: input.value,
                    className: input.className,
                    placeholder: input.getAttribute("placeholder") || ""
                }};
            }})();
            "#,
            tag = tag
        );

        let ret = self
            .page
            .run_js(&input_script)
            .map_err(BossError::map_element("写入自定义关键词失败"))?;

        log::info!("[Debug] 自定义关键词写入结果: {:?}", ret);

        Ok(format!("{:?}", ret).contains("ok") && format!("{:?}", ret).contains("true"))
    }

    /// 点击自定义关键词的小弹窗确认按钮。
    fn try_click_custom_tag_confirm(&mut self) -> BResult<bool> {
        let selectors = [
            ".job-skill-add-dialog .btn-sure-v2",
            ".job-skill-add-container .btn-sure-v2",
            ".add-skill-dialog .btn-sure-v2",
            ".ui-dialog .btn-sure-v2",
            ".dialog .btn-sure-v2",
            ".btn-sure-v2",
        ];

        for selector in selectors {
            if let Ok(Some(btn)) = self.page.ele(selector) {
                btn.click()
                    .map_err(BossError::map_element("点击自定义关键词确认按钮失败"))?;
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// 点击关键词总弹窗的最终确认按钮。如果没有按钮，则认为无需确认。
    fn try_click_tag_final_confirm(&mut self) -> BResult<bool> {
        let selectors = [
            ".job-skill-dialog .btn-v2.btn-sure-v2",
            ".job-skill-dialog .btn-sure-v2",
            ".ui-dialog .btn-v2.btn-sure-v2",
            ".ui-dialog .btn-sure-v2",
            ".dialog .btn-v2.btn-sure-v2",
            ".dialog .btn-sure-v2",
            ".btn-v2.btn-sure-v2",
        ];

        for selector in selectors {
            if let Ok(Some(btn)) = self.page.ele(selector) {
                btn.click()
                    .map_err(BossError::map_element("点击最终确认按钮失败"))?;
                sleep_random_ms(300, 500);
                return Ok(true);
            }
        }

        log::info!("[Info] 未找到关键词最终确认按钮，可能当前页面无需确认");
        Ok(false)
    }

    /// Split the Excel keyword cell into clean single tags.
    fn split_tag_items(raw: &str) -> Vec<String> {
        raw.split(|c: char| {
            c == ',' || c == '，' || c == ';' || c == '；' || c.is_whitespace()
        })
        .map(str::trim)
        .filter(|s| !s.is_empty() && *s != "无")
        .map(ToOwned::to_owned)
        .collect()
    }

    /// Open the second-level custom keyword dialog from the preference area.
    fn open_custom_tag_dialog(&mut self) -> BResult<bool> {
        let selectors = [
            ".add-skill",
            ".publish-content .add-skill",
            ".content .add-skill",
            ".job-skill-add",
            ".add-keyword",
            ".add-tag",
        ];

        for selector in selectors {
            let custom_tag_dialog_ele = self
                .page
                .ele(selector)
                .map_err(BossError::map_cdp("查找关键词入口失败"))?;

            if let Some(ele) = custom_tag_dialog_ele {
                ele.click()
                    .map_err(BossError::map_element("点击关键词入口失败"))?;
                sleep_random_ms(200, 500);
                return Ok(true);
            }
        }

        Ok(false)
    }
}
