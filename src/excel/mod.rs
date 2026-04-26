//! Excel读取模块。
//!
//! 入口文件只负责组织子模块；列检测、行解析和测试分别放在
//! `excel/` 目录下，避免单个文件过长。

mod columns;
mod reader;
mod record;
mod row;
mod writer;

#[cfg(test)]
mod tests;

pub use reader::ExcelReader;
pub use record::JobRecord;
pub use writer::{export_failed_jobs, FailedJob};
