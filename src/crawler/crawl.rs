use crate::types::CrawlConfig;
use scraper::{Html, Selector};
use reqwest::Client;
use std::collections::HashSet;
use url::Url;
use std::pin::Pin;
use std::future::Future;

/// Recursively crawls pages up to max depth.
/// Returns the set of visited URLs.
pub fn crawl(
    client: &Client,
    url: Url,
    depth: usize,
    max_depth: usize,
    visited: &mut HashSet<String>,
) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
    Box::pin(async move {
        if depth > max_depth {
            return;
        }

        let url_str = url.as_str().to_string();
        if visited.contains(&url_str) {
            return;
        }

        println!("{} [depth={}]", url_str, depth);
        visited.insert(url_str.clone());

        let Ok(resp) = client.get(url.clone()).send().await else {
            eprintln!("Failed to fetch: {url_str}");
            return;
        };

        let Ok(body) = resp.text().await else {
            eprintln!("Failed to read body: {url_str}");
            return;
        };

        let document = Html::parse_document(&body);
        let selector = Selector::parse("a[href]").unwrap();

        for element in document.select(&selector) {
            if let Some(link) = element.value().attr("href") {
                if let Ok(next_url) = url.join(link) {
                    crawl(client, next_url, depth + 1, max_depth, visited).await;
                }
            }
        }
    })
}
