//! Encoders for supplemental 2-digit and 5-digit EAN barcodes.
//!
//! EAN-2 barcodes are used in magazines and newspapers to indicate issue number.
//!
//! EAN-5 barcodes are often used to indicate the suggested retail price of books.
//!
//! These supplemental barcodes never appear without a full EAN-13 barcode alongside them.

use sym::Parse;
use error::{Error, Result};
use sym::ean13::ENCODINGS;
use sym::helpers;
use std::ops::Range;
use std::char;

const LEFT_GUARD: [u8; 4] = [1, 0, 1, 1];

/// Maps parity (odd/even) for the EAN-5 barcodes based on the check digit.
const EAN5_PARITY: [[usize; 5]; 10] = [
    [0,0,1,1,1], [1,0,1,0,0], [1,0,0,1,0],
    [1,0,0,0,1], [0,1,1,0,0], [0,0,1,1,0],
    [0,0,0,1,1], [0,1,0,1,0], [0,1,0,0,1],
    [0,0,1,0,1],
];

/// Maps parity (odd/even) for the EAN-2 barcodes based on the check digit.
const EAN2_PARITY: [[usize; 5]; 4] = [
    [0,0,0,0,0], [0,1,0,0,0], [1,0,0,0,0],
    [1,1,0,0,0],
];

/// The Supplemental EAN barcode type.
#[derive(Debug)]
pub enum EANSUPP {
    /// EAN-2 supplemental barcode type.
    EAN2(Vec<u8>),
    /// EAN-5 supplemental barcode type.
    EAN5(Vec<u8>),
}

impl EANSUPP {
    /// Creates a new barcode.
    /// Returns Result<EANSUPP, Error> indicating parse success.
    /// Either a EAN2 or EAN5 variant will be returned depending on
    /// the length of `data`.
    pub fn new<T: AsRef<str>>(data: T) -> Result<EANSUPP> {
        EANSUPP::parse(data.as_ref()).and_then(|d| {
            let digits: Vec<u8> = d.chars()
                                   .map(|c| c.to_digit(10).expect("Unknown character") as u8)
                                   .collect();

            match digits.len() {
                2 => Ok(EANSUPP::EAN2(digits)),
                5 => Ok(EANSUPP::EAN5(digits)),
                _ => Err(Error::Length),
            }
        })
    }

    fn raw_data(&self) -> &[u8] {
        match *self {
            EANSUPP::EAN2(ref d) |
            EANSUPP::EAN5(ref d) => &d[..],
        }
    }

    fn char_encoding(&self, side: usize, d: &u8) -> [u8; 7] {
        ENCODINGS[side][*d as usize]
    }

    /// Calculates the checksum digit using a modified modulo-10 weighting
    /// algorithm. This only makes sense for EAN5 barcodes.
    pub fn checksum_digit(&self) -> u8 {
        let mut odds = 0;
        let mut evens = 0;
        let data = self.raw_data();

        for (i, d) in data.iter().enumerate() {
            match i % 2 {
                1 => evens += *d,
                _ => odds += *d,
            }
        }

        match ((odds * 3) + (evens * 9)) % 10 {
            10 => 0,
            n => n,
        }
    }

    fn parity(&self) -> [usize; 5] {
        match *self {
            EANSUPP::EAN2(ref d) => {
                let modulo = ((d[0] * 10) + d[1]) % 4;
                EAN2_PARITY[modulo as usize]
            }
            EANSUPP::EAN5(ref _d) => {
                let check = self.checksum_digit() as usize;
                EAN5_PARITY[check]
            }
        }
    }

    fn payload(&self) -> Vec<u8> {
        let mut p = vec![];
        let slices: Vec<[u8; 7]> = self.raw_data()
                                       .iter()
                                       .zip(self.parity().iter())
                                       .map(|(d, s)| self.char_encoding(*s, &d))
                                       .collect();

        for (i, d) in slices.iter().enumerate() {
            if i > 0 {
                p.push(0);
                p.push(1);
            }

            p.extend(d.iter().cloned());
        }

        p
    }

    /// Encodes the barcode.
    /// Returns a Vec<u8> of binary digits.
    pub fn encode(&self) -> Vec<u8> {
        helpers::join_slices(&[&LEFT_GUARD[..], &self.payload()[..]][..])
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
    use sym::ean_supp::*;
    use error::Error;
    use std::char;

    fn collapse_vec(v: Vec<u8>) -> String {
        let chars = v.iter().map(|d| char::from_digit(*d as u32, 10).unwrap());
        chars.collect()
    }

    #[test]
    fn new_ean2() {
        let ean2 = EANSUPP::new("12".to_owned());

        assert!(ean2.is_ok());
    }

    #[test]
    fn new_ean5() {
        let ean5 = EANSUPP::new("12345".to_owned());

        assert!(ean5.is_ok());
    }

    #[test]
    fn invalid_data_ean2() {
        let ean2 = EANSUPP::new("AT".to_owned());

        assert_eq!(ean2.err().unwrap(), Error::Character);
    }

    #[test]
    fn invalid_len_ean2() {
        let ean2 = EANSUPP::new("123".to_owned());

        assert_eq!(ean2.err().unwrap(), Error::Length);
    }

    #[test]
    fn ean2_encode() {
        let ean21 = EANSUPP::new("34".to_owned()).unwrap();

        assert_eq!(collapse_vec(ean21.encode()),
                   "10110100001010100011".to_owned());
    }

    #[test]
    fn ean5_encode() {
        let ean51 = EANSUPP::new("51234".to_owned()).unwrap();

        assert_eq!(collapse_vec(ean51.encode()),
                   "10110110001010011001010011011010111101010011101".to_owned());
    }

}
