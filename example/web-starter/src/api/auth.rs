use axum::extract::State;
use axum::{Extension, Router, debug_handler, routing};
use daoyi_axum_app::app::AppState;
use daoyi_axum_app::app::auth::jwt::middleware::get_auth_layer;
use daoyi_axum_app::app::auth::jwt::{Principal, default_jwt};
use daoyi_axum_support::support::error::ApiError;
use daoyi_axum_support::support::passwd::{hash_passwd, verify_passwd};
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
#[tracing::instrument(name = "login", skip_all, fields(account = %account, password = %password))]
async fn login(
    State(AppState { db }): State<AppState>,
    // ConnectInfo(addr): ConnectInfo<SocketAddr>,
    ValidJson(LoginParams { account, password }): ValidJson<LoginParams>,
) -> CommonResult<LoginResult> {
    tracing::info!("开始处理登录请求...");
    let hashed_password = hash_passwd("1")?;
    let user = DemoSysUser::find()
        .filter(demo_sys_user::Column::Account.eq(&account))
        .one(&db)
        .await?
        .ok_or_else(|| {
            let _x = verify_passwd(&password, &hashed_password);
            ApiError::Biz(String::from("账号或密码错误"))
        })?;
    if !verify_passwd(&password, &user.password)? {
        return Err(ApiError::Biz(String::from("账号或密码错误")));
    }
    let principal = Principal {
        id: user.id,
        name: user.name,
    };
    let access_token = default_jwt().encode(principal)?;
    tracing::info!("登录成功，生成 Token：{access_token}");
    success(LoginResult { access_token })
}

#[debug_handler]
async fn user_info(Extension(principal): Extension<Principal>) -> CommonResult<Principal> {
    success(principal)
}
