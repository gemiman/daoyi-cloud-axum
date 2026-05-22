//! 日志模块。
//!
//! 基于 [`tracing`] + [`tracing-subscriber`] 提供结构化日志初始化。
//!
//! ## 特性
//!
//! - 使用 `EnvFilter` 支持通过 `RUST_LOG` 环境变量动态控制日志级别，默认 `info`
//! - 使用 `ChronoLocal` 以本地时间记录，并携带 ISO 8601 时区偏移 (`+08:00`)
//! - 在每行日志中附加文件路径和行号，便于定位问题
//! - 记录线程 ID 和线程名称，支持多线程场景排查

use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// 初始化全局日志订阅器。
///
/// 该方法应在 `main` 函数中尽早调用。日志输出格式包含：
///
/// - 时间戳（微秒精度，本地时间 + 时区偏移）
/// - 日志级别
/// - 线程 ID 与线程名称
/// - 源文件名与行号
///
/// ## `%:z` 时区格式说明
///
/// | 格式    | 示例          | 说明                             |
/// |---------|---------------|----------------------------------|
/// | `%z`    | `+0800`       | 无冒号，标准 strftime 格式       |
/// | `%:z`   | `+08:00`      | 带冒号，ISO 8601 扩展格式（推荐）|
/// | `%::z`  | `+08:00:00`   | 带秒精度（通常不需要）           |
/// | `%#z`   | `+08`         | 仅小时（部分平台支持）           |
///
/// 推荐使用 `%:z`，符合 ISO 8601 扩展格式，也是最常见的写法。
/// 使用带时区偏移的本地时间后，既能知道是本地时间，也能知道相对于 UTC 的偏移量，完全没有歧义。
pub fn init() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(
            tracing_subscriber::fmt::layer()
                .with_timer(ChronoLocal::new(String::from("%Y-%m-%dT%H:%M:%S%.6f%:z")))
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_target(false),
        )
        .init();
}
