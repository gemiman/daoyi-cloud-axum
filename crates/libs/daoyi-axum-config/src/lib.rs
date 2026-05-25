//! `daoyi-axum-config` — 应用配置管理 crate。
//!
//! 提供 YAML 配置文件加载、环境变量覆盖和命令行参数支持。
//! 全局配置通过 [`LazyLock`] 实现线程安全的单例模式，
//! 在首次访问时自动初始化，后续调用直接返回缓存结果。
//!
//! ## 配置加载优先级（从高到低）
//!
//! 1. 命令行参数 `--config_file` / `-c` — 指定配置文件路径
//! 2. 环境变量 `APP_*` — 覆盖配置文件中的同名字段
//! 3. YAML 配置文件 — 从 `resources/{APP_NAME}-{APP_PROFILE}.yaml` 加载

pub mod config;
