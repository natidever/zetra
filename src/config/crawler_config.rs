use crate::constants::number_constants::CRAWLER_PAGE_LIMIT;
pub struct CrawlerConfig{
    pub page_limt: usize,
}

impl Default for CrawlerConfig{
    fn default() -> Self {
        Self { page_limt:CRAWLER_PAGE_LIMIT }
    }
}