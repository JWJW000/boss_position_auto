use super::*;

impl<'a> Poster<'a> {
    /// Select the recruitment type before the rest of the form renders.
    pub(super) fn fill_job_type(&mut self, job: &JobRecord) -> BResult<()> {
        let recruit_type = job.招聘类型.trim();
        if !Self::has_excel_value(recruit_type) {
            return Err(BossError::element("Excel中的招聘类型为空"));
        }
        let target_text = RecruitmentKind::parse(recruit_type)?.label();

        let target_clean = target_text.replace(char::is_whitespace, "");

        // 1) 等待招聘类型区域出现
        let mut ready = false;
        for _ in 0..20 {
            if let Ok(Some(_)) = self.page.ele("css:.job-type-container .recruitment-type-content") {
                ready = true;
                break;
            }
            sleep_random_ms(900, 1300);
        }

        if !ready {
            return Err(BossError::element("招聘类型区域未加载"));
        }

        // 2) 读取候选项并定位目标元素
        let items = self
            .page
            .eles("css:.job-type-container .job-type-item")
            .map_err(BossError::map_cdp("查找招聘类型列表失败"))?;

        if items.is_empty() {
            return Err(BossError::element("招聘类型候选项为空"));
        }

        let mut debug_texts = Vec::new();
        let mut target_el = None;

        for item in items {
            let text = item
                .text()
                .unwrap_or_default()
                .replace('\n', "")
                .replace(' ', "")
                .trim()
                .to_string();

            debug_texts.push(text.clone());

            if text == target_clean || text.contains(&target_clean) || target_clean.contains(&text) {
                target_el = Some(item);
            }
        }

        log::info!("  [DEBUG] 招聘类型候选项: {:?}", debug_texts);

        let el = target_el.ok_or_else(|| {
            BossError::element(format!("未找到招聘类型选项: {}", target_text))
        })?;

        // 3) 滚动到可见区域
        let _ = el.run_js("this.scrollIntoView({block:'center', inline:'center'});");
        sleep_random_ms(900, 1300);

        // 4) 先尝试原生点击
        if let Err(e) = el.click() {
            log::warn!("  [WARN] 招聘类型原生点击失败: {}", e);

            // 5) 原生点击失败时，补一次 JS 事件点击
            let js_click = format!(
                r#"
                (() => {{
                    const targetText = '{}';
                    const clean = s => (s || '').replace(/\s+/g, '');
                    const items = Array.from(document.querySelectorAll('.job-type-container .job-type-item'));
                    const target = items.find(el => {{
                        const txt = clean(el.innerText);
                        return txt === targetText || txt.includes(targetText) || targetText.includes(txt);
                    }});
                    if (!target) return 'not_found';

                    target.scrollIntoView({{ block: 'center', inline: 'center' }});

                    ['pointerdown', 'mousedown', 'mouseup', 'click'].forEach(type => {{
                        target.dispatchEvent(new MouseEvent(type, {{
                            view: window,
                            bubbles: true,
                            cancelable: true
                        }}));
                    }});

                    return 'clicked';
                }})()
                "#,
                target_clean
            );

            let js_result = self
                .page
                .run_js(&js_click)
                .map_err(BossError::map_cdp("招聘类型JS点击失败"))?
                .get("value")
                .and_then(|x| x.as_str())
                .unwrap_or("unknown")
                .to_string();

            log::info!("  [DEBUG] 招聘类型JS点击结果: {}", js_result);

            if js_result == "not_found" {
                return Err(BossError::element(format!(
                    "未找到招聘类型选项: {}",
                    target_text
                )));
            }
        } else {
            log::info!("  [DEBUG] 招聘类型原生点击已执行: {}", target_text);
        }

        // 6) 点击后页面会重渲染，这里不要再做强校验，只等待页面稳定
        sleep_random_ms(900, 1300);

        log::info!("  [√] 招聘类型: {}", target_text);
        Ok(())
    }

}

