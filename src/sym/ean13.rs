//! Encoder for UPC and EAN barcodes.
//!
//! EAN13 barcodes are very common in retail. 90% of the products you purchase from a supermarket
//! will use EAN13.
//!
//! This module defines types for:
//!   * UPC-A
//!   * EAN-13
//!   * Bookland
//!   * JAN

use sym::{Parse, helpers};
use error::Result;
use std::ops::Range;
use std::char;

/// Encoding mappings for EAN barcodes.
/// 1 = bar, 0 = no bar.
///
/// The three indices are:
/// * Left side A (odd parity).
/// * Left side B (even parity).
/// * Right side encodings.
pub const ENCODINGS: [[[u8; 7]; 10]; 3] = [
    [[0,0,0,1,1,0,1], [0,0,1,1,0,0,1], [0,0,1,0,0,1,1], [0,1,1,1,1,0,1], [0,1,0,0,0,1,1],
     [0,1,1,0,0,0,1], [0,1,0,1,1,1,1], [0,1,1,1,0,1,1], [0,1,1,0,1,1,1], [0,0,0,1,0,1,1],],
    [[0,1,0,0,1,1,1], [0,1,1,0,0,1,1], [0,0,1,1,0,1,1], [0,1,0,0,0,0,1], [0,0,1,1,1,0,1],
     [0,1,1,1,0,0,1], [0,0,0,0,1,0,1], [0,0,1,0,0,0,1], [0,0,0,1,0,0,1], [0,0,1,0,1,1,1],],
    [[1,1,1,0,0,1,0], [1,1,0,0,1,1,0], [1,1,0,1,1,0,0], [1,0,0,0,0,1,0], [1,0,1,1,1,0,0],
     [1,0,0,1,1,1,0], [1,0,1,0,0,0,0], [1,0,0,0,1,0,0], [1,0,0,1,0,0,0], [1,1,1,0,1,0,0],],
];

/// Maps parity (odd/even) for the left-side digits based on the first digit in
/// the number system portion of the barcode data.
const PARITY: [[usize; 5]; 10] = [
    [0,0,0,0,0], [0,1,0,1,1], [0,1,1,0,1],
    [0,1,1,1,0], [1,0,0,1,1], [1,1,0,0,1],
    [1,1,1,0,0], [1,0,1,0,1], [1,0,1,1,0],
    [1,1,0,1,0],
];

/// The left-hand guard pattern.
pub const LEFT_GUARD: [u8; 3] = [1, 0, 1];
/// The middle guard pattern.
pub const MIDDLE_GUARD: [u8; 5] = [0, 1, 0, 1, 0];
/// The right-hand guard pattern.
pub const RIGHT_GUARD: [u8; 3] = [1, 0, 1];

/// The EAN-13 barcode type.
#[derive(Debug)]
pub struct EAN13(Vec<u8>);

/// The Bookland barcode type.
/// Bookland are EAN-13 that use number system 978.
pub type Bookland = EAN13;

/// The UPC-A barcode type.
/// UPC-A are EAN-13 that start with a 0.
pub type UPCA = EAN13;

/// The JAN barcode type.
/// JAN are EAN-13 that use number system 49.
pub type JAN = EAN13;

impl EAN13 {
    /// Creates a new barcode.
    /// Returns Result<EAN13, Error> indicating parse success.
    pub fn new<T: AsRef<str>>(data: T) -> Result<EAN13> {
        EAN13::parse(data.as_ref()).and_then(|d| {
            let digits = d.chars()
                          .map(|c| c.to_digit(10).expect("Unknown character") as u8)
                          .collect();
            Ok(EAN13(digits))
        })
    }

    /// Calculates the checksum digit using a modulo-10 weighting algorithm.
    fn checksum_digit(&self) -> u8 {
        helpers::modulo_10_checksum(&self.0[..], true)
    }

    fn number_system_digit(&self) -> u8 {
        self.0[1]
    }

    fn number_system_encoding(&self) -> [u8; 7] {
        self.char_encoding(0, self.number_system_digit())
    }

    fn checksum_encoding(&self) -> [u8; 7] {
        self.char_encoding(2, self.checksum_digit())
    }

    fn char_encoding(&self, side: usize, d: u8) -> [u8; 7] {
        ENCODINGS[side][d as usize]
    }

    fn left_digits(&self) -> &[u8] {
        &self.0[2..7]
    }

    fn right_digits(&self) -> &[u8] {
        &self.0[7..]
    }

    fn parity_mapping(&self) -> [usize; 5] {
        PARITY[self.0[0] as usize]
    }

    fn left_payload(&self) -> Vec<u8> {
        let slices: Vec<[u8; 7]> = self.left_digits()
                                       .iter()
                                       .zip(self.parity_mapping().iter())
                                       .map(|(d, s)| self.char_encoding(*s, *d))
                                       .collect();

        helpers::join_iters(slices.iter())
    }

    fn right_payload(&self) -> Vec<u8> {
        let slices: Vec<[u8; 7]> = self.right_digits()
                                       .iter()
                                       .map(|d| self.char_encoding(2, *d))
                                       .collect();

        helpers::join_iters(slices.iter())
    }

    /// Encodes the barcode.
    /// Returns a Vec<u8> of binary digits.
    pub fn encode(&self) -> Vec<u8> {
        helpers::join_slices(&[&LEFT_GUARD[..],
                               &self.number_system_encoding()[..],
                               &self.left_payload()[..],
                               &MIDDLE_GUARD[..],
                               &self.right_payload()[..],
                               &self.checksum_encoding()[..],
                               &RIGHT_GUARD[..]][..])
    }
}

impl Parse for EAN13 {
    /// Returns the valid length of data acceptable in this type of barcode.
    fn valid_len() -> Range<u32> {
        12..13
    }

    /// Returns the set of valid characters allowed in this type of barcode.
    fn valid_chars() -> Vec<char> {
        (0..10).map(|i| char::from_digit(i, 10).unwrap()).collect()
    }
}

#[cfg(test)]
mod tests {
    use ::sym::ean13::*;
    use std::char;
    use error::Error;

    fn collapse_vec(v: Vec<u8>) -> String {
        let chars = v.iter().map(|d| char::from_digit(*d as u32, 10).unwrap());
        chars.collect()
    }

    #[test]
    fn new_ean13() {
        let ean13 = EAN13::new("123456123456");

        assert!(ean13.is_ok());
    }

    #[test]
    fn new_bookland() {
        let bookland = Bookland::new("978456123456");

        assert!(bookland.is_ok());
    }

    #[test]
    fn invalid_data_ean13() {
        let ean13 = EAN13::new("1234er123412");

        assert_eq!(ean13.err().unwrap(), Error::Character)
    }

    #[test]
    fn invalid_len_ean13() {
        let ean13 = EAN13::new("1111112222222333333");

        assert_eq!(ean13.err().unwrap(), Error::Length)
    }

    #[test]
    fn ean13_encode_as_upca() {
        let ean131 = UPCA::new("012345612345").unwrap(); // Check digit: 8
        let ean132 = UPCA::new("000118999561").unwrap(); // Check digit: 3

        assert_eq!(collapse_vec(ean131.encode()), "10100110010010011011110101000110110001010111101010110011011011001000010101110010011101001000101");
        assert_eq!(collapse_vec(ean132.encode()), "10100011010001101001100100110010110111000101101010111010011101001001110101000011001101000010101");
    }

    #[test]
    fn ean13_encode_as_bookland() {
        let bookland1 = Bookland::new("978345612345").unwrap(); // Check digit: 5
        let bookland2 = Bookland::new("978118999561").unwrap(); // Check digit: 5

        assert_eq!(collapse_vec(bookland1.encode()), "10101110110001001010000101000110111001010111101010110011011011001000010101110010011101001110101");
        assert_eq!(collapse_vec(bookland2.encode()), "10101110110001001011001100110010001001000101101010111010011101001001110101000011001101001110101");
    }

    #[test]
    fn ean13_encode() {
        let ean131 = EAN13::new("750103131130").unwrap(); // Check digit: 5
        let ean132 = EAN13::new("983465123499").unwrap(); // Check digit: 5

        assert_eq!(collapse_vec(ean131.encode()), "10101100010100111001100101001110111101011001101010100001011001101100110100001011100101110100101");
        assert_eq!(collapse_vec(ean132.encode()), "10101101110100001001110101011110111001001100101010110110010000101011100111010011101001000010101");
    }
}
