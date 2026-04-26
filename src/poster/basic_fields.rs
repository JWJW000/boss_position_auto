use super::*;

impl<'a> Poster<'a> {
    /// Fill the job title input from the Excel job name.
    pub(super) fn fill_job_title(&mut self, job: &JobRecord) -> BResult<()> {
        if !Self::has_excel_value(&job.职位名称) {
            return Ok(());
        }
        let el = SelectorMap::find_first(self.page, &self.selectors.job_title)
            .ok_or_else(|| BossError::element("职位名称输入框"))?;

        el.input(&job.职位名称)
            .map_err(BossError::map_post("填写职位名称失败"))?;

        log::info!("  [√] 职位名称: {}", job.职位名称);
        Ok(())
    }

    /// Fill the job description textarea or rich-text editor.
    pub(super) fn fill_job_desc(&mut self, job: &JobRecord) -> BResult<()> {
        if !Self::has_excel_value(&job.职位描述) {
            return Ok(());
        }
        let el = SelectorMap::find_first(self.page, &self.selectors.job_desc)
            .ok_or_else(|| BossError::element("职位描述编辑器"))?;

        let desc = &job.职位描述;
        let tag = el.tag().unwrap_or_default();
        if tag == "textarea" {
            el.input(desc)
                .map_err(BossError::map_post("填写职位描述失败"))?;
        } else {
            let escaped = desc.replace('\'', "\\'").replace('\n', "\\n");
            let script = format!(
                "arguments[0].innerText = '{}'; arguments[0].dispatchEvent(new Event('input', {{bubbles: true}}));",
                escaped
            );
            el.run_js(&script)
                .map_err(BossError::map_cdp("职位描述JS注入失败"))?;
        }

        log::info!("  [√] 职位描述已填写 ({}字符)", job.职位描述.len());
        Ok(())
    }

    /// Fill the overseas work selection (是否驻外).
    pub(super) fn fill_overseas(&mut self, job: &JobRecord) -> BResult<()> {
        if !Self::has_excel_value(&job.是否驻外) {
            return Ok(());
        }

        let container = SelectorMap::find_first(self.page, &self.selectors.overseas);
        if container.is_none() {
            log::warn!("  [WARN] 未找到是否驻外选择区域，跳过");
            return Ok(());
        }

        let target_text = Self::clean_text(&job.是否驻外);

        // 查找所有选项
        let items = self
            .page
            .eles("css:.overseas-entry-container .chose-item")
            .or_else(|_| self.page.eles("xpath://div[contains(@class,'overseas-entry-container')]//p[contains(@class,'chose-item')]"))
            .map_err(BossError::map_cdp("查找是否驻外选项失败"))?;

        if items.is_empty() {
            log::warn!("  [WARN] 是否驻外选项为空，跳过");
            return Ok(());
        }

        let mut target_el = None;
        for item in items {
            let text = Self::clean_text(&item.text().unwrap_or_default());
            if text.contains(&target_text) || target_text.contains(&text) {
                target_el = Some(item);
                break;
            }
        }

        if let Some(el) = target_el {
            let _ = el.run_js("this.scrollIntoView({block:'center', inline:'center'});");
            sleep_random_ms(300, 500);

            el.click()
                .map_err(BossError::map_post("点击是否驻外选项失败"))?;

            sleep_random_ms(300, 500);
            log::info!("  [√] 是否驻外: {}", job.是否驻外);
        } else {
            log::warn!("  [WARN] 未找到匹配的是否驻外选项: {}", job.是否驻外);
        }

        Ok(())
    }
}
