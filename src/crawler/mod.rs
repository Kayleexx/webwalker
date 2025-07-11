mod fetcher;
mod parser;
mod visited;

use fetcher::fetch;
use parser::extract_links;
pub use visited::Visited;

use futures::stream::{FuturesUnordered, StreamExt};
use reqwest::Client;
use tokio::sync::Semaphore;
use tracing::{info};
use url::Url;

pub async fn crawl(
    client:     Client,
    root:       Url,
    max_depth:  usize,
    rate:       usize,
    timeout_s:  u64,
    same_domain: bool,
) -> Visited {
    let limiter = Semaphore::new(rate);
    let visited = Visited::default();

    inner(
        &client, root.clone(), 0, max_depth,
        &limiter, timeout_s, same_domain, &root, &visited
    ).await;
    visited
}

async fn inner(
    client:      &Client,
    url:         Url,
    depth:       usize,
    max_depth:   usize,
    limiter:     &Semaphore,
    timeout_s:   u64,
    same_domain: bool,
    root:        &Url,
    visited:     &Visited,
) {
    if depth > max_depth { return; }
    if same_domain && url.host_str() != root.host_str() { return; }
    if !visited.insert_if_new(url.as_str()) { return; }

    info!(%url, depth, "crawl");

    let Some(body) = fetch(client, limiter, &url, timeout_s).await else { return; };
    let links = extract_links(&body, &url);

    let mut tasks = FuturesUnordered::new();
    for link in links {
        tasks.push(inner(
            client, link, depth+1, max_depth,
            limiter, timeout_s, same_domain, root, visited
        ));
    }
    while tasks.next().await.is_some() {}
}
