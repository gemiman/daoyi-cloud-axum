# daoyi-cloud-axum

Rust cloud-native microservice scaffold based on Axum + SeaORM + MySQL.

一个基于 **Axum** 框架的 Rust 云原生微服务脚手架，集成 **SeaORM** ORM 与 **MySQL** 数据库，提供开箱即用的项目模板。

## 特性

- **Axum 0.8** — 高性能异步 Web 框架
- **SeaORM 2.0** — 异步 ORM，支持 MySQL / PostgreSQL / SQLite
- **Tokio** — 异步运行时，全特性支持
- **Tracing** — 结构化日志，支持本地时间与时区偏移，自动记录请求耗时
- **Config** — 灵活的 YAML 配置加载，支持命令行参数与环境变量覆盖
- **Validator + axum-valid** — 声明式参数校验，支持查询参数、路径参数、JSON Body
- **统一错误处理** — 自动映射业务错误到标准 HTTP 状态码与 JSON 响应
- **分布式 ID** — 基于雪花算法的全局唯一 ID 生成器
- **密码安全** — bcrypt 密码哈希与验证
- **JWT 认证** — HMAC-SHA256 Token 编解码 + Bearer Token 中间件
- **分页支持** — 内置通用分页参数与分页响应结构
- **Cargo Workspace** — 模块化管理，示例与主项目独立

## 快速开始

### 环境要求

- Rust **1.94.0**+
- Cargo
- MySQL（可选，若使用数据库功能）

### 准备数据库

```sql
CREATE DATABASE IF NOT EXISTS demo DEFAULT CHARSET utf8mb4;

CREATE TABLE IF NOT EXISTS demo_sys_user
(
    id           BIGINT PRIMARY KEY AUTO_INCREMENT,
    name         VARCHAR(64) NOT NULL,
    gender       VARCHAR(8),
    account      VARCHAR(64),
    password     VARCHAR(128),
    mobile_phone VARCHAR(32),
    birthday     DATE,
    enabled      BOOLEAN              DEFAULT TRUE,
    creator      VARCHAR(64),
    create_time  DATETIME    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater      VARCHAR(64),
    update_time  DATETIME    NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    deleted      BOOLEAN              DEFAULT FALSE,
    tenant_id    BIGINT               DEFAULT 0
) COMMENT '系统用户表';
```

### 运行示例

```bash
# 进入 web-starter 示例目录
cd example/web-starter

# 基本运行（会加载 resources/application-dev.yaml）
APP_NAME=example-web-starter cargo run

# 指定端口
APP_SERVER_PORT=8080 cargo run

# 指定自定义配置文件
cargo run -- -c resources/example-web-starter-dev.yaml
```

服务默认监听 `http://0.0.0.0:3000`，启动后可访问：

- `GET /` — 欢迎页
- `GET /api/users` — 条件查询用户
- `GET /api/users/page?pageNo=1&pageSize=10` — 分页查询用户
- `GET /api/users/page?keyword=李四&pageNo=1&pageSize=10` — 关键词搜索
- `POST /api/users` — 创建用户
- `PUT /api/users/{id}` — 更新用户
- `DELETE /api/users/{id}` — 删除用户

## 配置说明

### 配置文件

配置文件存放在 `resources/` 目录下，命名规则为 `{APP_NAME}-{APP_PROFILE}.yaml`。

| 环境变量          | 默认值           | 说明     |
|---------------|---------------|--------|
| `APP_NAME`    | `application` | 应用名称   |
| `APP_PROFILE` | `dev`         | 运行环境标识 |

配置文件示例（`resources/example-web-starter-dev.yaml`）：

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
  page_size_min: 10
  page_size_max: 100
  page_size_default: 20
  page_no_default: 1
```

### 命令行参数

| 参数                     | 说明            |
|------------------------|---------------|
| `-c <path>`            | 指定配置文件路径      |
| `--config_file <path>` | 指定配置文件路径（长格式） |
| `-c=<path>`            | 等号形式          |
| `--config_file=<path>` | 等号长格式         |

### 环境变量覆盖

所有以 `APP_` 为前缀的环境变量会自动映射到配置项，使用下划线分隔层级。例如：

```bash
# 覆盖 server.port
APP_SERVER_PORT=8080 cargo run

# 覆盖 database.host
APP_DATABASE_HOST=192.168.1.100 cargo run

# 覆盖 sys.page_size_max
APP_SYS_PAGE_SIZE_MAX=500 cargo run
```

## 项目架构

项目采用 **Cargo Workspace** 管理，核心功能拆分为独立 crate，示例项目仅包含路由与业务逻辑。

```
daoyi-cloud-axum/
├── Cargo.toml                                    # Workspace 配置与共享依赖
├── README.md
├── LICENSE
├── src/
│   └── main.rs                                   # 根 crate 入口（占位）
├── crates/
│   ├── libs/
│   │   ├── daoyi-axum-config/                    # 配置管理 crate
│   │   │   └── src/
│   │   │       └── config/
│   │   │           ├── mod.rs                    # 配置加载（YAML + 环境变量 + CLI）
│   │   │           ├── server.rs                 # 服务器配置
│   │   │           ├── database.rs               # 数据库配置
│   │   │           └── sys.rs                    # 系统通用配置（分页参数）
│   │   ├── daoyi-axum-support/                   # 基础设施支撑 crate
│   │   │   └── src/support/
│   │   │       ├── error.rs                      # 统一错误枚举（自动映射 HTTP 状态码）
│   │   │       ├── response.rs                   # 统一 API 响应格式
│   │   │       ├── enumeration.rs                # 通用枚举类型（性别等）
│   │   │       ├── id.rs                         # 分布式 ID 生成器（雪花算法）
│   │   │       ├── passwd.rs                     # bcrypt 密码哈希与验证
│   │   │       ├── valid.rs                      # 校验型参数提取器
│   │   │       ├── json.rs / query.rs / path.rs  # 自定义提取器（自动错误转换）
│   │   │       ├── serde.rs                      # 通用反序列化工具
│   │   │       ├── logger.rs                     # 结构化日志初始化
│   │   │       └── latency.rs                    # 请求耗时记录
│   │   └── daoyi-axum-app/                       # 应用启动与核心功能 crate
│   │       └── src/app/
│   │           ├── mod.rs                        # 应用启动入口 + AppState
│   │           ├── server.rs                     # HTTP 服务器构建（中间件链）
│   │           ├── database.rs                   # 数据库连接池初始化
│   │           ├── common.rs                     # 通用数据结构（分页参数/分页结果）
│   │           ├── validation.rs                 # 自定义参数校验函数
│   │           ├── sea_orm_utils.rs              # SeaORM 工具函数（占位）
│   │           └── auth/
│   │               └── jwt/
│   │                   ├── mod.rs                # JWT 编解码器
│   │                   └── middleware.rs          # Bearer Token 认证中间件
│   └── sea-orm-entities/
│       └── daoyi-sea-orm-entity-demo/            # SeaORM Entity 模型 crate
│           └── src/demo/entity/
│               ├── demo_sys_user.rs              # 系统用户表
│               ├── demo_category.rs              # 分类表
│               ├── demo_contact.rs               # 联系人表
│               ├── demo_course.rs                # 课程表
│               ├── demo_grade.rs                 # 成绩表
│               └── demo_student.rs               # 学生表
├── example/
│   └── web-starter/                              # Web 服务示例
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs                           # 服务入口
│           └── api/
│               ├── mod.rs                        # API 路由组装
│               └── user.rs                       # 用户 API（完整 CRUD）
└── resources/
    └── example-web-starter-dev.yaml              # 示例开发环境配置
```

### 架构设计

```
请求 → Timeout (120s) → BodyLimit (2 GiB) → TraceLayer (日志/追踪)
     → CORS → NormalizePath (路径规范化)
     → Router → JWT Auth Middleware → 路由匹配 → 404/405 Fallback
     → Handler → 参数提取 (ValidQuery/ValidPath/ValidJson)
               → 参数校验 (validator)
               → 业务处理
               → SeaORM (数据库查询)
               → ApiResponse (JSON 响应)
```

### Crate 依赖关系

```
web-starter (示例)
├── daoyi-axum-app（应用启动、中间件、JWT 认证）
│   ├── daoyi-axum-support（错误处理、ID 生成、密码等）
│   └── daoyi-axum-config（配置管理）
└── daoyi-sea-orm-entity-demo（数据库实体）
    └── daoyi-axum-support
```

## API 响应格式

所有 API 接口返回统一的 JSON 格式：

```json
{
  "code": 0,
  "msg": "",
  "data": {
    ...
  }
}
```

- `code = 0`：成功
- `code = 1`：业务错误
- `code != 0 || 1`：由 HTTP 状态码决定（400 / 404 / 405 / 500）

## 生成 SeaORM Entity

```bash
cargo install sea-orm-cli@^2.0.0-rc

sea-orm-cli generate entity \
  -u mysql://root:123456@127.0.0.1:3306/demo \
  --with-serde both \
  --model-extra-attributes 'serde(rename_all = "camelCase")' \
  --date-time-crate chrono \
  -o ./crates/sea-orm-entities/daoyi-sea-orm-entity-demo/src/demo/entity
```

## License

MIT License. 详见 [LICENSE](./LICENSE)。

Copyright (c) 2026 兰陵王
