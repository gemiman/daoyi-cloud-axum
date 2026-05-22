//! API 路由模块。
//!
//! 统一管理所有 API 路由的定义与组装，包括根路径、业务子路由、
//! 全局 404 fallback 和 405 method_not_allowed fallback。
//!
//! ## 路由结构
//!
//! ```text
//! GET  /           → 欢迎页
//! GET  /api/users  → 用户列表（条件查询）
//! GET  /api/users/page → 用户分页查询
//! ```
//!
//! 子模块：
//! - [`user`] — 用户相关 API

use axum::{Router, debug_handler, routing};
use daoyi_axum_app::app::AppState;
use daoyi_axum_support::support::error::{ApiError, ApiResult};
use daoyi_axum_support::support::response::{CommonResult, success};

pub mod user;

/// 创建并组装所有 API 路由。
///
/// 返回的 [`Router`] 包含：
/// - 根路径 `/` 的 GET 处理器
/// - `/api/users` 嵌套路由
/// - 全局 404 fallback（返回 JSON 错误）
/// - 全局 405 method_not_allowed fallback（返回 JSON 错误）
pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", routing::get(index))
        .nest("/api", Router::new().nest("/users", user::create_router()))
        // 所有未匹配的路由返回 404
        .fallback(async || -> ApiResult<()> {
            tracing::warn!("Not Found");
            Err(ApiError::NotFound)
        })
        // 路由匹配但方法错误时返回 405
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
async fn index() -> CommonResult<&'static str> {
    success("Hello DaoYi Cloud Axum !")
}
