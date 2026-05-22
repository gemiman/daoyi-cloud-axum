//! 统一 API 响应模块。
//!
//! 定义了 [`ApiResponse<T>`] 作为所有 API 接口的标准化响应格式，
//! 并提供便捷的 [`success`] / [`fail`] 构造函数。
//!
//! ## 响应格式
//!
//! ```json
//! {
//!   "code": 0,
//!   "msg": "",
//!   "data": { ... }
//! }
//! ```
//!
//! - `code = 0` 表示成功，`code = 1` 表示失败
//! - `data` 字段在值为 `None` 时自动省略

use crate::error::{ApiError, ApiResult};
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

/// 通用 API 响应类型别名。
///
/// 所有处理器函数应返回 `CommonResult<T>` 以自动应用成功/失败包装。
pub type CommonResult<T> = ApiResult<ApiResponse<T>>;

/// API 统一响应结构体。
///
/// 包含 `code`（业务状态码）、`msg`（提示消息）、`data`（业务数据）三个字段。
/// 实现 `IntoResponse` 以直接作为 Axum 处理器的返回值。
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// 业务状态码：`0` 表示成功，`1` 表示失败。
    code: i32,

    /// 提示消息，成功时为空，失败时包含错误描述。
    msg: String,

    /// 业务数据，为 `None` 时序列化时自动省略。
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl<T> ApiResponse<T> {
    /// 构造一个自定义状态的响应。
    pub fn new<M: AsRef<str>>(code: i32, msg: M, data: Option<T>) -> Self {
        Self {
            code,
            msg: String::from(msg.as_ref()),
            data,
        }
    }

    /// 构造一个成功响应（`code = 0`）。
    pub fn ok(data: Option<T>) -> Self {
        Self::new(0, String::from(""), data)
    }

    /// 构造一个错误响应（`code = 1`）。
    pub fn error<M: AsRef<str>>(msg: M) -> Self {
        Self::new(1, msg, None)
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    /// `IntoResponse` 实现。
    ///
    /// 成功响应返回 HTTP 200，响应体为 JSON 格式。
    /// Content-Type 自动设为 `application/json`。
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}

/// 快捷构造成功响应。
///
/// 等价于 `Ok(ApiResponse::ok(Some(data)))`。
///
/// ## 示例
///
/// ```rust,ignore
/// return success(user);
/// ```
pub fn success<T>(data: T) -> CommonResult<T> {
    Ok(ApiResponse::ok(Some(data)))
}

/// 快捷构造业务失败响应。
///
/// 等价于 `Err(ApiError::Biz(msg.into()))`。
///
/// ## 示例
///
/// ```rust,ignore
/// return fail("用户不存在");
/// ```
pub fn fail<M: AsRef<str>>(msg: M) -> CommonResult<()> {
    Err(ApiError::Biz(String::from(msg.as_ref())))
}
