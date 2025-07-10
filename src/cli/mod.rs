use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Rusty Crawler", long_about = "none")]

pub struct Cli {
    pub url: String,
    
    #[arg(short, long, default_value_t = 2)]
    pub depth: usize,
}