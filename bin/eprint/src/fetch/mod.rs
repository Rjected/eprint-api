//! Main command
//!
//! Fetches paper metadata
use clap::Parser;
use eprint_lib::{get_eprint_metadata, EprintVenue};
use serde_json::to_string_pretty;

/// Fetch the venue metadata
#[derive(Debug, Parser)]
pub struct Command {
    /// The venue to fetch papers for
    venue: String,

    /// The year to fetch papers for
    year: u16,
}

impl Command {
    /// Execute `fetch` command
    pub async fn execute(self) -> eyre::Result<()> {
        let venue: EprintVenue = self.venue.parse().map_err(|err: String| eyre::eyre!(err))?;
        let papers = get_eprint_metadata(venue, self.year).await?;
        println!("{}", to_string_pretty(&papers)?);
        Ok(())
    }
}
