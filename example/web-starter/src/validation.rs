//! 自定义参数校验函数。
//!
//! 提供与 `validator` crate 配合使用的校验函数，通过 `#[validate(custom)]` 属性
//! 标注在字段上即可启用。
//!
//! ## 使用方式
//!
//! ```rust,ignore
//! #[derive(Validate)]
//! struct Params {
//!     #[validate(custom(function = "validate_page_size"))]
//!     page_size: u64,
//!     #[validate(custom(function = "validate_mobile_phone"))]
//!     mobile_phone: String,
//! }
//! ```

use daoyi_axum_config::config;
use regex::Regex;
use std::borrow::Cow;
use std::cell::LazyCell;
use validator::ValidationError;

/// 国际手机号正则。
///
/// 规则：`+` 开头可选，后跟 1-9 开头的 7-15 位数字。
/// 使用 [`LazyCell`] 确保正则只编译一次。
const MOBILE_PHONE_REGEX: LazyCell<Regex> = LazyCell::new(|| {
    Regex::new(r"^\+?[1-9]\d{6,14}$").expect("Failed to compile mobile phone regex")
});

/// 校验分页每页条数是否在配置的上下限范围内。
///
/// 下限和上限分别从全局配置 `sys.page_size_min/max` 读取。
///
/// ## 错误消息
///
/// - 小于最小值：`"每页条数最小值为 {min}"`
/// - 大于最大值：`"每页条数最大值为 {max}"`
pub fn validate_page_size(page_size: u64) -> Result<(), ValidationError> {
    let sys_config = config::get().sys();
    let min = sys_config.page_size_min();
    let max = sys_config.page_size_max();
    match page_size {
        s if s < min => {
            let mut err = ValidationError::new("page_size_range");
            err.message = Some(format!("每页条数最小值为 {min}").into());
            Err(err)
        }
        s if s > max => {
            let mut err = ValidationError::new("page_size_range");
            err.message = Some(format!("每页条数最大值为 {max}").into());
            Err(err)
        }
        _ => Ok(()),
    }
}

/// 校验国际手机号格式。
///
/// 使用 [`MOBILE_PHONE_REGEX`] 正则进行匹配：
/// - 必须以 `+` 或 1-9 开头
/// - 总长度 7-15 位数字
pub fn validate_mobile_phone(value: &str) -> Result<(), ValidationError> {
    if MOBILE_PHONE_REGEX.is_match(value) {
        Ok(())
    } else {
        Err(build_validation_error("手机号格式不正确"))
    }
}

/// 构建一个包含自定义消息的 `ValidationError`。
///
/// 错误码固定为 `"invalid"`。
fn build_validation_error(message: &'static str) -> ValidationError {
    ValidationError {
        code: Cow::from("invalid"),
        message: Some(Cow::from(message)),
        params: Default::default(),
    }
}
