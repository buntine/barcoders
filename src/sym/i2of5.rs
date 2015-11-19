//! This module provides types for Interleaved 2-of-5 barcodes.
//! I2of5 barcodes are often used by Airlines and in some industrial settings.
//! they also make an appearance in retail where they are sometimes used for the outer cartons on
//! groups of products (cartons of Cola, etc).

use ::sym::Parse;
use ::sym::EncodedBarcode;
use ::sym::helpers;
use std::ops::Range;
use std::char;

// TODO: Implement.
const I2OF5_ENCODINGS: [[u8; 5]; 10] = [
    [1,0,0,0,0],
    [1,0,0,0,0],
    [1,0,0,0,0],
    [1,0,0,0,0],
    [1,0,0,0,0],
    [1,0,0,0,0],
    [1,0,0,0,0],
    [1,0,0,0,0],
    [1,0,0,0,0],
    [1,0,0,0,0],
];

const I2OF5_START: [u8; 4] = [1,0,1,0];
const I2OF5_STOP: [u8; 4] = [1,1,0,1];

/// The Interleaved 2-of-5 barcode type.
pub struct I2OF5 {
    data: Vec<u8>,
}

impl I2OF5 {
    /// Creates a new barcode.
    /// If the length of the given data is odd, a checksum value will be computed and appended to
    /// the data for encoding.
    ///
    /// Returns Result<I2OF5, String> indicating parse success.
    pub fn new(data: String) -> Result<I2OF5, String> {
        match I2OF5::parse(data) {
            Ok(d) => {
                let digits: Vec<u8> = d.chars().map(|c| c.to_digit(10).expect("Unknown character") as u8).collect();
                let checksum_required = digits.len() % 2 == 1;
                let mut i2of5 = I2OF5{data: digits};

                if checksum_required {
                    let check_digit = i2of5.checksum_digit();
                    i2of5.data.push(check_digit);
                }

                Ok(i2of5)
            }
            Err(e) => Err(e),
        }
    }

    /// Returns the data as was passed into the constructor.
    pub fn raw_data(&self) -> &[u8] {
        &self.data[..]
    }

    /// Calculates the checksum digit using a weighting algorithm.
    pub fn checksum_digit(&self) -> u8 {
        let mut odds = 0;
        let mut evens = 0;

        for (i, d) in self.data.iter().enumerate() {
            match i % 2 {
                1 => { evens += *d }
                _ => { odds += *d }
            }
        }

        match 10 - (((odds * 3) + evens) % 10) {
            10    => 0,
            n @ _ => n,
        }
    }

    // TODO: Implement.
    fn payload(&self) -> Vec<u8> {
        vec![1,0,1]
    }

    /// Encodes the barcode.
    /// Returns a Vec<u8> of binary digits.
    pub fn encode(&self) -> EncodedBarcode {
        helpers::join_vecs(&[
            I2OF5_START.to_vec(), self.payload(), I2OF5_STOP.to_vec()][..])
    }
}

impl Parse for I2OF5 {
    /// Returns the valid length of data acceptable in this type of barcode.
    fn valid_len() -> Range<u32> {
        1..128
    }

    /// Returns the set of valid characters allowed in this type of barcode.
    fn valid_chars() -> Vec<char> {
        (0..10).into_iter().map(|i| char::from_digit(i, 10).unwrap()).collect()
    }
}

#[cfg(test)]
mod tests {
    use ::sym::i2of5::*;
    use std::char;

    #[test]
    fn new_i2of5() {
        let i2of5 = I2OF5::new("12345679".to_string());

        assert!(i2of5.is_ok());
    }

    #[test]
    fn new_i2of5_with_checksum() {
        let i2of5 = I2OF5::new("1234567".to_string());

        assert!(i2of5.unwrap().raw_data().len() % 2 == 0);
    }

    #[test]
    fn invalid_data_i2of5() {
        let i2of5 = I2OF5::new("1234er123412".to_string());

        assert!(i2of5.is_err());
    }

    #[test]
    fn i2of5_raw_data() {
        let i2of5 = I2OF5::new("12345679".to_string()).unwrap();

        assert_eq!(i2of5.raw_data(), &[1,2,3,4,5,6,7,9]);
    }
}
