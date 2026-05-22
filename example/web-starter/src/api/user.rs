//! 用户 API 模块。
//!
//! 提供用户相关的查询接口，包括分页查询和条件过滤。
//!
//! ## 端点
//!
//! | 方法 | 路径 | 说明 |
//! |------|------|------|
//! | GET | `/api/users` | 条件查询用户列表（演示用） |
//! | GET | `/api/users/page` | 分页查询用户（支持 keyword 搜索） |

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

/// 创建用户相关路由。
pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", routing::get(query_users))
        .route("/page", routing::get(find_page))
}

/// 用户分页查询参数。
///
/// 支持按 `keyword` 模糊搜索用户名或账号，继承通用的 [`PageParam`] 分页参数。
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserQueryParams {
    /// 搜索关键词，同时匹配 `name` 和 `account` 字段。
    keyword: Option<String>,

    /// 嵌套的分页参数，通过 `#[serde(flatten)]` 与 keyword 平级展开。
    #[validate(nested)]
    #[serde(flatten)]
    pagination: PageParam,
}

/// 用户分页查询处理器。
///
/// `GET /api/users/page?keyword=xxx&pageNo=1&pageSize=10`
///
/// 当提供 `keyword` 时，对 `name` 和 `account` 进行 `LIKE` 模糊匹配。
/// 结果按 `id` 降序排列。
#[debug_handler]
async fn find_page(
    State(AppState { db }): State<AppState>,
    ValidQuery(UserQueryParams {
        keyword,
        pagination,
    }): ValidQuery<UserQueryParams>,
) -> CommonResult<PageResult<demo_sys_user::Model>> {
    let paginator = DemoSysUser::find()
        // apply_if：仅当 keyword 非空时才附加条件
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

/// 用户条件查询处理器（演示用）。
///
/// `GET /api/users`
///
/// 固定条件：`gender = "female"` 且 `name` 以"李"开头并以"四"结尾。
/// 此端点主要用于演示 SeaORM 的 `Condition::all` / `Condition::any` 用法。
#[tracing::instrument(name = "query_users", skip_all, fields(target = "只是为了演示。。。"))]
#[debug_handler]
async fn query_users(
    State(AppState { db }): State<AppState>,
) -> CommonResult<Vec<demo_sys_user::Model>> {
    tracing::info!("开始处理业务……");
    let users = DemoSysUser::find()
        .filter(demo_sys_user::Column::Gender.eq("female"))
        // ALL：所有条件必须同时满足
        .filter(
            Condition::all()
                .add(demo_sys_user::Column::Name.starts_with("李"))
                .add(demo_sys_user::Column::Name.ends_with("四")),
        )
        // ANY：任一条件满足即可（此处仅为语法演示）
        .filter(
            Condition::any()
                .add(demo_sys_user::Column::Name.starts_with("李"))
                .add(demo_sys_user::Column::Name.ends_with("四")),
        )
        .all(&db)
        .await?;
    success(users)
}

/// 用户创建参数。
///
/// 包含所有允许从前端传入的用户字段，每个字段带有 `validator` 校验注解。
/// 通过 `#[derive(DeriveIntoActiveModel)]` 宏可自动转换为 SeaORM ActiveModel
/// 用于插入/更新操作。
#[derive(Debug, Deserialize, Validate, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct UserParams {
    /// 姓名，长度 1-16。
    #[validate(length(min = 1, max = 16, message = "姓名长度必须在1-16之间"))]
    pub name: String,

    /// 性别。
    pub gender: String,

    /// 账号，长度 1-16。
    #[validate(length(min = 1, max = 16, message = "账号长度必须在1-16之间"))]
    pub account: String,

    /// 密码，长度 6-16。
    #[validate(length(min = 6, max = 16, message = "密码长度必须在6-16之间"))]
    pub password: String,

    /// 手机号，需通过 [`validate_mobile_phone`] 校验。
    #[validate(custom(function = "validate_mobile_phone"))]
    pub mobile_phone: String,

    /// 生日。
    pub birthday: Date,

    /// 是否启用，默认 `false`。
    #[serde(default)]
    pub enabled: bool,
}
