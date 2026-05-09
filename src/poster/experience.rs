use super::*;
use std::thread;
use std::time::Duration;

impl<'a> Poster<'a> {
    /// Fill the experience dropdown when Excel provides a value.
    pub(super) fn fill_experience(&mut self, job: &JobRecord) -> BResult<()> {
        let exp = job.经验.trim();
        if !Self::has_excel_value(exp) {}

        return Ok(());
    }

    pub(super) fn fill_graduate_time(&mut self, job: &JobRecord) -> BResult<()> {
        let graduate_time = job.届别.trim();
        if !Self::has_excel_value(graduate_time) {
            return Ok(());
        }

        let graduate_btn = self
            .page
            .ele("css:.ui-select-inner .ui-select-placeholder")
            .map_err(BossError::map_element("未找到毕业时间按钮"))?
            .ok_or_else(|| BossError::element("时间按钮不存在"))?;

        graduate_btn
            .click()
            .map_err(BossError::map_element("点击毕业时间按钮失败"))?;

        sleep_random_ms(300, 500);

        let li_eles = self
            .page
            .eles("css:li")
            .map_err(BossError::map_element("未找到毕业时间选项"))?;

        for li_ele in li_eles {
            let li_content = li_ele
                .text_content()
                .map_err(BossError::map_element("读取毕业时间选项失败"))?
                .trim()
                .to_string();

            if graduate_time == li_content {
                li_ele
                    .click()
                    .map_err(BossError::map_element("点击毕业时间失败"))?;

                sleep_random_ms(100, 200);
                return Ok(());
            }
        }

        Err(BossError::element(format!(
            "毕业时间选项未找到: {}",
            graduate_time
        )))
    }
}
