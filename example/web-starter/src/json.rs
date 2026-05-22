//! 自定义 JSON 请求体提取器。
//!
//! 对 [`axum::extract::Json`] 的薄封装，将 JSON 解析错误自动映射为 [`ApiError::Json`]，
//! 并实现 [`HasValidate`](axum_valid::HasValidate) trait 以支持参数校验。

use crate::error::ApiError;
use axum::extract::FromRequest;
use axum_valid::HasValidate;

/// 自定义 JSON 提取器。
///
/// 用法与 `axum::extract::Json` 一致，但解析失败时会自动转换为 `ApiError`，
/// 无需在处理器中手动处理 `JsonRejection`。
///
/// ## 示例
///
/// ```rust,ignore
/// use crate::json::Json;
///
/// async fn handler(Json(payload): Json<MyPayload>) -> ApiResult<()> {
///     // payload 已被自动校验
/// }
/// ```
#[derive(Debug, Clone, Default, FromRequest)]
#[from_request(via(axum::extract::Json), rejection(ApiError))]
pub struct Json<T>(pub T);

impl<T> HasValidate for Json<T> {
    type Validate = T;

    /// 返回内部值的引用，供 `axum-valid` 进行校验。
    fn get_validate(&self) -> &Self::Validate {
        &self.0
    }
}
