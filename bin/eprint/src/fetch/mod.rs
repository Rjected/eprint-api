//! Main command
//!
//! Fetches paper metadata
use clap::Parser;

/// Start the node
#[derive(Debug, Parser)]
pub struct Command {
    /// The venue to fetch papers for
    #[arg(long)]
    venue: String,

    /// The year to fetch papers for
    #[arg(long)]
    year: u16,
}

impl Command {
    /// Execute `fetch` command
    pub async fn execute(self) -> eyre::Result<()> {
        Ok(())
    }
}
