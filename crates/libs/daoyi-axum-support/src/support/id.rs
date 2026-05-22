//! 分布式 ID 生成器模块。
//!
//! 基于 [`idgenerator`] crate 提供雪花算法（Snowflake）ID 生成能力。
//! 在应用启动时完成全局初始化，后续通过 [`next_id`] / [`next_id_str`]
//! 获取全局唯一的分布式 ID。
//!
//! ## 配置说明
//!
//! - **基准时间**：2026-05-01 00:00:00 UTC，以毫秒时间戳传入
//! - **worker_id 位长**：4 位，最多支持 16 个 worker
//! - **当前 worker_id**：1

use idgenerator::{IdGeneratorOptions, IdInstance};
use sea_orm::prelude::Date;

/// 初始化全局 ID 生成器。
///
/// 应在应用启动时尽早调用（在 `app::run` 中紧随日志初始化之后）。
/// 配置基准时间为 2026-05-01，work_id 位长 4。
///
/// ## 错误
///
/// - `OptionError::InvalidBaseTime`：基准时间不在合法范围内（1990-01-01 ~ 当前时间）
/// - 重复初始化不会报错
pub fn init() -> anyhow::Result<()> {
    let options = IdGeneratorOptions::new()
        .base_time(
            Date::from_ymd_opt(2026, 5, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc()
                .timestamp_millis(),
        )
        .worker_id(1)
        .worker_id_bit_len(4);
    Ok(IdInstance::init(options)?)
}

/// 生成下一个全局唯一 ID（`i64` 类型）。
///
/// ## 注意事项
///
/// 如果 ID 超过 `i64` 范围将会 panic，正常业务场景不会触发。
pub fn next_id() -> i64 {
    IdInstance::next_id()
}

/// 生成下一个全局唯一 ID 并转为字符串。
///
/// 等价于 `next_id().to_string()`，适用于需要字符串 ID 的场景。
pub fn next_id_str() -> String {
    next_id().to_string()
}
