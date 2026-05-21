use crate::error::{ApiError, ApiResult};
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

pub type CommonResult<T> = ApiResult<ApiResponse<T>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    code: i32,
    msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn new<M: AsRef<str>>(code: i32, msg: M, data: Option<T>) -> Self {
        Self {
            code,
            msg: String::from(msg.as_ref()),
            data,
        }
    }

    pub fn ok(data: Option<T>) -> Self {
        Self::new(0, String::from(""), data)
    }

    pub fn error<M: AsRef<str>>(msg: M) -> Self {
        Self::new(1, msg, None)
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}

pub fn success<T: Serialize>(data: T) -> CommonResult<T> {
    Ok(ApiResponse::ok(Some(data)))
}

pub fn fail<M: AsRef<str>>(msg: M) -> CommonResult<()> {
    Err(ApiError::Biz(String::from(msg.as_ref())))
}
