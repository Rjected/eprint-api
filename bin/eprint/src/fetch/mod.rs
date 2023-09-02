//! Main command
//!
//! Fetches paper metadata
use clap::Parser;
use eprint_lib::{get_eprint_metadata, EprintVenue};
use serde_json::to_string_pretty;

/// Start the node
#[derive(Debug, Parser)]
pub struct Command {
    /// The venue to fetch papers for
    #[arg(value_enum)]
    venue: EprintVenue,

    /// The year to fetch papers for
    year: u16,
}

impl Command {
    /// Execute `fetch` command
    pub async fn execute(self) -> eyre::Result<()> {
        let papers = get_eprint_metadata(self.venue, self.year).await?;
        println!("{}", to_string_pretty(&papers)?);
        Ok(())
    }
}
