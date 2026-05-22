use crate::config;
use regex::Regex;
use std::borrow::Cow;
use std::cell::LazyCell;
use validator::ValidationError;

const MOBILE_PHONE_REGEX: LazyCell<Regex> = LazyCell::new(|| {
    Regex::new(r"^\+?[1-9]\d{6,14}$").expect("Failed to compile mobile phone regex")
});

pub fn validate_page_size(page_size: u64) -> Result<(), ValidationError> {
    let sys_config = config::get().sys();
    let min = sys_config.page_size_min();
    let max = sys_config.page_size_max();
    match page_size {
        s if s < min => {
            let mut err = ValidationError::new("page_size_range");
            err.message = Some(format!("每页条数最小值为 {min}").into());
            Err(err)
        }
        s if s > max => {
            let mut err = ValidationError::new("page_size_range");
            err.message = Some(format!("每页条数最大值为 {max}").into());
            Err(err)
        }
        _ => Ok(()),
    }
}

pub fn validate_mobile_phone(value: &str) -> Result<(), ValidationError> {
    if MOBILE_PHONE_REGEX.is_match(value) {
        Ok(())
    } else {
        Err(build_validation_error("手机号格式不正确"))
    }
}

fn build_validation_error(message: &'static str) -> ValidationError {
    ValidationError {
        code: Cow::from("invalid"),
        message: Some(Cow::from(message)),
        params: Default::default(),
    }
}
