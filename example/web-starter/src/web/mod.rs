//! Web 静态资源处理模块。
//!
//! 使用 [`rust-embed`] 将前端构建产物（`web/dist/`）嵌入到编译后的二进制文件中，
//! 支持 SPA (Single Page Application) 部署模式。
//!
//! ## 设计说明
//!
//! 将前端构建产物拆分为两个嵌入实例：
//! - [`IndexHtml`] — 仅嵌入 `index.html`，用于 SPA fallback
//! - [`StaticAssets`] — 嵌入除 `index.html` 外的所有静态资源（JS/CSS/图片等）
//!
//! 这种设计的好处：
//! - `index.html` 不会被当作静态文件直接返回，而是由 [`index_handler`]
//!   统一处理，确保 SPA 路由（如 `/users/1`）能正确回退到前端路由
//! - 静态资源按文件类型自动设置正确的 `Content-Type`
//! - 支持压缩内容（gzip / brotli），减少传输体积
//!
//! ## 路由结构
//!
//! ```text
//! GET /static/{*file} → static_handler  → 静态资源（带压缩）
//! GET /*               → index_handler  → SPA fallback
//! ```

use axum::http::{Method, header};
use axum::response::{IntoResponse, Response};
use daoyi_axum_support::support::error::ApiError;
use daoyi_axum_support::support::path::Path;
use rust_embed::Embed;

/// 嵌入 `web/dist/index.html`，用于 SPA 入口和 fallback 处理。
#[derive(Embed)]
#[folder = "web/dist"]
#[include = "index.html"]
struct IndexHtml;

/// 嵌入 `web/dist/` 下除 `index.html` 之外的所有静态资源。
///
/// `rust-embed` 会自动检测文件的 MIME 类型，并支持压缩内容
/// （gzip 或 brotli，取决于编译时是否启用了对应 feature）。
#[derive(Embed)]
#[folder = "web/dist"]
#[exclude = "index.html"]
struct StaticAssets;

/// 静态文件的 HTTP 响应包装。
///
/// 根据文件路径从 [`StaticAssets`] 中查找并返回对应的资源，
/// 自动设置正确的 `Content-Type` Header。文件不存在时返回 404。
struct StaticFiles<T>(T);

impl<T: AsRef<str>> IntoResponse for StaticFiles<T> {
    /// 将嵌入的静态文件转换为 HTTP 响应。
    ///
    /// 通过 `rust-embed` 的元数据自动获取 MIME 类型，
    /// 响应的 Content-Type 由文件扩展名决定。
    fn into_response(self) -> Response {
        let path = self.0.as_ref();
        match StaticAssets::get(path) {
            Some(file) => {
                let mime = file.metadata.mimetype();
                let body = file.data;
                ([(header::CONTENT_TYPE, mime)], body).into_response()
            }
            None => ApiError::NotFound.into_response(),
        }
    }
}

/// 静态资源处理器。
///
/// `GET /static/{*file}`
///
/// 从嵌入的静态资源中查找并返回匹配的文件。
/// 通过 `CompressionLayer` 中间件提供 gzip / brotli 压缩支持。
///
/// ## 示例
///
/// - `GET /static/assets/index.js` → 返回 `web/dist/assets/index.js`
/// - `GET /static/favicon.ico` → 返回 `web/dist/favicon.ico`
pub async fn static_handler(Path(path): Path<String>) -> impl IntoResponse {
    StaticFiles(path).into_response()
}

/// SPA 入口处理器。
///
/// 对所有非 API / 非静态资源的 GET 请求，统一返回 `index.html`。
/// 前端路由（如 React Router / Vue Router）负责后续页面解析。
///
/// 非 GET 方法（如 POST / PUT / DELETE 对未匹配路径）返回 404。
pub async fn index_handler(method: Method) -> impl IntoResponse {
    if method == Method::GET {
        let file = IndexHtml::get("index.html").expect("index.html not found");
        ([(header::CONTENT_TYPE, "text/html")], file.data).into_response()
    } else {
        ApiError::NotFound.into_response()
    }
}
