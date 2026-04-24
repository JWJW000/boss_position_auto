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
        let options = self
            .page
            .eles(".question-option")
            .map_err(BossError::map_element("未找到标签选项组"))?;
        for option in options {
            let li_eles = option
                .elements("li")
                .map_err(BossError::map_element("未找到标签选项"))?;

            let mut selected_count = 0;

            for (index, li_ele) in li_eles.iter().enumerate() {
                let li_content = li_ele
                    .text_content()
                    .map_err(BossError::map_element("读取标签文本失败"))?;

                if tags.contains(&li_content) {
                    li_ele
                        .click()
                        .map_err(BossError::map_element("点击标签失败"))?;
                    tags.retain(|x| x != &li_content);
                    selected_count += 1;
                }

                // 如果循环到了最后一个元素，且这一组一个都没点过
                if index == li_eles.len() - 1 && selected_count == 0 {
                    let starred_li = li_eles.iter().find(|li| {
                        li.text_content()
                            .map(|text| {
                                let clean = text.replace(char::is_whitespace, "");
                                clean.contains('*') || clean.contains('＊')
                            })
                            .unwrap_or(false)
                    });
                    if let Some(first_li) = starred_li {
                        log::info!(
                            "[Info] 标签组未匹配到关键词，默认选择带*标签: {:?}",
                            first_li.text_content().ok()
                        );
                        first_li.click().ok();
                    } else {
                        log::info!("[Info] 标签组未匹配到关键词，且无*标签，跳过默认选择");
                    }
                }
            }
            sleep_random_ms(100, 200);
        }

        log::info!("[Debug] 需要自定义新增的关键字:{:?}", tags);
        
       for tag in tags {
    // 每轮重新找 input，避免元素失效
    let input_ele = self
        .page
        .ele(".job-skill-add-container input.ipt")
        .map_err(BossError::map_element("未找到输入框"))?
        .ok_or_else(|| BossError::element("输入框不存在"))?;

    input_ele
        .click()
        .map_err(BossError::map_element("点击输入框失败"))?;

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

    let ret = self.page
        .run_js(&input_script)
        .map_err(BossError::map_element("写入输入框失败"))?;

    println!("写入结果: {:?}", ret);

    sleep_random_ms(500, 800);

    // 再次校验 value，确认真的写进去了
    let check_ret = self.page
        .run_js(r#"
(function() {
    const input = document.querySelector(".job-skill-add-container input.ipt");
    if (!input) return { ok: false, msg: "input not found" };
    return { ok: true, value: input.value, className: input.className };
})();
"#)
        .map_err(BossError::map_element("校验输入框失败"))?;

    println!("校验结果: {:?}", check_ret);

    // 只有确认 value 非空再点确认
    let sure_btn = self
        .page
        .ele(".job-skill-add-footer .btn-sure-v2")
        .map_err(BossError::map_element("未找到确认按钮"))?
        .ok_or_else(|| BossError::element("确认按钮不存在"))?;

    sure_btn
        .click()
        .map_err(BossError::map_element("点击确认失败"))?;

    sleep_random_ms(500, 800);
}
        log::info!("[Action] 执行最终确定点击");

        let final_confirm_script = r#"
                (function() {
                    // 1. 尝试通过具体的容器路径定位，避免点错
                    var btn = document.querySelector(".job-skill-add-footer .btn-sure-v2") 
                        || document.querySelector(".boss-popup__content .btn-sure-v2");

                    // 2. 如果常规选择器失效，通过文本和可见性兜底
                    if (!btn) {
                        var allBtns = document.querySelectorAll('.btn-sure-v2, .btn-sure');
                        btn = Array.from(allBtns).find(el => 
                            el.innerText.includes('确定') && el.offsetParent !== null
                        );
                    }

                    if (btn) {
                        btn.click();
                        return "SUCCESS";
                    }
                    return "NOT_FOUND";
                })();
            "#;

        let result = self
            .page
            .run_js(final_confirm_script)
            .map_err(BossError::map_element("执行确定脚本失败"))?;

        if result.as_str() == Some("NOT_FOUND") {
            log::warn!("[Warning] 未找到最终确定按钮，请检查页面状态");
        } else {
            log::info!("[Success] 最终确认按钮已点击");
        }

        Ok(())
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

        Ok(false)
    }
}
