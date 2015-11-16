//! This module provides types for encoding supplemental 2-digit and 5-digit EAN barcodes.
//! Supplemental EAN-2 barcodes are used in magazines and newspapers to indicate issue number and
//! EAN-5 barcodes are often used to indicate the suggested retail price of books.

use ::sym::Parse;
use ::sym::EncodedBarcode;
use ::sym::ean13::EAN_ENCODINGS;
use ::sym::helpers;
use std::ops::Range;
use std::char;

pub const EANSUPP_LEFT_GUARD: [u8; 4] = [1,0,1,1];

/// The Supplemental EAN barcode type.
pub enum EANSUPP {
    EAN2 {
        data: Vec<u8>,
    },
    EAN5 {
        data: Vec<u8>,
    },
}

impl EANSUPP {
    /// Creates a new barcode.
    /// Returns Result<EANSUPP, String> indicating parse success.
    /// Either a EAN2 or EAN5 variant will be returned depending on
    /// the length of `data`.
    pub fn new(data: String) -> Result<EANSUPP, String> {
        match EANSUPP::parse(data) {
            Ok(d) => {
                let digits: Vec<u8> = d.chars().map(|c| c.to_digit(10).expect("Unknown character") as u8).collect();

                match digits.len() {
                    2 => Ok(EANSUPP::EAN2{data: digits}),
                    5 => Ok(EANSUPP::EAN5{data: digits}),
                    n @ _ => Err(format!("Invalid supplemental length: {}", n)),
                }
            }
            Err(e) => Err(e),
        }
    }

    /// Returns the data as was passed into the constructor.
    pub fn raw_data(&self) -> &[u8] {
        match *self {
            EANSUPP::EAN2{data: ref d} => &d[..],
            EANSUPP::EAN5{data: ref d} => &d[..],
        }
    }

    fn payload(&self) -> Vec<u8> {
        vec![1,1,0]
    }

    /// Encodes the barcode.
    /// Returns a Vec<u8> of binary digits.
    pub fn encode(&self) -> EncodedBarcode {
        helpers::join_vecs(&[
            EANSUPP_LEFT_GUARD.to_vec(), self.payload()][..])
    }
}

impl Parse for EANSUPP {
    /// Returns the valid length of data acceptable in this type of barcode.
    fn valid_len() -> Range<u32> {
        2..5
    }

    /// Returns the set of valid characters allowed in this type of barcode.
    fn valid_chars() -> Vec<char> {
        (0..10).into_iter().map(|i| char::from_digit(i, 10).unwrap()).collect()
    }
}

#[cfg(test)]
mod tests {
    use ::sym::ean_supp::*;
    use std::char;

    fn collapse_vec(v: Vec<u8>) -> String {
        let chars = v.iter().map(|d| char::from_digit(*d as u32, 10).unwrap());
        chars.collect()
    }

    #[test]
    fn new_ean2() {
        let ean2 = EANSUPP::new("12".to_string());

        assert!(ean2.is_ok());
    }

    #[test]
    fn new_ean5() {
        let ean5 = EANSUPP::new("12345".to_string());

        assert!(ean5.is_ok());
    }

    #[test]
    fn invalid_data_ean2() {
        let ean2 = EANSUPP::new("AT".to_string());

        assert!(ean2.is_err());
    }

    #[test]
    fn invalid_len_ean2() {
        let ean2 = EANSUPP::new("123".to_string());

        assert!(ean2.is_err());
    }

    #[test]
    fn ean2_raw_data() {
        let ean2 = EANSUPP::new("98".to_string()).unwrap();

        assert_eq!(ean2.raw_data(), &[9,8]);
    }

    #[test]
    fn ean5_raw_data() {
        let ean5 = EANSUPP::new("98567".to_string()).unwrap();

        assert_eq!(ean5.raw_data(), &[9,8,5,6,7]);
    }
}
