//! 用户 API 模块。
//!
//! 提供用户资源的完整 CRUD 接口，包括分页查询、条件过滤、
//! 创建、更新、删除等操作。所有接口返回统一的 [`CommonResult`] 响应。
//!
//! ## 端点
//!
//! | 方法 | 路径 | 说明 |
//! |------|------|------|
//! | GET | `/api/users` | 条件查询用户列表（演示用） |
//! | GET | `/api/users/page` | 分页查询用户（支持 keyword 搜索） |
//! | POST | `/api/users` | 创建用户 |
//! | PUT | `/api/users/{id}` | 更新用户 |
//! | DELETE | `/api/users/{id}` | 删除用户 |

use crate::app::AppState;
use crate::common::{PageParam, PageResult};
use crate::validation::validate_mobile_phone;
use axum::extract::State;
use axum::{Router, debug_handler, routing};
use daoyi_axum_support::support::enumeration::Gender;
use daoyi_axum_support::support::error::ApiError;
use daoyi_axum_support::support::passwd::hash_passwd;
use daoyi_axum_support::support::path::Path;
use daoyi_axum_support::support::response::{CommonResult, success};
use daoyi_axum_support::support::valid::{ValidJson, ValidQuery};
use daoyi_sea_orm_entity_demo::demo::entity::demo_sys_user;
use daoyi_sea_orm_entity_demo::demo::entity::demo_sys_user::ActiveModel;
use daoyi_sea_orm_entity_demo::demo::entity::prelude::*;
use sea_orm::prelude::*;
use sea_orm::{ActiveValue, Condition, IntoActiveModel, QueryTrait};
use serde::Deserialize;
use validator::Validate;

/// 创建用户相关路由。
/// 创建用户相关路由。
///
/// 注册以下端点：
///
/// | 方法 | 路径 | 处理器 | 说明 |
/// |------|------|--------|------|
/// | GET | `/api/users` | [`query_users`] | 条件查询用户列表（演示用） |
/// | GET | `/api/users/page` | [`find_page`] | 分页查询用户 |
/// | POST | `/api/users` | [`create`] | 创建用户 |
/// | PUT | `/api/users/{id}` | [`update`] | 更新用户 |
/// | DELETE | `/api/users/{id}` | [`delete`] | 删除用户 |
pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", routing::get(query_users))
        .route("/page", routing::get(find_page))
        .route("/", routing::post(create))
        .route("/{id}", routing::put(update))
        .route("/{id}", routing::delete(delete))
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
        // .filter(demo_sys_user::Column::Gender.eq("female"))
        // // ALL：所有条件必须同时满足
        // .filter(
        //     Condition::all()
        //         .add(demo_sys_user::Column::Name.starts_with("李"))
        //         .add(demo_sys_user::Column::Name.ends_with("四")),
        // )
        // // ANY：任一条件满足即可（此处仅为语法演示）
        // .filter(
        //     Condition::any()
        //         .add(demo_sys_user::Column::Name.starts_with("李"))
        //         .add(demo_sys_user::Column::Name.ends_with("四")),
        // )
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
    pub gender: Gender,

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

/// 创建用户处理器。
///
/// `POST /api/users`
///
/// 接收 [`UserParams`] JSON 请求体，对密码进行 bcrypt 哈希后插入数据库。
/// 返回新创建用户的 ID。
#[debug_handler]
async fn create(
    State(AppState { db }): State<AppState>,
    ValidJson(params): ValidJson<UserParams>,
) -> CommonResult<i64> {
    let password = hash_passwd(&params.password)?;
    let mut active_model = params.into_active_model();
    active_model.password = ActiveValue::Set(password);
    let result = active_model.insert(&db).await?;
    success(result.id)
}

/// 更新用户处理器。
///
/// `PUT /api/users/{id}`
///
/// 先查询用户是否存在（不存在返回业务错误），若传入的密码为空则保留原密码，
/// 否则对新密码进行 bcrypt 哈希后更新。返回 `true` 表示更新成功。
#[debug_handler]
async fn update(
    State(AppState { db }): State<AppState>,
    Path(id): Path<i64>,
    ValidJson(params): ValidJson<UserParams>,
) -> CommonResult<bool> {
    let existed_user = DemoSysUser::find_by_id(id)
        .one(&db)
        .await?
        .ok_or_else(|| ApiError::Biz(String::from("用户不存在")))?;
    let password = if params.password.is_empty() {
        existed_user.password
    } else {
        hash_passwd(&params.password)?
    };
    let mut active_model = params.into_active_model();
    active_model.id = ActiveValue::Unchanged(id);
    active_model.password = ActiveValue::Set(password);
    let _result = active_model.update(&db).await?;
    success(true)
}

/// 删除用户处理器。
///
/// `DELETE /api/users/{id}`
///
/// 按主键硬删除用户记录，返回受影响的行数。
#[debug_handler]
async fn delete(State(AppState { db }): State<AppState>, Path(id): Path<i64>) -> CommonResult<u64> {
    let result = DemoSysUser::delete_by_id(id).exec(&db).await?;
    success(result.rows_affected)
}
