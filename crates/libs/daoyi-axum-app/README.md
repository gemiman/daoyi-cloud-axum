# daoyi-axum-app

应用启动与核心功能 crate，负责组装中间件链、数据库连接、JWT 认证等核心功能。

通过 `app::run()` 提供一键启动 HTTP 服务的入口函数，依赖 `daoyi-axum-config` 和 `daoyi-axum-support` 两个底层 crate。

## 核心模块

| 模块                   | 功能                                         |
|----------------------|--------------------------------------------|
| `app::server`        | HTTP 服务器构建与启动（中间件链组装）                      |
| `app::database`      | 数据库连接池初始化（自适应 CPU 核心数）                     |
| `app::common`        | 通用数据结构（`PageParam`、`PageResult`）           |
| `app::auth::jwt`     | JWT 认证（HMAC-SHA256 编解码 + Bearer Token 中间件） |
| `app::validation`    | 自定义参数校验函数（分页范围、手机号正则等）                     |
| `app::sea_orm_utils` | SeaORM 扩展工具函数（占位）                          |

## 中间件链

请求按 middleware layer 顺序从外到内依次经过：

```
请求 → TimeoutLayer (120s)
     → DefaultBodyLimit (2 GiB)
     → TraceLayer (日志/追踪/耗时)
     → CorsLayer (跨域)
     → NormalizePathLayer (路径规范化)
     → Router → JWT Auth Middleware → Handler
```

## 使用示例

```rust
use axum::{Router, routing::get};
use daoyi_axum_app::app::{self, AppState};

// 定义路由
async fn health_check() -> &'static str { "OK" }

let router = Router::new()
.route("/health", get(health_check));

// 一键启动服务（自动完成日志、数据库、配置等初始化）
app::run(router).await?;
```

## `AppState`

全局应用状态，通过 Axum 的 `State` 提取器在处理器中共享：

```rust
use daoyi_axum_app::app::AppState;

async fn handler(state: State<AppState>) -> impl IntoResponse {
    let db = &state.db; // SeaORM 数据库连接池
    // ...
}
```

## JWT 认证

```rust
use daoyi_axum_app::app::auth::jwt::{JWT, Principal, JwtConfig};

// 创建 JWT 实例
let jwt_config = JwtConfig::new("secret", 60);
let jwt = JWT::new(jwt_config);

// 编码 Token
let token = jwt.encode(Principal::new("user_1")) ?;

// 解码 Token
let claims = jwt.decode( & token) ?;
assert_eq!(claims.sub, "user_1");

// 添加 Bearer Token 认证中间件
use daoyi_axum_app::app::auth::jwt::middleware::get_auth_layer;
let router = router.layer(get_auth_layer());
```

## 依赖

| 依赖                    | 用途               |
|-----------------------|------------------|
| `daoyi-axum-config`   | 配置管理             |
| `daoyi-axum-support`  | 基础设施工具           |
| `axum` / `tower-http` | HTTP 框架与中间件      |
| `sea-orm`             | 异步 ORM           |
| `jsonwebtoken`        | JWT 编码/解码        |
| `num_cpus`            | CPU 核心数检测（连接池大小） |
| `xid`                 | 全局唯一请求 ID        |
| `bytesize`            | 人类可读字节大小         |
| `regex`               | 正则表达式（校验）        |
