use crate::response::ApiResponse;
use axum::extract::rejection::{JsonRejection, PathRejection, QueryRejection};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_valid::ValidRejection;

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
    #[error("参数校验失败:{0}")]
    Validation(String),
}

impl From<ValidRejection<ApiError>> for ApiError {
    fn from(value: ValidRejection<ApiError>) -> Self {
        match value {
            ValidRejection::Valid(errors) => {
                ApiError::Validation(format_validation_errors(&errors))
            }
            ValidRejection::Inner(error) => error,
        }
    }
}

/// 将 validator 的 ValidationErrors 格式化为易读的字段级错误信息
fn format_validation_errors(errors: &validator::ValidationErrors) -> String {
    format_to_vec(errors).join("; ")
}
fn format_to_vec(errors: &validator::ValidationErrors) -> Vec<String> {
    use validator::ValidationErrorsKind;
    errors
        .errors()
        .iter()
        .flat_map(|(field, errors_kind)| match errors_kind {
            ValidationErrorsKind::Field(field_errors) => field_errors
                .iter()
                .map(|error| {
                    let message = error
                        .message
                        .as_ref()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| "验证失败".to_string());
                    format!("[{}]{}", field, message)
                })
                .collect::<Vec<_>>(),
            ValidationErrorsKind::Struct(struct_errors) => format_to_vec(struct_errors),
            ValidationErrorsKind::List(list_errors) => list_errors
                .iter()
                .flat_map(|(_index, errors)| format_to_vec(errors))
                .collect::<Vec<_>>(),
        })
        .collect()
}

impl ApiError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            ApiError::Query(_)
            | ApiError::Path(_)
            | ApiError::Json(_)
            | ApiError::Validation(_) => StatusCode::BAD_REQUEST,
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
