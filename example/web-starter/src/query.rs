//! 自定义查询参数提取器。
//!
//! 对 [`axum::extract::Query`] 的薄封装，将查询参数解析错误自动映射为 [`ApiError::Query`]，
//! 并实现 [`HasValidate`](axum_valid::HasValidate) trait 以支持参数校验。

use crate::error::ApiError;
use axum::extract::FromRequestParts;
use axum_valid::HasValidate;

/// 自定义查询参数提取器。
///
/// 用法与 `axum::extract::Query` 一致，但解析失败时会自动转换为 `ApiError`。
///
/// ## 示例
///
/// ```rust,ignore
/// use crate::query::Query;
///
/// async fn handler(Query(params): Query<MyParams>) -> ApiResult<()> {
///     // params 已被自动提取并校验
/// }
/// ```
#[derive(Debug, Clone, Default, FromRequestParts)]
#[from_request(via(axum::extract::Query), rejection(ApiError))]
pub struct Query<T>(pub T);

impl<T> HasValidate for Query<T> {
    type Validate = T;

    fn get_validate(&self) -> &Self::Validate {
        &self.0
    }
}
