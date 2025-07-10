mod crawl;
pub use crawl::crawl;

use crate::cli::Cli;
use crate::types::CrawlConfig;
use url::Url;

pub fn validate_and_build_config(cli: Cli) -> Result<CrawlConfig, String> {
    match Url::parse(&cli.url) {
        Ok(parsed) if parsed.has_host() && parsed.scheme().starts_with("http") => {
            Ok(CrawlConfig {
                start_url: parsed,
                max_depth: cli.depth,
            })
        }
        _ => Err(format!("Invalid URL: {}", cli.url)),
    }
}
