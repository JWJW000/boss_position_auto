// 四个岗位类型的独立实现
// 每个文件包含完整的从上到下的填写流程，带详细注释

// 重新导出父模块的所有类型，让子模块可以使用
use super::*;

mod fulltime;      // 社招全职
mod campus;        // 应届生校园招聘
mod internship;    // 实习生招聘
mod parttime;      // 兼职招聘
