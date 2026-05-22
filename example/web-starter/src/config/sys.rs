use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct SysConfig {
    page_size_min: Option<u64>,
    page_size_max: Option<u64>,
    page_size_default: Option<u64>,
    page_no_default: Option<u64>,
}

impl SysConfig {
    pub fn page_size_min(&self) -> u64 {
        self.page_size_min.unwrap_or(1)
    }

    pub fn page_size_max(&self) -> u64 {
        self.page_size_max.unwrap_or(200)
    }

    pub fn page_size_default(&self) -> u64 {
        self.page_size_default.unwrap_or(10)
    }

    pub fn page_no_default(&self) -> u64 {
        self.page_no_default.unwrap_or(1)
    }
}
