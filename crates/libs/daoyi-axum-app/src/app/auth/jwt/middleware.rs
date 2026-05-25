//! JWT 认证中间件模块。
//!
//! 基于 [`tower_http::auth`] 的 `AsyncAuthorizeRequest` trait 实现 Bearer Token
//! 认证中间件。从 HTTP 请求的 `Authorization` Header 中提取 Token，
//! 解码成功后将 [`Principal`] 注入请求的 Extensions，解码失败则返回 401 响应。
//!
//! ## 认证流程
//!
//! ```text
//! HTTP Request
//!   → 提取 Authorization Header
//!   → 验证 Bearer 前缀
//!   → JWT 解码与校验
//!   → 注入 Principal 到 Extensions
//!   → 放行请求 / 返回 401
//! ```
//!
//! ## 使用方式
//!
//! ```rust,ignore
//! use daoyi_axum_app::app::auth::jwt::middleware::get_auth_layer;
//!
//! Router::new()
//!     .route_layer(get_auth_layer())
//!     // ... 受保护的路由
//! ```

use crate::app::auth::jwt::{JWT, default_jwt};
use axum::body::Body;
use axum::http::{Request, header};
use axum::response::Response;
use daoyi_axum_support::support::error::ApiError;
use std::pin::Pin;
use tower_http::auth::{AsyncAuthorizeRequest, AsyncRequireAuthorizationLayer};

/// JWT 认证中间件。
///
/// 持有 [`JWT`] 编解码器的静态引用，实现 [`AsyncAuthorizeRequest`] trait，
/// 通过 [`AsyncRequireAuthorizationLayer`] 注册为 Tower Layer。
///
/// ## 认证逻辑
///
/// 1. 从 `Authorization` Header 中提取 Bearer Token
/// 2. 若 Header 缺失 → 返回 401（Unauthenticated）
/// 3. 若缺少 Bearer 前缀 → 返回 401
/// 4. Token 解码失败 → 返回 500（Internal，因为 Token 格式问题通常由服务端引起）
/// 5. 解码成功 → 将 [`Principal`] 注入 `request.extensions_mut()`
#[derive(Debug, Clone)]
pub struct JWTAuth {
    /// JWT 编解码器的静态引用。
    jwt: &'static JWT,
}

impl JWTAuth {
    /// 创建新的 JWT 认证中间件实例。
    pub fn new(jwt: &'static JWT) -> Self {
        Self { jwt }
    }
}

impl AsyncAuthorizeRequest<Body> for JWTAuth {
    type RequestBody = Body;
    type ResponseBody = Body;
    type Future = Pin<
        Box<
            dyn Future<Output = Result<Request<Self::RequestBody>, Response<Self::ResponseBody>>>
                + Send,
        >,
    >;

    /// 异步执行认证逻辑。
    ///
    /// 从请求中提取并验证 Bearer Token，成功则将 Principal 注入 Extensions，
    /// 失败则返回包含错误信息的 HTTP 响应。
    fn authorize(&mut self, mut request: Request<Body>) -> Self::Future {
        let jwt = self.jwt;
        Box::pin(async move {
            let token = request
                .headers()
                .get(header::AUTHORIZATION)
                .map(|value| -> Result<_, ApiError> {
                    let token = value
                        .to_str()
                        .map_err(|_| {
                            ApiError::Unauthenticated(String::from(
                                "Authorization Header 不是一个有效的字符串",
                            ))
                        })?
                        .strip_prefix("Bearer ")
                        .ok_or_else(|| {
                            ApiError::Unauthenticated(String::from(
                                "Authorization Header 缺少 Bearer 前缀",
                            ))
                        })?;
                    Ok(token)
                })
                .transpose()?
                .ok_or_else(|| {
                    ApiError::Unauthenticated(String::from("Authorization Header 缺失"))
                })?;
            let principal = jwt.decode(token).map_err(|err| ApiError::Internal(err))?;
            request.extensions_mut().insert(principal);
            Ok(request)
        })
    }
}

/// 构造 JWT 认证 Layer。
///
/// 使用全局默认 JWT 实例创建 [`AsyncRequireAuthorizationLayer`]，
/// 可直接通过 `Router::route_layer()` 注册。
///
/// ## 示例
///
/// ```rust,ignore
/// Router::new()
///     .nest("/api", protected_routes)
///     .route_layer(get_auth_layer())
/// ```
pub fn get_auth_layer() -> AsyncRequireAuthorizationLayer<JWTAuth> {
    AsyncRequireAuthorizationLayer::new(JWTAuth::new(default_jwt()))
}
