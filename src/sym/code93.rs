//! Encoder for Code93 barcodes.
//!
//! Code93 is intented to improve upon Code39 barcodes by offering a wider array of encodable
//! ASCII characters. It also produces denser barcodes than Code39.
//!
//! Code93 is a continuous, variable-length symbology.

use sym::Parse;
use sym::helpers;
use error::Result;
use std::ops::Range;

/// The Code93 barcode type.
#[derive(Debug)]
pub struct Code93(Vec<u8>);

impl Code93 {
    /// Creates a new barcode.
    /// Returns Result<Code93, Error> indicating parse success.
    pub fn new<T: AsRef<str>>(data: T) -> Result<Code93> {
        Ok(Code93(vec![]))
    }

    /// Encodes the barcode.
    /// Returns a Vec<u8> of encoded binary digits.
    pub fn encode(&self) -> Vec<u8> {
        vec![]
    }
}

impl Parse for Code93 {
    /// Returns the valid length of data acceptable in this type of barcode.
    /// Codabar barcodes are variable-length.
    fn valid_len() -> Range<u32> {
        1..256
    }

    /// Returns the set of valid characters allowed in this type of barcode.
    fn valid_chars() -> Vec<char> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use sym::code93::*;
    use error::Error;
    use std::char;

    fn collapse_vec(v: Vec<u8>) -> String {
        let chars = v.iter().map(|d| char::from_digit(*d as u32, 10).unwrap());
        chars.collect()
    }
}
