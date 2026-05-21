use crate::app::AppState;
use crate::demo::entity::demo_sys_user;
use crate::demo::entity::prelude::*;
use crate::response::{CommonResult, success};
use axum::extract::State;
use axum::{Router, debug_handler, routing};
use sea_orm::Condition;
use sea_orm::prelude::*;

pub fn create_router() -> Router<AppState> {
    Router::new().route("/", routing::get(query_users))
}

#[debug_handler]
async fn query_users(
    State(AppState { db }): State<AppState>,
) -> CommonResult<Vec<demo_sys_user::Model>> {
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
    success(users)
}
