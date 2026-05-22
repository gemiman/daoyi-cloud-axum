//! `SeaORM` Entity 模块。
//!
//! 由 `sea-orm-cli generate` 自动生成，包含 `demo` 数据库中所有表的
//! Entity Model 定义。每个子模块对应一张数据库表。
//!
//! ## 包含的表
//!
//! | 模块 | 表名 | 说明 |
//! |------|------|------|
//! | `demo_sys_user` | `demo_sys_user` | 系统用户表 |
//! | `demo_category` | `demo_category` | 分类表（层级结构） |
//! | `demo_contact`  | `demo_contact`  | 联系人表 |
//! | `demo_course`   | `demo_course`   | 课程表 |
//! | `demo_grade`    | `demo_grade`    | 成绩表 |
//! | `demo_student`  | `demo_student`  | 学生表 |
//!
//! ## 重新生成
//!
//! 当数据库表结构发生变化时，参考 README 中的 `sea-orm-cli generate` 命令重新生成。

pub mod prelude;

pub mod demo_category;
pub mod demo_contact;
pub mod demo_course;
pub mod demo_grade;
pub mod demo_student;
pub mod demo_sys_user;
