//! SeaORM Entity 预导入模块。
//!
//! 将常用的 Entity 类型以别名形式导出，方便在其他模块中通过
//! `use crate::demo::entity::prelude::*;` 一次性导入。
//!
//! 使用类型别名而非直接 re-export 的好处：
//! - 避免 `Entity` 名称冲突（多张表都有 `Entity` 类型）
//! - 提供更语义化的名称，代码可读性更好
//! - IDE 自动补全仅显示已导出类型，减少噪音
//!
//! ## 当前已导出
//!
//! | 别名 | 类型 | 说明 |
//! |------|------|------|
//! | [`DemoSysUser`] | `demo_sys_user::Entity` | 系统用户表 Entity |
//!
//! > **提示**：如需使用其他表的 Entity，按相同格式在此文件中添加即可。

pub use super::demo_sys_user::Entity as DemoSysUser;
