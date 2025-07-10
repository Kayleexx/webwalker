use url::Url;

#[derive(Debug, Clone)]
pub struct CrawlConfig {
    pub start_url: Url,
    pub max_depth: usize,
}