pub mod server;

use anyhow::Context;
use config::{Config, FileFormat};
use serde::Deserialize;
pub use server::ServerConfig;
use std::sync::LazyLock;

static CONFIG: LazyLock<AppConfig> =
    LazyLock::new(|| AppConfig::load().expect("Failed to load config"));

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
}

impl AppConfig {
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

pub fn get() -> &'static AppConfig {
    &CONFIG
}
