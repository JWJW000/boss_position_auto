use super::*;

impl<'a> Poster<'a> {
    /// Fill job category / 职位类型.
    pub(super) fn fill_job_category(&mut self, job: &JobRecord) -> BResult<()> {
        let cat = job.职位类型.trim();

        if !Self::has_excel_value(cat) {
            log::warn!("  [跳过] 职位类型为空");
            return Ok(());
        }

        let selectors = [
            "css:input[name='jobCategory']",
            "css:.job-category-container input[name='jobCategory']",
            "css:.job-category-container input[placeholder*='职位类型']",
            "css:.job-category-container input[placeholder*='选择职位类型']",
            "css:.job-category-container .ipt-wrap input",
        ];

        let el = SelectorMap::find_first(
            self.page,
            &selectors.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
        )
        .ok_or_else(|| BossError::element("未找到职位类型输入框 input[name='jobCategory']"))?;

        let _ = el.run_js("this.scrollIntoView({block:'center', inline:'center'});");
        sleep_random_ms(300, 500);

        el.click()
            .map_err(BossError::map_post("点击职位类型输入框失败"))?;

        sleep_random_ms(900, 1300);

        if self.select_job_category_from_dialog(cat)? {
            log::info!("  [√] 职位类型: {}", cat);
            return Ok(());
        }

        Err(BossError::element(format!(
            "职位类型未从弹窗中成功选择: {}",
            cat
        )))
    }
    /// Search inside the BOSS category dialog and click the matching category.
    /// Search inside the BOSS category dialog and click the matching category.
    /// 通用版：不依赖固定 aliases，适配计算机 / 销售 / 财务 / 人事 / 运营 / 设计等岗位。
    pub(super) fn select_job_category_from_dialog(&mut self, category: &str) -> BResult<bool> {
        let category_json =
            serde_json::to_string(category).map_err(BossError::map_config("职位类型序列化失败"))?;

        // 1. 等待职位类型弹窗出现
        for _ in 0..10 {
            let found = self
                .page
                .ele("css:.job-mormal-position-select-dialog .position-selecter")
                .map_err(BossError::map_cdp("检查职位类型弹窗状态失败"))?
                .is_some();

            if found {
                break;
            }

            sleep_random_ms(900, 1300);
        }

        let dialog_exists = self
            .page
            .ele("css:.job-mormal-position-select-dialog .position-selecter")
            .map_err(BossError::map_cdp("检查职位类型弹窗状态失败"))?
            .is_some();

        if !dialog_exists {
            log::warn!("  [WARN] 职位类型弹窗未出现");
            return Ok(false);
        }

        // 2. 在弹窗搜索框输入 Excel 中的职位类型
        let input_script = format!(
            r#"
        (() => {{
            const dialog = document.querySelector('.job-mormal-position-select-dialog');
            if (!dialog) return {{ ok: false, msg: 'dialog not found' }};

            const input = dialog.querySelector(`
                input.ui-select-input,
                .position-search input,
                input[placeholder*="职位"],
                input[placeholder*="搜索"],
                input[placeholder]
            `);

            if (!input) return {{ ok: false, msg: 'input not found' }};

            input.focus();
            input.value = {category};

            input.dispatchEvent(new InputEvent('input', {{
                bubbles: true,
                inputType: 'insertText',
                data: {category}
            }}));

            input.dispatchEvent(new Event('change', {{ bubbles: true }}));

            return {{ ok: true }};
        }})()
        "#,
            category = category_json
        );

        let input_result = self
            .page
            .run_js(&input_script)
            .map_err(BossError::map_cdp("职位类型弹窗输入失败"))?;

        log::info!("  [Debug] 职位类型搜索输入结果: {:?}", input_result);

        let input_ok = input_result
            .get("value")
            .and_then(|v| v.get("ok"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        if !input_ok {
            return Ok(false);
        }

        sleep_random_ms(800, 1200);

        let items = self
            .page
            .eles("css:.ui-select-dropdown .ui-dropdown-list li")
            .map_err(BossError::map_cdp("获取职位类型下拉项失败"))?;

        let first = items
            .first()
            .ok_or_else(|| BossError::element("职位类型下拉项为空"))?;

        first
            .click()
            .map_err(BossError::map_post("点击职位类型下拉第一项失败"))?;

        sleep_random_ms(800, 1200);

        return Ok(true);
    }
}
