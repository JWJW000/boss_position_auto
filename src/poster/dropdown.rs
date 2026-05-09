use super::*;

impl<'a> Poster<'a> {
    /// Fallback open: click placeholder parent, like `document.querySelectorAll('.ui-select-placeholder')[i].parentElement.click()`.
    pub(super) fn click_select_by_placeholder_hint(
        &mut self,
        hint: &str,
        index: usize,
    ) -> BResult<bool> {
        let hint_json =
            serde_json::to_string(hint).map_err(BossError::map_config("占位提示序列化失败"))?;
        let js = format!(
            r#"
            (() => {{
                const hint = {hint};
                const idx = {index};
                const clean = text => (text || '').replace(/\s+/g, '');
                const list = Array.from(document.querySelectorAll('.ui-select-placeholder'))
                    .filter(el => clean(el.innerText).includes(clean(hint)));
                const all = Array.from(document.querySelectorAll('.ui-select-placeholder'));
                const p = list[idx] || list[0] || all[idx] || all[0];
                const target = p && p.parentElement;
                if (!target) return false;
                target.scrollIntoView({{ block: 'center', inline: 'center' }});
                target.dispatchEvent(new MouseEvent('mousedown', {{ bubbles: true }}));
                target.dispatchEvent(new MouseEvent('mouseup', {{ bubbles: true }}));
                target.click && target.click();
                target.dispatchEvent(new MouseEvent('click', {{ bubbles: true }}));
                return true;
            }})();
            "#,
            hint = hint_json,
            index = index
        );
        self.page
            .run_js(&js)
            .map_err(BossError::map_cdp("按占位文案点击下拉失败"))
            .map(|v| v.get("value").and_then(|x| x.as_bool()).unwrap_or(false))
    }

    /// Prefer selecting from the currently opened dropdown via JS to avoid clicking hidden options.
    pub(super) fn choose_open_dropdown_option_js(&mut self, value: &str) -> BResult<bool> {
        let value_json =
            serde_json::to_string(value).map_err(BossError::map_config("下拉选项序列化失败"))?;
        let js = format!(
            r#"
            (() => {{
                const value = {value};
                const clean = text => (text || '').replace(/\s+/g, '');
                const isVisible = el => {{
                    const style = window.getComputedStyle(el);
                    return style.display !== 'none'
                        && style.visibility !== 'hidden'
                        && style.opacity !== '0'
                        && el.getClientRects().length > 0;
                }};
                const optionSelectors = [
                    '.ui-select-item',
                    '[role="option"]',
                    '.option-item',
                    '.item',
                    'li'
                ].join(',');
                const popupRoots = Array.from(document.querySelectorAll(
                    '.ui-select-dropdown,.boss-popup,.boss-dialog__wrapper,[class*="dropdown"],[class*="select-menu"]'
                )).filter(isVisible);
                let items = [];
                if (popupRoots.length) {{
                    const root = popupRoots[popupRoots.length - 1];
                    items = Array.from(root.querySelectorAll(optionSelectors));
                }}
                if (!items.length) {{
                    items = Array.from(document.querySelectorAll(optionSelectors));
                }}
                items = items.filter(el => isVisible(el) && clean(el.innerText));
                let target = items.find(el => clean(el.innerText) === clean(value));
                if (!target) target = items.find(el => clean(el.innerText).includes(clean(value)));
                if (!target) return false;
                target.scrollIntoView({{ block: 'center' }});
                target.dispatchEvent(new MouseEvent('mousedown', {{ bubbles: true }}));
                target.dispatchEvent(new MouseEvent('mouseup', {{ bubbles: true }}));
                target.click && target.click();
                target.dispatchEvent(new MouseEvent('click', {{ bubbles: true }}));
                return true;
            }})();
            "#,
            value = value_json
        );
        self.page
            .run_js(&js)
            .map_err(BossError::map_cdp("JS选择下拉选项失败"))
            .map(|v| v.get("value").and_then(|x| x.as_bool()).unwrap_or(false))
    }

    /// Fallback option clicker for pages where list items are not `.ui-select-item`.
    pub(super) fn click_any_visible_option_by_text_js(&mut self, value: &str) -> BResult<bool> {
        let value_json =
            serde_json::to_string(value).map_err(BossError::map_config("通用选项序列化失败"))?;
        let js = format!(
            r#"
            (() => {{
                const value = {value};
                const clean = text => (text || '').replace(/\s+/g, '');
                const isVisible = el => {{
                    const style = window.getComputedStyle(el);
                    return style.display !== 'none'
                        && style.visibility !== 'hidden'
                        && style.opacity !== '0'
                        && el.getClientRects().length > 0;
                }};
                const roots = Array.from(document.querySelectorAll(
                    '.boss-popup,.boss-dialog__wrapper,.ui-select-dropdown,[class*="dropdown"],[class*="popover"]'
                )).filter(isVisible);
                const optionSelectors = [
                    '.ui-select-item',
                    '[role="option"]',
                    '.option-item',
                    '.address-item',
                    '.menu-item',
                    'li',
                    'button',
                    'span'
                ].join(',');
                let candidates = [];
                const pickRoots = roots.length ? roots : [document];
                for (const root of pickRoots) {{
                    const nodes = Array.from(root.querySelectorAll(optionSelectors));
                    for (const el of nodes) {{
                        const text = clean(el.innerText);
                        if (!text || text.length > 80 || !isVisible(el)) continue;
                        candidates.push(el);
                    }}
                }}
                let target = candidates.find(el => clean(el.innerText) === clean(value));
                if (!target) target = candidates.find(el => clean(el.innerText).includes(clean(value)));
                if (!target) return false;
                target.scrollIntoView({{ block: 'center', inline: 'center' }});
                target.dispatchEvent(new MouseEvent('mousedown', {{ bubbles: true }}));
                target.dispatchEvent(new MouseEvent('mouseup', {{ bubbles: true }}));
                target.click && target.click();
                target.dispatchEvent(new MouseEvent('click', {{ bubbles: true }}));
                return true;
            }})();
            "#,
            value = value_json
        );
        self.page
            .run_js(&js)
            .map_err(BossError::map_cdp("通用选项点击失败"))
            .map(|v| v.get("value").and_then(|x| x.as_bool()).unwrap_or(false))
    }

    fn placeholder_hint_for_row(row_label: &str) -> Option<&'static str> {
        match row_label {
            "经验" => Some("请选择经验要求"),
            "学历" => Some("请选择最低学历"),
            "毕业时间" => Some("毕业时间"),
            "结算方式" => Some("结算方式"),
            _ => None,
        }
    }

    /// Choose an option from a row-scoped dropdown, skipping blank Excel values.
    pub(super) fn choose_row_select_option(
        &mut self,
        row_label: &str,
        index: usize,
        value: &str,
    ) -> BResult<bool> {
        if !Self::has_excel_value(value) {
            return Ok(false);
        }
        for _ in 0..4 {
            let mut opened = self.click_row_select_by_label(row_label, index)?;
            if !opened {
                if let Some(hint) = Self::placeholder_hint_for_row(row_label) {
                    opened = self.click_select_by_placeholder_hint(hint, index)?;
                }
            }
            if !opened {
                sleep_random_ms(140, 240);
                continue;
            }
            sleep_random_ms(220, 420);
            if self.choose_open_dropdown_option_js(value)? {
                return Ok(true);
            }
            if self.choose_visible_option_exact_or_contains(value) {
                return Ok(true);
            }
            if self.click_any_visible_option_by_text_js(value)? {
                return Ok(true);
            }
            sleep_random_ms(120, 220);
        }
        Ok(false)
    }
}
