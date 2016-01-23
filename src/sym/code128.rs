//! Encoder for Code128 barcodes.
//!
//! Code128 is a high-density symbology that allows for the encoding of alphanumeric data. It's
//! very popular and supported by most scanners.

use sym::Parse;
use sym::helpers;
use error::Result;
use std::ops::Range;

// Character -> Binary mappings for each of the allowable character.
const CODE128_CHARS: [(char, [u8; 11]); 3] = [
    ('0', [1,0,1,0,0,1,1,0,1,1,0]), ('1', [1,1,0,1,0,0,1,0,1,0,1]), ('2', [1,0,1,1,0,0,1,0,1,0,1]),
];
 
/// The Code128 barcode type.
#[derive(Debug)]
pub struct Code128(Vec<char>);

impl Code128 {
    /// Creates a new barcode.
    /// Returns Result<Code128, Error> indicating parse success.
    pub fn new(data: String) -> Result<Code128> {
        match Code128::parse(data) {
            Ok(d) => Ok(Code128(d.chars().collect()))
            Err(e) => Err(e),
        }
    }

    /// Returns the data as was passed into the constructor.
    pub fn raw_data(&self) -> &[char] {
        &self.data[..]
    }

    /// Calculates the checksum character using a modulo-103 algorithm.
    pub fn checksum_char(&self) -> Option<char> {
        Some('a')
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

        for c in &self.data {
            self.push_encoding(&mut enc, self.char_encoding(&c));
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

impl Parse for Code128 {
    fn valid_len() -> Range<u32> {
        1..512
    }

    fn valid_chars() -> Vec<char> {
        let (chars, _): (Vec<_>, Vec<_>) = CODE128_CHARS.iter().cloned().unzip();
        chars
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
        let code128 = Code128::new("12345".to_owned());

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
        let code128 = Code128::new("12345".to_owned()).unwrap();

        assert_eq!(code128.raw_data(), &['1', '2', '3', '4', '5']);
    }
}
