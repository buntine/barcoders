//! This module provides types for encoding UPC and EAN barcodes.
//! EAN13 barcodes are very common in retail. 90% of the products you purchase from a supermarket
//! will use EAN13.
//!
//! This module defines types for:
//!   * UPC-A
//!   * EAN-13
//!   * Bookland
//!   * JAN

use ::sym::Encode;
use ::sym::Parse;
use ::sym::EncodedBarcode;
use std::ops::Range;
use std::char;

/// Encoding mappings for EAN barcodes.
/// 1 = bar, 0 = no bar.
///
/// The three indices are:
/// * Left side A (odd parity).
/// * Left side B (even parity).
/// * Right side encodings.
pub const EAN13_ENCODINGS: [[[u8; 7]; 10]; 3] = [
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
    [0,0,0,0,0],
    [0,1,0,1,1],
    [0,1,1,0,1],
    [0,1,1,1,0],
    [1,0,0,1,1],
    [1,1,0,0,1],
    [1,1,1,0,0],
    [1,0,1,0,1],
    [1,0,1,1,0],
    [1,1,0,1,0],
];

/// The patterns for the guards. These are the separators that often stick down when
/// a barcode is printed.
pub const EAN13_LEFT_GUARD: [u8; 3] = [1,0,1];
pub const EAN13_MIDDLE_GUARD: [u8; 5] = [0,1,0,1,0];
pub const EAN13_RIGHT_GUARD: [u8; 3] = [1,0,1];

/// The EAN-13 barcode type.
pub struct EAN13 {
    data: Vec<u8>,
}

/// The Bookland barcode type.
/// Bookland are EAN-13 that use number system 978.
pub type Bookland = EAN13;

/// The UPC-A barcode type.
/// UPC-A are EAN-13 that start with a 0.
pub type UPCA = EAN13;

/// The JAN barcode type.
/// JAN are EAN-13 that use number system of 49.
pub type JAN = EAN13;

impl EAN13 {
    /// Creates a new barcode.
    /// Returns Result<EAN13, String> indicating parse success.
    pub fn new(data: String) -> Result<EAN13, String> {
        match EAN13::parse(data) {
            Ok(d) => {
                let digits = d.chars().map(|c| c.to_digit(10).expect("Unknown character") as u8).collect();
                Ok(EAN13{data: digits})
            }
            Err(e) => Err(e),
        }
    }

    /// Returns the data as was passed into the constructor.
    pub fn raw_data(&self) -> &[u8] {
        &self.data[..]
    }

    /// Calculates the checksum digit using a modulo-10 weighting algorithm.
    pub fn checksum_digit(&self) -> u8 {
        let mut odds = 0;
        let mut evens = 0;

        for (i, d) in self.data.iter().enumerate() {
            match i % 2 {
                1 => { odds += *d }
                _ => { evens += *d }
            }
        }

        match 10 - (((odds * 3) + evens) % 10) {
            10    => 0,
            n @ _ => n,
        }
    }

    fn number_system_digit(&self) -> u8 {
        self.data[1]
    }

    fn number_system_encoding(&self) -> Vec<u8> {
        self.char_encoding(0, &self.number_system_digit()).to_vec()
    }

    fn checksum_encoding(&self) -> Vec<u8> {
        self.char_encoding(2, &self.checksum_digit()).to_vec()
    }

    fn char_encoding(&self, side: usize, d: &u8) -> [u8; 7] {
        EAN13_ENCODINGS[side][*d as usize]
    }

    fn left_digits(&self) -> &[u8] {
        &self.data[2..7]
    }

    fn right_digits(&self) -> &[u8] {
        &self.data[7..]
    }

    fn parity_mapping(&self) -> [usize; 5] {
        PARITY[self.data[0] as usize]
    }

    fn left_payload(&self) -> Vec<u8> {
        let slices: Vec<[u8; 7]> = self.left_digits()
            .iter()
            .zip(self.parity_mapping().iter())
            .map(|d| self.char_encoding(*d.1, &d.0))
            .collect();

        slices.iter().flat_map(|e| e.iter()).cloned().collect()
    }

    fn right_payload(&self) -> Vec<u8> {
        let slices: Vec<[u8; 7]> = self.right_digits()
            .iter()
            .map(|d| self.char_encoding(2, &d))
            .collect();

        slices.iter().flat_map(|e| e.iter()).cloned().collect()
    }
}

impl Parse for EAN13 {
    /// Returns the valid length of data acceptable in this type of barcode.
    fn valid_len() -> Range<u32> {
        12..13
    }

    /// Returns the set of valid characters allowed in this type of barcode.
    fn valid_chars() -> Vec<char> {
        (0..10).into_iter().map(|i| char::from_digit(i, 10).unwrap()).collect()
    }
}

impl Encode for EAN13 {
    /// Encodes the barcode.
    /// Returns a Vec<u8> of binary digits.
    fn encode(&self) -> EncodedBarcode {
        self.join_vecs(&[EAN13_LEFT_GUARD.to_vec(), self.number_system_encoding(), self.left_payload(),
                         EAN13_MIDDLE_GUARD.to_vec(), self.right_payload(), self.checksum_encoding(),
                         EAN13_RIGHT_GUARD.to_vec()][..])
    }
}

#[cfg(test)]
mod tests {
    use ::sym::ean13::*;
    use ::sym::Encode;
    use std::char;

    fn collapse_vec(v: Vec<u8>) -> String {
        let chars = v.iter().map(|d| char::from_digit(*d as u32, 10).unwrap());
        chars.collect()
    }

    #[test]
    fn new_ean13() {
        let ean13 = EAN13::new("123456123456".to_string());

        assert!(ean13.is_ok());
    }

    #[test]
    fn new_bookland() {
        let bookland = Bookland::new("978456123456".to_string());

        assert!(bookland.is_ok());
    }

    #[test]
    fn invalid_data_ean13() {
        let ean13 = EAN13::new("1234er123412".to_string());

        assert!(ean13.is_err());
    }

    #[test]
    fn invalid_len_ean13() {
        let ean13 = EAN13::new("1111112222222333333".to_string());

        assert!(ean13.is_err());
    }

    #[test]
    fn ean13_raw_data() {
        let ean13 = EAN13::new("123456123456".to_string()).unwrap();

        assert_eq!(ean13.raw_data(), &[1,2,3,4,5,6,1,2,3,4,5,6]);
    }

    #[test]
    fn ean13_encode_as_upca() {
        let ean131 = UPCA::new("012345612345".to_string()).unwrap(); // Check digit: 8
        let ean132 = UPCA::new("000118999561".to_string()).unwrap(); // Check digit: 3

        assert_eq!(collapse_vec(ean131.encode()), "10100110010010011011110101000110110001010111101010110011011011001000010101110010011101001000101".to_string());
        assert_eq!(collapse_vec(ean132.encode()), "10100011010001101001100100110010110111000101101010111010011101001001110101000011001101000010101".to_string());
    }

    #[test]
    fn ean13_encode_as_bookland() {
        let bookland1 = Bookland::new("978345612345".to_string()).unwrap(); // Check digit: 5
        let bookland2 = Bookland::new("978118999561".to_string()).unwrap(); // Check digit: 5

        assert_eq!(collapse_vec(bookland1.encode()), "10101110110001001010000101000110111001010111101010110011011011001000010101110010011101001110101".to_string());
        assert_eq!(collapse_vec(bookland2.encode()), "10101110110001001011001100110010001001000101101010111010011101001001110101000011001101001110101".to_string());
    }

    #[test]
    fn ean13_encode() {
        let ean131 = EAN13::new("750103131130".to_string()).unwrap(); // Check digit: 5
        let ean132 = EAN13::new("983465123499".to_string()).unwrap(); // Check digit: 5

        assert_eq!(collapse_vec(ean131.encode()), "10101100010100111001100101001110111101011001101010100001011001101100110100001011100101110100101".to_string());
        assert_eq!(collapse_vec(ean132.encode()), "10101101110100001001110101011110111001001100101010110110010000101011100111010011101001000010101".to_string());
    }

    #[test]
    fn ean13_as_upca_checksum_calculation() {
        let ean131 = UPCA::new("003600029145".to_string()).unwrap(); // Check digit: 2
        let ean132 = UPCA::new("012345612345".to_string()).unwrap(); // Check digit: 8

        assert_eq!(ean131.checksum_digit(), 2);
        assert_eq!(ean132.checksum_digit(), 8);
    }

    #[test]
    fn ean13_as_bookland_checksum_calculation() {
        let bookland1 = Bookland::new("978600029145".to_string()).unwrap(); // Check digit: 7
        let bookland2 = Bookland::new("978345612345".to_string()).unwrap(); // Check digit: 5

        assert_eq!(bookland1.checksum_digit(), 7);
        assert_eq!(bookland2.checksum_digit(), 5);
    }

    #[test]
    fn ean13_checksum_calculation() {
        let ean131 = EAN13::new("457567816412".to_string()).unwrap(); // Check digit: 6
        let ean132 = EAN13::new("953476324586".to_string()).unwrap(); // Check digit: 2

        assert_eq!(ean131.checksum_digit(), 6);
        assert_eq!(ean132.checksum_digit(), 2);
    }
}
