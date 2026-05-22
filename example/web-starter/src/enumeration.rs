//! 枚举类型定义模块。
//!
//! 提供项目中使用的各类枚举，包括 [`Gender`] 等。
//! 所有枚举均实现了 `Serialize` / `Deserialize`（用于 JSON 序列化）
//! 以及 `DeriveActiveEnum`（用于 SeaORM 数据库映射）。

use sea_orm::prelude::*;
use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};

/// 性别枚举。
///
/// 在数据库中存储为 `snake_case` 的字符串（`male`、`female`、`unknown`），
/// JSON 序列化同理。实现了 SeaORM 的 `DeriveActiveEnum`，可直接用于 Model 字段。
///
/// ## 变体
///
/// | 变体 | 数据库值 | 含义 |
/// |------|---------|------|
/// | `Male` | `"male"` | 男性 |
/// | `Female` | `"female"` | 女性 |
/// | `Unknown` | `"unknown"` | 未知 |
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[serde(rename_all = "snake_case")]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
pub enum Gender {
    Male,
    Female,
    Unknown,
}
