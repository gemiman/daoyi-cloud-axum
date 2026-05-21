//! 应用配置模块。
//!
//! 负责加载、解析并提供全局应用配置。配置文件为 YAML 格式，
//! 支持通过命令行参数、环境变量等多种方式进行覆盖。
//!
//! ## 配置加载优先级（从高到低）
//!
//! 1. 命令行参数 `--config_file` / `-c` — 指定配置文件路径
//! 2. 环境变量 `APP_*` — 覆盖配置文件中的同名字段
//! 3. YAML 配置文件 — 从 `resources/{APP_NAME}-{APP_PROFILE}.yaml` 加载
//!
//! ## 使用方式
//!
//! ```rust,ignore
//! let port = config::get().server.port();
//! ```

pub mod server;

use anyhow::Context;
use config::{Config, FileFormat};
use serde::Deserialize;
pub use server::ServerConfig;
use std::sync::LazyLock;

/// 全局配置单例。
///
/// 使用 [`LazyLock`] 延迟初始化，在首次访问时自动加载配置文件。
/// 加载失败时会直接 panic，因为配置缺失属于不可恢复的错误。
static CONFIG: LazyLock<AppConfig> =
    LazyLock::new(|| AppConfig::load().expect("Failed to load config"));

/// 应用顶层配置结构体。
///
/// 对应 YAML 配置文件的根节点，包含 `server` 等子配置。
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    /// 服务器相关配置。
    pub server: ServerConfig,
}

impl AppConfig {
    /// 从 YAML 文件和环境变量中加载并解析配置。
    ///
    /// ## 配置文件路径确定逻辑
    ///
    /// 1. 检查命令行参数 `--config_file=value` 或 `-c=value`（等号形式）
    /// 2. 检查命令行参数 `--config_file value` 或 `-c value`（空格形式）
    /// 3. 若均未指定，则默认使用 `resources/{APP_NAME}-{APP_PROFILE}.yaml`
    ///
    /// 其中 `APP_NAME` 默认值为 `"application"`，`APP_PROFILE` 默认值为 `"dev"`。
    ///
    /// ## 环境变量覆盖
    ///
    /// 所有以 `APP_` 为前缀的环境变量会被自动映射到配置树的对应路径。
    /// 例如 `APP_SERVER_PORT=8080` 会覆盖 `server.port`。
    ///
    /// ## 错误处理
    ///
    /// 返回 `anyhow::Result`，失败可能由以下原因引起：
    /// - 配置文件不存在或无法读取
    /// - YAML 格式错误
    /// - 配置结构与 `AppConfig` 不匹配
    pub fn load() -> anyhow::Result<Self> {
        let app_name = std::env::var("APP_NAME")
            .ok()
            .unwrap_or_else(|| "application".to_string());
        let active_profile = std::env::var("APP_PROFILE")
            .ok()
            .unwrap_or_else(|| "dev".to_string());
        let config_file_name = std::env::args()
            // 先处理 --config_file=value 或 -c=value 形式
            .find_map(|arg| {
                arg.strip_prefix("--config_file=")
                    .or_else(|| arg.strip_prefix("-c="))
                    .map(|v| v.to_string())
            })
            // 再处理 --config_file value 或 -c value 形式
            .or_else(|| {
                let args: Vec<_> = std::env::args().collect();
                args.windows(2)
                    .find(|w| w[0] == "--config_file" || w[0] == "-c")
                    .and_then(|w| w.get(1).cloned())
            })
            .unwrap_or_else(|| format!("resources/{}-{}", app_name, active_profile));
        Config::builder()
            .add_source(
                config::File::with_name(&config_file_name)
                    .format(FileFormat::Yaml)
                    .required(true),
            )
            .add_source(
                config::Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(","),
            )
            .build()
            .with_context(|| anyhow::anyhow!("Failed to load config"))?
            .try_deserialize()
            .with_context(|| anyhow::anyhow!("Failed to deserialize config"))
    }
}

/// 获取全局配置的不可变引用。
///
/// 首次调用会触发配置加载，后续调用直接返回缓存结果。
/// 由于使用 [`LazyLock`] 实现，该函数是线程安全的。
pub fn get() -> &'static AppConfig {
    &CONFIG
}
