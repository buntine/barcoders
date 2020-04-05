//! Supported barcode symbologies.
//!
//! Symbologies are separated into logical modules and thus you must `use` the appropriate one(s).
//!
//! For example:
//!
//! ```rust
//! use barcoders::sym::ean13::*;
//!
//! let barcode = EAN13::new("750103131130").unwrap();
//! let encoded = barcode.encode();
//! ```
//! Each encoder accepts a `String` to be encoded. Valid data is barcode-specific and thus
//! constructors return an Option<T>.

pub mod ean13;
pub mod ean8;
pub mod ean_supp;
pub mod code39;
pub mod code93;
pub mod code11;
pub mod code128;
pub mod codabar;
pub mod tf;
mod helpers;

use std::ops::Range;
use std::iter::Iterator;
use crate::error::Error;

trait Parse {
    fn valid_chars() -> Vec<char>;
    fn valid_len() -> Range<u32>;

    fn parse(data: &str) -> Result<&str, Error> {
        let valid_chars = Self::valid_chars();
        let valid_len = Self::valid_len();
        let data_len = data.len() as u32;

        if data_len < valid_len.start || data_len > valid_len.end {
            return Err(Error::Length);
        }

        let bad_char = data
            .chars()
            .find(|&c| valid_chars
                           .iter()
                           .find(|&vc| *vc == c)
                           .is_none());

        match bad_char {
            Some(_) => Err(Error::Character),
            None => Ok(data),
        }
    }
}
