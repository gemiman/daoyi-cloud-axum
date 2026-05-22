//! SeaORM Entity 预导入模块。
//!
//! 将常用的 Entity 类型以别名形式导出，方便在其他模块中通过
//! `use crate::demo::entity::prelude::*;` 一次性导入。
//!
//! 当前已导出的类型：
//! - [`DemoSysUser`] — 系统用户表 Entity

pub use super::demo_sys_user::Entity as DemoSysUser;
