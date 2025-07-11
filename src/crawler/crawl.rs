use scraper::{Html, Selector};
use reqwest::Client;
use std::{
    collections::HashSet,
    future::Future,
    pin::Pin,
};
use url::Url;

/// Recursively crawls pages up to `max_depth`.
/// Keeps a mutable `visited` set to avoid duplicates.

pub fn crawl<'a>(
    client: &'a Client,
    url: Url,
    depth: usize,
    max_depth: usize,
    visited: &'a mut HashSet<String>,
) -> Pin<Box<dyn Future<Output = ()> + 'a>> {
    Box::pin(async move {
        // 1️⃣ depth / duplicate guards
        if depth > max_depth {
            return;
        }
        let url_str = url.as_str().to_string();
        if !visited.insert(url_str.clone()) {
            return;                 // already visited
        }
        println!("{url_str}  [depth={depth}]");

        // 2️⃣ fetch the page (await point #1)
        let Ok(resp) = client.get(url.clone()).send().await else {
            eprintln!("Failed to fetch: {url_str}");
            return;
        };
        let Ok(body) = resp.text().await else {
            eprintln!("Failed to read body: {url_str}");
            return;
        };

        // 3️⃣ ---------- NO await below here ----------
        // Parse HTML and collect all next URLs *before* we await again
        let document  = Html::parse_document(&body);
        let selector  = Selector::parse("a[href]").unwrap();
        let mut next_urls = Vec::new();

        for elem in document.select(&selector) {
            if let Some(link) = elem.value().attr("href") {
                if let Ok(abs) = url.join(link) {
                    next_urls.push(abs);
                }
            }
        }
        drop(document);   // explicit ‑ makes it obvious HTML tree is gone

        // 4️⃣ recurse – each call is awaited, but the future is now Send‑safe
        for next in next_urls {
            crawl(client, next, depth + 1, max_depth, visited).await;
        }
    })
}
