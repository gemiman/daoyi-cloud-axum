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

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/user-info", routing::get(user_info))
        .route_layer(get_auth_layer())
        .route("/login", routing::post(login))
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginParams {
    /// 账号，长度 1-16。
    #[validate(length(min = 1, max = 16, message = "账号长度必须在1-16之间"))]
    pub account: String,

    /// 密码，长度 6-16。
    #[validate(length(min = 6, max = 16, message = "密码长度必须在6-16之间"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResult {
    access_token: String,
}

#[debug_handler]
#[tracing::instrument(name = "login", skip_all, fields(account = %account))]
async fn login(
    State(AppState { db }): State<AppState>,
    // ConnectInfo(addr): ConnectInfo<SocketAddr>,
    ValidJson(LoginParams { account, password }): ValidJson<LoginParams>,
) -> CommonResult<LoginResult> {
    tracing::info!("开始处理登录请求...");
    let user = DemoSysUser::find()
        .filter(demo_sys_user::Column::Account.eq(&account))
        .one(&db)
        .await?;

    match user {
        Some(user) => {
            if !verify_passwd_async(password, user.password).await? {
                return Err(ApiError::Biz(String::from("账号或密码错误")));
            }
            let principal = Principal {
                tenant_id: user.tenant_id,
                id: user.id,
                name: user.name,
            };
            let access_token = default_jwt().encode(principal)?;
            tracing::info!("登录成功，生成 Token：{access_token}");
            success(LoginResult { access_token })
        }
        None => {
            // 用户不存在时也执行一次 bcrypt 验证（用假密码），
            // 使响应时间与"用户存在但密码错误"的情况相近，防止时序攻击枚举有效账号。
            verify_passwd_async(
                "dummy_input".to_string(),
                "$2b$12$LJ3m4ys3GmzgMBx7cE2KZOm6QhYhW0rPqM5L0zR0RjLmVR0aFdrXK".to_string(),
            )
            .await?;
            Err(ApiError::Biz(String::from("账号或密码错误")))
        }
    }
}

#[debug_handler]
async fn user_info(Extension(principal): Extension<Principal>) -> CommonResult<Principal> {
    success(principal)
}
