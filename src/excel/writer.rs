use rust_xlsxwriter::{Format, Workbook, Worksheet};
use std::path::Path;

use crate::error::BossError;
use super::record::JobRecord;

/// 失败记录
#[derive(Debug, Clone)]
pub struct FailedJob {
    pub row_number: usize,
    pub job: JobRecord,
    pub error_message: String,
}

/// 导出失败的岗位到 Excel
pub fn export_failed_jobs(failed_jobs: &[FailedJob], output_path: &Path) -> Result<(), BossError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // 设置表头格式
    let header_format = Format::new()
        .set_bold()
        .set_background_color(rust_xlsxwriter::Color::RGB(0xD3D3D3));

    // 设置错误信息格式
    let error_format = Format::new()
        .set_font_color(rust_xlsxwriter::Color::RGB(0xFF0000));

    // 写入表头
    write_header(worksheet, &header_format)?;

    // 写入失败的岗位数据
    for (idx, failed) in failed_jobs.iter().enumerate() {
        let row = (idx + 1) as u32;
        write_job_row(worksheet, row, failed, &error_format)?;
    }

    // 自动调整列宽
    adjust_column_widths(worksheet)?;

    // 保存文件
    workbook.save(output_path)
        .map_err(|e| BossError::Excel(format!("保存失败记录Excel失败: {}", e)))?;

    Ok(())
}

/// 写入表头
fn write_header(worksheet: &mut Worksheet, format: &Format) -> Result<(), BossError> {
    let headers = vec![
        "原始行号",
        "失败原因",
        "招聘类型",
        "职位名称",
        "职位描述",
        "是否急招",
        "是否驻外",
        "职位类型",
        "经验",
        "城市",
        "学历",
        "薪资下限",
        "薪资上限",
        "薪资备注",
        "薪资单位",
        "结算方式",
        "职位关键词",
        "届别",
        "最少实习月数",
        "最少周到岗天数",
        "招聘截止时间",
    ];

    for (col, header) in headers.iter().enumerate() {
        worksheet.write_string_with_format(0, col as u16, *header, format)
            .map_err(|e| BossError::Excel(format!("写入表头失败: {}", e)))?;
    }

    Ok(())
}

/// 写入单行岗位数据
fn write_job_row(
    worksheet: &mut Worksheet,
    row: u32,
    failed: &FailedJob,
    error_format: &Format,
) -> Result<(), BossError> {
    let job = &failed.job;

    // 原始行号
    worksheet.write_number(row, 0, failed.row_number as f64)
        .map_err(|e| BossError::Excel(format!("写入行号失败: {}", e)))?;

    // 失败原因（红色）
    worksheet.write_string_with_format(row, 1, &failed.error_message, error_format)
        .map_err(|e| BossError::Excel(format!("写入失败原因失败: {}", e)))?;

    // 岗位信息
    let values = vec![
        &job.招聘类型,
        &job.职位名称,
        &job.职位描述,
        &job.是否急招,
        &job.是否驻外,
        &job.职位类型,
        &job.经验,
        &job.城市,
        &job.学历,
        &job.薪资低,
        &job.薪资高,
        &job.薪资备注,
        &job.薪资单位,
        &job.结算方式,
        &job.关键词,
        &job.届别,
        &job.最少实习月数,
        &job.最少周到岗天数,
        &job.截止日期,
    ];

    for (col, value) in values.iter().enumerate() {
        worksheet.write_string(row, (col + 2) as u16, value.as_str())
            .map_err(|e| BossError::Excel(format!("写入数据失败: {}", e)))?;
    }

    Ok(())
}

/// 自动调整列宽
fn adjust_column_widths(worksheet: &mut Worksheet) -> Result<(), BossError> {
    // 设置常见列宽
    worksheet.set_column_width(0, 10)  // 原始行号
        .map_err(|e| BossError::Excel(format!("设置列宽失败: {}", e)))?;
    worksheet.set_column_width(1, 50)  // 失败原因
        .map_err(|e| BossError::Excel(format!("设置列宽失败: {}", e)))?;
    worksheet.set_column_width(2, 15)  // 招聘类型
        .map_err(|e| BossError::Excel(format!("设置列宽失败: {}", e)))?;
    worksheet.set_column_width(3, 30)  // 职位名称
        .map_err(|e| BossError::Excel(format!("设置列宽失败: {}", e)))?;
    worksheet.set_column_width(4, 50)  // 职位描述
        .map_err(|e| BossError::Excel(format!("设置列宽失败: {}", e)))?;

    // 其他列使用默认宽度 12
    for col in 5..21 {
        worksheet.set_column_width(col, 12)
            .map_err(|e| BossError::Excel(format!("设置列宽失败: {}", e)))?;
    }

    Ok(())
}
