# daoyi-axum-config

应用配置管理 crate，提供 YAML 配置文件加载、环境变量覆盖和命令行参数支持。

全局配置通过 `LazyLock` 实现线程安全的单例模式，首次访问时自动初始化，后续调用直接返回缓存结果。

## 配置加载优先级

| 优先级   | 来源        | 说明                                        |
|-------|-----------|-------------------------------------------|
| 1（最高） | 命令行参数     | `--config_file <path>` 或 `-c <path>`      |
| 2     | 环境变量      | 以 `APP_` 为前缀，下划线分隔层级                      |
| 3（最低） | YAML 配置文件 | `resources/{APP_NAME}-{APP_PROFILE}.yaml` |

## 模块

| 模块                 | 说明                                        |
|--------------------|-------------------------------------------|
| `config::server`   | 服务器端口配置                                   |
| `config::database` | 数据库连接配置（host/port/user/password/database） |
| `config::sys`      | 系统通用配置（分页参数限制等）                           |

## 使用示例

```rust
use daoyi_axum_config::config;

// 获取全局配置单例
let port = config::get().server().port();
let db_host = config::get().database().host();
let page_size_max = config::get().sys().page_size_max();
```

## 环境变量覆盖

所有以 `APP_` 为前缀的环境变量自动映射到配置项：

```bash
APP_SERVER_PORT=8080          # 覆盖 server.port
APP_DATABASE_HOST=192.168.1.1 # 覆盖 database.host
APP_SYS_PAGE_SIZE_MAX=500     # 覆盖 sys.page_size_max
```

## 配置文件示例

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

## 依赖

- `config` — 配置加载与反序列化
- `serde` — 序列化/反序列化
- `anyhow` — 错误处理
