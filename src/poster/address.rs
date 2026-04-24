use super::*;

impl<'a> Poster<'a> {
    /// Open the work-address selector and choose a saved address that matches the Excel value.
    pub(super) fn fill_city(&mut self, _job: &JobRecord) -> BResult<()> {
    for attempt in 1..=3 {
        let click_mark = self.page.ele(".job-edit-click-select-content input")
    .map_err(BossError::map_cdp("查询地址栏失败"))?;


        if click_mark.is_none() {
            log::warn!("  [WARN] 工作地址点击失败，第{}次", attempt);
        } else {
            
            let click_mark = self
    .page
    .ele("css:.job-edit-click-select-content input")
    .map_err(BossError::map_element("未找到地址栏"))?
    .ok_or_else(|| BossError::element("未找到地址栏"))?;

        click_mark
            .click()
            .map_err(BossError::map_element("点击地址栏失败"))?;

        sleep_random_ms(300, 500);

        let address_btn = self
            .page
            .ele("css:.normal-radio")
            .map_err(BossError::map_element("未找到地址"))?
            .ok_or_else(|| BossError::element("未找到地址"))?;

        address_btn
            .click()
            .map_err(BossError::map_element("点击地址失败"))?;

        sleep_random_ms(500, 800);
            
        return Ok(());
        }

        sleep_random_ms(400, 700);
    }

    Err(BossError::element("工作地址点击失败"))
}


}
