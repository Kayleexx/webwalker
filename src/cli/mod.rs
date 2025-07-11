use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "WebWalker", long_about = "none")]

pub struct Cli {
    /// Starting URL
    pub url: String,

    /// Maximum recursion depth
    #[arg(short, long, default_value_t = 2)]
    pub depth: usize,

    /// Max parallel requests
    #[arg(long, default_value_t = 5)]
    pub rate: usize,

    /// Per-request timeout in seconds
    #[arg(long, default_value_t = 10)]
    pub timeout: u64,

    /// Restrict crawl to same domain
    #[arg(long, default_value_t = false)]
    pub same_domain: bool,

    /// Optional output file (json|txt based on extension)
    #[arg(long)]
    pub out: Option<std::path::PathBuf>,
}
