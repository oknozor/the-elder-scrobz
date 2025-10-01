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

#[derive(serde::Serialize, ToSchema, Debug)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
    pub total_pages: i64,
    pub previous_page: Option<i64>,
    pub next_page: Option<i64>,
    pub last_page: i64,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: i64, page: i64, page_size: i64) -> Self {
        let total_pages = (total + page_size - 1) / page_size;
        let previous_page = if page > 1 { Some(page - 1) } else { None };
        let next_page = if page < total_pages {
            Some(page + 1)
        } else {
            None
        };
        let last_page = total_pages;

        Self {
            data,
            total,
            page,
            page_size,
            total_pages,
            previous_page,
            next_page,
            last_page,
        }
    }

    pub fn from_query(data: Vec<T>, total: i64, query: impl ToOffset) -> Self {
        Self::new(data, total, query.page(), query.per_page())
    }
}
