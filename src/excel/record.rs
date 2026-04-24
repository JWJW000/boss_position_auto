/// One job posting record parsed from a single Excel row.
#[derive(Debug, Clone, Default)]
pub struct JobRecord {
    /// 招聘类型：实习生招聘 / 应届生校园招聘 / 社招全职
    pub 招聘类型: String,
    /// 职位名称
    pub 职位名称: String,
    /// 职位描述（富文本）
    pub 职位描述: String,
    /// 是否急招/是否驻外
    pub 是否急招: String,
    /// 职位类型：如 Java
    pub 职位类型: String,
    /// 经验要求：如 在校/应届、1-3年
    pub 经验: String,
    /// 工作地点或城市
    pub 城市: String,
    /// 学历要求
    pub 学历: String,
    /// 薪资低值
    pub 薪资低: String,
    /// 薪资高值
    pub 薪资高: String,
    /// 薪资备注，如 13薪 或 无
    pub 薪资备注: String,
    /// 薪资单位：仅兼职招聘会选择；社招/校招默认元/月，实习默认元/天
    pub 薪资单位: String,
    /// 兼职结算方式：仅兼职招聘使用，如日结、周结、月结等
    pub 结算方式: String,
    /// 职位关键词，通常用空格或逗号分隔
    pub 关键词: String,
    /// 公司福利
    pub 福利: String,
    /// 届别/毕业时间
    pub 届别: String,
    /// 实习月数
    pub 实习时长: String,
    /// 周到岗天数或其他说明
    pub 其他说明: String,
    /// 招聘截止日期
    pub 截止日期: String,
}
