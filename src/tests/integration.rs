use rusty_crawler::crawler;
use reqwest::Client;
use url::Url;

#[tokio::test]
async fn crawl_example_depth1() {
    let client = Client::new();
    let start  = Url::parse("https://example.com").unwrap();
    let visited = crawler::crawl(client, start, 1, 4, 10, true).await;
    assert!(visited.len() >= 1); // at least the root
}
