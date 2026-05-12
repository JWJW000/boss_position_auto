use super::*;

impl<'a> Poster<'a> {
    /// Fill job category / 职位类型.
    pub(super) fn fill_job_category(&mut self, job: &JobRecord) -> BResult<()> {
        let cat = job.职位类型.trim();

        if !Self::has_excel_value(cat) {
            log::warn!("  [跳过] 职位类型为空");
            return Ok(());
        }

        let selectors = [
            "css:input[name='jobCategory']",
            "css:.publish-component input[name='jobCategory']",
            "css:.form-row input[name='jobCategory']",
            "css:.publish-edit-form-row input[name='jobCategory']",
            "css:.publish-component input[placeholder*='职位类型']",
            "css:.form-row input[placeholder*='职位类型']",
            "css:.publish-edit-form-row input[placeholder*='职位类型']",
            "css:.publish-component input[placeholder*='选择职位类型']",
            "css:.form-row input[placeholder*='选择职位类型']",
            "css:.publish-edit-form-row input[placeholder*='选择职位类型']",
            "css:.publish-component .job-category-container .ipt-wrap input",
            "css:.form-row .job-category-container .ipt-wrap input",
            "css:.publish-edit-form-row .job-category-container .ipt-wrap input",
            "css:.job-category-container input[name='jobCategory']",
            "css:.job-category-container input[placeholder*='职位类型']",
            "css:.job-category-container input[placeholder*='选择职位类型']",
            "css:.job-category-container .ipt-wrap input",
            "xpath://input[contains(@placeholder, '职位类型')]",
            "xpath://div[contains(@class, 'publish-title') and contains(text(), '职位类型')]/following-sibling::div//input",
        ];

        let el = self.wait_and_find(
            &selectors.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            5000,
        )
        .map_err(|_| BossError::element("未找到职位类型输入框"))?;

        let _ = el.run_js("this.scrollIntoView({block:'center', inline:'center'});");
        std::thread::sleep(Duration::from_millis(300));

        el.click()
            .map_err(BossError::map_post("点击职位类型输入框失败"))?;

        sleep_random_ms(1200, 1800);

        if self.select_job_category_from_dialog(cat)? {
            log::info!("  [√] 职位类型: {}", cat);
            return Ok(());
        }

        Err(BossError::element(format!(
            "职位类型未从弹窗中成功选择: {}",
            cat
        )))
    }

    /// Search inside the BOSS category dialog and click the matching category.
    pub(super) fn select_job_category_from_dialog(&mut self, category: &str) -> BResult<bool> {
        let category_json =
            serde_json::to_string(category).map_err(BossError::map_config("职位类型序列化失败"))?;

        // 1. 等待职位类型弹窗出现 (增强版选择器)
        let dialog_selectors = [
            ".job-mormal-position-select-dialog",
            ".job-category-dialog",
            ".job-category-container-dialog",
            ".boss-dialog",
            ".ui-dialog",
            ".dialog-category",
            ".position-selecter-dialog",
            ".job-category-select-wrap",
            ".job-recommend-footer", // 用户提供的新版推荐弹窗标识
            ".job-category-tag-container" // 用户提供的新版推荐弹窗标识
        ];

        let mut dialog_found = false;
        for _ in 0..15 {
            let check_js = format!(r#"
                (function() {{
                    const selectors = {selectors:?};
                    for (const sel of selectors) {{
                        const el = document.querySelector(sel);
                        if (el && window.getComputedStyle(el).display !== 'none') return true;
                    }}
                    // 额外检查特定的类名
                    if (document.querySelector('.job-recommend-content_title')) return true;
                    return false;
                }})()
            "#, selectors = dialog_selectors);

            if let Ok(v) = self.page.run_js(&check_js) {
                if v.get("value").and_then(|x| x.as_bool()).unwrap_or(false) {
                    dialog_found = true;
                    break;
                }
            }
            sleep_random_ms(800, 1200);
        }

        if !dialog_found {
            log::warn!("  [WARN] 职位类型弹窗未出现");
            return Ok(false);
        }

        log::info!("  [职位类型] 弹窗已出现，尝试搜索或直接选择: {}", category);

        // 2. 尝试搜索并选择
        let select_script = format!(
            r#"
        (async () => {{
            const category = {category};
            const dialogSelectors = {dialog_selectors:?};
            
            // 优先检查用户提到的“推荐职位类型”弹窗结构
            const recommendItems = document.querySelectorAll('.job-recommend-content_title');
            if (recommendItems.length > 0) {{
                // 用户要求：出现这样的弹窗默认点第一个
                const first = recommendItems[0];
                const text = first.innerText.trim();
                first.click();
                return {{ ok: true, method: 'recommend-first', text: text }};
            }}

            let dialog = null;
            for (const sel of dialogSelectors) {{
                const d = document.querySelector(sel);
                if (d && window.getComputedStyle(d).display !== 'none') {{
                    dialog = d;
                    break;
                }}
            }}
            
            if (!dialog) {{
                // 如果没找到明确的 dialog 容器但有推荐项，上面已经处理了。
                // 否则看是否有通用的 boss-popup 之类的关闭按钮存在来判定弹窗
                if (!document.querySelector('.boss-popup__close')) return {{ ok: false, msg: 'dialog lost' }};
                dialog = document.body; // 兜底到 body 查找
            }}

            // 查找搜索输入框
            const input = dialog.querySelector(`
                input.ui-select-input,
                .position-search input,
                input[placeholder*="职位"],
                input[placeholder*="搜索"],
                input[placeholder]
            `);

            if (input) {{
                input.focus();
                input.value = category;
                input.dispatchEvent(new InputEvent('input', {{ bubbles: true, inputType: 'insertText', data: category }}));
                input.dispatchEvent(new Event('change', {{ bubbles: true }}));
                
                // 等待下拉列表加载
                await new Promise(r => setTimeout(r, 1200));

                // 查找下拉项
                const items = Array.from(document.querySelectorAll('.ui-select-dropdown .ui-dropdown-list li, .suggest-list li, .position-suggest-list li'));
                if (items.length > 0) {{
                    // 尝试精确匹配或最接近匹配
                    let best = items[0];
                    let bestScore = 3;
                    const target = category.toLowerCase();

                    for (const it of items) {{
                        const text = (it.innerText || '').trim().toLowerCase();
                        if (text === target) {{ best = it; bestScore = 0; break; }}
                        if (text.startsWith(target)) {{ if (bestScore > 1) {{ best = it; bestScore = 1; }} }}
                        else if (text.includes(target)) {{ if (bestScore > 2) {{ best = it; bestScore = 2; }} }}
                    }}
                    
                    best.click();
                    return {{ ok: true, method: 'search', text: (best.innerText || '').trim() }};
                }}
            }}

            // 3. 如果搜索失败或无输入框，尝试点击第一个分类/职位 (用户要求的 fallback)
            const firstItem = dialog.querySelector(`
                .job-recommend-content_title, 
                .position-list li, 
                .category-list li, 
                .item-list li, 
                .position-item, 
                .category-item
            `);
            if (firstItem) {{
                const text = firstItem.innerText.trim();
                firstItem.click();
                return {{ ok: true, method: 'fallback-first', text: text }};
            }}

            // 兜底：点击任何看起来像选项的东西
            const anyItem = dialog.querySelector('li, .item, [class*="item"]');
            if (anyItem) {{
                const text = anyItem.innerText.trim();
                anyItem.click();
                return {{ ok: true, method: 'fallback-any', text: text }};
            }}

            return {{ ok: false, msg: 'no items found in dialog' }};
        }})()
        "#,
            category = category_json,
            dialog_selectors = dialog_selectors
        );

        let result = self
            .page
            .run_js_await(&select_script)
            .map_err(BossError::map_cdp("职位类型选择脚本执行失败"))?;

        let res_val = result.get("value");
        let ok = res_val.and_then(|v| v.get("ok")).and_then(|v| v.as_bool()).unwrap_or(false);
        let method = res_val.and_then(|v| v.get("method")).and_then(|v| v.as_str()).unwrap_or("unknown");
        let text = res_val.and_then(|v| v.get("text")).and_then(|v| v.as_str()).unwrap_or("");

        if ok {
            log::info!("  [职位类型] 已通过 [{}] 选中: {}", method, text);
            sleep_random_ms(800, 1200);
            return Ok(true);
        }

        log::warn!("  [职位类型] 选择失败: {:?}", result);
        Ok(false)
    }
}
