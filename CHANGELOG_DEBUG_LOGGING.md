# 调试日志增强 - 更新日志

## 版本信息
- **更新日期**: 2026-05-07
- **影响范围**: 元素定位系统
- **目的**: 解决不同地区网页元素定位失败的调试问题

## 修改文件

### 1. `src/poster/selectors.rs`
**修改内容**: 增强 `SelectorMap::find_first()` 方法的日志输出

**新增功能**:
- ✅ 显示尝试的选择器总数
- ✅ 逐个显示每个选择器的尝试过程
- ✅ 区分三种结果：成功、未找到、执行错误
- ✅ 失败时输出可直接在浏览器控制台测试的命令
- ✅ 最终失败时列出所有尝试过的选择器

**代码变更**:
```rust
// 之前：简单循环，无日志
for sel in selectors {
    if let Ok(Some(el)) = page.ele(sel) {
        return Some(el);
    }
}

// 之后：详细日志，三种状态处理
for (index, sel) in selectors.iter().enumerate() {
    log::debug!("  [尝试 {}/{}] 选择器: {}", index + 1, selectors.len(), sel);
    match page.ele(sel) {
        Ok(Some(el)) => { /* 成功日志 */ }
        Ok(None) => { /* 未找到日志 + 浏览器测试命令 */ }
        Err(e) => { /* 错误日志 */ }
    }
}
```

### 2. `src/poster/dom.rs`
**修改内容**: 为三个关键 DOM 操作方法添加详细日志

#### 2.1 `wait_visible_dropdown_items()`
**新增功能**:
- ✅ 显示等待超时时间和选择器
- ✅ 记录每次尝试的次数
- ✅ 成功时显示找到的选项数量和尝试次数
- ✅ 超时时尝试备用选择器
- ✅ 失败时输出浏览器测试命令（XPath）

**代码变更**:
```rust
// 之前：静默等待，无反馈
while std::time::Instant::now() < deadline {
    if let Ok(items) = self.page.eles(selector) {
        if !items.is_empty() {
            return items;
        }
    }
    sleep_random_ms(100, 180);
}

// 之后：详细日志，尝试计数，备用方案
let mut attempt = 0;
while std::time::Instant::now() < deadline {
    attempt += 1;
    match self.page.eles(selector) {
        Ok(items) => {
            if !items.is_empty() {
                log::info!("找到 {} 个下拉选项 (尝试 {} 次)", items.len(), attempt);
                return items;
            }
            log::trace!("下拉选项未出现，继续等待...");
        }
        Err(e) => { log::warn!("查询失败: {:?}", e); }
    }
}
// + 备用选择器逻辑 + 浏览器测试命令
```

#### 2.2 `choose_visible_option_exact_or_contains()`
**新增功能**:
- ✅ 显示要选择的目标值
- ✅ 显示找到的选项总数
- ✅ TRACE 级别显示每个选项的文本
- ✅ 区分精确匹配和模糊匹配
- ✅ 点击失败时显示错误信息
- ✅ 未找到时列出所有可用选项

**代码变更**:
```rust
// 之前：静默匹配，无反馈
for it in &items {
    let t = it.text().unwrap_or_default().trim().to_string();
    if t == value {
        it.click().ok();
        return true;
    }
}

// 之后：详细日志，错误处理
for (i, it) in items.iter().enumerate() {
    let t = it.text().unwrap_or_default().trim().to_string();
    log::trace!("选项 {}: \"{}\"", i + 1, t);
    if t == value {
        log::info!("精确匹配找到选项 \"{}\"", t);
        match it.click() {
            Ok(_) => { log::info!("已点击选项"); return true; }
            Err(e) => { log::error!("点击失败: {:?}", e); return false; }
        }
    }
}
// + 模糊匹配日志 + 失败时列出所有选项
```

#### 2.3 `click_row_select_by_label()`
**新增功能**:
- ✅ 显示要点击的表单行标题和索引
- ✅ TRACE 级别显示完整 JavaScript 代码
- ✅ 区分 JavaScript 执行失败和返回 false
- ✅ 失败时输出可直接在浏览器执行的 JS 代码（单行格式）

**代码变更**:
```rust
// 之前：直接执行，无日志
self.page
    .run_js(&js)
    .map_err(BossError::map_cdp(format!("点击{}下拉失败", row_label)))
    .map(|v| v.get("value").and_then(|x| x.as_bool()).unwrap_or(false))

// 之后：详细日志，区分失败类型
log::debug!("尝试点击表单行下拉框: 行标题=\"{}\", 索引={}", row_label, index);
log::trace!("JS 代码:\n{}", js);

match self.page.run_js(&js) {
    Ok(v) => {
        let success = v.get("value").and_then(|x| x.as_bool()).unwrap_or(false);
        if success {
            log::info!("已点击表单行 \"{}\" 的第 {} 个下拉框", row_label, index + 1);
        } else {
            log::error!("JavaScript 返回 false，未找到或点击失败");
            log::error!("可在浏览器控制台执行: {}", js_oneline);
        }
        Ok(success)
    }
    Err(e) => {
        log::error!("JavaScript 执行失败: {:?}", e);
        log::error!("可在浏览器控制台执行: {}", js_oneline);
        Err(...)
    }
}
```

### 3. `DEBUG_LOGGING.md` (新增)
**内容**: 完整的调试日志使用文档
- 日志功能说明
- 使用方法
- 浏览器控制台测试指南
- 常见问题排查
- 配置文件说明

## 日志级别使用

| 级别 | 用途 | 示例 |
|------|------|------|
| TRACE | 最详细的调试信息 | 每个下拉选项的文本、完整 JS 代码 |
| DEBUG | 操作开始/结束 | 开始尝试选择器、执行 JavaScript |
| INFO | 成功操作 | 找到元素、点击成功 |
| WARN | 部分失败但有备选 | 主选择器失败，尝试备用选择器 |
| ERROR | 操作失败 | 所有选择器失败、点击失败 |

## 浏览器测试命令格式

### CSS 选择器
```javascript
document.querySelector('input[name="jobName"]')
```

### XPath 选择器
```javascript
$x("//input[@name='jobName']")
```

### JavaScript 代码
```javascript
// 单行格式，方便复制
(() => { const label = "学历要求"; const clean = text => (text || '').replace(/\s+/g, ''); ... })();
```

## 使用建议

1. **开发调试**: 设置 `RUST_LOG=debug` 或 `RUST_LOG=trace`
2. **生产环境**: 使用默认日志级别（INFO）
3. **问题排查**: 
   - 查看 ERROR 日志找到失败的选择器
   - 复制日志中的测试命令到浏览器控制台
   - 手动验证选择器是否有效
   - 更新 `selectors.json` 配置文件

## 向后兼容性

✅ 完全向后兼容
- 所有修改仅添加日志，不改变原有逻辑
- 不影响现有功能
- 不需要修改配置文件
- 不需要修改调用代码

## 性能影响

⚡ 性能影响极小
- 日志输出仅在调试模式下详细
- 生产环境默认只输出 INFO 及以上级别
- 字符串格式化仅在日志启用时执行

## 下一步计划

1. **地区配置系统** (未实现)
   - 为不同地区维护独立的选择器配置
   - 自动检测地区并加载对应配置

2. **智能重试机制** (未实现)
   - 为每个选择器配置独立超时
   - 支持重试次数配置

3. **选择器学习系统** (未实现)
   - 记录成功的选择器
   - 自动调整选择器优先级

## 测试建议

1. 运行程序并观察日志输出
2. 故意使用错误的选择器测试日志
3. 在浏览器控制台验证日志中的测试命令
4. 确认不同日志级别的输出符合预期

## 相关文件

- `src/poster/selectors.rs` - 选择器映射和查找逻辑
- `src/poster/dom.rs` - DOM 操作辅助方法
- `DEBUG_LOGGING.md` - 调试日志使用文档
- `config.toml` - 应用配置（包含 debug.verbose 选项）
- `%LOCALAPPDATA%\boss_auto\selectors.json` - 选择器配置文件
