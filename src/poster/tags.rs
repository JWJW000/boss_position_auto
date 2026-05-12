use super::*;

impl<'a> Poster<'a> {
    /// Fill the job keyword tags.
    ///
    /// 兼容三种主要的页面结构：
    /// 1. 页面直接输入：直接在表单页找到 input 输入。
    /// 2. 弹窗直接输入：打开弹窗后，弹窗内是个 input 输入框。
    /// 3. 弹窗选择 + 自定义：打开弹窗后，有预设的标签组（.question-item 或 .skill-group-item）可选，也可手动新增。
    pub(super) fn fill_tags(&mut self, job: &JobRecord) -> BResult<()> {
        let mut tags = Self::split_tag_items(&job.关键词);
        if tags.is_empty() {
            return Ok(());
        }

        log::info!("  [DEBUG] 职位标签拆分: {:?}", tags);

        // ==================== 方式一：页面直接输入 (无需弹窗) ====================
        if self.try_fill_tags_by_direct_input(&tags)? {
            log::info!("  [√] 职位关键词: 已通过页面直接输入方式填写 {} 个", tags.len());
            return Ok(());
        }

        // ==================== 准备进入弹窗逻辑 ====================
        let opened = self.open_custom_tag_dialog()?;
        if !opened {
            log::warn!("  [跳过] 未找到职位关键词入口，且页面无直接输入框");
            return Ok(());
        }

        sleep_random_ms(500, 800);

        // ==================== 方式二：弹窗内直接输入 ====================
        // 部分职位点击后弹出的直接就是一个输入框
        if self.try_fill_tags_by_direct_input(&tags)? {
            log::info!("  [√] 职位关键词: 已通过弹窗输入框方式填写 {} 个", tags.len());
            self.try_click_tag_final_confirm()?;
            return Ok(());
        }

        // ==================== 方式三：弹窗内的标签组 / 技能组 ====================
        // 兼容旧版 .question-item 和新版 .skill-group-item (如 Java架构师 弹窗)
        // 增加循环检测逻辑，处理点击一个选项后出现新必填项的情况
        for attempt in 1..=5 {
            let skill_group_selectors = [".question-item", ".skill-group-item"];
            let mut groups = Vec::new();
            for sel in skill_group_selectors {
                if let Ok(eles) = self.page.eles(sel) {
                    if !eles.is_empty() {
                        groups = eles;
                        break;
                    }
                }
            }

            if groups.is_empty() {
                if attempt == 1 {
                    log::info!("  [提示] 未在弹窗中找到预定义标签组，尝试自定义新增");
                }
                break;
            }

            log::info!("  [开始] 弹窗标签处理 (尝试 {}/5, 组数: {})", attempt, groups.len());
            let mut changed = false;

            for group in groups {
                let option_selectors = [".question-option li", ".skill-item", ".group-content span"];
                let mut options = Vec::new();
                for sel in option_selectors {
                    if let Ok(eles) = group.elements(sel) {
                        if !eles.is_empty() {
                            options = eles;
                            break;
                        }
                    }
                }

                if options.is_empty() {
                    continue;
                }

                // 检查该组是否已选 (部分结构可以通过 class 判断)
                let mut group_has_selection = false;
                for opt in &options {
                    if let Ok(c) = opt.attr("class") {
                        if c.contains("active") || c.contains("selected") {
                            group_has_selection = true;
                            break;
                        }
                    }
                }

                // 检查是否必填 (仅对 .question-item 有效)
                let is_required = group
                    .element(".question-title .required")
                    .map(|x| x.is_some())
                    .unwrap_or(false);

                let mut current_group_filled = false;

                for opt in options.iter() {
                    let text = opt.text_content().unwrap_or_default().trim().to_string();
                    if tags.contains(&text) {
                        let is_selected = opt
                            .attr("class")
                            .map(|c| c.contains("active") || c.contains("selected"))
                            .unwrap_or(false);

                        if !is_selected {
                            if opt.click().is_ok() {
                                changed = true;
                                sleep_random_ms(200, 400);
                            }
                        }

                        tags.retain(|x| x != &text);
                        current_group_filled = true;
                        group_has_selection = true;
                        log::info!("    [选中] {}", text);
                    }
                }

                // 兜底逻辑：如果是必填且目前仍然没有任何选中项，则强制点第一个
                if is_required && !group_has_selection {
                    if let Some(first) = options.first() {
                        let first_text = first.text_content().unwrap_or_default();
                        log::info!("    [必填兜底] 组未选择，默认选第一个: {}", first_text);
                        if first.click().is_ok() {
                            changed = true;
                            sleep_random_ms(200, 400);
                        }
                    }
                }
            }

            if !changed {
                log::info!("  [完成] 标签组处理已稳定，无更多变化");
                break;
            }
            // 如果发生了点击，可能触发了新 UI，循环下一次继续检测
            sleep_random_ms(500, 800);
        }

        // ==================== 方式四：手动新增剩余的自定义标签 ====================
        if !tags.is_empty() {
            log::info!("  [自定义] 剩余未填写的标签: {:?}", tags);
            for tag in tags {
                // 弹窗内的 "新增" 按钮通常是 .add-skill 或 .job-skill-add
                let add_btn_selectors = [".add-skill", ".job-skill-add", ".add-keyword"];
                let mut found_add = false;
                for sel in add_btn_selectors {
                    if let Ok(Some(btn)) = self.page.ele(sel) {
                        if btn.click().is_ok() {
                            sleep_random_ms(400, 600);
                            if self.try_input_custom_tag_by_js(&tag)? {
                                if self.try_click_custom_tag_confirm()? {
                                    log::info!("    [新增成功] {}", tag);
                                    found_add = true;
                                    break;
                                }
                            }
                        }
                    }
                }
                if !found_add {
                    log::warn!("    [新增失败] 未找到自定义输入入口或确认按钮: {}", tag);
                }
                sleep_random_ms(400, 600);
            }
        }

        // 最终点击确认关闭弹窗
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
                // 必须是可见的且不是弹窗外的遮罩等
                if ele.is_displayed().unwrap_or(false) {
                    input_ele = Some(ele);
                    break;
                }
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

            input_ele.click().ok();
            sleep_random_ms(150, 250);

            input_ele.input(tag).ok();
            sleep_random_ms(200, 350);

            input_ele.input("\n").ok();
            sleep_random_ms(300, 500);

            log::info!("    [输入] {}", tag);
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
                    if (input && input.offsetParent !== null) break;
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
                    value: input.value
                }};
            }})();
            "#,
            tag = tag
        );

        let ret = self.page.run_js(&input_script).ok();
        let res_str = format!("{:?}", ret);
        Ok(res_str.contains("ok") && res_str.contains("true"))
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
                if btn.is_displayed().unwrap_or(false) {
                    btn.click().ok();
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    /// 点击关键词总弹窗的最终确认按钮。使用 JS 增强识别和点击。
    fn try_click_tag_final_confirm(&mut self) -> BResult<bool> {
        let confirm_script = r#"
            (function() {
                const selectors = [
                    ".job-skill-dialog .btn-v2.btn-sure-v2",
                    ".job-skill-dialog .btn-sure-v2",
                    ".boss-dialog__footer .btn-primary",
                    ".boss-dialog__footer .btn-sure",
                    ".job-skill-select-wrap .btn-sure-v2",
                    ".btns .btn-sure-v2",
                    ".ui-dialog .btn-sure-v2",
                    ".dialog .btn-sure-v2",
                    ".btn-sure-v2"
                ];

                function isVisible(el) {
                    const style = window.getComputedStyle(el);
                    return style.display !== 'none' && style.visibility !== 'hidden' && el.offsetParent !== null;
                }

                // 1. 尝试选择器
                for (const sel of selectors) {
                    const btn = document.querySelector(sel);
                    if (btn && isVisible(btn)) {
                        btn.click();
                        return { ok: true, method: "selector", selector: sel };
                    }
                }

                // 2. 尝试文本匹配 (更通用)
                const allElements = document.querySelectorAll('button, span, a');
                for (const el of allElements) {
                    const text = el.innerText || el.textContent || "";
                    if (isVisible(el) && (text.trim() === "确定" || text.trim() === "确认")) {
                        // 优先检查是否在弹窗内
                        if (el.closest('.boss-dialog') || el.closest('.dialog') || el.closest('.ui-dialog') || el.closest('.job-skill-select-wrap')) {
                            el.click();
                            return { ok: true, method: "text-match", text: text.trim() };
                        }
                    }
                }

                return { ok: false, msg: "confirm button not found or not visible" };
            })();
        "#;

        let ret = self.page.run_js(confirm_script).ok();
        let res_str = format!("{:?}", ret);
        if res_str.contains("ok") && res_str.contains("true") {
            log::info!("[Info] 已通过脚本点击确认按钮: {}", res_str);
            sleep_random_ms(500, 800);
            return Ok(true);
        }

        log::info!("[Info] 未找到关键词最终确认按钮: {}", res_str);
        Ok(false)
    }

    /// Split the Excel keyword cell into clean single tags.
    fn split_tag_items(raw: &str) -> Vec<String> {
        raw.split(|c: char| c == ',' || c == '，' || c == ';' || c == '；' || c.is_whitespace())
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
                if ele.is_displayed().unwrap_or(false) {
                    ele.click().map_err(BossError::map_element("点击关键词入口失败"))?;
                    sleep_random_ms(500, 800);
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }
}
