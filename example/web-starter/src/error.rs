//! 统一错误处理模块。
//!
//! 定义了 [`ApiError`] 错误枚举与 [`ApiResult`] 类型别名，覆盖 HTTP 层、业务层、
//! 数据库层等各层面的错误。所有错误均通过 [`IntoResponse`] 自动映射为统一的
//! JSON 响应格式 [`ApiResponse`]。
//!
//! ## 错误类型 → HTTP 状态码映射
//!
//! | 错误类型 | 状态码 |
//! |----------|--------|
//! | `NotFound` | 404 |
//! | `MethodNotAllowed` | 405 |
//! | `Query` / `Path` / `Json` / `Validation` | 400 |
//! | `Internal` / `SeaOrmDb` | 500 |
//! | `Biz` | 200（业务错误） |

use crate::response::ApiResponse;
use axum::extract::rejection::{JsonRejection, PathRejection, QueryRejection};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_valid::ValidRejection;

/// API 层统一结果类型别名。
///
/// 所有处理器函数应返回此类型，以便错误自动转换为 HTTP 响应。
pub type ApiResult<T> = Result<T, ApiError>;

/// API 层统一错误枚举。
///
/// 覆盖路由匹配、请求参数、业务逻辑、数据库、内部服务等各层面的错误场景。
/// 使用 `thiserror` 派生标准 `Error` trait 和 `Display` 实现。
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    /// 路由未匹配（404）。
    #[error("服务器迷路了😵")]
    NotFound,

    /// HTTP 方法不被允许（405）。
    #[error("这个HTTP方法不合法呀🈲")]
    MethodNotAllowed,

    /// 业务错误，消息由业务方自行定义。
    #[error("{0}")]
    Biz(String),

    /// 内部服务错误（500），可自动从 `anyhow::Error` 转换。
    #[error("内部错误:{0}")]
    Internal(#[from] anyhow::Error),

    /// 数据库操作错误（500），可自动从 `sea_orm::DbErr` 转换。
    #[error("数据库错误:{0}")]
    SeaOrmDb(#[from] sea_orm::DbErr),

    /// 查询参数解析失败（400），可自动从 [`QueryRejection`] 转换。
    #[error("查询参数错误:{0}")]
    Query(#[from] QueryRejection),

    /// 路径参数解析失败（400），可自动从 [`PathRejection`] 转换。
    #[error("路径参数错误:{0}")]
    Path(#[from] PathRejection),

    /// JSON Body 解析失败（400），可自动从 [`JsonRejection`] 转换。
    #[error("Body参数错误:{0}")]
    Json(#[from] JsonRejection),

    /// 参数校验失败（400），包含字段级错误详情。
    #[error("参数校验失败:{0}")]
    Validation(String),
}

impl From<ValidRejection<ApiError>> for ApiError {
    /// 将 [`axum_valid`] 的校验结果转换为统一错误。
    ///
    /// - [`ValidRejection::Valid`]：将 [`validator::ValidationErrors`] 格式化为可读字符串
    /// - [`ValidRejection::Inner`]：透传内部的 `ApiError`
    fn from(value: ValidRejection<ApiError>) -> Self {
        match value {
            ValidRejection::Valid(errors) => {
                ApiError::Validation(format_validation_errors(&errors))
            }
            ValidRejection::Inner(error) => error,
        }
    }
}

/// 将 validator 的 `ValidationErrors` 格式化为易读的字段级错误信息。
///
/// 格式：`[字段名]错误描述; [字段名]错误描述`
fn format_validation_errors(errors: &validator::ValidationErrors) -> String {
    format_to_vec(errors).join("; ")
}

/// 递归地将 `ValidationErrors` 树展开为 `[field]message` 形式的字符串列表。
///
/// 支持嵌套结构（Struct）、列表（List）和字段（Field）三种错误类型。
fn format_to_vec(errors: &validator::ValidationErrors) -> Vec<String> {
    use validator::ValidationErrorsKind;
    errors
        .errors()
        .iter()
        .flat_map(|(field, errors_kind)| match errors_kind {
            ValidationErrorsKind::Field(field_errors) => field_errors
                .iter()
                .map(|error| {
                    let message = error
                        .message
                        .as_ref()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| "验证失败".to_string());
                    format!("[{}]{}", field, message)
                })
                .collect::<Vec<_>>(),
            ValidationErrorsKind::Struct(struct_errors) => format_to_vec(struct_errors),
            ValidationErrorsKind::List(list_errors) => list_errors
                .iter()
                .flat_map(|(_index, errors)| format_to_vec(errors))
                .collect::<Vec<_>>(),
        })
        .collect()
}

impl ApiError {
    /// 获取该错误对应的 HTTP 状态码。
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            ApiError::Query(_)
            | ApiError::Path(_)
            | ApiError::Json(_)
            | ApiError::Validation(_) => StatusCode::BAD_REQUEST,
            ApiError::Internal(_) | ApiError::SeaOrmDb(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Biz(_) => StatusCode::OK,
        }
    }
}

impl IntoResponse for ApiError {
    /// 将 `ApiError` 转换为 HTTP 响应。
    ///
    /// 根据错误变体设置对应的 HTTP 状态码，响应体为 JSON 格式的 [`ApiResponse`]：
    /// - `Biz` → HTTP 200，`code = 1`
    /// - `NotFound` → HTTP 404
    /// - `MethodNotAllowed` → HTTP 405
    /// - `Query` / `Path` / `Json` / `Validation` → HTTP 400
    /// - `Internal` / `SeaOrmDb` → HTTP 500
    fn into_response(self) -> Response {
        let status_code = self.status_code();
        let json = axum::Json(ApiResponse::<()>::error(self.to_string()));
        (status_code, json).into_response()
    }
}
