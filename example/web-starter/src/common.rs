use crate::serde::deserialize_number;
use serde::{Deserialize, Serialize};
use validator::Validate;

const DEFAULT_PAGE_NO: u64 = 1;
const DEFAULT_PAGE_SIZE: u64 = 10;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PageParam {
    #[validate(range(min = 1, message = "页码最小值为 1"))]
    #[serde(default = "default_page_no", deserialize_with = "deserialize_number")]
    pub page_no: u64,
    #[validate(range(min = 1, max = 200, message = "每页条数最小值为 1,最大值为 200"))]
    #[serde(default = "default_page_size", deserialize_with = "deserialize_number")]
    pub page_size: u64,
}

fn default_page_no() -> u64 {
    DEFAULT_PAGE_NO
}

fn default_page_size() -> u64 {
    DEFAULT_PAGE_SIZE
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResult<T> {
    page_no: u64,
    page_size: u64,
    total_page: u64,
    total: u64,
    list: Vec<T>,
}

impl<T> PageResult<T> {
    pub fn new(page_no: u64, page_size: u64, total: u64, list: Vec<T>) -> Self {
        let total_page = (total + page_size - 1) / page_size;
        Self {
            page_no,
            page_size,
            total_page,
            total,
            list,
        }
    }

    pub fn from_page_param(page_param: PageParam, total: u64, list: Vec<T>) -> Self {
        Self::new(page_param.page_no, page_param.page_size, total, list)
    }
}
