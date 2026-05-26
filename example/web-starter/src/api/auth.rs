//! 认证 API 模块。
//!
//! 提供用户登录认证与用户信息查询接口。
//! 登录接口验证账号密码后签发 JWT Token，
//! 用户信息接口从 Bearer Token 中解析当前登录用户身份。
//!
//! ## 安全设计
//!
//! - 密码使用 bcrypt 异步验证，避免阻塞 tokio 工作线程
//! - 用户不存在时同样执行一次假 bcrypt 验证，使响应时间与"用户存在但密码错误"
//!   相近，**防止时序攻击枚举有效账号**
//!
//! ## 端点
//!
//! | 方法 | 路径 | 认证 | 说明 |
//! |------|------|------|------|
//! | POST | `/api/auth/login` | 无需 | 用户登录，返回 JWT Token |
//! | GET | `/api/auth/user-info` | 需要 | 获取当前登录用户信息 |

use axum::extract::State;
use axum::{Extension, Router, debug_handler, routing};
use daoyi_axum_app::app::AppState;
use daoyi_axum_app::app::auth::Principal;
use daoyi_axum_app::app::auth::jwt::default_jwt;
use daoyi_axum_app::app::auth::jwt::middleware::get_auth_layer;
use daoyi_axum_support::support::error::ApiError;
use daoyi_axum_support::support::passwd::verify_passwd_async;
use daoyi_axum_support::support::response::{CommonResult, success};
use daoyi_axum_support::support::valid::ValidJson;
use daoyi_sea_orm_entity_demo::demo::entity::demo_sys_user;
use daoyi_sea_orm_entity_demo::demo::entity::prelude::*;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// 创建认证相关的路由。
///
/// 路由结构：
/// - `/login` — POST，用户登录（无需认证）
/// - `/user-info` — GET，获取当前用户信息（需 Bearer Token 认证）
pub fn create_router() -> Router<AppState> {
    Router::new()
        // 需要认证的路由放在前面，通过 route_layer 包裹
        .route("/user-info", routing::get(user_info))
        .route_layer(get_auth_layer())
        // 公开路由放在后面
        .route("/login", routing::post(login))
}

/// 登录请求参数。
///
/// 账号和密码均需通过 validator 的基本长度校验，
/// 实际安全性由后续 bcrypt 验证保证。
#[derive(Debug, Deserialize, Validate)]
pub struct LoginParams {
    /// 账号，长度 1-16。
    #[validate(length(min = 1, max = 16, message = "账号长度必须在1-16之间"))]
    pub account: String,

    /// 密码，长度 6-16。
    #[validate(length(min = 6, max = 16, message = "密码长度必须在6-16之间"))]
    pub password: String,
}

/// 登录响应结果。
///
/// 仅包含一个处于有效期的 JWT Bearer Token，客户端应存储在
/// localStorage / sessionStorage 或 httpOnly cookie 中，
/// 后续请求通过 `Authorization: Bearer <token>` Header 进行身份验证。
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResult {
    /// JWT 访问令牌。
    access_token: String,
}

/// 用户登录处理器。
///
/// `POST /api/auth/login`
///
/// ## 处理流程
///
/// 1. 校验请求参数（validator）
/// 2. 按账号查询用户
/// 3. 若用户存在 → bcrypt 验证密码
/// 4. 密码正确 → 签发 JWT Token
/// 5. 用户不存在或密码错误 → 返回业务错误（401）
///
/// ## 安全说明
///
/// 当账号不存在时，执行一次虚拟 bcrypt 验证（用固定假密码），
/// 确保响应时间与"账号存在但密码错误"的情况相近，
/// 防止攻击者通过响应时间差异枚举有效用户名。
#[debug_handler]
#[tracing::instrument(name = "login", skip_all, fields(account = %account))]
async fn login(
    State(AppState { db }): State<AppState>,
    ValidJson(LoginParams { account, password }): ValidJson<LoginParams>,
) -> CommonResult<LoginResult> {
    tracing::info!("开始处理登录请求...");
    let user = DemoSysUser::find()
        .filter(demo_sys_user::Column::Account.eq(&account))
        .one(&db)
        .await?;

    match user {
        Some(user) => {
            // 验证密码
            if !verify_passwd_async(password, user.password).await? {
                return Err(ApiError::Biz(String::from("账号或密码错误")));
            }
            // 构建认证主体并签发 Token
            let principal = Principal {
                tenant_id: user.tenant_id,
                id: user.id,
                name: user.name,
            };
            let access_token = default_jwt().encode(principal)?;
            tracing::info!("登录成功，签发 Token");
            success(LoginResult { access_token })
        }
        None => {
            // 用户不存在时也执行一次 bcrypt 验证（用假密码），
            // 使响应时间与"用户存在但密码错误"的情况相近，防止时序攻击枚举有效账号。
            let _ = verify_passwd_async(
                "dummy_input".to_string(),
                "$2b$12$LJ3m4ys3GmzgMBx7cE2KZOm6QhYhW0rPqM5L0zR0RjLmVR0aFdrXK".to_string(),
            )
            .await;
            Err(ApiError::Biz(String::from("账号或密码错误")))
        }
    }
}

/// 获取当前登录用户信息处理器。
///
/// `GET /api/auth/user-info`
///
/// 从经过 JWT 中间件认证后的请求 Extensions 中提取
/// [`Principal`]，直接返回用户身份信息。
/// 在执行到此处理器之前，JWT 认证中间件已完成 Token 解码与验证。
///
/// ## 依赖
///
/// 此接口受 [`get_auth_layer()`] 保护，未携带有效 Bearer Token
/// 的请求将被中间件拦截并返回 401，不会到达此处理器。
#[debug_handler]
async fn user_info(Extension(principal): Extension<Principal>) -> CommonResult<Principal> {
    success(principal)
}
