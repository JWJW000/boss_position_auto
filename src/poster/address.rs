use super::*;

impl<'a> Poster<'a> {
        pub(super) fn fill_city(&mut self) -> BResult<()> {
            log::info!("  [开始] 填写工作地址");

            // 1. 查找工作地址输入框并点击
            let city_input = match self
                .page
                .ele(".publish-edit-form-row .job-edit-click-select-content .ipt-wrap input") {
                Ok(Some(input)) => input,
                _ => {
                    log::warn!("  [跳过] 未找到工作地址输入框，该字段可能不存在");
                    return Ok(());
                }
            };

            if city_input.click().is_err() {
                log::warn!("  [跳过] 点击工作地址输入框失败");
                return Ok(());
            }

            sleep_random_ms(500, 800);

            // 2. 查找弹窗中的第一个地址项
            let address_items = match self
                .page
                .eles(".single-address-select-wrap .table-content-box .address-item") {
                Ok(items) => items,
                Err(_) => {
                    log::warn!("  [跳过] 未找到地址项列表");
                    return Ok(());
                }
            };

            let first_item = match address_items.first() {
                Some(item) => item,
                None => {
                    log::warn!("  [跳过] 地址列表为空");
                    return Ok(());
                }
            };

            // 3. 点击第一个地址的单选框
            let radio_box = match first_item.element(".radio-box") {
                Ok(Some(rb)) => rb,
                _ => {
                    log::warn!("  [跳过] 未找到地址选择框");
                    return Ok(());
                }
            };

            if radio_box.click().is_err() {
                log::warn!("  [跳过] 点击地址选择框失败");
                return Ok(());
            }

            log::info!("  [√] 选择了第一个工作地址");

            sleep_random_ms(300, 500);

            // 4. 点击「使用该地址」按钮
            let sure_btn = match self
                .page
                .ele(".single-address-select-wrap .address-footer .btn-sure-v2") {
                Ok(Some(btn)) => btn,
                _ => {
                    log::warn!("  [跳过] 未找到使用该地址按钮，该按钮可能不存在");
                    return Ok(());
                }
            };

            if sure_btn.click().is_err() {
                log::warn!("  [跳过] 点击使用该地址按钮失败");
                return Ok(());
            }

            log::info!("  [√] 已确认使用该工作地址");

            sleep_random_ms(300, 500);

            Ok(())
        }
}
