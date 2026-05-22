//! 参数校验型提取器模块。
//!
//! 提供 [`Valid<T>`]、[`ValidQuery<T>`]、[`ValidPath<T>`]、[`ValidJson<T>`]
//! 四个提取器，将参数提取与 `validator` 校验合并为一步操作。
//!
//! ## 设计思路
//!
//! 通过宏统一生成 `FromRequest` / `FromRequestParts` 的实现，避免重复代码。
//! 链式调用逻辑：
//!
//! ```text
//! 请求 → Query<T> → Valid<Query<T>> → ValidQuery<T>（自动校验并提取）
//! 请求 → Path<T>  → Valid<Path<T>>  → ValidPath<T>
//! 请求 → Json<T>  → Valid<Json<T>>  → ValidJson<T>
//! ```

use crate::error::ApiError;
use crate::json::Json;
use crate::path::Path;
use crate::query::Query;
use axum::extract::{FromRequest, FromRequestParts, Request};
use axum::http::request::Parts;

/// 通用校验提取器。
///
/// 包装任意提取器 `T`，自动调用其 `Validate` 实现进行校验。
/// 使用时带两层包装（例如 `Valid<Query<Params>>`），建议使用
/// [`ValidQuery`] / [`ValidPath`] / [`ValidJson`] 简写。
#[derive(Debug, Clone, Default, FromRequest, FromRequestParts)]
#[from_request(via(axum_valid::Valid), rejection(ApiError))]
pub struct Valid<T>(pub T);

/// 带校验的查询参数提取器。
///
/// 等价于 `Valid<Query<T>>`，对查询参数进行自动校验。
#[derive(Debug, Clone, Default)]
pub struct ValidQuery<T>(pub T);

/// 带校验的路径参数提取器。
///
/// 等价于 `Valid<Path<T>>`，对路径参数进行自动校验。
#[derive(Debug, Clone, Default)]
pub struct ValidPath<T>(pub T);

/// 带校验的 JSON Body 提取器。
///
/// 等价于 `Valid<Json<T>>`，对 JSON 请求体进行自动校验。
#[derive(Debug, Clone, Default)]
pub struct ValidJson<T>(pub T);

/// 宏：为校验型提取器统一实现 `FromRequestParts` 或 `FromRequest`。
///
/// ## 用法
///
/// ```ignore
/// impl_from_request!(ValidQuery, Query, FromRequestParts);
/// impl_from_request!(ValidJson,  Json,  FromRequest);
/// ```
macro_rules! impl_from_request {
    ($name:ident, $wrapper:ident, FromRequestParts) => {
        impl<S, T> FromRequestParts<S> for $name<T>
        where
            S: Send + Sync,
            Valid<$wrapper<T>>: FromRequestParts<S, Rejection = ApiError>,
        {
            type Rejection = ApiError;

            async fn from_request_parts(
                parts: &mut Parts,
                state: &S,
            ) -> Result<Self, Self::Rejection> {
                Ok($name(Valid::from_request_parts(parts, state).await?.0.0))
            }
        }
    };
    ($name:ident, $wrapper:ident, FromRequest) => {
        impl<S, T> FromRequest<S> for $name<T>
        where
            S: Send + Sync,
            Valid<$wrapper<T>>: FromRequest<S, Rejection = ApiError>,
        {
            type Rejection = ApiError;

            async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
                Ok($name(Valid::from_request(req, state).await?.0.0))
            }
        }
    };
}

// 使用宏为三种提取器生成实现
impl_from_request!(ValidQuery, Query, FromRequestParts);
impl_from_request!(ValidPath, Path, FromRequestParts);
impl_from_request!(ValidJson, Json, FromRequest);
