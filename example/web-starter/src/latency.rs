//! 请求延迟记录模块。
//!
//! 作为 [`TraceLayer`](tower_http::trace::TraceLayer) 的 `on_response` 回调，
//! 在每个 HTTP 请求完成后自动记录响应状态码与处理耗时。

use axum::http::Response;
use std::fmt::Display;
use std::time::Duration;
use tower_http::trace::OnResponse;
use tracing::Span;

/// TraceLayer 的响应回调，用于记录请求耗时。
///
/// 实现了 [`OnResponse`] trait，在 [`TraceLayer`] 的 span 中追加
/// `latency` 和 `status` 字段。
#[derive(Debug, Clone, Copy)]
pub struct LatencyOnResponse;

impl<B> OnResponse<B> for LatencyOnResponse {
    fn on_response(self, response: &Response<B>, latency: Duration, _span: &Span) {
        tracing::info!(
            latency = %Latency(latency),
            status = response.status().as_u16(),
            "finished processing request"
        )
    }
}

/// 延迟时间的 Display 包装。
///
/// 自动选择合适的单位：>= 1ms 用毫秒，否则用微秒。
struct Latency(Duration);

impl Display for Latency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.as_millis() > 0 {
            write!(f, "{}ms", self.0.as_millis())
        } else {
            write!(f, "{}μs", self.0.as_micros())
        }
    }
}
