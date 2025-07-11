use reqwest::Client;
use tokio::{sync::Semaphore, time::{timeout, Duration}};
use tracing::{error, info};

pub async fn fetch(
    client: &Client,
    limiter: &Semaphore,
    url: &reqwest::Url,
    timeout_s: u64,
) -> Option<String> {
    let _permit = limiter.acquire().await.unwrap();
    let span = tracing::span!(tracing::Level::INFO, "fetch", %url);
    let _enter = span.enter();

    let res = timeout(Duration::from_secs(timeout_s), client.get(url.clone()).send()).await;

    // ✅ Fix double unwrapping
    let resp = match res {
        Ok(Ok(r)) => r,
        Ok(Err(e)) => {
            error!("request error: {e}");
            return None;
        }
        Err(_) => {
            error!("timeout");
            return None;
        }
    };

    match resp.text().await {
        Ok(body) => Some(body),
        Err(e) => {
            error!("body error: {e}");
            None
        }
    }
}
