use axum::http::{Method, header};
use axum::response::{IntoResponse, Response};
use daoyi_axum_support::support::error::ApiError;
use daoyi_axum_support::support::path::Path;
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "web/dist"]
#[include = "index.html"]
struct IndexHtml;

#[derive(Embed)]
#[folder = "web/dist"]
#[exclude = "index.html"]
struct StaticAssets;

struct StaticFiles<T>(T);

impl<T: AsRef<str>> IntoResponse for StaticFiles<T> {
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

pub async fn static_handler(Path(path): Path<String>) -> impl IntoResponse {
    StaticFiles(path).into_response()
}

pub async fn index_handler(method: Method) -> impl IntoResponse {
    if method == Method::GET {
        let file = IndexHtml::get("index.html").expect("index.html not found");
        ([(header::CONTENT_TYPE, "text/html")], file.data).into_response()
    } else {
        ApiError::NotFound.into_response()
    }
}
