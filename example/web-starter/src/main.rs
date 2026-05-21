//! `web-starter` — daoyi-cloud-axum 的 Web 服务启动示例。
//!
//! 本 crate 演示如何使用 `axum` 搭建一个最小化的 HTTP 服务器，
//! 包括路由注册、配置加载、日志初始化等基础功能。

pub mod app;
pub mod config;
pub mod database;
pub mod demo;
pub mod logger;
pub mod sea_orm_utils;
pub mod server;

use crate::app::AppState;
use crate::demo::entity::demo_sys_user;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Router, debug_handler, routing};
use demo::entity::prelude::*;
use sea_orm::Condition;
use sea_orm::prelude::*;

/// 主入口函数。
///
/// 依次完成以下步骤：
/// 1. 初始化 `tracing` 日志订阅器
/// 2. 加载应用配置
/// 3. 注册路由
/// 4. 绑定端口并启动 HTTP 服务
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let router = Router::new()
        .route("/", routing::get(index))
        .route("/users", routing::get(query_users));

    app::run(router).await
}

/// 根路径 `/` 的 GET 处理器。
///
/// 返回一段静态欢迎文本。使用 `#[debug_handler]` 宏以便在
/// 编译期获得更好的类型错误提示。
#[debug_handler]
async fn index() -> &'static str {
    "Hello DaoYi Cloud Axum !"
}

#[debug_handler]
async fn query_users(State(AppState { db }): State<AppState>) -> impl IntoResponse {
    let users = DemoSysUser::find()
        .filter(demo_sys_user::Column::Gender.eq("female"))
        .filter(
            Condition::all()
                .add(demo_sys_user::Column::Name.starts_with("李"))
                .add(demo_sys_user::Column::Name.ends_with("四")),
        )
        .filter(
            Condition::any()
                .add(demo_sys_user::Column::Name.starts_with("李"))
                .add(demo_sys_user::Column::Name.ends_with("四")),
        )
        .all(&db)
        .await
        .unwrap();
    axum::Json(users)
}
