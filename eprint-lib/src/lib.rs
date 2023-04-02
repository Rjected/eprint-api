//! Bindings for the IACR Eprint CryptoDB API
#![warn(missing_debug_implementations, missing_docs, unreachable_pub)]
#![deny(unused_must_use, rust_2018_idioms)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]

mod metadata;
pub use metadata::{EprintMetadata, EprintMetadataCollection};

mod venues;
pub use venues::EprintVenue;

mod client;
pub use client::get_eprint_metadata;
