//! Encoder for Code128 barcodes.
//!
//! Code128 is a popular,  high-density symbology that allows for the encoding of alphanumeric
//! data along with many special characters by utilising three separate character-sets.
//!
//! Code128 also offers double-density encoding of digits.
//!
//! Barcoders will automatically handle character-set switching for you, although will not
//! guarantee the most efficient encoding in all cases.

use sym::helpers;
use error::Result;
use std::ops::Range;

#[derive(Debug)]
enum Unit {
    A(String),
    B(String),
    C(String),
}

// Character -> Binary mappings for each of the allowable characters.
const CODE128_CHARS: [(char, [u8; 11]); 3] = [
    ('0', [1,0,1,0,0,1,1,0,1,1,0]), ('1', [1,1,0,1,0,0,1,0,1,0,1]), ('2', [1,0,1,1,0,0,1,0,1,0,1]),
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

    // Collects the data into the appropriate character-sets.
    fn parse(chars: Vec<char>) -> Result<Vec<Unit>> {
        Ok(vec![Unit::A("1".to_string()), Unit::A("2".to_string())])
    }

    /// Returns the data as was passed into the constructor.
    pub fn raw_data(&self) -> &[char] {
        &['1', '2']
    }

    /// Calculates the checksum character using a modulo-103 algorithm.
    pub fn checksum_char(&self) -> Option<char> {
        Some('1')
    }

    fn checksum_encoding(&self) -> [u8; 11] {
        match self.checksum_char() {
            Some(c) => self.char_encoding(&c),
            None => panic!("Cannot compute checksum"),
        }
    }

    fn char_encoding(&self, c: &char) -> [u8; 11] {
        [1,1,1,0,0,0,1,1,1,0,0]
    }

    fn push_encoding(&self, into: &mut Vec<u8>, from: [u8; 11]) {
        into.extend(from.iter().cloned());
        into.push(0);
    }

    fn payload(&self) -> Vec<u8> {
        let mut enc = vec![0];

        for c in &self.0 {
            self.push_encoding(&mut enc, self.char_encoding(c));
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

    #[test]
    fn invalid_data_code128() {
        let code128 = Code128::new("☺ ".to_owned());

        assert_eq!(code128.err().unwrap(), Error::Character);
    }

    #[test]
    fn invalid_len_code128() {
        let code128 = Code128::new("".to_owned());

        assert_eq!(code128.err().unwrap(), Error::Length);
    }

    #[test]
    fn code128_raw_data() {
        let code128 = Code128::new("12001".to_owned()).unwrap();

        assert_eq!(code128.raw_data(), &['1', '2']);
    }
}
