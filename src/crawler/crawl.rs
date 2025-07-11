use tokio::{sync::Semaphore, time::{timeout, Duration}};
use futures::stream::{FuturesUnordered, StreamExt};
use std::{collections::HashSet, sync::{Arc, Mutex}};
use reqwest::Client;
use scraper::{Html, Selector};
use url::Url;

pub async fn crawl(
    client:   Arc<Client>,
    url:      Url,
    depth:    usize,
    max_depth: usize,
    visited:  Arc<Mutex<HashSet<String>>>,
    limiter:  Arc<Semaphore>,
    timeout_s: u64,
    same_domain: bool,
    root:     Url,               // root to compare hosts
) {
    if depth > max_depth { return; }

    // dedup
    {
        let mut v = visited.lock().unwrap();
        if !v.insert(url.as_str().to_owned()) { return; }
    }

    // optional domain filter
    if same_domain && url.host_str() != root.host_str() {
        return;
    }

    let _permit = limiter.acquire().await.unwrap();

    let body = match timeout(
        Duration::from_secs(timeout_s),
        client.get(url.clone()).send()
    ).await {
        Ok(Ok(resp)) => match resp.text().await {
            Ok(b) => b,
            Err(_) => return,
        },
        _ => return, // timeout or network error
    };

    // parse links
    let doc = Html::parse_document(&body);
    let sel = Selector::parse("a[href]").unwrap();
    let mut tasks = FuturesUnordered::new();

    for el in doc.select(&sel) {
        if let Some(href) = el.value().attr("href") {
            if let Ok(next) = url.join(href) {
                let task = crawl(
                    client.clone(),
                    next,
                    depth + 1,
                    max_depth,
                    visited.clone(),
                    limiter.clone(),
                    timeout_s,
                    same_domain,
                    root.clone(),
                );
                tasks.push(task);
            }
        }
    }
    while tasks.next().await.is_some() {}
}
