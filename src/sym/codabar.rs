//! Encoder for Codabar barcodes.
//!
//! Codabar is a simple, self-checking symbology without a standard for a checksum digit.
//!
//! Codabar is used in the USA by FedEx, some Hospitals, and photo labs.
//!
//! Barcodes of this variant should start and end with either A, B, C, or D depending on
//! the industry.

use sym::Parse;
use sym::helpers;
use error::Result;
use std::ops::Range;
use std::char;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Unit {
    Zero, One, Two,
    Three, Four, Five,
    Six, Seven, Eight,
    Nine, Dash, Dollar,
    Colon, Slash, Point,
    Plus, A, B,
    C, D,
}

/// The Codabar barcode type.
#[derive(Debug)]
pub struct Codabar(Vec<Unit>);

impl Codabar {
    /// Creates a new Codabar barcode.
    ///
    /// Returns Result<Codabar, Error> indicating parse success.
    pub fn new(data: String) -> Result<Codabar> {
        Ok(Codabar(vec![Unit::One]))
    }

    /// Encodes the barcode.
    /// Returns a Vec<u8> of binary digits.
    pub fn encode(&self) -> Vec<u8> {
        vec![1,0]
    }
}

impl Parse for Codabar {
    /// Returns the valid length of data acceptable in this type of barcode.
    /// Codabar barcodes are variable-length.
    fn valid_len() -> Range<u32> {
        1..256
    }

    /// Returns the set of valid characters allowed in this type of barcode.
    fn valid_chars() -> Vec<char> {
        (0..10).into_iter().map(|i| char::from_digit(i, 10).unwrap()).collect()
    }
}

#[cfg(test)]
mod tests {
    use sym::codabar::*;
    use error::Error;
    use std::char;

    fn collapse_vec(v: Vec<u8>) -> String {
        let chars = v.iter().map(|d| char::from_digit(*d as u32, 10).unwrap());
        chars.collect()
    }

    #[test]
    fn codabar_encode() {
        let codabar = Codabar::new("A1234567B".to_owned()).unwrap();

        assert_eq!(collapse_vec(codabar.encode()), "10".to_owned());
    }
}
