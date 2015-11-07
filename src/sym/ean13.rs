//! This module provides types for encoding UPC and EAN barcodes.
//! Specifically:
//!   * UPC-A
//!   * EAN-13
//!   * Bookland
//!   * JAN

use ::sym::Encode;
use ::sym::Parse;
use std::ops::Range;
use std::char;

/// Encoding mappings for EAN barcodes.
/// 1 = bar, 0 = no bar.
///
/// The three indices are:
/// * Left side A (odd parity).
/// * Left side B (even parity).
/// * Right side encodings.
pub const EAN13_ENCODINGS: [[&'static str; 10]; 3] = [
    ["0001101", "0011001", "0010011", "0111101", "0100011",
     "0110001", "0101111", "0111011", "0110111", "0001011",],
    ["0100111", "0110011", "0011011", "0100001", "0011101",
     "0111001", "0000101", "0010001", "0001001", "0010111",],
    ["1110010", "1100110", "1101100", "1000010", "1011100",
     "1001110", "1010000", "1000100", "1001000", "1110100",],
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

/// The patterns for the guards. These are the separators that often stick down when
/// a barcode is printed.
const EAN13_GUARDS: [&'static str; 3] = [
    "101",   // Left.
    "01010", // Middle.
    "101",   // Right.
];

/// The EAN-13 barcode type.
pub struct EAN13 {
    data: Vec<u32>,
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
                let digits = d.chars().map(|c| c.to_digit(10).expect("Unknown character")).collect();
                Ok(EAN13{data: digits})
            }
            Err(e) => Err(e),
        }
    }

    /// Returns the data as was passed into the constructor.
    pub fn raw_data(&self) -> String {
        self.data.iter().map(|d| char::from_digit(*d, 10).unwrap()).collect::<String>()
    }

    /// Calculates the checksum digit using a weighting algorithm.
    pub fn checksum_digit(&self) -> u32 {
        let mut odds = 0;
        let mut evens = 0;

        for (i, d) in self.data.iter().enumerate() {
            match i % 2 {
                1 => { odds += *d }
                _ => { evens += *d }
            }
        }

        10 - (((odds * 3) + evens) % 10)
    }

    fn number_system_digit(&self) -> u32 {
        self.data[1]
    }

    fn number_system_encoding(&self) -> &'static str {
        self.char_encoding(0, &self.number_system_digit())
    }

    fn checksum_encoding(&self) -> &'static str {
        self.char_encoding(2, &self.checksum_digit())
    }

    fn char_encoding(&self, side: usize, d: &u32) -> &'static str {
        EAN13_ENCODINGS[side][*d as usize]
    }

    fn left_digits(&self) -> &[u32] {
        &self.data[2..7]
    }

    fn right_digits(&self) -> &[u32] {
        &self.data[7..]
    }

    fn parity_mapping(&self) -> [usize; 5] {
        PARITY[self.data[0] as usize]
    }

    fn left_payload(&self) -> String {
        self.left_digits()
            .iter()
            .zip(self.parity_mapping().iter())
            .map(|d| self.char_encoding(*d.1, &d.0))
            .collect::<Vec<&str>>()
            .concat()
    }

    fn right_payload(&self) -> String {
        self.right_digits()
            .iter()
            .map(|d| self.char_encoding(2, &d))
            .collect::<Vec<&str>>()
            .concat()
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
    /// Returns a String.
    fn encode(&self) -> String {
        format!("{}{}{}{}{}{}{}", EAN13_GUARDS[0], self.number_system_encoding(), self.left_payload(),
                                  EAN13_GUARDS[1], self.right_payload(), self.checksum_encoding(), EAN13_GUARDS[2])
    }
}

#[cfg(test)]
mod tests {
    use ::sym::ean13::*;
    use ::generators::ascii::*;
    use ::sym::Encode;

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

        assert_eq!(ean13.raw_data(), "123456123456".to_string());
    }

    #[test]
    fn ean13_encode_as_upca() {
        let ean131 = UPCA::new("012345612345".to_string()).unwrap(); // Check digit: 8
        let ean132 = UPCA::new("000118999561".to_string()).unwrap(); // Check digit: 3

        assert_eq!(ean131.encode(), "10100110010010011011110101000110110001010111101010110011011011001000010101110010011101001000101".to_string());
        assert_eq!(ean132.encode(), "10100011010001101001100100110010110111000101101010111010011101001001110101000011001101000010101".to_string());
    }

    #[test]
    fn ean13_encode_as_bookland() {
        let bookland1 = Bookland::new("978345612345".to_string()).unwrap(); // Check digit: 5
        let bookland2 = Bookland::new("978118999561".to_string()).unwrap(); // Check digit: 5

        assert_eq!(bookland1.encode(), "10101110110001001010000101000110111001010111101010110011011011001000010101110010011101001110101".to_string());
        assert_eq!(bookland2.encode(), "10101110110001001011001100110010001001000101101010111010011101001001110101000011001101001110101".to_string());
    }

    #[test]
    fn ean13_encode() {
        let ean131 = EAN13::new("750103131130".to_string()).unwrap(); // Check digit: 5
        let ean132 = EAN13::new("983465123499".to_string()).unwrap(); // Check digit: 5

        assert_eq!(ean131.encode(), "10101100010100111001100101001110111101011001101010100001011001101100110100001011100101110100101".to_string());
        assert_eq!(ean132.encode(), "10101101110100001001110101011110111001001100101010110110010000101011100111010011101001000010101".to_string());
    }

    #[test]
    fn ean13_as_upca_checksum_calculation() {
        let ean131 = UPCA::new("003600029145".to_string()).unwrap(); // Check digit: 2
        let ean132 = UPCA::new("012345612345".to_string()).unwrap(); // Check digit: 8
        let two_encoding = EAN13_ENCODINGS[2][2];
        let eight_encoding = EAN13_ENCODINGS[2][8];
        let checksum_digit1 = &ean131.encode()[85..92];
        let checksum_digit2 = &ean132.encode()[85..92];

        assert_eq!(ean131.checksum_digit(), 2);
        assert_eq!(ean132.checksum_digit(), 8);
        assert_eq!(checksum_digit1, two_encoding);
        assert_eq!(checksum_digit2, eight_encoding);
    }

    #[test]
    fn ean13_as_bookland_checksum_calculation() {
        let bookland1 = Bookland::new("978600029145".to_string()).unwrap(); // Check digit: 7
        let bookland2 = Bookland::new("978345612345".to_string()).unwrap(); // Check digit: 5
        let seven_encoding = EAN13_ENCODINGS[2][7];
        let five_encoding = EAN13_ENCODINGS[2][5];
        let checksum_digit1 = &bookland1.encode()[85..92];
        let checksum_digit2 = &bookland2.encode()[85..92];

        assert_eq!(bookland1.checksum_digit(), 7);
        assert_eq!(bookland2.checksum_digit(), 5);
        assert_eq!(checksum_digit1, seven_encoding);
        assert_eq!(checksum_digit2, five_encoding);
    }

    #[test]
    fn ean13_checksum_calculation() {
        let ean131 = EAN13::new("457567816412".to_string()).unwrap(); // Check digit: 6
        let ean132 = EAN13::new("953476324586".to_string()).unwrap(); // Check digit: 2
        let six_encoding = EAN13_ENCODINGS[2][6];
        let two_encoding = EAN13_ENCODINGS[2][2];
        let checksum_digit1 = &ean131.encode()[85..92];
        let checksum_digit2 = &ean132.encode()[85..92];

        assert_eq!(ean131.checksum_digit(), 6);
        assert_eq!(ean132.checksum_digit(), 2);
        assert_eq!(checksum_digit1, six_encoding);
        assert_eq!(checksum_digit2, two_encoding);
    }

    #[test]
    fn ean13_to_ascii() {
        let ean13 = EAN13::new("123456123456".to_string()).unwrap();
        let ascii = ASCII::new();

        assert_eq!(ascii.generate(&ean13), "SWAG".to_string());
    }

    #[test]
    fn ean13_to_ascii_with_large_height() {
        let ean13 = EAN13::new("123456123456".to_string()).unwrap();
        let ascii = ASCII::new().height(40).xdim(2);

        assert_eq!(ascii.height, 40);
        assert_eq!(ascii.xdim, 2);
        assert_eq!(ascii.generate(&ean13), "SWAG".to_string());
    }
}
