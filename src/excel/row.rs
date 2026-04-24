use calamine::Data;

use super::columns::ColIdx;
use super::record::JobRecord;

/// Parse one Excel row into a job record using detected column indexes.
pub(crate) fn parse_row(row: &[Data], cols: &ColIdx) -> JobRecord {
    JobRecord {
        招聘类型: cell(row, cols.招聘类型),
        职位名称: cell(row, cols.职位名称),
        职位描述: cell(row, cols.职位描述),
        是否急招: cell_opt(row, cols.是否急招),
        职位类型: cell(row, cols.职位类型),
        经验: cell_opt(row, cols.经验),
        城市: cell(row, cols.城市),
        学历: cell(row, cols.学历),
        薪资低: cell(row, cols.薪资低),
        薪资高: cell(row, cols.薪资高),
        薪资备注: cell_opt(row, cols.薪资备注),
        薪资单位: cell_opt(row, cols.薪资单位),
        结算方式: cell_opt(row, cols.结算方式),
        关键词: cell(row, cols.关键词),
        福利: cell_opt(row, cols.福利),
        届别: cell_opt(row, cols.届别),
        实习时长: cell_opt(row, cols.实习时长),
        其他说明: cell_opt(row, cols.其他说明),
        截止日期: cell_opt(row, cols.截止日期),
    }
}

/// Convert a cell at a required index into a trimmed string.
fn cell(row: &[Data], idx: usize) -> String {
    row.get(idx).and_then(cell_to_string).unwrap_or_default()
}

/// Convert a cell at an optional index into a trimmed string.
fn cell_opt(row: &[Data], idx: Option<usize>) -> String {
    idx.and_then(|i| row.get(i).and_then(cell_to_string))
        .unwrap_or_default()
}

/// Convert supported calamine cell variants into trimmed strings.
pub(crate) fn cell_to_string(cell: &Data) -> Option<String> {
    match cell {
        Data::String(s) => Some(s.clone()),
        Data::Int(i) => Some(i.to_string()),
        Data::Float(f) => Some(format!("{}", *f as i64)),
        Data::Bool(b) => Some(b.to_string()),
        Data::DateTime(dt) => Some(dt.to_string()),
        Data::DateTimeIso(s) => Some(s.clone()),
        Data::DurationIso(s) => Some(s.clone()),
        _ => None,
    }
    .map(|s| s.trim().to_string())
}
