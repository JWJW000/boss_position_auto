use super::*;

impl<'a> Poster<'a> {
    /// Fill the publish deadline when the Excel row provides a non-empty date.
    pub(super) fn fill_deadline(&mut self, job: &JobRecord) -> BResult<()> {
        let deadline = job.截止日期.trim();
        if !Self::has_excel_value(deadline) {
            return Ok(());
        }

        let el = SelectorMap::find_first(self.page, &self.selectors.deadline);
        if let Some(el) = el {
            el.click().ok();
            sleep_random_ms(250, 450);
            let deadline_json = serde_json::to_string(deadline)
                .map_err(BossError::map_config("截止日期序列化失败"))?;
            let script = format!(
                "this.value = {}; this.dispatchEvent(new Event('input', {{bubbles:true}})); this.dispatchEvent(new Event('change', {{bubbles:true}})); true;",
                deadline_json
            );
            if let Err(js_err) = el.run_js(&script) {
                log::warn!("  [WARN] 截止日期JS填写失败，尝试直接输入: {}", js_err);
                el.input(deadline)
                    .map_err(BossError::map_post("填写截止日期失败"))?;
            }
            log::info!("  [√] 截止日期: {}", deadline);
            return Ok(());
        }

        Err(BossError::element("招聘截止时间输入框"))
    }

    /// Remove whitespace used only for visual layout in BOSS labels.
    pub(super) fn clean_text(value: &str) -> String {
        value
            .replace('\n', "")
            .replace('\t', "")
            .replace(' ', "")
            .trim()
            .to_string()
    }

    /// Return false for blank cells and explicit "无" placeholders.
    pub(super) fn has_excel_value(value: &str) -> bool {
        let clean = Self::clean_text(value);
        !(clean.is_empty() || clean == "无")
    }

    /// Normalize education aliases from Excel into the BOSS dropdown labels.
    pub(super) fn normalize_education_value(value: &str) -> String {
        let v = Self::clean_text(value);

        if v.contains("不限") {
            return "不限".to_string();
        }
        if v.contains("初中") {
            return "初中及以下".to_string();
        }
        if v.contains("中专") || v.contains("中技") {
            return "中专/中技".to_string();
        }
        if v.contains("高中") {
            return "高中".to_string();
        }
        if v.contains("大专") || v.contains("专科") {
            return "大专".to_string();
        }
        if v.contains("本科") {
            return "本科".to_string();
        }
        if v.contains("硕士") {
            return "硕士".to_string();
        }
        if v.contains("博士") {
            return "博士".to_string();
        }

        v
    }

    fn is_publish_success_url(url: &str) -> bool {
        let u = url.to_ascii_lowercase();
        // 只要离开了编辑页（/job/edit），且没有回到登录页，通常就代表成功（或进入了审核/管理页）
        if u.contains("/job/edit") || u.contains("login") {
            return false;
        }
        // 包含以下关键字之一即视为成功
        u.contains("success")
            || u.contains("published")
            || u.contains("/job/detail")
            || u.contains("/job/manage")
            || u.contains("/job/list")
            || u.contains("/web/boss/index")
    }

    fn has_publish_success_tip(&mut self) -> bool {
        let selectors = [
            "xpath://div[contains(@class,'boss-dialog')]//*[contains(text(),'发布成功')]",
            "xpath://div[contains(@class,'boss-dialog')]//*[contains(text(),'职位发布成功')]",
            "css:.publish-success-popup",
            "xpath://h3[contains(text(),'发布成功')]",
            "xpath://*[contains(text(),'已发布')]",
            "xpath://*[contains(text(),'审核中')]",
        ];
        for sel in selectors {
            if let Ok(Some(el)) = self.page.ele(sel) {
                if el.is_displayed().unwrap_or(false) {
                    if let Ok(text) = el.text() {
                        let text = text.trim();
                        if !text.is_empty() {
                            log::info!("  [检测到成功/状态提示] {}", text);
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    fn is_non_blocking_form_hint(text: &str) -> bool {
        let compact = Self::clean_text(text).to_ascii_lowercase();
        compact.contains("1k=1千元") && compact.contains("10k=1万元")
    }

    fn collect_form_errors(&mut self) -> Vec<String> {
        let mut form_errors = Vec::new();
        let selectors = [
            "css:.error-tip",
            "css:.form-error",
            "css:.error-message",
            "css:.tip-error",
            "css:.ui-form-item-error",
            "xpath://div[contains(@class,'error')]",
            "xpath://*[contains(text(),'请选择') or contains(text(),'必填') or contains(text(),'不能为空')]",
        ];
        for sel in selectors {
            if let Ok(els) = self.page.eles(sel) {
                for el in els {
                    if let Ok(text) = el.text() {
                        let text = text.trim();
                        if text.is_empty() || Self::is_non_blocking_form_hint(text) {
                            continue;
                        }
                        log::warn!("  发布表单提示: {}", text);
                        form_errors.push(text.to_string());
                    }
                }
            }
        }
        form_errors.sort();
        form_errors.dedup();
        form_errors
    }

    fn detect_paid_competitive_job_dialog(&mut self) -> Option<String> {
        let script = r#"
        (() => {
            const bodyText = (document.body?.innerText || document.body?.textContent || '').replace(/\s+/g, '');
            if (
                bodyText.includes('当前职位为竞招职位，需付费发布') ||
                (bodyText.includes('竞招职位') && bodyText.includes('需付费发布')) ||
                (bodyText.includes('竞招岗位') && bodyText.includes('需付费发布')) ||
                (bodyText.includes('竞招职位') && bodyText.includes('VIP账号') && bodyText.includes('直豆'))
            ) {
                return {
                    ok: true,
                    message: '当前职位为竞招职位，需付费发布',
                    source: 'body',
                    preview: bodyText.slice(0, 200)
                };
            }

            const panels = Array.from(document.querySelectorAll('.block-vip2, .boss-dialog, .boss-popup__wrapper, [class*="vip"]'));
            for (const panel of panels) {
                const text = (panel.innerText || panel.textContent || '').replace(/\s+/g, '');
                if (
                    text.includes('当前职位为竞招职位，需付费发布') ||
                    (text.includes('竞招职位') && text.includes('需付费发布')) ||
                    (text.includes('竞招岗位') && text.includes('需付费发布')) ||
                    (text.includes('竞招职位') && text.includes('VIP账号') && text.includes('直豆')) ||
                    (panel.classList.contains('block-vip2') && text.includes('竞招职位'))
                ) {
                    return {
                        ok: true,
                        message: '当前职位为竞招职位，需付费发布',
                        source: 'panel',
                        preview: text.slice(0, 160)
                    };
                }
            }
            return { ok: false };
        })()
        "#;

        let Ok(ret) = self.page.run_js(script) else {
            return None;
        };

        let value = ret.get("value").unwrap_or(&ret);
        let ok = value.get("ok").and_then(|v| v.as_bool()).unwrap_or(false);
        if !ok {
            return None;
        }

        Some(
            value
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("当前职位为竞招职位，需付费发布")
                .to_string(),
        )
    }

    fn wait_for_paid_competitive_job_dialog_after_success(&mut self) -> Option<String> {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(8);
        while std::time::Instant::now() < deadline {
            if let Some(message) = self.detect_paid_competitive_job_dialog() {
                return Some(message);
            }
            sleep_random_ms(450, 650);
        }
        None
    }

    /// Click publish and turn the resulting page or validation hints into a result.
    pub(super) fn submit(&mut self, _job: &JobRecord) -> BResult<String> {
        let btn = SelectorMap::find_first(self.page, &self.selectors.submit_btn).or_else(|| {
            SelectorMap::find_first(
                self.page,
                &[
                    "css:.btn-primary".to_string(),
                    "css:button.btn".to_string(),
                    "xpath://button[contains(@class,'primary')]".to_string(),
                ],
            )
        });

        let btn = btn.ok_or_else(|| BossError::element("发布按钮"))?;

        log::info!("  点击发布按钮...");
        btn.click().ok();

        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
        let mut url = String::new();
        let mut last_url = String::new();

        while std::time::Instant::now() < deadline {
            url = self
                .page
                .url()
                .map_err(BossError::map_cdp("读取发布结果URL失败"))?;

            // 1. 检查 URL 变化（只要离开了编辑页且没报错，就算成功）
            if url != last_url && !url.is_empty() {
                log::info!("  [URL变化] {}", url);
                last_url = url.clone();
                
                if Self::is_publish_success_url(&url) {
                    log::info!("  [判定成功] URL 已跳转至管理或成功页面: {}", url);
                    return Ok(url);
                }
            }

            // 2. 检查是否有可见的成功提示（即使用户说没有，我们也保留作为辅助）
            if self.has_publish_success_tip() {
                log::info!("  [判定成功] 检测到页面成功提示");
                return Ok(url);
            }

            // 3. 检查竞招付费弹窗（这种弹窗意味着“没发成”，属于特定业务失败）
            if let Some(message) = self.detect_paid_competitive_job_dialog() {
                log::warn!("  [发布失败] 检测到付费/竞招限制: {}", message);
                return Err(BossError::PostFailed(message));
            }

            // 4. 检查是否有表单校验错误（如果还在编辑页，且出现了错误提示，那肯定没成功）
            if url.contains("/job/edit") {
                let form_errors = self.collect_form_errors();
                if !form_errors.is_empty() {
                    log::warn!("  [发布失败] 检测到表单错误提示: {}", form_errors.join("；"));
                    return Err(BossError::PostFailed(format!(
                        "发布未成功，表单校验提示: {}",
                        form_errors.join("；")
                    )));
                }
            }

            sleep_random_ms(600, 1000);
        }

        // 超时后的最终判定
        url = self.page.url().unwrap_or_default();
        if Self::is_publish_success_url(&url) {
             return Ok(url);
        }

        Err(BossError::PostFailed(format!(
            "发布超时：点击按钮后页面未跳转且未检测到成功状态，当前URL: {}",
            url
        )))
    }
}
