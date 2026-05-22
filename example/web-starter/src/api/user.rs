use crate::app::AppState;
use crate::common::{PageParam, PageResult};
use crate::demo::entity::demo_sys_user;
use crate::demo::entity::demo_sys_user::ActiveModel;
use crate::demo::entity::prelude::*;
use crate::response::{CommonResult, success};
use crate::valid::ValidQuery;
use crate::validation::validate_mobile_phone;
use axum::extract::State;
use axum::{Router, debug_handler, routing};
use sea_orm::prelude::*;
use sea_orm::{Condition, QueryTrait};
use serde::Deserialize;
use validator::Validate;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", routing::get(query_users))
        .route("/page", routing::get(find_page))
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserQueryParams {
    keyword: Option<String>,
    #[validate(nested)]
    #[serde(flatten)]
    pagination: PageParam,
}

#[debug_handler]
async fn find_page(
    State(AppState { db }): State<AppState>,
    ValidQuery(UserQueryParams {
        keyword,
        pagination,
    }): ValidQuery<UserQueryParams>,
) -> CommonResult<PageResult<demo_sys_user::Model>> {
    let paginator = DemoSysUser::find()
        .apply_if(keyword.as_ref(), |query, var| {
            query.filter(
                Condition::any()
                    .add(demo_sys_user::Column::Name.contains(var))
                    .add(demo_sys_user::Column::Account.contains(var)),
            )
        })
        .order_by_id_desc()
        .paginate(&db, pagination.page_size);
    let total = paginator.num_items().await?;
    let users = paginator.fetch_page(pagination.page_no - 1).await?;
    let page_result = PageResult::from_page_param(pagination, total, users);
    success(page_result)
}

#[tracing::instrument(name = "query_users", skip_all, fields(target = "只是为了演示。。。"))]
#[debug_handler]
async fn query_users(
    State(AppState { db }): State<AppState>,
) -> CommonResult<Vec<demo_sys_user::Model>> {
    tracing::info!("开始处理业务……");
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
        .await?;
    success(users)
}

#[derive(Debug, Deserialize, Validate, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct UserParams {
    #[validate(length(min = 1, max = 16, message = "姓名长度必须在1-16之间"))]
    pub name: String,
    pub gender: String,
    #[validate(length(min = 1, max = 16, message = "账号长度必须在1-16之间"))]
    pub account: String,
    #[validate(length(min = 6, max = 16, message = "密码长度必须在6-16之间"))]
    pub password: String,
    #[validate(custom(function = "validate_mobile_phone"))]
    pub mobile_phone: String,
    pub birthday: Date,
    #[serde(default)]
    pub enabled: bool,
}
