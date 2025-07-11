use clap::Parser;
use rusty_crawler::{cli::Cli, crawler::crawl};
use tokio::{sync::Semaphore, fs::File, io::AsyncWriteExt};
use std::{collections::HashSet, sync::{Arc, Mutex}};
use url::Url;
use reqwest::Client;

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let client  = Arc::new(Client::new());
    let visited = Arc::new(Mutex::new(HashSet::new()));
    let limiter = Arc::new(Semaphore::new(args.rate));

    let start_url = Url::parse(&args.url).expect("invalid start url");
    crawl(
        client.clone(),
        start_url.clone(),
        0,
        args.depth,
        visited.clone(),
        limiter,
        args.timeout,
        args.same_domain,
        start_url.clone(), // pass root for domain check
    ).await;

    // ---------- output ----------
    if let Some(path) = args.out {
    let set = visited.lock().unwrap();
    let is_json = path.extension().map(|e| e == "json").unwrap_or(false);
    let mut file = tokio::fs::File::create(&path).await.unwrap();

    if is_json {
        let json = serde_json::to_string_pretty(&*set).unwrap();
        file.write_all(json.as_bytes()).await.unwrap();
    } else {
        for u in &*set {
            file.write_all(u.as_bytes()).await.unwrap();
            file.write_all(b"\n").await.unwrap();
        }
    }

    println!("Saved {} URLs → {}", set.len(), path.display());
}

}
