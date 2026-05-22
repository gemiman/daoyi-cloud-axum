use crate::config;
use crate::serde::deserialize_number;
use crate::validation::validate_page_size;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PageParam {
    #[validate(range(min = 1, message = "页码最小值为 1"))]
    #[serde(default = "default_page_no", deserialize_with = "deserialize_number")]
    pub page_no: u64,
    #[validate(custom(function = "validate_page_size"))]
    #[serde(default = "default_page_size", deserialize_with = "deserialize_number")]
    pub page_size: u64,
}

fn default_page_no() -> u64 {
    config::get().sys().page_no_default()
}

fn default_page_size() -> u64 {
    config::get().sys().page_size_default()
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
