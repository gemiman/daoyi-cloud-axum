//! 系统通用配置子模块。
//!
//! 提供分页限制、默认值等系统级运行参数，所有字段均为可选，
//! 未配置时使用内置默认值。

use serde::Deserialize;

/// 系统通用配置。
///
/// 对应 YAML 配置文件中 `sys` 节点，可通过 `APP_SYS_PAGE_SIZE_MIN` 等
/// 环境变量进行覆盖。
///
/// ## 示例 YAML
///
/// ```yaml
/// sys:
///   page_size_min: 10
///   page_size_max: 100
///   page_size_default: 20
///   page_no_default: 1
/// ```
#[derive(Debug, Deserialize, Default)]
pub struct SysConfig {
    /// 分页每页条数最小值，默认 `1`。
    page_size_min: Option<u64>,

    /// 分页每页条数最大值，默认 `200`。
    page_size_max: Option<u64>,

    /// 分页每页条数默认值，默认 `10`。
    page_size_default: Option<u64>,

    /// 分页页码默认值，默认 `1`。
    page_no_default: Option<u64>,
}

impl SysConfig {
    /// 获取分页每页条数的最小值限制。
    pub fn page_size_min(&self) -> u64 {
        self.page_size_min.unwrap_or(1)
    }

    /// 获取分页每页条数的最大值限制。
    pub fn page_size_max(&self) -> u64 {
        self.page_size_max.unwrap_or(200)
    }

    /// 获取分页每页条数的默认值。
    pub fn page_size_default(&self) -> u64 {
        self.page_size_default.unwrap_or(10)
    }

    /// 获取分页页码的默认值。
    pub fn page_no_default(&self) -> u64 {
        self.page_no_default.unwrap_or(1)
    }
}
