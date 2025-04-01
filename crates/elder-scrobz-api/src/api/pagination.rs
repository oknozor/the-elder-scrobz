use utoipa::{IntoParams, ToSchema};

pub trait ToOffset {
    fn to_offset(&self) -> i64 {
        (self.page() - 1) * self.per_page()
    }

    fn page(&self) -> i64;
    fn per_page(&self) -> i64;
}

#[derive(serde::Deserialize, ToSchema, IntoParams, Debug)]
#[serde(default)]
pub struct PageQuery {
    page: i64,
    page_size: i64,
}

impl Default for PageQuery {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 100,
        }
    }
}

impl ToOffset for PageQuery {
    fn page(&self) -> i64 {
        self.page
    }

    fn per_page(&self) -> i64 {
        self.page_size
    }
}
