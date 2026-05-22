//! 自定义 Serde 反序列化工具。
//!
//! 提供 [`deserialize_number`] 等通用反序列化函数，用于处理前端可能以
//! 字符串或数字形式传递的数值类型。

use serde::{Deserialize, Deserializer};
use std::fmt::Display;
use std::str::FromStr;

/// 内部辅助枚举，用于区分字符串和数字两种 JSON 表示形式。
///
/// 使用 `#[serde(untagged)]` 使得反序列化时按顺序尝试匹配。
#[derive(Deserialize)]
#[serde(untagged)]
enum StringOrNumber<T> {
    String(String),
    Number(T),
}

/// 通用的数字反序列化函数。
///
/// 支持从 JSON 字符串（如 `"42"`）或数字（如 `42`）中解析出数值类型。
/// 适用于前端可能将分页参数、ID 等字段以字符串形式传递的场景。
///
/// ## 示例
///
/// ```rust,ignore
/// #[derive(Deserialize)]
/// struct Params {
///     #[serde(deserialize_with = "deserialize_number")]
///     page_no: u64,
/// }
/// ```
pub fn deserialize_number<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + Deserialize<'de>,
    T::Err: Display,
{
    match StringOrNumber::deserialize(deserializer)? {
        StringOrNumber::String(s) => s.parse().map_err(serde::de::Error::custom),
        StringOrNumber::Number(n) => Ok(n),
    }
}
