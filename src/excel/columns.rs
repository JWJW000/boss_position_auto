use calamine::Data;
use std::collections::HashMap;

use crate::error::BossError;

use super::row::cell_to_string;

/// Column indexes detected from the Excel header row.
pub(crate) struct ColIdx {
    pub(crate) 招聘类型: usize,
    pub(crate) 职位名称: usize,
    pub(crate) 职位描述: usize,
    pub(crate) 是否急招: Option<usize>,
    pub(crate) 职位类型: usize,
    pub(crate) 经验: Option<usize>,
    pub(crate) 城市: usize,
    pub(crate) 学历: usize,
    pub(crate) 薪资低: usize,
    pub(crate) 薪资高: usize,
    pub(crate) 薪资备注: Option<usize>,
    pub(crate) 薪资单位: Option<usize>,
    pub(crate) 结算方式: Option<usize>,
    pub(crate) 关键词: usize,
    pub(crate) 福利: Option<usize>,
    pub(crate) 届别: Option<usize>,
    pub(crate) 实习时长: Option<usize>,
    pub(crate) 其他说明: Option<usize>,
    pub(crate) 截止日期: Option<usize>,
}

impl ColIdx {
    /// Detect all required and optional column indexes from a header row.
    pub(crate) fn detect(header: &[Data]) -> Result<Self, BossError> {
        let idx = build_header_index(header);
        macro_rules! get {
            ($name:expr) => {
                idx.get($name)
                    .copied()
                    .ok_or_else(|| BossError::Excel(format!("未找到列: {}", $name)))?
            };
        }

        let salary_low_idx = required_any(
            &idx,
            &["薪资下限", "薪资最低", "最低薪资", "最低月薪", "最低日薪", "薪资范围"],
            "薪资下限/薪资范围",
        )?;
        let salary_high_idx = find_any(
            &idx,
            &["薪资上限", "薪资最高", "最高薪资", "最高月薪", "最高日薪", "薪资范围.1"],
        )
        .unwrap_or(salary_low_idx);
        let city_idx = idx
            .get("城市")
            .copied()
            .or_else(|| idx.get("工作地点").copied())
            .ok_or_else(|| BossError::Excel("未找到列: 城市".to_string()))?;
        let graduate_idx = idx
            .get("届别")
            .copied()
            .or_else(|| idx.get("毕业时间").copied());

        Ok(Self {
            招聘类型: get!("招聘类型"),
            职位名称: get!("职位名称"),
            职位描述: get!("职位描述"),
            是否急招: idx.get("是否急招").copied().or_else(|| idx.get("是否驻外").copied()),
            职位类型: get!("职位类型"),
            经验: idx.get("经验").copied().or_else(|| idx.get("工作经验").copied()),
            城市: city_idx,
            学历: get!("学历"),
            薪资低: salary_low_idx,
            薪资高: salary_high_idx,
            薪资备注: idx.get("薪资备注").copied().or_else(|| idx.get("薪资范围.2").copied()),
            薪资单位: find_any(&idx, &["薪资单位", "计薪单位", "薪资类型"]),
            结算方式: find_any(&idx, &["结算方式", "兼职结算方式"]),
            关键词: get!("职位关键词"),
            福利: idx.get("公司福利").copied(),
            届别: graduate_idx,
            实习时长: idx.get("实习时长").copied().or_else(|| idx.get("最少实习月数").copied()),
            其他说明: idx.get("其他说明").copied().or_else(|| idx.get("最少周到岗天数").copied()),
            截止日期: idx.get("招聘截止").copied().or_else(|| idx.get("招聘截止时间").copied()),
        })
    }
}

/// Find the first existing header from a list of aliases.
fn find_any(idx: &HashMap<String, usize>, names: &[&str]) -> Option<usize> {
    names.iter().find_map(|name| idx.get(*name).copied())
}

/// Find a required header from aliases and return a clear error label.
fn required_any(
    idx: &HashMap<String, usize>,
    names: &[&str],
    label: &str,
) -> Result<usize, BossError> {
    find_any(idx, names).ok_or_else(|| BossError::Excel(format!("未找到列: {}", label)))
}

/// Build a map from header text to column index, adding `.1` suffixes for duplicates.
fn build_header_index(header: &[Data]) -> HashMap<String, usize> {
    let mut idx = HashMap::new();
    for (i, cell) in header.iter().enumerate() {
        if let Some(value) = cell_to_string(cell) {
            let value = value.trim();
            if value.is_empty() {
                continue;
            }
            let key = unique_header_key(&idx, value);
            idx.insert(key, i);
        }
    }
    idx
}

/// Return a header key that mirrors pandas-style duplicate column suffixes.
fn unique_header_key(idx: &HashMap<String, usize>, value: &str) -> String {
    if !idx.contains_key(value) {
        return value.to_string();
    }
    let mut n = 1;
    while idx.contains_key(&format!("{}.{}", value, n)) {
        n += 1;
    }
    format!("{}.{}", value, n)
}
