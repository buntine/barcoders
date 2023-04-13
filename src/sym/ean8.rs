//! Encoder for EAN-8 barcodes.
//!
//! EAN-8 barcodes are EAN style barcodes for smaller packages on products like
//! cigaretts, chewing gum, etc where package space is limited.

use crate::error::Result;
use crate::sym::ean13::{ENCODINGS, LEFT_GUARD, MIDDLE_GUARD, RIGHT_GUARD};
use crate::sym::{helpers, Parse};
use std::char;
use std::ops::Range;

/// The EAN-8 barcode type.
#[derive(Debug)]
pub struct EAN8(Vec<u8>);

impl EAN8 {
    /// Creates a new barcode.
    /// Returns Result<EAN8, String> indicating parse success.
    pub fn new<T: AsRef<str>>(data: T) -> Result<EAN8> {
        EAN8::parse(data.as_ref()).map(|d| {
            let digits = d
                .chars()
                .map(|c| c.to_digit(10).expect("Unknown character") as u8)
                .collect();
            EAN8(digits)
        })
    }

    /// Calculates the checksum digit using a weighting algorithm.
    fn checksum_digit(&self) -> u8 {
        helpers::modulo_10_checksum(&self.0[..], false)
    }

    fn number_system_digits(&self) -> &[u8] {
        &self.0[0..2]
    }

    fn number_system_encoding(&self) -> Vec<u8> {
        let mut ns = vec![];

        for d in self.number_system_digits() {
            ns.extend(self.char_encoding(0, *d).iter().cloned());
        }

        ns
    }

    fn checksum_encoding(&self) -> [u8; 7] {
        self.char_encoding(2, self.checksum_digit())
    }

    fn char_encoding(&self, side: usize, d: u8) -> [u8; 7] {
        ENCODINGS[side][d as usize]
    }

    fn left_digits(&self) -> &[u8] {
        &self.0[2..4]
    }

    fn right_digits(&self) -> &[u8] {
        &self.0[4..]
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
            .map(|d| self.char_encoding(2, *d))
            .collect();

        helpers::join_iters(slices.iter())
    }

    /// Encodes the barcode.
    /// Returns a Vec<u8> of binary digits.
    pub fn encode(&self) -> Vec<u8> {
        helpers::join_slices(
            &[
                &LEFT_GUARD[..],
                &self.number_system_encoding()[..],
                &self.left_payload()[..],
                &MIDDLE_GUARD[..],
                &self.right_payload()[..],
                &self.checksum_encoding()[..],
                &RIGHT_GUARD[..],
            ][..],
        )
    }
}

impl Parse for EAN8 {
    /// Returns the valid length of data acceptable in this type of barcode.
    fn valid_len() -> Range<u32> {
        7..8
    }

    /// Returns the set of valid characters allowed in this type of barcode.
    fn valid_chars() -> Vec<char> {
        (0..10).map(|i| char::from_digit(i, 10).unwrap()).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Error;
    use crate::sym::ean8::*;
    use std::char;

    fn collapse_vec(v: Vec<u8>) -> String {
        let chars = v.iter().map(|d| char::from_digit(*d as u32, 10).unwrap());
        chars.collect()
    }

    #[test]
    fn new_ean8() {
        let ean8 = EAN8::new("1234567");

        assert!(ean8.is_ok());
    }

    #[test]
    fn invalid_data_ean8() {
        let ean8 = EAN8::new("1234er1");

        assert_eq!(ean8.err().unwrap(), Error::Character);
    }

    #[test]
    fn invalid_len_ean8() {
        let ean8 = EAN8::new("1111112222222333333");

        assert_eq!(ean8.err().unwrap(), Error::Length);
    }

    #[test]
    fn ean8_encode() {
        let ean81 = EAN8::new("5512345").unwrap(); // Check digit: 7
        let ean82 = EAN8::new("9834651").unwrap(); // Check digit: 3

        assert_eq!(
            collapse_vec(ean81.encode()),
            "1010110001011000100110010010011010101000010101110010011101000100101"
        );
        assert_eq!(
            collapse_vec(ean82.encode()),
            "1010001011011011101111010100011010101010000100111011001101010000101"
        );
    }
}
