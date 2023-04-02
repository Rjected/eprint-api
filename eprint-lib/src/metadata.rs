//! Paper metadata, used to deserialize responses from the CryptoDB API.
use serde::{Deserialize, Serialize};

/// Metadata for a single paper.
#[derive(Debug, Serialize, Deserialize)]
pub struct EprintMetadata {
    /// The paper's unique identifier
    pubkey: u64,
    /// The paper's DOI
    #[serde(rename = "DOI")]
    doi: String,
    /// The paper's title
    title: String,
    /// The paper's YouTube link, if any
    youtube: Option<String>,
    /// The venue where the paper was published
    venue: String,
    /// The year the paper was published
    year: u16,
    /// The paper's URL, if any
    #[serde(rename = "URL")]
    url: Option<String>,
    /// The paper's presentation URL, if any
    presentationurl: Option<String>,
    /// The paper's abstract, if any
    #[serde(rename = "abstract")]
    paper_abstract: Option<String>,
    /// The pages of the requested proceedings where the paper is located
    pages: String,
    /// The paper's awards, if any
    award: Option<String>,
    /// The authors of the paper
    authors: Vec<String>,
}

/// A collection of metadata for a set of papers.
#[derive(Debug, Serialize, Deserialize)]
pub struct EprintMetadataCollection {
    /// The metadata for the requested papers.
    papers: Vec<EprintMetadata>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_test() {
        let example_input = r#"
            {
                "papers": [
                    {
                        "pubkey": 28175,
                        "DOI": "10.1007\/978-3-319-63688-7_4",
                        "title": "Memory-Tight Reductions",
                        "youtube": "0gpJ8kbCiFQ",
                        "venue": "crypto",
                        "year": 2017,
                        "URL": null,
                        "presentationurl": null,
                        "abstract": null,
                        "pages": "101-132",
                        "award": null,
                        "authors": [
                            "Benedikt Auerbach",
                            "David Cash",
                            "Manuel Fersch",
                            "Eike Kiltz"
                        ]
                    }
                ]
            }
            "#;

        let _metadata: EprintMetadataCollection = serde_json::from_str(example_input).unwrap();
    }
}
