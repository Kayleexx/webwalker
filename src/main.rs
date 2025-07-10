use clap::Parser;
use rusty_crawler::cli::Cli;
use rusty_crawler::crawler::{validate_and_build_config, crawl};
use std::collections::HashSet;
use reqwest::Client;

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match validate_and_build_config(args) {
        Ok(config) => {
            println!("Start URL: {}", config.start_url);
            println!("Max Depth: {}", config.max_depth);

            let client = Client::new();
            let mut visited = HashSet::new();

            crawl(
                &client,
                config.start_url,
                0,
                config.max_depth,
                &mut visited,
            )
            .await;
        println!("\nVisited URLs:");
        for url in &visited {
            println!("- {}", url);
        }
        }
        Err(err) => {
            eprintln!("❌ {}", err);
            std::process::exit(1);
        }
    }
}
