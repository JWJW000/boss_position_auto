use anyhow::Context;
use calamine::{open_workbook, Data, Reader, Xlsx};

use super::columns::ColIdx;
use super::record::JobRecord;
use super::row::{cell_to_string, parse_row};

/// Reads one Excel workbook and returns job records from its first sheet.
pub struct ExcelReader {
    path: std::path::PathBuf,
}

impl ExcelReader {
    /// Create a reader for a concrete Excel path.
    pub fn new(path: impl Into<std::path::PathBuf>) -> Self {
        Self { path: path.into() }
    }

    /// Read the first worksheet and parse non-empty job-name rows.
    pub fn read(&self) -> anyhow::Result<Vec<JobRecord>> {
        let path = &self.path;
        let mut workbook: Xlsx<_> =
            open_workbook(path).with_context(|| format!("无法打开Excel文件: {:?}", path))?;

        let sheet_name = workbook
            .sheet_names()
            .first()
            .cloned()
            .context("Excel中没有工作表")?;

        let range = workbook
            .worksheet_range(&sheet_name)
            .map_err(|e| anyhow::anyhow!("读取工作表失败: {}", e))?;

        let mut rows = range.rows();
        let header: Vec<Data> = rows
            .next()
            .ok_or_else(|| anyhow::anyhow!("Excel表头为空"))?
            .to_vec();
        let cols = detect_columns_or_log(&header)?;

        log::info!(
            "检测到列: 招聘类型={}, 职位名称={}, 城市={}, 薪资低={}, 薪资高={}",
            cols.招聘类型,
            cols.职位名称,
            cols.城市,
            cols.薪资低,
            cols.薪资高
        );

        let mut records = Vec::new();
        for (row_idx, row) in rows.enumerate() {
            let row_data: Vec<Data> = row.to_vec();
            let record = parse_row(&row_data, &cols);
            if !record.职位名称.is_empty() {
                log::debug!("第{}行解析成功: {}", row_idx + 2, record.职位名称);
                records.push(record);
            }
        }

        log::info!("共解析 {} 条有效职位记录", records.len());
        Ok(records)
    }
}

/// Detect columns and print the full header when detection fails.
fn detect_columns_or_log(header: &[Data]) -> anyhow::Result<ColIdx> {
    match ColIdx::detect(header) {
        Ok(cols) => Ok(cols),
        Err(e) => {
            log::error!("列检测失败: {}", e);
            log::info!("Excel表头共{}列:", header.len());
            for (i, cell) in header.iter().enumerate() {
                if let Some(v) = cell_to_string(cell) {
                    log::info!("  [{}] {}", i, v);
                }
            }
            Err(anyhow::anyhow!("列检测失败: {}", e))
        }
    }
}
