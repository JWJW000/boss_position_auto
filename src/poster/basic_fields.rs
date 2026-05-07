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

}