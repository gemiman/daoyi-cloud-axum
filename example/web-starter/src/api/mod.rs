//! API 路由模块。
//!
//! 统一管理所有 API 路由的定义与组装，包括业务子路由、
//! 认证中间件注入、静态资源路由、全局 404 fallback 和 405 method_not_allowed fallback。
//!
//! ## 路由结构
//!
//! ```text
//! GET  /              → index_handler  → SPA fallback（返回 index.html）
//! GET  /api           → index          → 欢迎页（"Hello DaoYi Cloud Axum !"）
//! POST /api/auth/login → login          → 用户登录（无需认证）
//! GET  /api/auth/user-info → user_info  → 获取当前用户信息（需认证）
//! GET  /api/users      → query_users    → 条件查询用户列表（需认证）
//! GET  /api/users/page → find_page      → 分页查询用户（需认证）
//! POST /api/users      → create         → 创建用户（需认证）
//! PUT  /api/users/{id}  → update         → 更新用户（需认证）
//! DELETE /api/users/{id} → delete        → 删除用户（需认证）
//! GET  /static/{*file}  → static_handler → 静态资源（JS/CSS/图片，带压缩）
//! *    /other           → fallback       → 404 Not Found（JSON 错误）
//! *    (other method)   → method_not_allowed → 405（JSON 错误）
//! ```
//!
//! ## 认证策略
//!
//! - `/api/auth/*` — 公开路由，无需 Bearer Token
//! - `/api/users/*` — 受保护路由，需携带有效的 JWT Bearer Token
//! - `/api` 下的所有未匹配路径返回 404
//!
//! ## 子模块
//!
//! - [`auth`] — 认证相关 API（登录 / 用户信息）
//! - [`user`] — 用户相关 API（完整 CRUD）
//! - [`web`](crate::web) — 静态资源处理

use crate::web;
use axum::{Router, debug_handler, routing};
use daoyi_axum_app::app::AppState;
use daoyi_axum_app::app::auth::jwt::middleware::get_auth_layer;
use daoyi_axum_support::support::error::{ApiError, ApiResult};
use daoyi_axum_support::support::response::{CommonResult, success};
use tower_http::compression::CompressionLayer;

pub mod auth;
pub mod user;

/// 创建并组装所有 API 路由。
///
/// 返回的 [`Router`] 包含三层结构：
/// - **`/api`** 路由组：
///   - `/api/users/*` — 用户 CRUD（受 JWT 认证保护）
///   - `/api/auth/*` — 认证接口（登录公开，用户信息受保护）
///   - `/api` 根路径 — 欢迎页
///   - fallback — 404 JSON 错误响应
/// - **`/static/{*file}`** 路由组：
///   - 静态资源文件，带 gzip / brotli 压缩支持
/// - **全局 fallback**：
///   - 路由不匹配 → 404 JSON 错误
///   - 方法不匹配 → 405 JSON 错误
pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest(
            "/api",
            Router::new()
                // 用户 CRUD 路由（需 JWT 认证）
                .nest("/users", user::create_router())
                .route_layer(get_auth_layer())
                // 未来可在此添加其他中间件
                .route("/", routing::get(index))
                // 认证路由（内部自行管理认证粒度）
                .nest("/auth", auth::create_router())
                // 所有未匹配的 /api/* 路径返回 404
                .fallback(async || -> ApiResult<()> {
                    tracing::warn!("API 路由未匹配: Not Found");
                    Err(ApiError::NotFound)
                }),
        )
        // 静态资源路由（带压缩）
        .nest(
            "/static",
            Router::new()
                .route("/{*file}", routing::get(web::static_handler))
                .route_layer(CompressionLayer::new()),
        )
        // 路由匹配但 HTTP 方法不被允许时返回 405
        .method_not_allowed_fallback(async || -> ApiResult<()> {
            tracing::warn!("HTTP 方法不被允许: Method Not Allowed");
            Err(ApiError::MethodNotAllowed)
        })
        // 所有未匹配的非 API 路径 → SPA fallback（返回 index.html）
        .fallback(web::index_handler)
}

/// API 根路径 `/api` 的 GET 处理器。
///
/// 返回一段静态欢迎文本，可用于健康检查或基本信息展示。
/// 使用 `#[debug_handler]` 宏以便在编译期获得更好的类型错误提示。
#[debug_handler]
async fn index() -> CommonResult<&'static str> {
    success("Hello DaoYi Cloud Axum !")
}
