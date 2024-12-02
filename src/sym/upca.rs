//! Encoder for UPCA barcodes.
//!
//! UPCA barcodes are common in retail in the US.
//!
//! This module defines types for:
//!   * UPC-A

use crate::error::{Error, Result};
use crate::sym::{helpers, Parse};
use core::char;
use core::ops::Range;
use helpers::Vec;

/// Encoding mappings for UPC barcodes.
/// 1 = bar, 0 = no bar.
///
/// The two indices are:
/// * Left side encodings.
/// * Right side encodings.
pub const ENCODINGS: [[[u8; 7]; 10]; 2] = [
    [
        [0, 0, 0, 1, 1, 0, 1],
        [0, 0, 1, 1, 0, 0, 1],
        [0, 0, 1, 0, 0, 1, 1],
        [0, 1, 1, 1, 1, 0, 1],
        [0, 1, 0, 0, 0, 1, 1],
        [0, 1, 1, 0, 0, 0, 1],
        [0, 1, 0, 1, 1, 1, 1],
        [0, 1, 1, 1, 0, 1, 1],
        [0, 1, 1, 0, 1, 1, 1],
        [0, 0, 0, 1, 0, 1, 1],
    ],
    [
        [1, 1, 1, 0, 0, 1, 0],
        [1, 1, 0, 0, 1, 1, 0],
        [1, 1, 0, 1, 1, 0, 0],
        [1, 0, 0, 0, 0, 1, 0],
        [1, 0, 1, 1, 1, 0, 0],
        [1, 0, 0, 1, 1, 1, 0],
        [1, 0, 1, 0, 0, 0, 0],
        [1, 0, 0, 0, 1, 0, 0],
        [1, 0, 0, 1, 0, 0, 0],
        [1, 1, 1, 0, 1, 0, 0],
    ],
];

/// Maps parity (odd/even) for the left-side digits based on the first digit in
/// the number system portion of the barcode data.
const PARITY: [[usize; 5]; 10] = [
    [0, 0, 0, 0, 0],
    [0, 1, 0, 1, 1],
    [0, 1, 1, 0, 1],
    [0, 1, 1, 1, 0],
    [1, 0, 0, 1, 1],
    [1, 1, 0, 0, 1],
    [1, 1, 1, 0, 0],
    [1, 0, 1, 0, 1],
    [1, 0, 1, 1, 0],
    [1, 1, 0, 1, 0],
];

/// The left-hand guard pattern.
pub const LEFT_GUARD: [u8; 3] = [1, 0, 1];
/// The middle guard pattern.
pub const MIDDLE_GUARD: [u8; 5] = [0, 1, 0, 1, 0];
/// The right-hand guard pattern.
pub const RIGHT_GUARD: [u8; 3] = [1, 0, 1];

/// The UPCA barcode type.
#[derive(Debug)]
pub struct UPCA(Vec<u8>);

impl UPCA {
    /// Creates a new barcode.
    /// Returns Result<UPCA, Error> indicating parse success.
    pub fn new<T: AsRef<str>>(data: T) -> Result<UPCA> {
        let d = UPCA::parse(data.as_ref())?;
        let digits: Vec<u8> = d
            .chars()
            .map(|c| c.to_digit(10).expect("Unknown character") as u8)
            .collect();

        let upca = UPCA(digits[0..11].to_vec());

        // If checksum digit is provided, check the checksum.
        if digits.len() == 12 && upca.checksum_digit() != digits[11] {
            return Err(Error::Checksum);
        }

        Ok(upca)
    }

    /// Calculates the checksum digit using a modulo-10 weighting algorithm.
    fn checksum_digit(&self) -> u8 {
        helpers::modulo_10_checksum(&self.0[..], false)
    }

    fn checksum_encoding(&self) -> [u8; 7] {
        self.char_encoding(1, self.checksum_digit())
    }

    fn char_encoding(&self, side: usize, d: u8) -> [u8; 7] {
        ENCODINGS[side][d as usize]
    }

    fn left_digits(&self) -> &[u8] {
        &self.0[0..6]
    }

    fn right_digits(&self) -> &[u8] {
        &self.0[6..]
    }

    fn left_payload(&self) -> Vec<u8> {
        let slices: Vec<[u8; 7]> = self
            .left_digits()
            .iter()
            .map(|d| self.char_encoding(0, *d))
            .collect();

        helpers::join_iters(slices.iter())
    }

    fn right_payload(&self) -> Vec<u8> {
        let slices: Vec<[u8; 7]> = self
            .right_digits()
            .iter()
            .map(|d| self.char_encoding(1, *d))
            .collect();

        helpers::join_iters(slices.iter())
    }

    /// Encodes the barcode.
    /// Returns a Vec<u8> of binary digits.
    pub fn encode(&self) -> Vec<u8> {
        helpers::join_slices(
            &[
                &LEFT_GUARD[..],
                &self.left_payload()[..],
                &MIDDLE_GUARD[..],
                &self.right_payload()[..],
                &self.checksum_encoding()[..],
                &RIGHT_GUARD[..],
            ][..],
        )
    }
}

impl Parse for UPCA {
    /// Returns the valid length of data acceptable in this type of barcode.
    fn valid_len() -> Range<u32> {
        11..12
    }

    /// Returns the set of valid characters allowed in this type of barcode.
    fn valid_chars() -> Vec<char> {
        (0..10).map(|i| char::from_digit(i, 10).unwrap()).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Error;
    use crate::sym::upca::*;
    #[cfg(not(feature = "std"))]
    use alloc::string::String;
    use core::char;

    fn collapse_vec(v: Vec<u8>) -> String {
        let chars = v.iter().map(|d| char::from_digit(*d as u32, 10).unwrap());
        chars.collect()
    }

    #[test]
    fn new_upca() {
        let upca = UPCA::new("72527273070");

        assert!(upca.is_ok());
    }

    #[test]
    fn invalid_data_upca() {
        let upca = UPCA::new("012345612a45");

        assert_eq!(upca.err().unwrap(), Error::Character)
    }

    #[test]
    fn invalid_len_upca() {
        let upca = UPCA::new("1234561234589");

        assert_eq!(upca.err().unwrap(), Error::Length)
    }

    #[test]
    fn invalid_checksum_upca() {
        let upca = UPCA::new("725272730705");

        assert_eq!(upca.err().unwrap(), Error::Checksum)
    }
    
    #[test]
    fn valid_checksum_upca() {
        let upca = UPCA::new("725272730706");

        assert!(upca.is_ok());
    }

    #[test]
    fn upce_encode() {
        let upca1 = UPCA::new("72527273070").unwrap();
        let upca2 = UPCA::new("738312014094").unwrap();
        let upca3 = UPCA::new("095421076611").unwrap();

        assert_eq!(collapse_vec(upca1.encode()), "10101110110010011011000100100110111011001001101010100010010000101110010100010011100101010000101");
        assert_eq!(collapse_vec(upca2.encode()), "10101110110111101011011101111010011001001001101010111001011001101011100111001011101001011100101");
        assert_eq!(collapse_vec(upca3.encode()), "10100011010001011011000101000110010011001100101010111001010001001010000101000011001101100110101");
    }

}
