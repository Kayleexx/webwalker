use clap::Parser;
use reqwest::Client;
use rusty_crawler::{cli::Cli, crawler, output};
use tracing_subscriber::FmtSubscriber;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // logging
    let sub = FmtSubscriber::builder().with_max_level(tracing::Level::INFO).finish();
    tracing::subscriber::set_global_default(sub).unwrap();

    let args = Cli::parse();
    let client = Client::new();
    let start  = url::Url::parse(&args.url)?;

    let visited = crawler::crawl(
        client,
        start.clone(),
        args.depth,
        args.rate,
        args.timeout,
        args.same_domain,
    ).await;

    if let Some(path) = args.out {
        output::write_results(&visited, path).await?;
    }

    tracing::info!("Finished – {} unique pages", visited.len());
    Ok(())
}
