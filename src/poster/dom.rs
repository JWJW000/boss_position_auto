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
    pub(super) fn wait_visible_dropdown_items(
        &mut self,
        timeout_ms: u64,
    ) -> Vec<rust_drission::Element> {
        let selector = "xpath://div[contains(@class,'ui-select-dropdown') and not(contains(@style,'display: none'))]//li[contains(@class,'ui-select-item')]";
        log::debug!("等待下拉选项出现 (超时: {}ms)", timeout_ms);
        log::debug!("  选择器: {}", selector);

        let deadline = std::time::Instant::now() + Duration::from_millis(timeout_ms);
        let mut attempt = 0;

        while std::time::Instant::now() < deadline {
            attempt += 1;
            match self.page.eles(selector) {
                Ok(items) => {
                    if !items.is_empty() {
                        log::info!(
                            "  [✓ 成功] 找到 {} 个下拉选项 (尝试 {} 次)",
                            items.len(),
                            attempt
                        );
                        return items;
                    }
                    log::trace!("  [尝试 {}] 下拉选项未出现，继续等待...", attempt);
                }
                Err(e) => {
                    log::warn!("  [尝试 {}] 查询下拉选项失败: {:?}", attempt, e);
                }
            }
            sleep_random_ms(100, 180);
        }

        log::warn!("[超时] 可见下拉选项未出现，尝试查找所有下拉选项");
        let fallback_selector = "xpath://li[contains(@class,'ui-select-item')]";
        log::debug!("  备用选择器: {}", fallback_selector);

        match self.page.eles(fallback_selector) {
            Ok(items) => {
                if items.is_empty() {
                    log::error!("  [✗ 失败] 备用选择器也未找到任何下拉选项");
                    log::error!("    → 可在浏览器控制台测试:");
                    log::error!("       $x(\"{}\")", selector.trim_start_matches("xpath:"));
                    log::error!(
                        "       $x(\"{}\")",
                        fallback_selector.trim_start_matches("xpath:")
                    );
                } else {
                    log::warn!(
                        "  [部分成功] 备用选择器找到 {} 个选项（可能包含隐藏项）",
                        items.len()
                    );
                }
                items
            }
            Err(e) => {
                log::error!("  [✗ 失败] 备用选择器执行失败: {:?}", e);
                vec![]
            }
        }
    }

    /// Choose an item from the currently visible dropdown by exact or fuzzy text.
    pub(super) fn choose_visible_option_exact_or_contains(&mut self, value: &str) -> bool {
        log::debug!("尝试选择下拉选项: \"{}\"", value);

        let items = self.wait_visible_dropdown_items(3500);
        if items.is_empty() {
            log::error!("  [✗ 失败] 未找到任何下拉选项");
            return false;
        }

        log::debug!("  找到 {} 个选项，开始精确匹配", items.len());
        for (i, it) in items.iter().enumerate() {
            let t = it.text().unwrap_or_default().trim().to_string();
            log::trace!("    选项 {}: \"{}\"", i + 1, t);
            if t == value {
                log::info!("  [✓ 精确匹配] 找到选项 \"{}\", 点击中...", t);
                match it.click() {
                    Ok(_) => {
                        log::info!("  [✓ 成功] 已点击选项");
                        return true;
                    }
                    Err(e) => {
                        log::error!("  [✗ 失败] 点击选项失败: {:?}", e);
                        return false;
                    }
                }
            }
        }

        log::debug!("  精确匹配失败，尝试模糊匹配");
        for (_i, it) in items.iter().enumerate() {
            let t = it.text().unwrap_or_default();
            if t.contains(value) {
                log::info!("  [✓ 模糊匹配] 找到选项 \"{}\" (包含 \"{}\")", t, value);
                match it.click() {
                    Ok(_) => {
                        log::info!("  [✓ 成功] 已点击选项");
                        return true;
                    }
                    Err(e) => {
                        log::error!("  [✗ 失败] 点击选项失败: {:?}", e);
                        return false;
                    }
                }
            }
        }

        log::error!("  [✗ 失败] 未找到匹配的选项: \"{}\"", value);
        log::error!("    可用选项列表:");
        for (i, it) in items.iter().enumerate() {
            let t = it.text().unwrap_or_default();
            log::error!("      {}. \"{}\"", i + 1, t);
        }

        false
    }

    /// Open a dropdown inside the form row whose title contains `row_label`.
    pub(super) fn click_row_select_by_label(
        &mut self,
        row_label: &str,
        index: usize,
    ) -> BResult<bool> {
        log::debug!(
            "尝试点击表单行下拉框: 行标题=\"{}\", 索引={}",
            row_label,
            index
        );

        let label_json =
            serde_json::to_string(row_label).map_err(BossError::map_config("行标题序列化失败"))?;
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

        log::debug!("  执行 JavaScript 点击下拉框");
        log::trace!("  JS 代码:\n{}", js);

        match self.page.run_js(&js) {
            Ok(v) => {
                let success = v.get("value").and_then(|x| x.as_bool()).unwrap_or(false);
                if success {
                    log::info!(
                        "  [✓ 成功] 已点击表单行 \"{}\" 的第 {} 个下拉框",
                        row_label,
                        index + 1
                    );
                } else {
                    log::error!("  [✗ 失败] JavaScript 返回 false，未找到或点击失败");
                    log::error!("    → 可在浏览器控制台执行以下代码调试:");
                    log::error!(
                        "    {}",
                        js.lines().map(|l| l.trim()).collect::<Vec<_>>().join(" ")
                    );
                }
                Ok(success)
            }
            Err(e) => {
                log::error!("  [✗ 失败] JavaScript 执行失败: {:?}", e);
                log::error!("    → 可在浏览器控制台执行以下代码调试:");
                log::error!(
                    "    {}",
                    js.lines().map(|l| l.trim()).collect::<Vec<_>>().join(" ")
                );
                Err(BossError::map_cdp(format!("点击{}下拉失败", row_label))(
                    e,
                ))
            }
        }
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
