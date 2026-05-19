use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(
            tracing_subscriber::fmt::layer()
                // ## 不同 `%z` 变体的效果
                //
                // | 格式 | 示例 | 说明 |
                // |------|------|------|
                // | `%z` | `+0800` | 无冒号，标准 strftime 格式 |
                // | `%:z` | `+08:00` | 带冒号，ISO 8601 更推荐 |
                // | `%::z` | `+08:00:00` | 带秒精度（通常不需要） |
                // | `%#z` | `+08` | 仅小时（部分平台支持） |
                //
                // **推荐 `%:z`**，因为这符合 ISO 8601 扩展格式，也是最常见的写法。
                // 现在你的日志时间就变成了 **带时区偏移的本地时间**，既知道是本地时间，也知道相对于 UTC 的偏移量，完全没有歧义。
                .with_timer(ChronoLocal::new(String::from("%Y-%m-%dT%H:%M:%S%.6f%:z")))
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_target(false),
        )
        .init();
}
