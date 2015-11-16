//! This module provides types for encoding supplemental 2-digit and 5-digit EAN barcodes.
//! Supplemental EAN-2 barcodes are used in magazines and newspapers to indicate issue number and
//! EAN-5 barcodes are often used to indicate the suggested retail price of books.

use ::sym::Parse;
use ::sym::EncodedBarcode;
use ::sym::ean13::EAN_ENCODINGS;
use ::sym::ean13::EAN_LEFT_GUARD;
use ::sym::ean13::EAN_MIDDLE_GUARD;
use ::sym::ean13::EAN_RIGHT_GUARD;
//use ::sym::helpers;
use std::ops::Range;
use std::char;

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
