use super::*;

impl<'a> Poster<'a> {
    /// Publish a single job by filling every form section and submitting it.
    pub fn post(&mut self, job: &JobRecord) -> BResult<String> {
        log::info!("正在发布: {}", job.职位名称);

        self.navigate_to_publish_page()?;

        // 等待表单加载（等待职位名称输入框出现）
        log::info!("等待表单加载...");
        sleep_random_ms(1500, 2000);  // 增加初始等待时间

        let mut form_loaded = false;
        for attempt in 0..30 {  // 增加重试次数到30次
            if let Ok(Some(_)) = self.page.ele("css:input[name='jobName']") {
                log::info!("表单已加载");
                form_loaded = true;
                break;
            }
            if attempt % 5 == 0 && attempt > 0 {
                log::info!("等待表单加载... (尝试 {}/30)", attempt);
            }
            sleep_random_ms(900, 1300);
        }
        if !form_loaded {
            log::error!("表单加载超时（已等待30次重试）");
            return Err(BossError::element("职位名称输入框"));
        }

        self.run_step("招聘类型", |s| s.fill_job_type(job))?;
        self.run_step("职位名称", |s| s.fill_job_title(job))?;
        self.run_step("职位类型", |s| s.fill_job_category(job))?;
        self.run_step("职位描述", |s| s.fill_job_desc(job))?;
        self.fill_requirements_by_type(job)?;

        let result_url = self.submit(job)?;
        log::info!("[成功] {} -> {}", job.职位名称, result_url);
        Ok(result_url)
    }

    /// Publish all provided jobs in order and keep each result.
    pub fn post_all(&mut self, jobs: &[JobRecord]) -> Vec<BResult<String>> {
        jobs.iter().map(|j| self.post(j)).collect()
    }

    /// Navigate the active tab to the BOSS job publishing page.
    pub(super) fn navigate_to_publish_page(&mut self) -> BResult<()> {
        self.activate_current_tab()?;

        let publish_urls = [
            "https://www.zhipin.com/web/chat/job/edit?encryptId=0&enterSource=2"
        ];

        for url in &publish_urls {
            log::info!("尝试访问: {}", url);
            if let Ok(_) = self.page.get(url) {
                sleep_random_ms(1700, 2500);
                if let Ok(u) = self.page.url() {
                    log::info!("  实际URL: {}", u);
                    if !u.contains("login") {
                        log::info!("已进入发布页: {}", u);
                        return Ok(());
                    }
                    log::warn!("  被重定向到登录页，当前URL: {}", u);
                }
            }
        }

        // 从BOSS首页尝试导航
        log::info!("从BOSS首页尝试导航...");
        self.page.get("https://www.zhipin.com")
            .map_err(BossError::map_cdp("访问BOSS首页失败"))?;
        sleep_random_ms(1200, 1800);

        let publish_btns = [
            "xpath://a[contains(text(),'发布职位')]",
            "xpath://a[contains(text(),'发布岗位')]",
            "xpath://a[contains(@href,'publishJob')]",
            "css:.publish-btn",
            "css:a[href*='publishJob']",
        ];

        for sel in &publish_btns {
            if let Ok(Some(el)) = self.page.ele(sel) {
                log::info!("点击发布按钮: {}", sel);
                el.click().ok();
                sleep_random_ms(1700, 2500);
                break;
            }
        }

        Ok(())
    }

}

