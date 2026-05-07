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
        "css:.job-category-container input[name='jobCategory']",
        "css:.job-category-container input[placeholder*='职位类型']",
        "css:.job-category-container input[placeholder*='选择职位类型']",
        "css:.job-category-container .ipt-wrap input",
    ];

    let el = SelectorMap::find_first(
        self.page,
        &selectors.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
    )
    .ok_or_else(|| BossError::element("未找到职位类型输入框 input[name='jobCategory']"))?;

    let _ = el.run_js("this.scrollIntoView({block:'center', inline:'center'});");
    sleep_random_ms(300, 500);

    el.click()
        .map_err(BossError::map_post("点击职位类型输入框失败"))?;

    sleep_random_ms(900, 1300);

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
/// Search inside the BOSS category dialog and click the matching category.
/// 通用版：不依赖固定 aliases，适配计算机 / 销售 / 财务 / 人事 / 运营 / 设计等岗位。
pub(super) fn select_job_category_from_dialog(&mut self, category: &str) -> BResult<bool> {
    let category_json = serde_json::to_string(category)
        .map_err(BossError::map_config("职位类型序列化失败"))?;

    // 1. 等待职位类型弹窗出现
    for _ in 0..10 {
        let found = self
            .page
            .ele("css:.job-mormal-position-select-dialog .position-selecter")
            .map_err(BossError::map_cdp("检查职位类型弹窗状态失败"))?
            .is_some();

        if found {
            break;
        }

        sleep_random_ms(900, 1300);
    }

    let dialog_exists = self
        .page
        .ele("css:.job-mormal-position-select-dialog .position-selecter")
        .map_err(BossError::map_cdp("检查职位类型弹窗状态失败"))?
        .is_some();

    if !dialog_exists {
        log::warn!("  [WARN] 职位类型弹窗未出现");
        return Ok(false);
    }

    // 2. 在弹窗搜索框输入 Excel 中的职位类型
    let input_script = format!(
        r#"
        (() => {{
            const dialog = document.querySelector('.job-mormal-position-select-dialog');
            if (!dialog) return {{ ok: false, msg: 'dialog not found' }};

            const input = dialog.querySelector(`
                input.ui-select-input,
                .position-search input,
                input[placeholder*="职位"],
                input[placeholder*="搜索"],
                input[placeholder]
            `);

            if (!input) return {{ ok: false, msg: 'input not found' }};

            input.focus();
            input.value = {category};

            input.dispatchEvent(new InputEvent('input', {{
                bubbles: true,
                inputType: 'insertText',
                data: {category}
            }}));

            input.dispatchEvent(new Event('change', {{ bubbles: true }}));

            return {{ ok: true }};
        }})()
        "#,
        category = category_json
    );

    let input_result = self
        .page
        .run_js(&input_script)
        .map_err(BossError::map_cdp("职位类型弹窗输入失败"))?;

    log::info!("  [Debug] 职位类型搜索输入结果: {:?}", input_result);

    let input_ok = input_result
        .get("value")
        .and_then(|v| v.get("ok"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if !input_ok {
        return Ok(false);
    }

    sleep_random_ms(700, 1000);

    // 3. 从搜索结果中选择最匹配的职位类型
    for _ in 0..10 {
        let click_script = format!(
            r#"
            (() => {{
                const rawTarget = {category};
                const dialog = document.querySelector('.job-mormal-position-select-dialog');

                if (!dialog) {{
                    return {{ ok: false, msg: 'dialog not found' }};
                }}

                function clean(text) {{
                    return String(text || '')
                        .replace(/\s+/g, '')
                        .replace(/[（）()【】\[\]<>《》]/g, '')
                        .replace(/职位类型|职位类别|请选择|选择/g, '')
                        .toLowerCase();
                }}

                function similarity(a, b) {{
                    a = clean(a);
                    b = clean(b);

                    if (!a || !b) return 0;
                    if (a === b) return 100;
                    if (a.includes(b) || b.includes(a)) return 85;

                    const aChars = Array.from(new Set(Array.from(a)));
                    const bChars = Array.from(new Set(Array.from(b)));

                    let same = 0;
                    for (const ch of aChars) {{
                        if (bChars.includes(ch)) same++;
                    }}

                    const maxLen = Math.max(aChars.length, bChars.length);
                    if (maxLen === 0) return 0;

                    return Math.round((same / maxLen) * 100);
                }}

                const wanted = clean(rawTarget);

                const nodes = Array.from(dialog.querySelectorAll(`
                    .ui-select-item,
                    .stage-three,
                    .position-list-wrap span,
                    .position-list-wrap li,
                    .position-content span,
                    .position-content li,
                    .position-selecter span,
                    .position-selecter li,
                    [class*="position"] span,
                    [class*="position"] li,
                    [class*="item"]
                `))
                    .filter(n => {{
                        const text = clean(n.innerText || n.textContent);
                        if (!text) return false;

                        const rect = n.getBoundingClientRect();
                        const visible = rect.width > 0 && rect.height > 0;
                        return visible;
                    }});

                if (!nodes.length) {{
                    return {{
                        ok: false,
                        msg: 'no candidate nodes',
                        wanted
                    }};
                }}

                const candidates = nodes
                    .map(n => {{
                        const rawText = String(n.innerText || n.textContent || '').trim();
                        const text = clean(rawText);

                        let score = similarity(wanted, text);

                        if (text === wanted) score += 100;
                        else if (text.includes(wanted)) score += 80;
                        else if (wanted.includes(text)) score += 60;

                        return {{
                            node: n,
                            rawText,
                            text,
                            score
                        }};
                    }})
                    .filter(x => x.text)
                    .sort((a, b) => b.score - a.score);

                const best = candidates[0];

                if (!best) {{
                    return {{
                        ok: false,
                        msg: 'no best candidate',
                        wanted
                    }};
                }}

                // 阈值：避免乱选。低于 50 说明相似度太低。
                if (best.score < 50) {{
                    return {{
                        ok: false,
                        msg: 'best score too low',
                        wanted,
                        bestText: best.rawText,
                        bestScore: best.score,
                        candidates: candidates.slice(0, 10).map(x => ({{
                            text: x.rawText,
                            score: x.score
                        }}))
                    }};
                }}

                best.node.scrollIntoView({{ block: 'center', inline: 'center' }});
                best.node.click();

                return {{
                    ok: true,
                    wanted,
                    chosen: best.rawText,
                    score: best.score
                }};
            }})()
            "#,
            category = category_json
        );

        let result = self
            .page
            .run_js(&click_script)
            .map_err(BossError::map_cdp("职位类型弹窗选择失败"))?;

        log::info!("  [Debug] 职位类型选择结果: {:?}", result);

        let clicked = result
            .get("value")
            .and_then(|v| v.get("ok"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        if clicked {
            sleep_random_ms(350, 550);
            return Ok(true);
        }

        sleep_random_ms(250, 400);
    }

    Ok(false)
}

}