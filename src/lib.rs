//! boss_auto - BOSS直聘自动化发布岗位工具
//!
//! 主要模块:
//!   - [boss]: 扫码登录 + Cookie管理
//!   - [excel]: Excel读取
//!   - [poster]: 职位表单填写与发布
//!   - [config]: 配置文件管理

pub mod boss;
pub mod config;
pub mod error;
pub mod excel;
pub mod poster;
pub mod utils;

pub use boss::BossClient;
pub use config::AppConfig;
pub use excel::{export_failed_jobs, ExcelReader, FailedJob, JobRecord};
pub use poster::Poster;
