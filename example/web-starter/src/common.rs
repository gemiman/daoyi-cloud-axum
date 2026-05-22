//! 通用数据结构模块。
//!
//! 提供分页参数 [`PageParam`] 与分页响应 [`PageResult`] 等跨模块共享的数据类型。

use crate::validation::validate_page_size;
use daoyi_axum_config::config;
use daoyi_axum_support::support::serde::deserialize_number;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// 分页查询参数。
///
/// 自动从全局配置中读取默认页码和每页条数。
/// 支持前端通过字符串或数字形式传递参数（由 `deserialize_number` 处理）。
///
/// 校验规则：
/// - `page_no`：最小值为 1
/// - `page_size`：由 [`validate_page_size`] 根据配置的上下限进行校验
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PageParam {
    /// 页码，从 1 开始，默认值由配置 `sys.page_no_default` 决定。
    #[validate(range(min = 1, message = "页码最小值为 1"))]
    #[serde(default = "default_page_no", deserialize_with = "deserialize_number")]
    pub page_no: u64,

    /// 每页条数，默认值由配置 `sys.page_size_default` 决定。
    #[validate(custom(function = "validate_page_size"))]
    #[serde(default = "default_page_size", deserialize_with = "deserialize_number")]
    pub page_size: u64,
}

/// 返回 `page_no` 的默认值，从全局配置读取。
fn default_page_no() -> u64 {
    config::get().sys().page_no_default()
}

/// 返回 `page_size` 的默认值，从全局配置读取。
fn default_page_size() -> u64 {
    config::get().sys().page_size_default()
}

/// 分页查询结果。
///
/// 包含分页元信息与当前页数据列表，序列化时字段自动转为 camelCase。
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResult<T> {
    /// 当前页码。
    page_no: u64,

    /// 每页条数。
    page_size: u64,

    /// 总页数（根据 total 和 page_size 自动计算）。
    total_page: u64,

    /// 数据总条数。
    total: u64,

    /// 当前页数据列表。
    list: Vec<T>,
}

impl<T> PageResult<T> {
    /// 构造分页结果。
    ///
    /// `total_page` 由 `(total + page_size - 1) / page_size` 自动计算。
    pub fn new(page_no: u64, page_size: u64, total: u64, list: Vec<T>) -> Self {
        let total_page = (total + page_size - 1) / page_size;
        Self {
            page_no,
            page_size,
            total_page,
            total,
            list,
        }
    }

    /// 直接从 [`PageParam`] 构造分页结果。
    pub fn from_page_param(page_param: PageParam, total: u64, list: Vec<T>) -> Self {
        Self::new(page_param.page_no, page_param.page_size, total, list)
    }
}
