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
//! | `demo_sys_user` | `demo_sys_user` | 系统用户表 |
//! | `demo_category` | `demo_category` | 分类表（层级结构） |
//! | `demo_contact`  | `demo_contact`  | 联系人表 |
//! | `demo_course`   | `demo_course`   | 课程表 |
//! | `demo_grade`    | `demo_grade`    | 成绩表 |
//! | `demo_student`  | `demo_student`  | 学生表 |

pub mod demo;
