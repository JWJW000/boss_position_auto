use super::*;

impl<'a> Poster<'a> {
    /// Bring the publishing tab to the foreground before page operations.
    pub(super) fn activate_current_tab(&self) -> BResult<()> {
        self.page
            .browser()
            .activate_tab(self.page.tab().tab_id())
            .map_err(BossError::map_cdp("activate publish tab failed"))
    }

    /// Pause briefly after a completed step so Vue-rendered fields stabilize.
    pub(super) fn settle_after(label: &str) {
        log::info!("  等待页面状态稳定: {}", label);
        sleep_random_ms(650, STEP_SETTLE_MS);
    }

    /// Return visible dropdown options, falling back to all options for debugging.
    pub(super) fn wait_visible_dropdown_items(&mut self, timeout_ms: u64) -> Vec<rust_drission::Element> {
        let deadline = std::time::Instant::now() + Duration::from_millis(timeout_ms);
        while std::time::Instant::now() < deadline {
            if let Ok(items) = self.page.eles("xpath://div[contains(@class,'ui-select-dropdown') and not(contains(@style,'display: none'))]//li[contains(@class,'ui-select-item')]") {
                if !items.is_empty() {
                    return items;
                }
            }
            sleep_random_ms(100, 180);
        }
        self.page
            .eles("xpath://li[contains(@class,'ui-select-item')]")
            .unwrap_or_default()
    }

    /// Choose an item from the currently visible dropdown by exact or fuzzy text.
    pub(super) fn choose_visible_option_exact_or_contains(&mut self, value: &str) -> bool {
        let items = self.wait_visible_dropdown_items(3500);
        if items.is_empty() {
            return false;
        }
        for it in &items {
            let t = it.text().unwrap_or_default().trim().to_string();
            if t == value {
                it.click().ok();
                return true;
            }
        }
        for it in &items {
            let t = it.text().unwrap_or_default();
            if t.contains(value) {
                it.click().ok();
                return true;
            }
        }
        false
    }

    /// Open a dropdown inside the form row whose title contains `row_label`.
    pub(super) fn click_row_select_by_label(&mut self, row_label: &str, index: usize) -> BResult<bool> {
        let label_json = serde_json::to_string(row_label)
            .map_err(BossError::map_config("行标题序列化失败"))?;
        let js = format!(
            r#"
            (() => {{
                const label = {label};
                const clean = text => (text || '').replace(/\s+/g, '');
                const title = Array.from(document.querySelectorAll('.publish-edit-form-row .publish-title'))
                    .find(el => clean(el.innerText).includes(clean(label)));
                const row = title
                    ? title.closest('.publish-edit-form-row')
                    : Array.from(document.querySelectorAll('.publish-edit-form-row'))
                        .find(el => clean(el.innerText).includes(clean(label)));
                if (!row) return false;
                const fire = el => {{
                    if (!el) return false;
                    el.scrollIntoView({{ block: 'center', inline: 'center' }});
                    el.dispatchEvent(new MouseEvent('mousedown', {{ bubbles: true }}));
                    el.dispatchEvent(new MouseEvent('mouseup', {{ bubbles: true }}));
                    el.click && el.click();
                    el.dispatchEvent(new MouseEvent('click', {{ bubbles: true }}));
                    return true;
                }};
                const pick = (arr, i) => (i >= 0 && i < arr.length) ? arr[i] : null;
                const selects = Array.from(row.querySelectorAll('.ui-select'));
                const target = pick(selects, {index}) || null;
                const list = [];
                if (target) {{
                    list.push(target.querySelector('.ui-select-selection'));
                    list.push(target.querySelector('.ui-select-inner'));
                    const placeholder = target.querySelector('.ui-select-placeholder');
                    const selected = target.querySelector('.ui-select-selected-value');
                    list.push(placeholder && placeholder.parentElement);
                    list.push(selected && selected.parentElement);
                }}
                const selections = Array.from(row.querySelectorAll('.ui-select-selection'));
                const inners = Array.from(row.querySelectorAll('.ui-select-inner'));
                const placeholders = Array.from(row.querySelectorAll('.ui-select-placeholder'));
                const selectedValues = Array.from(row.querySelectorAll('.ui-select-selected-value'));
                list.push(pick(selections, {index}));
                list.push(pick(inners, {index}));
                const ph = pick(placeholders, {index});
                const sv = pick(selectedValues, {index});
                list.push(ph && ph.parentElement);
                list.push(sv && sv.parentElement);
                for (const node of list) {{
                    if (fire(node)) return true;
                }}
                return false;
            }})();
            "#,
            label = label_json,
            index = index
        );
        self.page
            .run_js(&js)
            .map_err(BossError::map_cdp(format!("点击{}下拉失败", row_label)))
            .map(|v| v.get("value").and_then(|x| x.as_bool()).unwrap_or(false))
    }

    /// Run a named posting step with consistent start/success/failure logging.
    pub(super) fn run_step<F>(&mut self, label: &str, mut step: F) -> BResult<()>
    where
        F: FnMut(&mut Self) -> BResult<()>,
    {
        log::info!("  -> 开始步骤: {}", label);
        match step(self) {
            Ok(()) => {
                log::info!("  [√] 步骤完成: {}", label);
                Self::settle_after(label);
                Ok(())
            }
            Err(e) => {
                log::error!("  [x] 步骤失败: {} | {}", label, e);
                Err(BossError::PostFailed(format!("步骤[{}]失败: {}", label, e)))
            }
        }
    }

}

