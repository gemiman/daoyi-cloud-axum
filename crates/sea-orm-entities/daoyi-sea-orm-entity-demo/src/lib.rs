//! `daoyi-sea-orm-entity-demo` — 数据库 Entity 模型演示 crate。
//!
//! 包含由 `sea-orm-cli generate` 自动生成的 ORM 实体模型，
//! 对应 `demo` 数据库中的所有表。每个实体文件包含对应的
//! Model、Relation 枚举和 ActiveModelBehavior 实现。
//!
//! ## 表一览
//!
//! | 实体 | 表名 | 说明 |
//! |------|------|------|
//! | `demo_sys_user` | `demo_sys_user` | 系统用户表（含账号、密码、手机号等完整用户字段） |
//! | `demo_category` | `demo_category` | 分类表（支持父子层级，通过 `parent_id` 自引用） |
//! | `demo_contact`  | `demo_contact`  | 联系人表（姓名、性别、生日、描述、头像） |
//! | `demo_course`   | `demo_course`   | 课程表（关联 `student_id`，课程名、分数） |
//! | `demo_grade`    | `demo_grade`    | 成绩表（关联 `student_id`，科目名称、授课教师） |
//! | `demo_student`  | `demo_student`  | 学生表（姓名、性别、生日、个人描述） |
//!
//! ## 依赖关系
//!
//! ```text
//! daoyi-sea-orm-entity-demo
//! └── daoyi-axum-support  — 仅用于 Gender 枚举类型
//! ```
//!
//! ## 重新生成
//!
//! 当数据库表结构变化时，参考
//! [README](../../../README.md#生成-sea-orm-entity) 中的
//! `sea-orm-cli generate entity` 命令重新生成。

pub mod demo;
