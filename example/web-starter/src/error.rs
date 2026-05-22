use crate::response::ApiResponse;
use axum::extract::rejection::{JsonRejection, PathRejection, QueryRejection};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("服务器迷路了😵")]
    NotFound,
    #[error("这个HTTP方法不合法呀🈲")]
    MethodNotAllowed,
    #[error("{0}")]
    Biz(String),
    #[error("内部错误:{0}")]
    Internal(#[from] anyhow::Error),
    #[error("数据库错误:{0}")]
    SeaOrmDb(#[from] sea_orm::DbErr),
    #[error("查询参数错误:{0}")]
    Query(#[from] QueryRejection),
    #[error("路径参数错误:{0}")]
    Path(#[from] PathRejection),
    #[error("Body参数错误:{0}")]
    Json(#[from] JsonRejection),
}

impl ApiError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            ApiError::Query(_) | ApiError::Path(_) | ApiError::Json(_) => StatusCode::BAD_REQUEST,
            ApiError::Internal(_) | ApiError::SeaOrmDb(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Biz(_) => StatusCode::OK,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = self.status_code();
        let json = axum::Json(ApiResponse::<()>::error(self.to_string()));
        (status_code, json).into_response()
    }
}
