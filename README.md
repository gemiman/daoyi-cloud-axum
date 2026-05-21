# daoyi-cloud-axum

Rust cloud-native microservice scaffold based on Axum + SeaORM + MySQL.

一个基于 **Axum** 框架的 Rust 云原生微服务脚手架，集成 **SeaORM** ORM 与 **MySQL** 数据库，提供开箱即用的项目模板。

## 特性

- **Axum 0.8** — 高性能异步 Web 框架
- **SeaORM** — 异步 ORM，支持 MySQL
- **Tokio** — 异步运行时，全特性支持
- **Tracing** — 结构化日志，支持本地时间与时区偏移
- **Config** — 灵活的 YAML 配置加载，支持环境变量覆盖
- **Cargo Workspace** — 模块化管理，示例与主项目独立

## 快速开始

### 环境要求

- Rust **1.94.0**+
- Cargo

### 运行示例

```bash
# 进入 web-starter 示例目录运行
cd example/web-starter
APP_NAME=example-web-starter cargo run

# 或指定自定义配置文件
cargo run -- -c resources/example-web-starter-dev.yaml
```

服务默认监听 `0.0.0.0:3000`（可在 YAML 配置中修改端口）。

## 配置说明

### 配置文件

配置文件存放在 `resources/` 目录下，命名规则为 `{APP_NAME}-{APP_PROFILE}.yaml`。

| 环境变量          | 默认值           | 说明     |
|---------------|---------------|--------|
| `APP_NAME`    | `application` | 应用名称   |
| `APP_PROFILE` | `dev`         | 运行环境标识 |

### 命令行参数

| 参数                     | 说明            |
|------------------------|---------------|
| `-c <path>`            | 指定配置文件路径      |
| `--config_file <path>` | 指定配置文件路径（长格式） |
| `-c=<path>`            | 等号形式          |
| `--config_file=<path>` | 等号长格式         |

### 环境变量覆盖

所有以 `APP_` 为前缀的环境变量会自动映射到配置项。例如：

```bash
APP_SERVER_PORT=8080 cargo run
```

这会覆盖 YAML 中 `server.port` 的值。

## 项目结构

```
daoyi-cloud-axum/
├── Cargo.toml                          # 根 crate 与 workspace 配置
├── README.md
├── LICENSE
├── src/
│   └── main.rs                         # 根 crate 入口（占位）
├── example/
│   └── web-starter/                    # Web 服务示例
│       ├── Cargo.toml
│       ├── README.md
│       └── src/
│           ├── main.rs                 # 服务入口
│           ├── logger.rs               # 日志初始化
│           └── config/
│               ├── mod.rs              # 配置加载逻辑
│               └── server.rs           # 服务器配置结构体
└── resources/
    └── example-web-starter-dev.yaml    # 示例开发环境配置
```

## License

MIT License. 详见 [LICENSE](./LICENSE)。

Copyright (c) 2026 兰陵王
