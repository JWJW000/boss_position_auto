//! boss_auto - BOSS直聘自动化发布岗位工具
//!
//! 主要模块:
//!   - [boss]: 扫码登录 + Cookie管理
//!   - [excel]: Excel读取
//!   - [poster]: 职位表单填写与发布

pub mod boss;
pub mod excel;
pub mod error;
pub mod poster;
pub mod utils;


pub use boss::BossClient;
pub use excel::{ExcelReader, JobRecord, FailedJob, export_failed_jobs};
pub use poster::Poster;