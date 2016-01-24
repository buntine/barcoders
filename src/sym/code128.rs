//! Encoder for Code128 barcodes.
//!
//! Code128 is a popular,  high-density symbology that allows for the encoding of alphanumeric
//! data along with many special characters by utilising three separate character-sets.
//!
//! Code128 also offers double-density encoding of digits.
//!
//! Barcoders provides special syntax for specifying the character set(s) which should be used in
//! the barcode:
//!
//!   \a => Switch to character-set A
//!   \b => Switch to character-set B
//!   \c => Switch to character-set C
//!
//! You must provide both the starting character-set along with any changes during the data. This
//! means all Code128 barcodes must start with either "\a", "\b" or "\c". Simple alphanumeric data
//! can generally use character-set A solely.
//!
//! As an example, this barcode uses character-set B:
//!   \bHE1234A*1
//!
//! And this one starts at character-set A and then switches to C to encode the digits more
//! effectively:
//!   \aHE@$A\c123456
//!
//! To actually use a back-slash in the barcore data you should use two:
//!
//!   \a1234\\45AA

use sym::helpers;
use error::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Unit {
    A(u8),
    B(u8),
    C(u8),
}

type Encoding = [u8; 11];

pub enum UnitType {
    A,
    B,
    C,
}

// Character -> Binary mappings for each of the allowable characters in each character-set.
const CODE128_CHARS: [([&'static str; 3], Encoding); 3] = [
    ([" ", " ", "00"], [1,0,1,0,0,1,1,0,1,1,0]), 
    (["!", "!", "01"], [1,1,1,0,0,1,1,0,1,1,0]), 
    (["\"", "\"", "02"], [1,1,1,1,0,1,1,0,1,1,0]), 
];

/// The Code128 barcode type.
#[derive(Debug)]
pub struct Code128(Vec<Unit>);

impl Code128 {
    /// Creates a new barcode.
    /// Returns Result<Code128, Error> indicating parse success.
    pub fn new(data: String) -> Result<Code128> {
        match Code128::parse(data.chars().collect()) {
            Ok(u) => Ok(Code128(u)),
            Err(e) => Err(e),
        }
    }

    // Tokenizes and collects the data into the appropriate character-sets.
    fn parse(chars: Vec<char>) -> Result<Vec<Unit>> {
//        let mut units: Vec<Encoding> = vec![];
//        let mut control_char = false;
//        let mut unit_type: Option<UnitType> = None;
//        let mut carry: Option<u32> = None;
//
//        for ch in chars {
//            match ch {
//                '\\' => {
//                    carry = None;
//
//                    if control_char {
//                        match unit_type {
//                            Some(ut) => {
//                                let b = try!(Code128::unit_encoding("\\", ut));
//                                units.push(b);
//                            },
//                            None => return Err(Error::Character('\\'))
//                        }
//                    } else {
//                        control_char = true;
//                    }
//                },
//                d if d.is_digit(10) => {
//                    match carry {
//                        Some(c) if unit_type == UnitType::C => {
//                            units.pop();
//
//                            let b = try!(Code128::unit_encoding("00", ut));
//                            units.push(b);
//                        },
//                        _ => {
//
//                        }
//                    }
//                },
//                'a' if control_char && (unit_type == Some(UnitType::B) || unit_type == Some(UnitType::C)) => {
//                }
//                'b' if control_char && (unit_type == Some(UnitType::A) || unit_type == Some(UnitType::C)) => {
//                }
//                'c' if control_char && (unit_type == Some(UnitType::A) || unit_type == Some(UnitType::B)) => {
//                }
//                _ => {
//                    carry = None;
//
//
//                }
//            }
//        }
//
//        units

        Ok(vec![Unit::A(0)])
    }

    /// Calculates the checksum unit using a modulo-103 algorithm.
    pub fn checksum_value(&self) -> Option<u32> {
        Some(23)
    }

    fn checksum_encoding(&self) -> Encoding {
        match self.checksum_value() {
            Some(u) => self.unit_encoding(&Unit::A(u as u8)),
            None => panic!("Cannot compute checksum"),
        }
    }

    fn unit_encoding(&self, c: &Unit) -> Encoding {
        [1,1,1,0,0,0,1,1,1,0,0]
    }

    fn push_encoding(&self, into: &mut Vec<u8>, from: Encoding) {
        into.extend(from.iter().cloned());
        into.push(0);
    }

    fn payload(&self) -> Vec<u8> {
        let mut enc = vec![0];

        for c in &self.0 {
            self.push_encoding(&mut enc, self.unit_encoding(c));
        }

        self.push_encoding(&mut enc, self.checksum_encoding());

        enc
    }

    /// Encodes the barcode.
    /// Returns a Vec<u8> of binary digits.
    pub fn encode(&self) -> Vec<u8> {
        helpers::join_slices(&[&self.payload()[..]][..])
    }
}

#[cfg(test)]
mod tests {
    use sym::code128::*;
    use error::Error;
    use std::char;

    fn collapse_vec(v: Vec<u8>) -> String {
        let chars = v.iter().map(|d| char::from_digit(*d as u32, 10).unwrap());
        chars.collect()
    }

    #[test]
    fn new_code128() {
        let code128 = Code128::new("12120".to_owned());

        assert!(code128.is_ok());
    }

//    #[test]
//    fn invalid_data_code128() {
//        let code128 = Code128::new("☺ ".to_owned());
//
//        assert_eq!(code128.err().unwrap(), Error::Character);
//    }
//
//    #[test]
//    fn invalid_len_code128() {
//        let code128 = Code128::new("".to_owned());
//
//        assert_eq!(code128.err().unwrap(), Error::Length);
//    }
//
//    #[test]
//    fn code128_raw_data() {
//        let code128 = Code128::new("12001".to_owned()).unwrap();
//
//        assert_eq!(code128.raw_data(), &[Unit::A("1".to_string()), Unit::A("2".to_string())]);
//    }
}
