use calamine::Data;

use super::columns::ColIdx;
use super::row::parse_row;

/// Build a string cell for compact parser tests.
fn s(v: &str) -> Data {
    Data::String(v.to_string())
}

#[test]
/// Ensure absent optional columns stay empty instead of reading column zero.
fn detect_optional_columns_missing_should_not_use_first_column() {
    let header = vec![
        s("招聘类型"),
        s("职位名称"),
        s("职位描述"),
        s("职位类型"),
        s("经验"),
        s("工作地点"),
        s("学历"),
        s("薪资范围"),
        s("薪资范围"),
        s("职位关键词"),
    ];
    let cols = ColIdx::detect(&header).expect("detect should pass");
    let row = vec![
        s("实习生招聘"),
        s("Java工程师"),
        s("描述"),
        s("Java"),
        s("在校/应届"),
        s("北京"),
        s("本科"),
        s("250"),
        s("300"),
        s("Java Spring"),
    ];

    let record = parse_row(&row, &cols);
    assert_eq!(record.招聘类型, "实习生招聘");
    assert_eq!(record.职位名称, "Java工程师");
    assert_eq!(record.经验, "在校/应届");
    assert_eq!(record.学历, "本科");
    assert_eq!(record.薪资低, "250");
    assert_eq!(record.薪资高, "300");
    assert_eq!(record.薪资单位, "");
    assert_eq!(record.结算方式, "");
    assert_eq!(record.关键词, "Java Spring");
    assert_eq!(record.是否急招, "");
    assert_eq!(record.薪资备注, "");
    assert_eq!(record.福利, "");
    assert_eq!(record.届别, "");
    assert_eq!(record.实习时长, "");
    assert_eq!(record.其他说明, "");
    assert_eq!(record.截止日期, "");
}

#[test]
/// Ensure the user's real header layout maps every field to the right column.
fn detect_real_headers_should_parse_sample_row() {
    let header = vec![
        s("招聘类型"),
        s("职位名称"),
        s("职位描述"),
        s("是否驻外"),
        s("职位类型"),
        s("经验"),
        s("学历"),
        s("薪资范围"),
        s("薪资范围"),
        s("薪资范围"),
        s("职位关键词"),
        s("工作地点"),
        s("毕业时间"),
        s("最少实习月数"),
        s("最少周到岗天数"),
        s("招聘截止时间"),
    ];
    let cols = ColIdx::detect(&header).expect("detect should pass for real headers");
    let row = vec![
        s("实习生招聘"),
        s("Java工程师-实习（J15117）"),
        s("职位描述内容"),
        s("境内岗位"),
        s("Java"),
        s("在校/应届"),
        s("本科"),
        s("250"),
        s("300"),
        s("无"),
        s("Java Spring Mysql"),
        s("北京海淀区中国外文大厦A座3层"),
        s("无"),
        s("3个月"),
        s("5天"),
        s("无"),
    ];

    let record = parse_row(&row, &cols);
    assert_eq!(record.招聘类型, "实习生招聘");
    assert_eq!(record.职位名称, "Java工程师-实习（J15117）");
    assert_eq!(record.是否急招, "境内岗位");
    assert_eq!(record.职位类型, "Java");
    assert_eq!(record.经验, "在校/应届");
    assert_eq!(record.学历, "本科");
    assert_eq!(record.薪资低, "250");
    assert_eq!(record.薪资高, "300");
    assert_eq!(record.薪资备注, "无");
    assert_eq!(record.薪资单位, "");
    assert_eq!(record.结算方式, "");
    assert_eq!(record.关键词, "Java Spring Mysql");
    assert_eq!(record.城市, "北京海淀区中国外文大厦A座3层");
    assert_eq!(record.届别, "无");
    assert_eq!(record.实习时长, "3个月");
    assert_eq!(record.其他说明, "5天");
    assert_eq!(record.截止日期, "无");
}

#[test]
/// Ensure the new template header names parse without repeated salary columns.
fn detect_template_headers_should_parse_explicit_salary_fields() {
    let header = vec![
        s("招聘类型"),
        s("职位名称"),
        s("职位描述"),
        s("职位类型"),
        s("经验"),
        s("学历"),
        s("薪资下限"),
        s("薪资上限"),
        s("薪资单位"),
        s("结算方式"),
        s("职位关键词"),
        s("工作地点"),
        s("毕业时间"),
        s("最少实习月数"),
        s("最少周到岗天数"),
        s("招聘截止时间"),
    ];
    let cols = ColIdx::detect(&header).expect("detect should pass for template headers");
    let row = vec![
        s("兼职招聘"),
        s("兼职Java讲师"),
        s("职位描述内容"),
        s("Java"),
        s("不限"),
        s("本科"),
        s("80"),
        s("120"),
        s("元/时"),
        s("日结"),
        s("Java Spring"),
        s("北京海淀区中国外文大厦A座3层"),
        s("不限"),
        s(""),
        s(""),
        s("2026-05-31"),
    ];

    let record = parse_row(&row, &cols);
    assert_eq!(record.招聘类型, "兼职招聘");
    assert_eq!(record.薪资低, "80");
    assert_eq!(record.薪资高, "120");
    assert_eq!(record.薪资单位, "元/时");
    assert_eq!(record.结算方式, "日结");
    assert_eq!(record.城市, "北京海淀区中国外文大厦A座3层");
    assert_eq!(record.届别, "不限");
    assert_eq!(record.截止日期, "2026-05-31");
}
