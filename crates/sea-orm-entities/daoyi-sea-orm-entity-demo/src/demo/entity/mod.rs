//! `SeaORM` Entity 模块。
//!
//! 由 `sea-orm-cli generate` 自动生成，包含 `demo` 数据库中所有表的
//! Entity Model 定义。每个子模块对应一张数据库表，包含：
//! - **Model** — 与数据库行对应的 Rust 结构体（含 `Serialize` / `Deserialize`）
//! - **Relation** — 表之间的关联关系定义（外键、join 等）
//! - **ActiveModelBehavior** — 插入/更新时的行为钩子
//!
//! ## 包含的表
//!
//! | 模块 | 表名 | 关键字段 | 说明 |
//! |------|------|----------|------|
//! | `demo_sys_user` | `demo_sys_user` | account, password, mobile_phone, birthday | 系统用户表 |
//! | `demo_category` | `demo_category` | name, parent_id (自引用层级) | 分类表 |
//! | `demo_contact`  | `demo_contact`  | name, sex, birthday, avatar | 联系人表 |
//! | `demo_course`   | `demo_course`   | student_id, name, score | 课程表 |
//! | `demo_grade`    | `demo_grade`    | student_id, name, teacher | 成绩表 |
//! | `demo_student`  | `demo_student`  | name, sex, birthday, description | 学生表 |
//!
//! ### 公共审计字段
//!
//! 所有表均包含以下标准字段（由 SeaORM CLI 生成时指定）：
//!
//! | 字段 | 类型 | 说明 |
//! |------|------|------|
//! | `creator` | `Option<String>` | 创建人 |
//! | `create_time` | `DateTime` | 创建时间 |
//! | `updater` | `Option<String>` | 更新人 |
//! | `update_time` | `DateTime` | 更新时间 |
//! | `deleted` | `bool` | 逻辑删除标记 |
//! | `tenant_id` | `i64` | 租户 ID（多租户隔离） |
//!
//! ## 重新生成
//!
//! 当数据库表结构发生变化时，使用以下命令重新生成：
//!
//! ```bash
//! sea-orm-cli generate entity \
//!   -u mysql://root:123456@127.0.0.1:3306/demo \
//!   --with-serde both \
//!   --model-extra-attributes 'serde(rename_all = "camelCase")' \
//!   --date-time-crate chrono \
//!   -o ./crates/sea-orm-entities/daoyi-sea-orm-entity-demo/src/demo/entity
//! ```
//!
//! > **注意**：生成后 `demo_sys_user` 的 `gender` 字段已手动改为自定义 `Gender`
//! > 枚举类型（定义于 `daoyi-axum-support::support::enumeration`），
//! > 重新生成后需手动调整该字段的类型和导入。

pub mod prelude;

pub mod demo_category;
pub mod demo_contact;
pub mod demo_course;
pub mod demo_grade;
pub mod demo_student;
pub mod demo_sys_user;
