///! Possible venues for a paper served by the CryptoDB API
#[allow(dead_code)]

/// All possible venues for the CryptoDB API.
///
/// From the [CryptoDB API documentation](https://www.iacr.org/cryptodb/data/api/):
///  * The venue may take the values "crypto", "eurocrypt", "asiacrypt", "fse", "ches", "tcc", or "pkc".
#[derive(Debug, PartialEq, Eq)]
pub enum EprintVenue {
    /// The Crypto conference
    Crypto,
    /// The Eurocrypt conference
    Eurocrypt,
    /// The Asiacrypt conference
    Asiacrypt,
    /// The FSE conference
    Fse,
    /// The CHES conference
    Ches,
    /// The TCC conference
    Tcc,
    /// The PKC conference
    Pkc,
}

impl ToString for EprintVenue {
    fn to_string(&self) -> String {
        match self {
            EprintVenue::Crypto => "crypto".to_string(),
            EprintVenue::Eurocrypt => "eurocrypt".to_string(),
            EprintVenue::Asiacrypt => "asiacrypt".to_string(),
            EprintVenue::Fse => "fse".to_string(),
            EprintVenue::Ches => "ches".to_string(),
            EprintVenue::Tcc => "tcc".to_string(),
            EprintVenue::Pkc => "pkc".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn venue_test() {
        // super useful test
        let venue_vectors = vec![
            (EprintVenue::Crypto, "crypto"),
            (EprintVenue::Eurocrypt, "eurocrypt"),
            (EprintVenue::Asiacrypt, "asiacrypt"),
            (EprintVenue::Fse, "fse"),
            (EprintVenue::Ches, "ches"),
            (EprintVenue::Tcc, "tcc"),
            (EprintVenue::Pkc, "pkc"),
        ];

        for (venue, expected_string) in venue_vectors {
            assert_eq!(venue.to_string(), expected_string);
        }
    }
}
