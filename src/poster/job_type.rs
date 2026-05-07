use super::*;

impl<'a> Poster<'a> {
    /// Select the recruitment type before the rest of the form renders.
    pub(super) fn fill_job_type(&mut self, job: &JobRecord) -> BResult<()> {
        let recruit_type = job.招聘类型.trim();
        if !Self::has_excel_value(recruit_type) {
            return Err(BossError::element("Excel中的招聘类型为空"));
        }
        let target_text = RecruitmentKind::parse(recruit_type)?.label();
        let target_clean = Self::clean_text(target_text);

        // 1) 等待招聘类型区域出现。不同账号/浏览器可能拿到不同 BOSS 页面版本，
        // 所以这里用多选择器 + 文本快照，而不是只依赖固定的 class 层级。
        log::info!("  [DEBUG] 开始等待招聘类型区域加载...");
        let mut items = Vec::new();
        let mut last_debug_texts = Vec::new();
        for attempt in 0..50 {
            items = self.collect_job_type_items();
            last_debug_texts = Self::job_type_texts(&items);
            if !items.is_empty() {
                log::info!("  [DEBUG] 招聘类型区域已加载，找到 {} 个候选元素", items.len());
                break;
            }

            if attempt % 5 == 0 && attempt > 0 {
                let page_state = self.job_type_page_state();
                log::info!(
                    "  [DEBUG] 等待招聘类型区域... (尝试 {}/50) {}",
                    attempt,
                    page_state
                );
            }
            sleep_random_ms(900, 1300);
        }

        if items.is_empty() {
            let page_state = self.job_type_page_state();
            log::error!("  [ERROR] 招聘类型区域加载超时（已等待50次重试）{}", page_state);
            return Err(BossError::element("招聘类型区域未加载"));
        }

        // 2) 读取候选项并定位目标元素
        log::info!("  [DEBUG] 招聘类型候选项: {:?}", last_debug_texts);
        let target_el = Self::best_job_type_match(items, &target_clean);

        // 3) 滚动到可见区域并等待渲染稳定
        let clicked = if let Some(el) = target_el {
            let _ = el.run_js("this.scrollIntoView({block:'center', inline:'center', behavior:'smooth'});");
            sleep_random_ms(900, 1300);
            match el.click() {
                Ok(()) => {
                    log::info!("  [DEBUG] 招聘类型原生点击已执行: {}", target_text);
                    true
                }
                Err(e) => {
                    log::warn!("  [WARN] 招聘类型原生点击失败: {}", e);
                    false
                }
            }
        } else {
            false
        };

        // 4) 找不到精确元素或原生点击失败时，用文本兜底点击。
        if !clicked {
            let js_result = self.click_job_type_by_text(&target_clean)?;
            log::info!("  [DEBUG] 招聘类型JS点击结果: {}", js_result);
            if js_result != "clicked" {
                return Err(BossError::element(format!(
                    "未找到招聘类型选项: {}，候选项={:?}",
                    target_text,
                    last_debug_texts
                )));
            }
        }

        // 5) 点击后页面会重渲染，等待页面稳定
        sleep_random_ms(1500, 2000);

        log::info!("  [√] 招聘类型: {}", target_text);
        Ok(())
    }

    fn collect_job_type_items(&mut self) -> Vec<rust_drission::Element> {
        let mut selectors = vec![
            "css:.job-type-container .job-type-item".to_string(),
            "css:.recruitment-type-content .job-type-item".to_string(),
            "css:.job-type-item".to_string(),
            "xpath://p[contains(@class,'job-type-item')]".to_string(),
            "xpath://*[contains(@class,'recruitment-type-content')]//*[contains(@class,'job-type-item')]".to_string(),
        ];
        selectors.extend(self.selectors.job_type.iter().cloned());

        for sel in selectors {
            if let Ok(items) = self.page.eles(&sel) {
                let items: Vec<_> = items
                    .into_iter()
                    .filter(|el| !Self::clean_text(&el.text().unwrap_or_default()).is_empty())
                    .collect();
                if !items.is_empty() {
                    log::info!("  [DEBUG] 招聘类型选择器命中: {}", sel);
                    return items;
                }
            }
        }
        Vec::new()
    }

    fn job_type_texts(items: &[rust_drission::Element]) -> Vec<String> {
        let mut texts = Vec::new();
        for item in items {
            let text = Self::clean_text(&item.text().unwrap_or_default());
            if !text.is_empty() {
                texts.push(text);
            }
        }
        texts.sort();
        texts.dedup();
        texts
    }

    fn best_job_type_match(
        items: Vec<rust_drission::Element>,
        target_clean: &str,
    ) -> Option<rust_drission::Element> {
        let mut best: Option<(usize, usize, rust_drission::Element)> = None;
        let target_len = target_clean.chars().count();
        for item in items {
            let text = Self::clean_text(&item.text().unwrap_or_default());
            if text.is_empty() {
                continue;
            }

            let score = if text == target_clean {
                0
            } else if text.contains(target_clean) {
                1
            } else if target_clean.contains(&text) {
                2
            } else {
                continue;
            };

            let len = text.chars().count();
            if score > 0 && len > target_len + 8 {
                continue;
            }
            let replace = best
                .as_ref()
                .map(|(best_score, best_len, _)| score < *best_score || (score == *best_score && len < *best_len))
                .unwrap_or(true);
            if replace {
                best = Some((score, len, item));
            }
        }
        best.map(|(_, _, item)| item)
    }

    fn click_job_type_by_text(&mut self, target_clean: &str) -> BResult<String> {
        let target_json = serde_json::to_string(target_clean)
            .map_err(BossError::map_config("招聘类型文本序列化失败"))?;
        let script = format!(
            r#"
            (() => {{
                const targetText = {target};
                const clean = s => (s || '').replace(/\s+/g, '').trim();
                const visible = el => {{
                    const r = el.getBoundingClientRect();
                    const st = window.getComputedStyle(el);
                    return r.width > 0 && r.height > 0 && st.display !== 'none' && st.visibility !== 'hidden';
                }};
                const selectors = [
                    '.job-type-container .job-type-item',
                    '.recruitment-type-content .job-type-item',
                    '.job-type-item',
                    '[class*="recruit"] p',
                    '[class*="job-type"] p',
                    'p',
                    'button',
                    'span',
                    'div'
                ];
                const seen = new Set();
                const candidates = [];
                for (const selector of selectors) {{
                    for (const el of Array.from(document.querySelectorAll(selector))) {{
                        if (seen.has(el)) continue;
                        seen.add(el);
                        if (!visible(el)) continue;
                        const txt = clean(el.innerText || el.textContent);
                        if (!txt) continue;
                        if (txt !== targetText && txt.length > targetText.length + 8) continue;
                        if (txt === targetText || txt.includes(targetText) || targetText.includes(txt)) {{
                            candidates.push({{ el, txt }});
                        }}
                    }}
                }}
                candidates.sort((a, b) => {{
                    const score = x => x.txt === targetText ? 0 : (x.txt.includes(targetText) ? 1 : 2);
                    return score(a) - score(b) || a.txt.length - b.txt.length;
                }});
                const target = candidates[0] && candidates[0].el;
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
            target = target_json
        );

        self.page
            .run_js(&script)
            .map_err(BossError::map_cdp("招聘类型JS点击失败"))
            .map(|v| {
                v.get("value")
                    .and_then(|x| x.as_str())
                    .unwrap_or("unknown")
                    .to_string()
            })
    }

    fn job_type_page_state(&mut self) -> String {
        let script = r#"
            (() => {
                const clean = s => (s || '').replace(/\s+/g, ' ').trim();
                const texts = Array.from(document.querySelectorAll('p,button,span,label'))
                    .map(el => clean(el.innerText || el.textContent))
                    .filter(Boolean)
                    .filter(t => /招聘|社招|校园|实习|兼职|全职/.test(t))
                    .slice(0, 12);
                return JSON.stringify({
                    url: location.href,
                    title: document.title,
                    viewport: `${window.innerWidth}x${window.innerHeight}`,
                    hints: texts
                });
            })()
        "#;
        self.page
            .run_js(script)
            .ok()
            .and_then(|v| v.get("value").and_then(|x| x.as_str()).map(str::to_string))
            .unwrap_or_else(|| "页面状态读取失败".to_string())
    }
}

