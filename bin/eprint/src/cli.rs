//! CLI definition and entrypoint to executable
use crate::fetch;
use clap::{Parser, Subcommand};

/// Parse CLI options, set up logging and run the chosen command.
pub fn run() -> eyre::Result<()> {
    let opt = Cli::parse();
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    match opt.command {
        Commands::Fetch(command) => rt.block_on(command.execute()),
    }
}

/// Commands to be executed
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Fetch paper metadata for a venue and year
    #[command(name = "fetch")]
    Fetch(fetch::Command),
}

#[derive(Debug, Parser)]
#[command(author, version = "0.1", about = "Eprint API", long_about = None)]
struct Cli {
    /// The command to run
    #[clap(subcommand)]
    command: Commands,
}
