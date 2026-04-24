use super::*;

impl<'a> Poster<'a> {
    /// Open the job-category picker and select the category from Excel.
    pub(super) fn fill_job_category(&mut self, job: &JobRecord) -> BResult<()> {
        let cat = job.职位类型.trim();
        if !Self::has_excel_value(cat) {
            return Ok(());
        }

        let el = SelectorMap::find_first(self.page, &[
            "css:input[name='jobCategory']".to_string(),
            "css:#jobCategory".to_string(),
            "css:input[placeholder*='职位']".to_string(),
            "xpath://input[contains(@placeholder,'职位')]".to_string(),
            "xpath://div[contains(@class,'ui-select')][.//input[contains(@placeholder,'职位')]]".to_string(),
            "xpath://div[contains(@class,'ui-select-inner')][.//input[contains(@placeholder,'职位')]]".to_string(),
        ]);
        if let Some(el) = el {
            el.click().ok();
            sleep_random_ms(900, 1300);

            if self.select_job_category_from_dialog(cat)? {
                log::info!("  [√] 职位类型: {}", cat);
                return Ok(());
            }

            el.input(cat)
                .map_err(BossError::map_post("填写职位类型失败"))?;
            log::info!("  [√] 职位类型(直接输入): {}", cat);
        }
        Ok(())
    }

    /// Search inside the BOSS category dialog and click the matching category.
    pub(super) fn select_job_category_from_dialog(&mut self, category: &str) -> BResult<bool> {
        let category_json = serde_json::to_string(category)
            .map_err(BossError::map_config("职位类型序列化失败"))?;

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

        if self
            .page
            .ele("css:.job-mormal-position-select-dialog .position-selecter")
            .map_err(BossError::map_cdp("检查职位类型弹窗状态失败"))?
            .is_none()
        {
            return Ok(false);
        }

        let input_script = format!(
            r#"
            (() => {{
                const dialog = document.querySelector('.job-mormal-position-select-dialog');
                if (!dialog) return false;
                const input = dialog.querySelector("input.ui-select-input, .position-search input, input[placeholder]");
                if (!input) return false;
                input.focus();
                input.value = {category};
                input.dispatchEvent(new InputEvent('input', {{ bubbles: true, inputType: 'insertText', data: {category} }}));
                input.dispatchEvent(new Event('change', {{ bubbles: true }}));
                return true;
            }})()
            "#,
            category = category_json
        );
        let input_ok = self
            .page
            .run_js(&input_script)
            .map_err(BossError::map_cdp("职位类型弹窗输入失败"))?
            .get("value")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        if !input_ok {
            return Ok(false);
        }

        sleep_random_ms(550, 850);
        for _ in 0..10 {
            let click_script = format!(
                r#"
                (() => {{
                    const target = {category}.trim();
                    const dialog = document.querySelector('.job-mormal-position-select-dialog');
                    if (!dialog || !target) return false;
                    const nodes = Array.from(dialog.querySelectorAll(
                        '.ui-select-item, .stage-three, .position-list-wrap span, .position-list-wrap li, .position-content span, .position-content li'
                    ));
                    const clean = text => (text || '').replace(/\s+/g, '');
                    const wanted = clean(target).toLowerCase();
                    const aliases = [
                        [['java', 'python', 'php', 'golang', 'go', 'c++', 'c#', '.net', 'node', 'node.js', '后端'], '后端开发'],
                        [['web', '前端', 'android', 'ios', '移动'], '前端/移动开发'],
                        [['测试', 'qa'], '测试'],
                        [['算法', '机器学习', '深度学习', 'ai', '人工智能'], '人工智能'],
                        [['数据', '数据分析', '数据开发'], '数据'],
                        [['运维', '技术支持'], '运维/技术支持'],
                    ];
                    const alias = aliases.find(([keys]) => keys.some(k => wanted.includes(clean(k).toLowerCase())));
                    const aliasWanted = alias ? clean(alias[1]).toLowerCase() : '';
                    const exact = nodes.find(n => clean(n.innerText).toLowerCase() === wanted);
                    const fuzzy = exact
                        || nodes.find(n => clean(n.innerText).toLowerCase().includes(wanted))
                        || (aliasWanted && nodes.find(n => clean(n.innerText).toLowerCase() === aliasWanted));
                    if (!fuzzy) return false;
                    const chosen = clean(fuzzy.innerText).toLowerCase();
                    fuzzy.scrollIntoView({{ block: 'center', inline: 'center' }});
                    fuzzy.click();
                    return chosen === wanted || chosen.includes(wanted);
                }})()
                "#,
                category = category_json
            );
            let clicked = self
                .page
                .run_js(&click_script)
                .map_err(BossError::map_cdp("职位类型弹窗选择失败"))?
                .get("value")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            if clicked {
                sleep_random_ms(350, 550);
                return Ok(true);
            }
            sleep_random_ms(220, 380);
        }

        Ok(false)
    }
}

