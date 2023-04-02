///! An extremely simple client for accessing the CryptoDB API.
use crate::{metadata::EprintMetadataCollection, venues::EprintVenue};
use reqwest::Client;

/// Get the metadata for all papers published at the given venue in the given year.
pub async fn get_eprint_metadata(
    venue: EprintVenue,
    year: u16,
) -> Result<EprintMetadataCollection, reqwest::Error> {
    let url = "https://www.iacr.org/cryptodb/data/api/conf.php";
    let client = Client::new();
    let response = client
        .get(url)
        .query(&[("year", year.to_string()), ("venue", venue.to_string())])
        .send()
        .await?;

    let response = response.json::<EprintMetadataCollection>().await?;

    Ok(response)
}
