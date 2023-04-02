///! An extremely simple client for accessing the CryptoDB API.
use crate::{metadata::EprintMetadataCollection, venues::EprintVenue};
use reqwest::Client;

/// Get the metadata for all papers published at the given venue in the given year.
pub async fn get_eprint_metadata(
    venue: EprintVenue,
    year: u16,
) -> Result<EprintMetadataCollection, reqwest::Error> {
    let url = format!(
        "https://eprint.iacr.org/cryptodb/data/api/conf.php?year={}&venue={}",
        year,
        venue.to_string()
    );
    let client = Client::new();
    let response = client
        .get(&url)
        .send()
        .await?
        .json::<EprintMetadataCollection>()
        .await?;

    Ok(response)
}
