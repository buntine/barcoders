//! This module provides types for EAN-8 barcodes, which are EAN style barcodes for smaller
//! packages on products like cigaretts, chewing gum, etc.

use ::sym::Encode;
use ::sym::Parse;
use std::ops::Range;
use std::char;

/// Encoding mappings for EAN barcodes.
/// 1 = bar, 0 = no bar.
///
/// The indices are:
/// * Left side (4 digits)
/// * Right side (4 digits)
pub const EAN8_ENCODINGS: [[&'static str; 10]; 2] = [
    ["0001101", "0011001", "0010011", "0111101", "0100011",
     "0110001", "0101111", "0111011", "0110111", "0001011",],
    ["1110010", "1100110", "1101100", "1000010", "1011100",
     "1001110", "1010000", "1000100", "1001000", "1110100",],
];

/// The patterns for the guards. These are the separators that often stick down when
/// a barcode is printed.
pub const EAN8_GUARDS: [&'static str; 3] = [
    "101",   // Left.
    "01010", // Middle.
    "101",   // Right.
];

/// The EAN-8 barcode type.
pub struct EAN8 {
    data: Vec<u32>,
}

impl EAN8 {
    /// Creates a new barcode.
    /// Returns Result<EAN8, String> indicating parse success.
    pub fn new(data: String) -> Result<EAN8, String> {
        match EAN8::parse(data) {
            Ok(d) => {
                let digits = d.chars().map(|c| c.to_digit(10).expect("Unknown character")).collect();
                Ok(EAN8{data: digits})
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
                1 => { evens += *d }
                _ => { odds += *d }
            }
        }

        10 - (((odds * 3) + evens) % 10)
    }

    fn number_system_digits(&self) -> &[u32] {
        &self.data[0..2]
    }

    fn number_system_encoding(&self) -> String {
        self.number_system_digits().iter().map(|d| self.char_encoding(0, d)).collect()
    }

    fn checksum_encoding(&self) -> &'static str {
        self.char_encoding(1, &self.checksum_digit())
    }

    fn char_encoding(&self, side: usize, d: &u32) -> &'static str {
        EAN8_ENCODINGS[side][*d as usize]
    }

    fn left_digits(&self) -> &[u32] {
        &self.data[2..4]
    }

    fn right_digits(&self) -> &[u32] {
        &self.data[4..]
    }

    fn left_payload(&self) -> String {
        self.left_digits()
            .iter()
            .map(|d| self.char_encoding(0, &d))
            .collect::<Vec<&str>>()
            .concat()
    }

    fn right_payload(&self) -> String {
        self.right_digits()
            .iter()
            .map(|d| self.char_encoding(1, &d))
            .collect::<Vec<&str>>()
            .concat()
    }
}

impl Parse for EAN8 {
    /// Returns the valid length of data acceptable in this type of barcode.
    fn valid_len() -> Range<u32> {
        7..8
    }

    /// Returns the set of valid characters allowed in this type of barcode.
    fn valid_chars() -> Vec<char> {
        (0..10).into_iter().map(|i| char::from_digit(i, 10).unwrap()).collect()
    }
}

impl Encode for EAN8 {
    /// Encodes the barcode.
    /// Returns a String of binary digits.
    fn encode(&self) -> String {
        format!("{}{}{}{}{}{}{}", EAN8_GUARDS[0], self.number_system_encoding(), self.left_payload(),
                                  EAN8_GUARDS[1], self.right_payload(), self.checksum_encoding(), EAN8_GUARDS[2])
    }
}

#[cfg(test)]
mod tests {
    use ::sym::ean8::*;
    use ::generators::ascii::*;
    use ::sym::Encode;

    #[test]
    fn new_ean8() {
        let ean8 = EAN8::new("1234567".to_string());

        assert!(ean8.is_ok());
    }

    #[test]
    fn invalid_data_ean8() {
        let ean8 = EAN8::new("1234er123412".to_string());

        assert!(ean8.is_err());
    }

    #[test]
    fn invalid_len_ean8() {
        let ean8 = EAN8::new("1111112222222333333".to_string());

        assert!(ean8.is_err());
    }

    #[test]
    fn ean8_raw_data() {
        let ean8 = EAN8::new("1234567".to_string()).unwrap();

        assert_eq!(ean8.raw_data(), "1234567".to_string());
    }

    #[test]
    fn ean8_encode() {
        let ean81 = EAN8::new("5512345".to_string()).unwrap(); // Check digit: 7
        let ean82 = EAN8::new("9834651".to_string()).unwrap(); // Check digit: 3

        assert_eq!(ean81.encode(), "1010110001011000100110010010011010101000010101110010011101000100101".to_string());
        assert_eq!(ean82.encode(), "1010001011011011101111010100011010101010000100111011001101010000101".to_string());
    }

    #[test]
    fn ean8_checksum_calculation() {
        let ean81 = EAN8::new("4575678".to_string()).unwrap(); // Check digit: 8
        let ean82 = EAN8::new("9534763".to_string()).unwrap(); // Check digit: 9
        let eight_encoding = EAN8_ENCODINGS[1][8];
        let nine_encoding = EAN8_ENCODINGS[1][9];
        let checksum_digit1 = &ean81.encode()[57..64];
        let checksum_digit2 = &ean82.encode()[57..64];

        assert_eq!(ean81.checksum_digit(), 8);
        assert_eq!(ean82.checksum_digit(), 9);
        assert_eq!(checksum_digit1, eight_encoding);
        assert_eq!(checksum_digit2, nine_encoding);
    }

    #[test]
    fn ean8_to_ascii() {
        let ean8 = EAN8::new("1234567".to_string()).unwrap();
        let ascii = ASCII::new();

        assert_eq!(ascii.generate(&ean8), "SWAG".to_string());
    }

    #[test]
    fn ean8_to_ascii_with_large_height() {
        let ean8 = EAN8::new("1234567".to_string()).unwrap();
        let ascii = ASCII::new().height(40).xdim(2);

        assert_eq!(ascii.height, 40);
        assert_eq!(ascii.xdim, 2);
        assert_eq!(ascii.generate(&ean8), "SWAG".to_string());
    }
}
