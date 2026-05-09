use super::*;

impl<'a> Poster<'a> {
    /// 填写工作地址（城市）
    /// 填写工作地址（城市）
    pub(super) fn fill_city(&mut self, job: &JobRecord) -> BResult<()> {
        if !Self::has_excel_value(&job.城市) {
            log::warn!("  [跳过] 城市字段为空");
            return Ok(());
        }

        let target_address = job.城市.trim();
        log::info!("  [开始] 填写工作地址: {}", target_address);

        // ---------- 1. 打开工作地址弹窗 ----------
        let city_selectors = [
            "css:input[placeholder='选择工作地点']",
            "css:input[placeholder*='工作地点']",
            "css:input[placeholder*='工作地址']",
            "css:input[placeholder*='地址']",
            "css:.job-edit-click-select-content .ipt-wrap input",
            "css:.publish-edit-form-row .job-edit-click-select-content input",
            "css:.form-row .job-edit-click-select-content input",
        ];

        let city_input = SelectorMap::find_first(
            self.page,
            &city_selectors
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
        )
        .ok_or_else(|| BossError::element("未找到工作地点输入框"))?;

        let _ = city_input.run_js("this.scrollIntoView({block:'center', inline:'center'});");
        sleep_random_ms(300, 500);
        let _ = city_input.click();
        sleep_random_ms(600, 900);

        let js_click = r#"
        const el = this;
        if (el.focus) el.focus();
        ['mouseover', 'mouseenter', 'mousedown', 'mouseup', 'click'].forEach(type => {
            el.dispatchEvent(new MouseEvent(type, { bubbles: true, cancelable: true, view: window }));
        });
        el.dispatchEvent(new Event('input', { bubbles: true }));
        el.dispatchEvent(new Event('change', { bubbles: true }));
        return true;
    "#;
        let _ = city_input.run_js(js_click);
        sleep_random_ms(1500, 2000);

        // ---------- 2. 尝试匹配已有地址 ----------
        let address_json = serde_json::to_string(target_address)
            .map_err(BossError::map_config("工作地址序列化失败"))?;
        let target_city = Self::extract_city_name(target_address);
        let city_json =
            serde_json::to_string(&target_city).map_err(BossError::map_config("城市序列化失败"))?;
        let building = Self::extract_building_name(target_address);
        let building_json = serde_json::to_string(&building)
            .map_err(BossError::map_config("办公大楼序列化失败"))?;

        let js_select_address = format!(
            r#"
        (() => {{
            const targetRaw = {address};
            const targetCityRaw = {city};
            const buildingRaw = {building};
            function clean(s) {{
                return String(s || '').replace(/\s+/g, '').replace(/[·,，。;；:：\-—_/]/g, '').toLowerCase();
            }}
            const target = clean(targetRaw);
            const targetCity = clean(targetCityRaw);
            const building = clean(buildingRaw);
            const dialog = document.querySelector('.job-address-select-new-dialog, .single-address-select-wrap, .dialog-job-address-select');
            if (!dialog) return {{ ok: false, msg: 'address dialog not found' }};
            const items = Array.from(dialog.querySelectorAll('.address-item'));
            if (!items.length) return {{ ok: false, msg: 'address items empty' }};
            const candidates = items.map(item => {{
                const addressText = item.querySelector('.address .address-text')?.innerText.trim() || '';
                const areaText = item.querySelector('.area')?.innerText.trim() || '';
                const addressClean = clean(addressText);
                const areaClean = clean(areaText);
                const fullText = clean(areaText + ' ' + addressText);
                let score = 0;
                if (fullText === target) score += 300;
                if (fullText.includes(target)) score += 200;
                if (target.includes(fullText)) score += 150;
                if (addressClean === target) score += 280;
                if (addressClean && target.includes(addressClean)) score += 260;
                if (target && addressClean.includes(target)) score += 220;
                if (building && addressClean.includes(building)) score += 180;
                if (targetCity && areaClean.includes(targetCity)) score += 80;
                if (targetCity && !areaClean.includes(targetCity)) score -= 200;
                for (let ch of new Set(Array.from(building || target))) if (fullText.includes(ch)) score += 1;
                return {{ item, addressText, areaText, addressClean, fullText, score }};
            }}).sort((a, b) => b.score - a.score);
            const best = candidates[0];
            if (!best || best.score < 120) {{
                return {{ ok: false, msg: 'no matched address', bestAddress: best ? best.addressText : '', bestArea: best ? best.areaText : '', bestScore: best ? best.score : 0 }};
            }}
            const radio = best.item.querySelector('.radio-box, .normal-radio');
            if (radio) radio.click();
            return {{ ok: true, chosenAddress: best.addressText, chosenArea: best.areaText, score: best.score }};
        }})()
        "#,
            address = address_json,
            city = city_json,
            building = building_json
        );

        let select_ret = self
            .page
            .run_js(&js_select_address)
            .map_err(BossError::map_cdp("选择工作地址失败"))?;
        log::info!("  [Debug] 工作地址选择结果: {:?}", select_ret);

        // 辅助函数：从 run_js 返回的 Value 中提取 ok 字段（兼容带 value 包装的结构）
        let is_ok = |val: &serde_json::Value| -> bool {
            val.get("value")
                .and_then(|v| v.get("ok"))
                .and_then(|v| v.as_bool())
                .or_else(|| val.get("ok").and_then(|v| v.as_bool()))
                .unwrap_or(false)
        };

        if is_ok(&select_ret) {
            sleep_random_ms(500, 800);
            let sure_btn = self
            .page
            .ele("css:.job-address-select-new-dialog .btn-sure-v2, .single-address-select-wrap .btn-sure-v2")
            .map_err(BossError::map_cdp("查找使用该地址按钮失败"))?
            .ok_or_else(|| BossError::element("未找到使用该地址按钮"))?;
            sure_btn
                .click()
                .map_err(BossError::map_post("点击使用该地址失败"))?;
            sleep_random_ms(500, 800);
            log::info!("  [√] 工作地址已选择: {}", target_address);
            return Ok(());
        }

        // ---------- 3. 未匹配到，点击“添加新地址” ----------
        log::warn!("  [WARN] 未匹配到现有地址，点击添加新地址");
        let add_btn = self
        .page
        .ele("css:.job-address-select-new-dialog .btn-outline-v2, .single-address-select-wrap .btn-outline-v2")
        .map_err(BossError::map_cdp("查找添加新地址按钮失败"))?
        .ok_or_else(|| BossError::element("未找到添加新地址按钮"))?;
        add_btn
            .click()
            .map_err(BossError::map_post("点击添加新地址按钮失败"))?;
        sleep_random_ms(1500, 2000);
        log::info!("  [√] 已打开添加新地址弹窗");

        // ---------- 4. 动态填写新地址 ----------
        // 4.1 点击“工作城市”输入框
        let city_input_add = self
            .page
            .ele("css:.create-address-dialog .city-address input[placeholder='选择城市']")
            .map_err(BossError::map_cdp("查找添加地址城市输入框失败"))?
            .ok_or_else(|| BossError::element("未找到添加地址城市输入框"))?;
        city_input_add
            .click()
            .map_err(BossError::map_post("点击添加地址城市输入框失败"))?;
        sleep_random_ms(800, 1000);

        log::info!("  [提取城市] 从地址中提取城市名: {}", target_city);

        // 4.2 选择工作城市。BOSS 的城市弹窗用全局 li 渲染省市列表：
        // 先选择一级，再倒序遍历全局 li 选择二级；直辖市二级再次选择自身。
        let select_city_js = format!(
            r#"
        (function() {{
            const targetCity = {city};
            const clean = text => String(text || '').replace(/\s+/g, '');
            const input = document.querySelector(".create-address-dialog .city-address input[placeholder='选择城市']");
            const clickNode = el => {{
                el.scrollIntoView({{ block: 'center', inline: 'center' }});
                el.dispatchEvent(new PointerEvent('pointerdown', {{ bubbles: true, cancelable: true, view: window }}));
                el.dispatchEvent(new PointerEvent('pointerup', {{ bubbles: true, cancelable: true, view: window }}));
                el.dispatchEvent(new MouseEvent('mousedown', {{ bubbles: true, cancelable: true, view: window }}));
                el.dispatchEvent(new MouseEvent('mouseup', {{ bubbles: true, cancelable: true, view: window }}));
                el.click();
                el.dispatchEvent(new MouseEvent('click', {{ bubbles: true, cancelable: true, view: window }}));
            }};
            const collectItems = () => Array.from(document.querySelectorAll('li'))
                .map((el, index) => ({{
                    el,
                    index,
                    text: clean(el.innerText || el.textContent || ''),
                    className: el.className || '',
                    parentClass: el.parentElement ? (el.parentElement.className || '') : ''
                }}))
                .filter(item => item.text && item.text !== '热门');
            const targetText = clean(targetCity);
            const provinceItems = collectItems();
            const province = provinceItems.find(item => item.text === targetText);
            if (!province) {{
                return {{
                    ok: false,
                    msg: 'province li not found by global exact match',
                    targetCity,
                    available: provinceItems.slice(0, 120).map(item => item.text)
                }};
            }}
            clickNode(province.el);

            const cityItems = collectItems();
            const city = cityItems.slice().reverse().find(item => item.text === targetText);
            if (!city) {{
                return {{
                    ok: false,
                    msg: 'city li not found by reverse global exact match after province click',
                    targetCity,
                    province: province.text,
                    available: cityItems.slice(-120).map(item => item.text)
                }};
            }}
            clickNode(city.el);

            const selected = input ? (input.value || input.getAttribute('value') || input.innerText || '') : '';
            return {{
                ok: true,
                province: province.text,
                provinceIndex: province.index,
                city: city.text,
                cityIndex: city.index,
                cityClassName: city.className,
                cityParentClass: city.parentClass,
                selected,
                targetCity,
                msg: 'province clicked, then city clicked by reverse global exact match'
            }};
        }})()
        "#,
            city = city_json
        );
        let city_select_ret = self
            .page
            .run_js(&select_city_js)
            .map_err(BossError::map_cdp("选择工作城市失败"))?;
        log::info!("  [Debug] 工作城市选择结果: {:?}", city_select_ret);
        if !is_ok(&city_select_ret) {
            return Err(BossError::element(format!(
                "选择工作城市失败: {}",
                target_city
            )));
        }
        sleep_random_ms(800, 1200);

        // 4.4 填写“工作地点”（办公大楼）
        let building_input = self
            .page
            .ele("css:.create-address-dialog .detail-address input")
            .map_err(BossError::map_cdp("查找办公大楼输入框失败"))?
            .ok_or_else(|| BossError::element("未找到办公大楼输入框"))?;
        log::info!("  [提取办公大楼] {}", building);
        building_input
            .click()
            .map_err(BossError::map_post("点击办公大楼输入框失败"))?;
        sleep_random_ms(200, 400);
        building_input
            .clear()
            .map_err(BossError::map_post("清空办公大楼输入框失败"))?;
        building_input
            .focus()
            .map_err(BossError::map_post("聚焦办公大楼输入框失败"))?;
        self.page
            .tab()
            .run_cdp(
                "Input.dispatchKeyEvent",
                Some(serde_json::json!({
                    "type": "keyDown",
                    "key": "a",
                    "code": "KeyA",
                    "windowsVirtualKeyCode": 65,
                    "nativeVirtualKeyCode": 65,
                    "modifiers": 2
                })),
            )
            .map_err(BossError::map_cdp("全选办公大楼输入框失败"))?;
        self.page
            .tab()
            .run_cdp(
                "Input.dispatchKeyEvent",
                Some(serde_json::json!({
                    "type": "keyUp",
                    "key": "a",
                    "code": "KeyA",
                    "windowsVirtualKeyCode": 65,
                    "nativeVirtualKeyCode": 65,
                    "modifiers": 2
                })),
            )
            .map_err(BossError::map_cdp("释放全选快捷键失败"))?;
        self.page
            .tab()
            .run_cdp(
                "Input.dispatchKeyEvent",
                Some(serde_json::json!({
                    "type": "keyDown",
                    "key": "Backspace",
                    "code": "Backspace",
                    "windowsVirtualKeyCode": 8,
                    "nativeVirtualKeyCode": 8
                })),
            )
            .map_err(BossError::map_cdp("清空办公大楼键盘输入失败"))?;
        self.page
            .tab()
            .run_cdp(
                "Input.dispatchKeyEvent",
                Some(serde_json::json!({
                    "type": "keyUp",
                    "key": "Backspace",
                    "code": "Backspace",
                    "windowsVirtualKeyCode": 8,
                    "nativeVirtualKeyCode": 8
                })),
            )
            .map_err(BossError::map_cdp("释放清空键失败"))?;

        for ch in building.chars() {
            let part = ch.to_string();
            self.page
                .tab()
                .run_cdp(
                    "Input.dispatchKeyEvent",
                    Some(serde_json::json!({
                        "type": "keyDown",
                        "key": part
                    })),
                )
                .map_err(BossError::map_cdp("办公大楼 keyDown 失败"))?;
            self.page
                .tab()
                .run_cdp(
                    "Input.dispatchKeyEvent",
                    Some(serde_json::json!({
                        "type": "char",
                        "text": part,
                        "unmodifiedText": part
                    })),
                )
                .map_err(BossError::map_cdp("办公大楼 char 输入失败"))?;
            self.page
                .tab()
                .run_cdp(
                    "Input.dispatchKeyEvent",
                    Some(serde_json::json!({
                        "type": "keyUp",
                        "key": part
                    })),
                )
                .map_err(BossError::map_cdp("办公大楼 keyUp 失败"))?;
            sleep_random_ms(650, 950);
        }
        sleep_random_ms(4000, 6000);

        let click_building_suggestion_js = r#"
        new Promise(resolve => {
            const clean = text => String(text || '').replace(/\s+/g, '');
            const clickNode = el => {
                el.scrollIntoView({ block: 'center', inline: 'center' });
                el.dispatchEvent(new PointerEvent('pointerdown', { bubbles: true, cancelable: true, view: window }));
                el.dispatchEvent(new PointerEvent('pointerup', { bubbles: true, cancelable: true, view: window }));
                el.dispatchEvent(new MouseEvent('mousedown', { bubbles: true, cancelable: true, view: window }));
                el.dispatchEvent(new MouseEvent('mouseup', { bubbles: true, cancelable: true, view: window }));
                el.click();
                el.dispatchEvent(new MouseEvent('click', { bubbles: true, cancelable: true, view: window }));
            };
            const pick = () => Array.from(document.querySelectorAll('li'))
                .find(el => el.querySelector('p.address, .address'));
            const started = Date.now();
            const timer = setInterval(() => {
                const first = pick();
                if (first) {
                    clearInterval(timer);
                    const address = first.querySelector('p.address, .address')?.innerText?.trim() || '';
                    const detail = first.querySelector('p.detail-address, .detail-address')?.innerText?.trim() || '';
                    clickNode(first);
                    resolve({ ok: true, address, detail, waitedMs: Date.now() - started });
                    return;
                }
                if (Date.now() - started > 12000) {
                    clearInterval(timer);
                    resolve({
                        ok: false,
                        msg: 'building suggestion li not found',
                        waitedMs: Date.now() - started,
                        inputValue: document.querySelector('.create-address-dialog .detail-address input')?.value || '',
                        allLi: Array.from(document.querySelectorAll('li'))
                            .slice(0, 120)
                            .map(el => clean(el.innerText || el.textContent || ''))
                    });
                }
            }, 300);
        })
        "#;
        let building_suggestion_ret = self
            .page
            .run_js_await(click_building_suggestion_js)
            .map_err(BossError::map_cdp("点击办公大楼候选地址失败"))?;
        log::info!(
            "  [Debug] 办公大楼候选点击结果: {:?}",
            building_suggestion_ret
        );
        if !is_ok(&building_suggestion_ret) {
            return Err(BossError::element(format!(
                "未找到办公大楼候选地址: {}",
                building
            )));
        }
        sleep_random_ms(1200, 1800);

        // 4.5 填写“详细地址”（楼层/门牌号）
        let mut detail_input = None;
        for _ in 0..12 {
            detail_input = self
                .page
                .ele("css:.create-address-dialog .detail-ipt input, .create-address-dialog input[placeholder*='详细']")
                .map_err(BossError::map_cdp("查找详细地址输入框失败"))?;
            if detail_input.is_some() {
                break;
            }
            sleep_random_ms(300, 500);
        }
        let detail_input =
            detail_input.ok_or_else(|| BossError::element("未找到详细地址输入框"))?;
        let detail = Self::extract_detail_address(target_address);
        log::info!("  [提取详细地址] {}", detail);
        detail_input
            .click()
            .map_err(BossError::map_post("点击详细地址输入框失败"))?;
        sleep_random_ms(200, 400);
        detail_input
            .input(&detail)
            .map_err(BossError::map_post("输入详细地址失败"))?;
        sleep_random_ms(900, 1300);

        // 4.6 点击保存按钮（等待 disabled 移除）
        let mut save_btn = None;
        for _ in 0..12 {
            save_btn = self
                .page
                .ele("css:.create-address-dialog .btn-sure, .create-address-dialog .btn-sure-v2")
                .map_err(BossError::map_cdp("查找保存按钮失败"))?;
            if save_btn.is_some() {
                break;
            }
            sleep_random_ms(300, 500);
        }
        let save_btn = save_btn.ok_or_else(|| BossError::element("未找到保存按钮"))?;
        // 等待 disabled 类消失（通过 JS 轮询）
        let wait_js = r#"
        (function() {
            const btn = document.querySelector('.create-address-dialog .btn-sure');
            if (!btn) return false;
            const start = Date.now();
            while (Date.now() - start < 5000) {
                if (!btn.classList.contains('disabled')) return true;
                // 短忙等
                const now = Date.now();
                while (Date.now() - now < 50) {}
            }
            return false;
        })()
    "#;
        let enabled = self
            .page
            .run_js(wait_js)
            .map(|v| {
                v.as_bool()
                    .or_else(|| v.get("value").and_then(|value| value.as_bool()))
                    .unwrap_or(false)
            })
            .unwrap_or(false);
        if !enabled {
            log::warn!("  [WARN] 保存按钮仍 disabled，尝试强制点击");
        }
        save_btn
            .click()
            .map_err(BossError::map_post("点击保存按钮失败"))?;
        sleep_random_ms(1800, 2600);
        log::info!("  [√] 新地址已保存");

        let js_confirm_new_address = format!(
            r#"
        new Promise(resolve => {{
            const targetRaw = {address};
            const clean = text => String(text || '')
                .replace(/\s+/g, '')
                .replace(/[·,，。;；:：\-—_/]/g, '')
                .toLowerCase();
            const target = clean(targetRaw);
            const clickNode = el => {{
                el.scrollIntoView({{ block: 'center', inline: 'center' }});
                el.dispatchEvent(new PointerEvent('pointerdown', {{ bubbles: true, cancelable: true, view: window }}));
                el.dispatchEvent(new PointerEvent('pointerup', {{ bubbles: true, cancelable: true, view: window }}));
                el.dispatchEvent(new MouseEvent('mousedown', {{ bubbles: true, cancelable: true, view: window }}));
                el.dispatchEvent(new MouseEvent('mouseup', {{ bubbles: true, cancelable: true, view: window }}));
                el.click();
                el.dispatchEvent(new MouseEvent('click', {{ bubbles: true, cancelable: true, view: window }}));
            }};
            const started = Date.now();
            const timer = setInterval(() => {{
                const dialogs = Array.from(document.querySelectorAll(
                    '.job-address-select-new-dialog, .single-address-select-wrap, .dialog-job-address-select'
                ));
                const dialog = dialogs[dialogs.length - 1];
                const sure = dialog && dialog.querySelector('.btn-sure-v2, .btn-sure, button[class*="sure"]');
                if (dialog && sure) {{
                    clearInterval(timer);
                    const items = Array.from(dialog.querySelectorAll('.address-item'));
                    let chosen = '';
                    let score = 0;
                    if (items.length) {{
                        const candidates = items.map(item => {{
                            const text = clean(item.innerText);
                            let s = 0;
                            if (text === target) s += 300;
                            if (text.includes(target)) s += 240;
                            if (target.includes(text)) s += 160;
                            for (const ch of new Set(Array.from(target))) {{
                                if (text.includes(ch)) s += 1;
                            }}
                            return {{ item, text, score: s, preview: item.innerText.trim().slice(0, 120) }};
                        }}).sort((a, b) => b.score - a.score);
                        const best = candidates[0] || {{ item: items[items.length - 1], score: 0, preview: '' }};
                        const pick = best.score > 0 ? best : {{ item: items[items.length - 1], score: 0, preview: items[items.length - 1].innerText.trim().slice(0, 120) }};
                        const radio = pick.item.querySelector('.radio-box, .normal-radio, input[type="radio"]') || pick.item;
                        clickNode(radio);
                        chosen = pick.preview;
                        score = pick.score;
                    }}
                    clickNode(sure);
                    resolve({{ ok: true, chosen, score, itemCount: items.length, waitedMs: Date.now() - started }});
                    return;
                }}
                if (Date.now() - started > 10000) {{
                    clearInterval(timer);
                    resolve({{ ok: false, msg: 'outer address confirm dialog not ready after save', waitedMs: Date.now() - started }});
                }}
            }}, 300);
        }})
        "#,
            address = address_json
        );
        let confirm_ret = self
            .page
            .run_js_await(&js_confirm_new_address)
            .map_err(BossError::map_cdp("确认新工作地址失败"))?;
        log::info!("  [Debug] 新工作地址确认结果: {:?}", confirm_ret);
        if !is_ok(&confirm_ret) {
            return Err(BossError::element(format!(
                "新增地址已保存，但未能在工作地址弹窗中确认选择: {}",
                target_address
            )));
        }
        sleep_random_ms(700, 1000);
        log::info!("  [√] 新工作地址已确认: {}", target_address);

        Ok(())
    }

    /// 从完整地址中提取办公大楼名称（如“新恒富大厦”）
    fn extract_building_name(addr: &str) -> String {
        let suffixes = ["大厦", "广场", "中心", "大楼", "写字楼", "办公楼", "商务楼"];
        for suffix in suffixes {
            if let Some(pos) = addr.find(suffix) {
                // 往前找到最近的行政区或分隔符，如“滨海新区新恒富大厦” -> “新恒富大厦”。
                let start = addr[..pos]
                    .char_indices()
                    .filter_map(|(idx, ch)| {
                        if matches!(
                            ch,
                            ' ' | ',' | '，' | '·' | '省' | '市' | '区' | '县' | '镇' | '乡'
                        ) {
                            Some(idx + ch.len_utf8())
                        } else {
                            None
                        }
                    })
                    .last()
                    .unwrap_or(0);
                let end = pos + suffix.len();
                if start < end && end <= addr.len() {
                    return addr[start..end].to_string();
                }
            }
        }

        addr.chars().take(20).collect()
    }

    /// 从完整地址中提取城市名，优先处理直辖市。
    fn extract_city_name(addr: &str) -> String {
        for city in ["北京", "上海", "天津", "重庆"] {
            if addr.contains(city) {
                return city.to_string();
            }
        }

        if let Some(pos) = addr.find('市') {
            let prefix = &addr[..pos];
            let start = prefix
                .char_indices()
                .filter_map(|(idx, ch)| {
                    if matches!(ch, '省' | '区' | '县' | ' ' | ',' | '，' | '·') {
                        Some(idx + ch.len_utf8())
                    } else {
                        None
                    }
                })
                .last()
                .unwrap_or(0);
            return addr[start..pos].to_string();
        }

        addr.chars().take(2).collect()
    }

    /// 从完整地址中提取详细地址（楼层/门牌号）
    fn extract_detail_address(addr: &str) -> String {
        // 使用简单的字符串搜索，不使用正则
        // 常见模式：数字+层、数字+楼、数字+室、数字+号
        let chars: Vec<char> = addr.chars().collect();
        let mut best = String::new();
        let mut best_len = 0;
        // 提取类似 "13层", "13楼", "1001室", "B座12层"
        for i in 0..chars.len() {
            if chars[i].is_ascii_digit() {
                let mut j = i;
                while j < chars.len()
                    && (chars[j].is_ascii_digit() || chars[j] == ' ' || chars[j] == '-')
                {
                    j += 1;
                }
                // 检查后面是否有单位
                if j < chars.len() && matches!(chars[j], '层' | '樓' | '楼' | '室' | '号' | '座')
                {
                    let mut start = i;
                    if i >= 2 && chars[i - 1] == '座' && chars[i - 2].is_ascii_alphabetic() {
                        start = i - 2;
                    }
                    let end = j + 1;
                    let candidate: String = chars[start..end].iter().collect();
                    if candidate.chars().count() > best_len {
                        best_len = candidate.chars().count();
                        best = candidate;
                    }
                }
            }
        }
        if !best.is_empty() {
            return best;
        }
        // 默认
        "13层".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Poster;

    #[test]
    fn extracts_municipality_city_name() {
        assert_eq!(
            Poster::extract_city_name("天津滨海新区新恒富大厦13层"),
            "天津"
        );
        assert_eq!(
            Poster::extract_city_name("北京海淀区中国外文大厦A座3层"),
            "北京"
        );
    }

    #[test]
    fn extracts_building_after_district() {
        assert_eq!(
            Poster::extract_building_name("天津滨海新区新恒富大厦13层"),
            "新恒富大厦"
        );
        assert_eq!(
            Poster::extract_building_name("北京海淀区中国外文大厦A座3层"),
            "中国外文大厦"
        );
    }

    #[test]
    fn extracts_detail_address_without_utf8_boundary_panic() {
        assert_eq!(
            Poster::extract_detail_address("天津滨海新区新恒富大厦13层"),
            "13层"
        );
        assert_eq!(
            Poster::extract_detail_address("北京海淀区中国外文大厦A座3层"),
            "A座3层"
        );
    }
}
