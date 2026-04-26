use super::*;

impl<'a> Poster<'a> {
    /// Open the work-address selector and choose a saved address that matches the Excel value.
 pub(super) fn fill_city(&mut self) -> BResult<()> {
    log::info!("  [开始] 填写工作地址");

    // 1. 查找工作地址输入框并点击
    let city_input = self
        .page
        .ele(".publish-edit-form-row .job-edit-click-select-content .ipt-wrap input")
        .map_err(BossError::map_element("未找到工作地址输入框"))?
        .ok_or_else(||BossError::element("未找到工作地址"))?;

    city_input.click().map_err(BossError::map_element("点击工作地址输入框失败"))?;

    sleep_random_ms(300, 500); // 添加适当的延时，模拟点击操作

    // 2. 查找弹窗中的地址项
    let address_items = self
        .page
        .eles(".single-address-select-wrap .table-content-box .address-item")
        .map_err(BossError::map_element("未找到地址项"))?;

    // 3. 选择第一个地址并点击
    if let Some(first_item) = address_items.first() {
        // 点击地址项的 "radio-box" 来选择该地址
        let radio_box = first_item
            .element(".radio-box")
            .map_err(BossError::map_element("未找到地址选择框"))?
            .ok_or_else(||BossError::element("未找到工作地址"))?;
        radio_box.click().map_err(BossError::map_element("点击地址选择框失败"))?;

        log::info!("  [√] 选择了第一个工作地址");

    

        Ok(())
    } else {
        Err(BossError::element("未找到地址项"))
    }
}


}
