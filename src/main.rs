// src/main.rs
/*
 * Main executable for AgentNftEngineAPINext
 */

use clap::Parser;
use agentnftengineapinext::{Result, run};

#[derive(Parser)]
#[command(version, about = "AgentNftEngineAPINext - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
