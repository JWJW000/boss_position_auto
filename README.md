# boss_auto

BOSS直聘自动化发布岗位工具。

## 功能

- 读取 Excel 中的职位信息（名称、描述、薪资、城市、学历等）
- 扫码登录 BOSS直聘（QR code），保存 Cookie 免重复登录
- 自动填写职位发布表单并提交
- 支持 dry-run 预览Excel内容不发布
- 支持断点续传（`--start-row N`）

## 使用方法

```bash
# 读取同目录Excel，扫码登录，发布全部职位
boss_auto

# 指定Excel路径
boss_auto --excel "职位信息.xlsx"

# 只读Excel，显示内容不发布
boss_auto --dry-run

# 强制重新扫码登录
boss_auto --relogin

# 从第3条开始发布（断点续传）
boss_auto --start-row 3
```

## Excel格式

Excel 第一行为表头，支持以下列：

| 列名 | 说明 |
|------|------|
| 招聘类型 | 实习生招聘 / 应届生校园招聘 / 社招全职 |
| 职位名称 | 完整职位名称 |
| 职位描述 | 职位详细介绍（支持多行文本） |
| 是否急招 | 填"急招"或"紧急"会勾选急招标记 |
| 职位类型 | 如 Java、Python、前端 |
| 城市 | 工作城市 |
| 学历 | 本科 / 大专 / 硕士 |
| 薪资范围 | 低薪资（数字或带k，如 10k、250） |
| 薪资范围.1 | 高薪资 |
| 薪资备注 | 如 13薪 |
| 职位关键词 | 逗号分隔的标签，如 Java,Spring,Mysql |
| 公司福利 | 公司福利描述 |
| 届别 | 2025-2026 等 |
| 实习时长 | 如 3个月 |
| 其他说明 | 补充信息 |
| 招聘截止 | 日期 |

## 数据存储

- Cookie: `%LOCALAPPDATA%\boss_auto\cookies.json`
- 日志: `%LOCALAPPDATA%\boss_auto\logs\`
- 选择器缓存: `%LOCALAPPDATA%\boss_auto\selectors.json`

## 打包为 EXE

```bash
cargo build --release --bin boss_auto
# 输出: target/release/boss_auto.exe
```

## 注意事项

1. **首次使用需要扫码登录** — 登录成功后 Cookie 会自动保存
2. **表单选择器可能需适配** — 如BOSS网站结构调整，删除 `selectors.json` 重新学习
3. **发布有冷却时间** — 建议每次发布间隔 30 秒以上
4. **验证码** — 如遇图形验证码，目前需要人工介入

## 目录结构

```
boss_auto/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs       # CLI入口
    ├── lib.rs        # 库入口
    ├── excel.rs      # Excel读取
    ├── boss.rs       # BOSS登录+Cookie管理
    ├── poster.rs     # 职位表单填写+发布
    └── error.rs      # 错误类型
```
