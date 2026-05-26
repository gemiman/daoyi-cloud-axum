# web-starter

daoyi-cloud-axum 的 Axum Web 服务完整示例。

完整演示 **Axum + SeaORM + Tracing** 微服务技术栈的最佳实践。本项目本身极其精简——
所有底层能力均由 Cargo Workspace 中的 lib crate 提供，示例项目仅负责**路由定义**与**业务逻辑**。

## 项目结构

```
example/web-starter/
├── Cargo.toml          # 依赖声明（所有版本由 workspace 统一管理）
└── src/
    ├── main.rs         # 入口：app::run(api::create_router())
    ├── api/
    │   ├── mod.rs      # 路由组装 + JWT 认证中间件 + 静态资源 + 404/405 fallback
    │   ├── user.rs     # 用户 CRUD 处理器（分页查询、条件查询、创建、更新、删除）
    │   └── auth.rs     # 认证处理器（登录、获取当前用户信息）
    └── web/
        └── mod.rs      # 前端 SPA 静态资源嵌入（rust-embed）
```

## 功能

以下功能均由底层 lib crate 提供，`web-starter` 通过调用库 API 直接使用：

| 功能                                                  | 提供方                                       |
|-----------------------------------------------------|-------------------------------------------|
| HTTP 服务器（中间件链：超时、Body 限制、日志、CORS、路径规范化）             | `daoyi-axum-app::app::server`             |
| YAML 配置加载（环境变量覆盖、命令行参数）                             | `daoyi-axum-config::config`               |
| 数据库 ORM（SeaORM 连接池，自适应 CPU 核心数）                     | `daoyi-axum-app::app::database`           |
| 参数校验（validator + axum-valid，支持 Query / Path / JSON） | `daoyi-axum-support::support::valid`      |
| 统一错误处理（自动映射 HTTP 状态码 + JSON 响应）                     | `daoyi-axum-support::support::error`      |
| 统一响应格式（code / msg / data）                           | `daoyi-axum-support::support::response`   |
| 分布式 ID 生成（雪花算法）                                     | `daoyi-axum-support::support::id`         |
| 密码哈希与验证（bcrypt，支持同步/异步）                             | `daoyi-axum-support::support::passwd`     |
| JWT 认证（HMAC-SHA256 编解码 + Bearer Token 中间件）          | `daoyi-axum-app::app::auth::jwt`          |
| 分页支持（通用分页参数 + 分页响应，含默认值自动读取）                        | `daoyi-axum-app::app::common`             |
| 自定义校验函数（手机号正则、分页范围等）                                | `daoyi-axum-app::app::validation`         |
| 结构化日志（tracing，请求 ID、IP、耗时、本地时间 + 时区）                | `daoyi-axum-support::support::logger`     |
| SeaORM Entity（6 张 demo 表）                           | `daoyi-sea-orm-entity-demo::demo::entity` |
| 静态资源嵌入（SPA 前端，支持 gzip/br 压缩）                        | `rust-embed`                              |

## 运行

```bash
# 在项目根目录运行
cargo run -p web-starter

# 指定应用名称与配置文件前缀
APP_NAME=example-web-starter cargo run -p web-starter

# 指定端口（通过环境变量覆盖）
APP_SERVER_PORT=8080 cargo run -p web-starter

# 指定配置文件
cargo run -p web-starter -- -c resources/example-web-starter-dev.yaml
```

## API 端点

### 用户 API

| 方法     | 路径                                              | 说明                                 | 认证 |
|--------|-------------------------------------------------|------------------------------------|----|
| GET    | `/api/users`                                    | 条件查询用户列表（固定条件演示）                   | 需要 |
| GET    | `/api/users/page?keyword=&pageNo=1&pageSize=20` | 分页查询（支持 keyword 模糊搜索 name/account） | 需要 |
| POST   | `/api/users`                                    | 创建用户（密码自动 bcrypt 哈希）               | 需要 |
| PUT    | `/api/users/{id}`                               | 更新用户（空密码则保留原密码）                    | 需要 |
| DELETE | `/api/users/{id}`                               | 删除用户                               | 需要 |

### 认证 API

| 方法   | 路径                    | 说明                     | 认证 |
|------|-----------------------|------------------------|----|
| POST | `/api/auth/login`     | 用户登录（返回 JWT Token）     | 无需 |
| GET  | `/api/auth/user-info` | 获取当前登录用户信息（从 Token 解析） | 需要 |

### 响应示例

分页查询响应：

```json
{
  "code": 0,
  "msg": "",
  "data": {
    "pageNo": 1,
    "pageSize": 20,
    "totalPage": 3,
    "total": 42,
    "list": [
      {
        "id": 1,
        "name": "张三",
        "gender": "male",
        "account": "zhangsan",
        "mobilePhone": "13800138000",
        ...
      }
    ]
  }
}
```

登录响应：

```json
{
  "code": 0,
  "msg": "",
  "data": {
    "accessToken": "eyJhbGciOiJIUzI1NiIs..."
  }
}
```

## 配置

默认配置文件路径规则：`resources/{APP_NAME}-{APP_PROFILE}.yaml`

示例配置文件 `resources/example-web-starter-dev.yaml`：

```yaml
server:
  port: 3001

database:
  host: 127.0.0.1
  port: 3306
  user: root
  password: 123456
  database: demo

sys:
  page_size_min: 10       # 分页最小条数
  page_size_max: 100      # 分页最大条数
  page_size_default: 20   # 分页默认条数
  page_no_default: 1      # 默认页码
```

## 模块说明

| 模块          | 文件                | 说明                                                       |
|-------------|-------------------|----------------------------------------------------------|
| `main`      | `src/main.rs`     | 服务入口，调用 `app::run()` 一键启动                                |
| `api`       | `src/api/mod.rs`  | 路由组装、JWT 认证中间件注入、静态资源路由、404/405 fallback                 |
| `api::user` | `src/api/user.rs` | 用户完整 CRUD（条件查询 + 分页 + 关键词搜索 + 创建 + 更新 + 删除）              |
| `api::auth` | `src/api/auth.rs` | 认证处理器（账号密码登录、JWT Token 签发、用户信息查询），含时序攻击防护                |
| `web`       | `src/web/mod.rs`  | 前端 SPA 静态资源嵌入（`rust-embed`），支持 gzip / brotli 压缩和 MIME 检测 |

> 其他 20+ 个模块已迁移至 Cargo Workspace 的 lib crate 中，详见：
> - [`daoyi-axum-config`](../../crates/libs/daoyi-axum-config/) — 配置管理
> - [`daoyi-axum-support`](../../crates/libs/daoyi-axum-support/) — 基础设施
> - [`daoyi-axum-app`](../../crates/libs/daoyi-axum-app/) — 应用启动与核心功能
> - [`daoyi-sea-orm-entity-demo`](../../crates/sea-orm-entities/daoyi-sea-orm-entity-demo/) — 数据库 Entity

## 开发指南

### 中间件链顺序

中间件按 layer 顺序从外到内执行（由 `daoyi-axum-app` 的 `server.rs` 组装）：

```
请求 → TimeoutLayer (120s) → DefaultBodyLimit (2 GiB)
     → TraceLayer (日志 / 追踪 / 耗时) → CorsLayer (跨域)
     → NormalizePathLayer (路径规范化)
     → Router → JWT Auth Middleware → 路由匹配 → Handler
           ├── /api/* (需要 Bearer Token)
           ├── /api/auth/* (公开接口)
           └── /static/* (静态资源，gzip/br 压缩)
```

### 添加新的 API

1. 在 `src/api/` 下创建新的路由模块文件
2. 定义 `create_router()` 函数返回 `Router<AppState>`
3. 在 `src/api/mod.rs` 的 `create_router()` 中通过 `.nest()` 引入
4. 如需认证保护，使用 `.route_layer(get_auth_layer())` 包裹

### 生成 SeaORM Entity

```bash
sea-orm-cli generate entity \
  -u mysql://root:123456@127.0.0.1:3306/demo \
  --with-serde both \
  --model-extra-attributes 'serde(rename_all = "camelCase")' \
  --date-time-crate chrono \
  -o ./crates/sea-orm-entities/daoyi-sea-orm-entity-demo/src/demo/entity
```

### 依赖关系

```
web-starter
├── daoyi-axum-app            # 应用启动、JWT、分页、校验
│   ├── daoyi-axum-support    # 错误处理、ID 生成、密码等工具
│   └── daoyi-axum-config     # 配置管理
└── daoyi-sea-orm-entity-demo # 数据库实体模型
    └── daoyi-axum-support
```
