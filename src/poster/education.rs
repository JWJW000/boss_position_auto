use super::*;

impl<'a> Poster<'a> {
    /// Select the minimum education value and verify the selected display text.
    pub(super) fn fill_education(&mut self, job: &JobRecord) -> BResult<()> {
        let raw = job.学历.trim();
        if !Self::has_excel_value(raw) {
            return Ok(());
        }

        let target = Self::normalize_education_value(raw);
        let target_clean = Self::clean_text(&target);
        log::info!("  [DEBUG] 学历原始值: {}, 归一化后: {}", raw, target);

        // 1) 通过标题文本定位“学历”这一行
        let mut edu_row = None;
        let mut title_samples = Vec::new();
        if let Ok(titles) = self.page.eles("css:.publish-title") {
            for title in titles {
                let text = Self::clean_text(&title.text().unwrap_or_default());
                if !text.is_empty() && title_samples.len() < 12 {
                    title_samples.push(text.clone());
                }
                if text.contains("学历") {
                    if let Ok(Some(row)) = title.parent(1) {
                        edu_row = Some(row);
                        break;
                    }
                }
            }
        }

        if edu_row.is_none() {
            if let Ok(Some(row)) = self.page.ele(
            "xpath://span[contains(@class,'ui-select-placeholder') and contains(.,'最低学历')]/ancestor::div[contains(@class,'publish-edit-form-row')]",
        ) {
            edu_row = Some(row);
        }
        }

        let edu_row = edu_row.ok_or_else(|| {
            log::warn!("  [WARN] 页面标题样本: {:?}", title_samples);
            BossError::element("学历行容器")
        })?;

        // 2) 已选值校验：已是目标则直接通过
        for sel in &[
            "css:.ui-select-selected-value",
            "css:.ui-select-placeholder",
        ] {
            if let Ok(Some(el)) = edu_row.element(sel) {
                let current = Self::clean_text(&el.text().unwrap_or_default());
                if !current.is_empty() {
                    log::info!("  [DEBUG] 学历当前值: {}", current);
                }
                if current == target_clean {
                    log::info!("  [√] 学历: {}", target);
                    return Ok(());
                }
            }
        }

        // 3) 定位并点击“学历”触发器
        let trigger = [
            "css:.ui-select-selection",
            "css:.ui-select-inner",
            "css:.ui-select",
        ]
        .iter()
        .find_map(|sel| edu_row.element(sel).ok().flatten())
        .ok_or_else(|| BossError::element("最低学历触发器"))?;
        trigger
            .run_js("this.scrollIntoView({block:'center', inline:'center'});")
            .ok();
        sleep_random_ms(140, 260);
        trigger.click().ok();
        sleep_random_ms(450, 700);

        // 4) 在可见下拉里选学历
        let options = self.wait_visible_dropdown_items(3000);
        let mut chosen = false;
        let mut option_texts = Vec::new();
        for item in options {
            let text = item.text().unwrap_or_default();
            let clean = Self::clean_text(&text);
            if !clean.is_empty() {
                option_texts.push(clean.clone());
            }
            if clean == target_clean
                || clean.contains(&target_clean)
                || target_clean.contains(&clean)
            {
                item.click().ok();
                chosen = true;
                break;
            }
        }
        if !chosen {
            if let Ok(local_items) = edu_row.elements("css:.ui-select-item") {
                for item in local_items {
                    let text = item.text().unwrap_or_default();
                    let clean = Self::clean_text(&text);
                    if !clean.is_empty() && !option_texts.contains(&clean) {
                        option_texts.push(clean.clone());
                    }
                    if clean == target_clean
                        || clean.contains(&target_clean)
                        || target_clean.contains(&clean)
                    {
                        item.click().ok();
                        chosen = true;
                        break;
                    }
                }
            }
        }

        if !chosen {
            let target_json =
                serde_json::to_string(&target).map_err(BossError::map_config("学历序列化失败"))?;
            let js_script = r#"
            function() {
                const target = __TARGET__;
                const clean = t => (t || '').replace(/\s+/g, '');
                const wanted = clean(target);
                const trigger = this.querySelector('.ui-select-selection') || this.querySelector('.ui-select-inner');
                if (trigger) trigger.click();
                const items = Array.from(this.querySelectorAll('.ui-select-item'));
                for (const it of items) {
                    const txt = clean(it.innerText);
                    if (txt === wanted || txt.includes(wanted) || wanted.includes(txt)) {
                        it.click();
                        return true;
                    }
                }
                return false;
            }
        "#
        .replace("__TARGET__", &target_json);
            if let Ok(value) = edu_row.run_js(&js_script) {
                if value
                    .get("value")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false)
                {
                    chosen = true;
                }
            }
        }

        if !chosen {
            log::warn!("  [WARN] 学历候选项: {:?}", option_texts);
            return Err(BossError::element(format!("学历选项未找到: {}", target)));
        }
        sleep_random_ms(500, 800);

        // 5) 二次校验
        for sel in &[
            "css:.ui-select-selected-value",
            "css:.ui-select-placeholder",
        ] {
            if let Ok(Some(el)) = edu_row.element(sel) {
                let current = Self::clean_text(&el.text().unwrap_or_default());
                if current == target_clean {
                    log::info!("  [√] 学历: {}", target);
                    return Ok(());
                }
            }
        }

        Err(BossError::PostFailed(format!(
            "学历选择后校验未通过，期望: {}",
            target_clean
        )))
    }
}
