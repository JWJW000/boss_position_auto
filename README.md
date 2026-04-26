# boss_auto

BOSS直聘自动化发布岗位工具 - 基于 Rust 开发的高效职位批量发布助手。

## 功能特性

- ✅ 批量读取 Excel 职位信息并自动发布
- ✅ 扫码登录 BOSS直聘，Cookie 自动保存免重复登录
- ✅ 支持多种招聘类型：实习生、应届生校园招聘、社招全职、兼职
- ✅ 智能表单填写，自动适配不同招聘类型的字段
- ✅ 断点续传功能，支持从指定行继续发布
- ✅ Dry-run 模式，预览数据不实际发布
- ✅ 详细日志记录，方便问题排查

## 快速开始

### 安装

```bash
# 克隆项目
git clone <repository-url>
cd boss_auto

# 编译
cargo build --release

# 可执行文件位于
# target/release/boss_auto.exe (Windows)
# target/release/boss_auto (Linux/Mac)
```

### 使用方法

```bash
# 基础用法：读取同目录下的 Excel 文件并发布
boss_auto

# 指定 Excel 文件路径
boss_auto --excel "职位信息.xlsx"

# 预览模式：只读取 Excel 内容，不实际发布
boss_auto --dry-run

# 强制重新扫码登录
boss_auto --relogin

# 断点续传：从第 5 行开始发布
boss_auto --start-row 5

# 组合使用
boss_auto --excel "招聘.xlsx" --start-row 3
```

## Excel 模板格式

Excel 第一行必须为表头，支持以下列（顺序不限）：

### 必填字段

| 列名 | 说明 | 示例 |
|------|------|------|
| 招聘类型 | 招聘类型 | 实习生招聘 / 应届生校园招聘 / 社招全职 / 兼职招聘 |
| 职位名称 | 完整职位名称 | Java开发工程师 |
| 职位描述 | 职位详细介绍（支持多行文本） | 负责后端开发... |
| 职位类型 | 职位分类 | Java / Python / 前端 |
| 城市 | 工作城市 | 北京 / 上海 |
| 学历 | 学历要求 | 本科 / 大专 / 硕士 / 博士 / 不限 |
| 薪资下限 | 最低薪资 | 10k / 250 |
| 薪资上限 | 最高薪资 | 20k / 350 |
| 职位关键词 | 逗号或空格分隔的标签 | Java,Spring,Mysql |

### 可选字段

| 列名 | 说明 | 示例 |
|------|------|------|
| 是否急招 | 填"急招"或"紧急"会勾选急招标记 | 急招 |
| 是否驻外 | 是否境外工作 | 长期驻境外 / 短期境外出差 / 境内岗位 |
| 经验 | 工作经验要求 | 在校/应届 / 1-3年 / 3-5年 |
| 薪资备注 | 薪资说明 | 13薪 / 14薪 |
| 薪资单位 | 计薪单位（兼职必填） | 元/月 / 元/天 / 元/时 |
| 结算方式 | 兼职结算方式 | 日结 / 周结 / 月结 |
| 届别 | 毕业年份（校招必填） | 2025 / 2026 |
| 最少实习月数 | 实习时长要求 | 3个月 / 6个月 |
| 最少周到岗天数 | 每周到岗天数 | 3天 / 5天 |
| 招聘截止时间 | 招聘截止日期 | 2026-05-31 |

### 表头别名支持

部分字段支持多种表头名称：

- **城市**: `城市` 或 `工作地点`
- **薪资**: `薪资下限`/`薪资最低`/`最低薪资`/`薪资范围`
- **薪资上限**: `薪资上限`/`薪资最高`/`最高薪资`/`薪资范围.1`
- **届别**: `届别` 或 `毕业时间`
- **经验**: `经验` 或 `工作经验`

## 数据存储位置

- **Cookie**: `%LOCALAPPDATA%\boss_auto\cookies.json` (Windows)
- **日志**: `%LOCALAPPDATA%\boss_auto\logs\`
- **选择器缓存**: `%LOCALAPPDATA%\boss_auto\selectors.json`

Linux/Mac 路径为 `~/.local/share/boss_auto/`

## 项目结构

```
boss_auto/
├── Cargo.toml
├── README.md
├── .gitignore
├── templates/
│   └── BOSS职位发布模板.xlsx    # Excel 模板文件
└── src/
    ├── main.rs                   # CLI 入口
    ├── lib.rs                    # 库入口
    ├── excel/
    │   ├── mod.rs               # Excel 读取模块
    │   ├── record.rs            # 职位记录结构
    │   ├── columns.rs           # 列索引检测
    │   ├── row.rs               # 行解析
    │   └── tests.rs             # 单元测试
    ├── boss/
    │   └── mod.rs               # BOSS 登录 + Cookie 管理
    ├── poster/
    │   ├── mod.rs               # 职位发布主逻辑
    │   ├── requirements.rs      # 职位要求填写
    │   ├── basic_fields.rs      # 基础字段填写
    │   ├── misc.rs              # 辅助方法
    │   └── job_types/           # 各招聘类型实现
    │       ├── fulltime.rs      # 社招全职
    │       ├── campus.rs        # 校园招聘
    │       ├── internship.rs    # 实习生
    │       └── parttime.rs      # 兼职
    └── error.rs                 # 错误类型定义
```

## 注意事项

1. **首次使用需要扫码登录** — 登录成功后 Cookie 会自动保存，后续无需重复登录
2. **表单选择器可能需适配** — 如 BOSS 网站结构调整，删除 `selectors.json` 让程序重新学习
3. **发布有冷却时间** — 建议每次发布间隔 30 秒以上，避免触发反爬机制
4. **验证码处理** — 如遇图形验证码，目前需要人工介入
5. **Excel 格式** — 确保 Excel 第一行为表头，数据从第二行开始
6. **字段匹配** — 程序会自动匹配表头名称，支持常见别名

## 开发

### 运行测试

```bash
cargo test
```

### 调试模式

```bash
# 开发模式运行
cargo run -- --dry-run

# 查看详细日志
RUST_LOG=debug cargo run
```

### 构建发布版本

```bash
cargo build --release
```

## 技术栈

- **Rust** - 高性能系统编程语言
- **rust_drission** - 浏览器自动化库
- **calamine** - Excel 文件读取
- **serde** - 序列化/反序列化
- **log** - 日志记录

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！
