//! `web-starter` — daoyi-cloud-axum 的 Web 服务完整示例。
//!
//! 本 crate 演示如何使用 daoyi-cloud-axum 的 lib crate 搭建一个完整的
//! HTTP 微服务，包括路由注册、JWT 认证、数据库 CRUD、静态资源嵌入等能力。
//!
//! ## 启动流程
//!
//! ```text
//! main() → app::run(api::create_router())
//!          ├── logger::init()            — 初始化结构化日志
//!          ├── id::init()                — 初始化雪花算法 ID 生成器
//!          ├── database::init()          — 建立数据库连接池（自适应 CPU 核心数）
//!          ├── config::get().server()    — 读取服务器配置
//!          └── server::start()           — 组装中间件链，启动 HTTP 服务
//!                                            ├── 路由解析
//!                                                ├── /api/* (JWT 认证)
//!                                                ├── /static/* (静态资源)
//!                                                └── /* (SPA fallback)
//!                                            └── Handler 执行
//!                                               → ApiResponse (JSON 响应)
//! ```
//!
//! ## 依赖库
//!
//! | Crate | 用途 |
//! |-------|------|
//! | `daoyi-axum-app` | 应用启动、中间件、JWT 认证、分页、校验 |
//! | `daoyi-axum-config` | 配置管理（YAML + 环境变量） |
//! | `daoyi-axum-support` | 基础设施（错误处理、响应格式、ID 生成等） |
//! | `daoyi-sea-orm-entity-demo` | 数据库实体模型（6 张表） |

use daoyi_axum_app::app;

pub mod api;
pub mod web;

/// 主入口函数。
///
/// 通过 [`app::run`] 一键启动服务，该函数内部会自动完成：
///
/// 1. `tracing` 日志订阅器初始化
/// 2. 雪花算法 ID 生成器初始化
/// 3. SeaORM 数据库连接池初始化
/// 4. 全局配置加载
/// 5. 中间件链组装
/// 6. TCP 端口绑定与 HTTP 服务启动
///
/// ## 运行方式
///
/// ```bash
/// # 在项目根目录运行
/// cargo run -p web-starter
///
/// # 指定端口
/// APP_SERVER_PORT=8080 cargo run -p web-starter
///
/// # 指定配置文件
/// cargo run -p web-starter -- -c resources/example-web-starter-dev.yaml
/// ```
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app::run(api::create_router()).await
}
