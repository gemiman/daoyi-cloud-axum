use crate::app::AppState;
use crate::error::{ApiError, ApiResult};
use axum::{Router, debug_handler, routing};

pub mod user;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", routing::get(index))
        .nest("/api", Router::new().nest("/users", user::create_router()))
        .fallback(async || -> ApiResult<()> {
            tracing::warn!("Not Found");
            Err(ApiError::NotFound)
        })
        .method_not_allowed_fallback(async || -> ApiResult<()> {
            tracing::warn!("Method Not Allowed");
            Err(ApiError::MethodNotAllowed)
        })
}

/// 根路径 `/` 的 GET 处理器。
///
/// 返回一段静态欢迎文本。使用 `#[debug_handler]` 宏以便在
/// 编译期获得更好的类型错误提示。
#[debug_handler]
async fn index() -> &'static str {
    "Hello DaoYi Cloud Axum !"
}

// 违反孤儿规则
// impl IntoResponse for anyhow::Error {
//     fn into_response(self) -> axum::response::Response {
//         // 违反孤儿规则
//     }
// }
