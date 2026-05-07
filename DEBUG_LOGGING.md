# 调试日志增强说明

## 概述

为了解决不同地区网页元素定位失败的问题，我们增强了元素定位的调试日志功能。现在当元素定位失败时，会输出详细的调试信息，方便直接在浏览器控制台测试选择器。

## 新增的日志功能

### 1. **SelectorMap::find_first() - 多重选择器尝试日志**

当使用多个备选选择器查找元素时，会输出：

```
[DEBUG] 开始尝试 4 个选择器定位元素
[DEBUG]   [尝试 1/4] 选择器: css:input[name='jobName']
[INFO]    [✓ 成功] 选择器定位成功: css:input[name='jobName']
```

**失败时的输出：**

```
[DEBUG] 开始尝试 4 个选择器定位元素
[DEBUG]   [尝试 1/4] 选择器: css:input[name='jobName']
[WARN]    [✗ 未找到] 选择器未匹配到元素: css:input[name='jobName']
[WARN]      → 可在浏览器控制台测试: document.querySelector('input[name='jobName']') 或 $x('input[name='jobName']')
[DEBUG]   [尝试 2/4] 选择器: css:.jobName
[ERROR]   [✗ 错误] 选择器执行失败: css:.jobName | 错误: CdpError(...)
[ERROR]     → 可在浏览器控制台测试该选择器语法是否正确
[ERROR] [失败] 所有 4 个选择器均未找到元素
[ERROR]   → 尝试的选择器列表:
[ERROR]     1. css:input[name='jobName']
[ERROR]     2. css:.jobName
[ERROR]     3. css:input.ipt[placeholder*='职位名称']
[ERROR]     4. xpath://input[@name='jobName']
```

### 2. **wait_visible_dropdown_items() - 下拉选项等待日志**

等待下拉选项出现时：

```
[DEBUG] 等待下拉选项出现 (超时: 3500ms)
[DEBUG]   选择器: xpath://div[contains(@class,'ui-select-dropdown') and not(contains(@style,'display: none'))]//li[contains(@class,'ui-select-item')]
[TRACE]   [尝试 1] 下拉选项未出现，继续等待...
[TRACE]   [尝试 2] 下拉选项未出现，继续等待...
[INFO]    [✓ 成功] 找到 5 个下拉选项 (尝试 3 次)
```

**超时失败时：**

```
[WARN] [超时] 可见下拉选项未出现，尝试查找所有下拉选项
[DEBUG]   备用选择器: xpath://li[contains(@class,'ui-select-item')]
[ERROR]   [✗ 失败] 备用选择器也未找到任何下拉选项
[ERROR]     → 可在浏览器控制台测试:
[ERROR]        $x("//div[contains(@class,'ui-select-dropdown') and not(contains(@style,'display: none'))]//li[contains(@class,'ui-select-item')]")
[ERROR]        $x("//li[contains(@class,'ui-select-item')]")
```

### 3. **choose_visible_option_exact_or_contains() - 选项选择日志**

选择下拉选项时：

```
[DEBUG] 尝试选择下拉选项: "本科"
[DEBUG]   找到 5 个选项，开始精确匹配
[TRACE]     选项 1: "不限"
[TRACE]     选项 2: "本科"
[INFO]    [✓ 精确匹配] 找到选项 "本科", 点击中...
[INFO]    [✓ 成功] 已点击选项
```

**未找到匹配选项时：**

```
[DEBUG] 尝试选择下拉选项: "硕士"
[DEBUG]   找到 5 个选项，开始精确匹配
[DEBUG]   精确匹配失败，尝试模糊匹配
[ERROR]   [✗ 失败] 未找到匹配的选项: "硕士"
[ERROR]     可用选项列表:
[ERROR]       1. "不限"
[ERROR]       2. "本科"
[ERROR]       3. "大专"
[ERROR]       4. "高中"
[ERROR]       5. "初中"
```

### 4. **click_row_select_by_label() - JavaScript 点击日志**

使用 JavaScript 点击表单行下拉框时：

```
[DEBUG] 尝试点击表单行下拉框: 行标题="学历要求", 索引=0
[DEBUG]   执行 JavaScript 点击下拉框
[INFO]    [✓ 成功] 已点击表单行 "学历要求" 的第 1 个下拉框
```

**失败时会输出可直接在浏览器执行的 JavaScript：**

```
[DEBUG] 尝试点击表单行下拉框: 行标题="学历要求", 索引=0
[DEBUG]   执行 JavaScript 点击下拉框
[ERROR]   [✗ 失败] JavaScript 返回 false，未找到或点击失败
[ERROR]     → 可在浏览器控制台执行以下代码调试:
[ERROR]     (() => { const label = "学历要求"; const clean = text => (text || '').replace(/\s+/g, ''); ... })();
```

## 如何使用调试日志

### 1. **启用详细日志**

在 `config.toml` 中设置：

```toml
[debug]
verbose = true
```

或者设置环境变量：

```bash
RUST_LOG=debug cargo run
```

### 2. **在浏览器控制台测试选择器**

当看到日志中的选择器失败时，可以直接复制日志中的测试命令到浏览器控制台：

**CSS 选择器：**
```javascript
document.querySelector('input[name="jobName"]')
```

**XPath 选择器：**
```javascript
$x("//input[@name='jobName']")
```

**JavaScript 代码：**
```javascript
// 直接复制日志中输出的完整 JS 代码
(() => { const label = "学历要求"; ... })();
```

### 3. **调试流程**

1. 运行程序，观察日志输出
2. 找到失败的选择器
3. 在浏览器控制台测试该选择器
4. 如果选择器在控制台也失败，说明页面结构已变化
5. 在控制台手动查找正确的选择器
6. 更新 `selectors.json` 配置文件

## 日志级别说明

- **TRACE**: 最详细的调试信息（每次尝试）
- **DEBUG**: 调试信息（开始/结束操作）
- **INFO**: 成功操作的信息
- **WARN**: 警告信息（部分失败但有备选方案）
- **ERROR**: 错误信息（操作失败）

## 配置选择器

选择器配置文件位置：
- Windows: `%LOCALAPPDATA%\boss_auto\selectors.json`
- macOS/Linux: `~/.local/share/boss_auto/selectors.json`

示例配置：

```json
{
  "job_title": [
    "css:input[name='jobName']",
    "css:.jobName",
    "css:input.ipt[placeholder*='职位名称']",
    "xpath://input[@name='jobName']"
  ],
  "education": [
    "xpath://div[contains(@class,'form-item')][.//*[contains(text(),'学历')]]//div[contains(@class,'ui-select')]",
    "css:.ui-select-inner"
  ]
}
```

## 常见问题排查

### 问题 1: 所有选择器都失败

**日志特征：**
```
[ERROR] [失败] 所有 4 个选择器均未找到元素
```

**解决方法：**
1. 检查页面是否完全加载
2. 在浏览器控制台手动测试选择器
3. 检查页面 DOM 结构是否变化
4. 添加新的选择器到配置文件

### 问题 2: 下拉选项未出现

**日志特征：**
```
[WARN] [超时] 可见下拉选项未出现
```

**解决方法：**
1. 检查下拉框是否被正确点击打开
2. 增加等待时间
3. 检查下拉选项的 class 名称是否变化

### 问题 3: JavaScript 执行失败

**日志特征：**
```
[ERROR] [✗ 失败] JavaScript 执行失败
```

**解决方法：**
1. 复制日志中的 JS 代码到浏览器控制台
2. 检查是否有语法错误
3. 检查页面结构是否匹配 JS 中的选择器

## 贡献

如果你在特定地区发现新的选择器模式，欢迎提交 PR 更新默认选择器列表。
