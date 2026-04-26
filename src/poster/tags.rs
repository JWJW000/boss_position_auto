use super::*;

impl<'a> Poster<'a> {
    /// Fill the job keyword tags through the custom keyword dialog.
    pub(super) fn fill_tags(&mut self, job: &JobRecord) -> BResult<()> {
        let mut tags = Self::split_tag_items(&job.关键词);
        if tags.is_empty() {
            return Ok(());
        }

        log::info!("  [DEBUG] 职位标签拆分: {:?}", tags);

        self.open_custom_tag_dialog()?;

        let question_items = self
    .page
    .eles(".question-item")
    .map_err(BossError::map_element("未找到标签题目组"))?;

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
            let add_skill_btn = self
                .page
                .ele(".add-skill")
                .map_err(BossError::map_element("未找到自定义招聘偏好入口"))?
                .ok_or_else(|| BossError::element("自定义招聘偏好入口不存在"))?;

            add_skill_btn
                .click()
                .map_err(BossError::map_element("点击自定义招聘偏好入口失败"))?;

            sleep_random_ms(300, 500);

            let input_script = format!(
                r#"
                (function() {{
                    const input = document.querySelector(".job-skill-add-container input.ipt");
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
                        className: input.className
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

            sleep_random_ms(500, 800);

            let dialog = self
                .page
                .ele(".job-skill-add-dialog")
                .map_err(BossError::map_element("未找到关键词弹窗"))?
                .ok_or_else(|| BossError::element("关键词弹窗不存在"))?;

            let sure_btn = dialog
                .element(".btn-sure-v2")
                .map_err(BossError::map_element("未找到确认按钮"))?
                .ok_or_else(|| BossError::element("确认按钮不存在"))?;

            sure_btn
                .click()
                .map_err(BossError::map_element("点击确认按钮失败"))?;

            log::info!("[Info] 已新增自定义关键词: {}", tag);

            sleep_random_ms(500, 800);
        }
        let final_btn = self.page.ele(".btn-v2.btn-sure-v2")
        .map_err(BossError::map_element("未找到确认按钮"))?
        .ok_or_else(|| BossError::element("确认按钮不存在"))?;
        
        final_btn.click().map_err(BossError::map_element("点击最终确认按钮失败"))?;

        Ok(())
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
        let custom_tag_dialog_ele = self
            .page
            .ele(".add-skill")
            .map_err(BossError::map_cdp("查找关键词入口失败"))?;

        if custom_tag_dialog_ele.is_none() {
            return Ok(false);
        }

        custom_tag_dialog_ele
            .unwrap()
            .click()
            .map_err(BossError::map_element("点击关键词入口失败"))?;

        sleep_random_ms(200, 500);

        Ok(true)
    }
}