//! 自定义路径参数提取器。
//!
//! 对 [`axum::extract::Path`] 的薄封装，将路径参数解析错误自动映射为 [`ApiError::Path`]，
//! 并实现 [`HasValidate`](axum_valid::HasValidate) trait 以支持参数校验。

use crate::support::error::ApiError;
use axum::extract::FromRequestParts;
use axum_valid::HasValidate;

/// 自定义路径参数提取器。
///
/// 用法与 `axum::extract::Path` 一致，但解析失败时会自动转换为 `ApiError`。
///
/// ## 示例
///
/// ```rust,ignore
/// use crate::path::Path;
///
/// async fn handler(Path(id): Path<u64>) -> ApiResult<()> {
///     // id 已被自动提取并校验
/// }
/// ```
#[derive(Debug, Clone, Default, FromRequestParts)]
#[from_request(via(axum::extract::Path), rejection(ApiError))]
pub struct Path<T>(pub T);

impl<T> HasValidate for Path<T> {
    type Validate = T;

    fn get_validate(&self) -> &Self::Validate {
        &self.0
    }
}
