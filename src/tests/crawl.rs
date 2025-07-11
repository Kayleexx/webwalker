use rusty_crawler::crawler::crawl;
use std::{collections::HashSet, sync::{Arc, Mutex}};
use tokio::sync::Semaphore;
use url::Url;
use reqwest::Client;

#[tokio::test]
async fn single_page_no_links() {
    // mock server skipped for brevity; assume https://example.com returns blank page
    let client  = Arc::new(Client::new());
    let visited = Arc::new(Mutex::new(HashSet::new()));
    let limiter = Arc::new(Semaphore::new(10));

    let start = Url::parse("https://example.com").unwrap();
    crawl(
        client,
        start.clone(),
        0,
        1,
        visited.clone(),
        limiter,
        5,
        true,
        start,
    )
    .await;

    assert_eq!(visited.lock().unwrap().len(), 1);
}
