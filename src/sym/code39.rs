//! This module provides types for encoding Code39 barcodes. Also known as 3-of-9 barcodes.
//! Code39 is the standard barcode used by the United States Department of Defense and is also
//! popular in non-retail environments. 

use ::sym::Parse;
use ::sym::EncodedBarcode;
use ::sym::helpers;
use std::ops::Range;

/// Character -> Binary mappings for each of the 43 allowable character.
pub const CODE39_CHARS: [(char, [u8; 12]); 43] = [
    ('0', [1,0,1,0,0,1,1,0,1,1,0,1]), ('1', [1,1,0,1,0,0,1,0,1,0,1,1]), ('2', [1,0,1,1,0,0,1,0,1,0,1,1]),
    ('3', [1,1,0,1,1,0,0,1,0,1,0,1]), ('4', [1,0,1,0,0,1,1,0,1,0,1,1]), ('5', [1,1,0,1,0,0,1,1,0,1,0,1]),
    ('6', [1,0,1,1,0,0,1,1,0,1,0,1]), ('7', [1,0,1,0,0,1,0,1,1,0,1,1]), ('8', [1,1,0,1,0,0,1,0,1,1,0,1]),
    ('9', [1,0,1,1,0,0,1,0,1,1,0,1]), ('A', [1,1,0,1,0,1,0,0,1,0,1,1]), ('B', [1,0,1,1,0,1,0,0,1,0,1,1]),
    ('C', [1,1,0,1,1,0,1,0,0,1,0,1]), ('D', [1,0,1,0,1,1,0,0,1,0,1,1]), ('E', [1,1,0,1,0,1,1,0,0,1,0,1]),
    ('F', [1,0,1,1,0,1,1,0,0,1,0,1]), ('G', [1,0,1,0,1,0,0,1,1,0,1,1]), ('H', [1,1,0,1,0,1,0,0,1,1,0,1]),
    ('I', [1,0,1,1,0,1,0,0,1,1,0,1]), ('J', [1,0,1,0,1,1,0,0,1,1,0,1]), ('K', [1,1,0,1,0,1,0,1,0,0,1,1]),
    ('L', [1,0,1,1,0,1,0,1,0,0,1,1]), ('M', [1,1,0,1,1,0,1,0,1,0,0,1]), ('N', [1,0,1,0,1,1,0,1,0,0,1,1]),
    ('O', [1,1,0,1,0,1,1,0,1,0,0,1]), ('P', [1,0,1,1,0,1,1,0,1,0,0,1]), ('Q', [1,0,1,0,1,0,1,1,0,0,1,1]),
    ('R', [1,1,0,1,0,1,0,1,1,0,0,1]), ('S', [1,0,1,1,0,1,0,1,1,0,0,1]), ('T', [1,0,1,0,1,1,0,1,1,0,0,1]),
    ('U', [1,1,0,0,1,0,1,0,1,0,1,1]), ('V', [1,0,0,1,1,0,1,0,1,0,1,1]), ('W', [1,1,0,0,1,1,0,1,0,1,0,1]),
    ('X', [1,0,0,1,0,1,1,0,1,0,1,1]), ('Y', [1,1,0,0,1,0,1,1,0,1,0,1]), ('Z', [1,0,0,1,1,0,1,1,0,1,0,1]),
    ('-', [1,0,0,1,0,1,0,1,1,0,1,1]), ('.', [1,1,0,0,1,0,1,0,1,1,0,1]), (' ', [1,0,0,1,1,0,1,0,1,1,0,1]),
    ('$', [1,0,0,1,0,0,1,0,0,1,0,1]), ('/', [1,0,0,1,0,0,1,0,1,0,0,1]), ('+', [1,0,0,1,0,1,0,0,1,0,0,1]),
    ('%', [1,0,1,0,0,1,0,0,1,0,0,1]),
];

/// Code39 barcodes must start and end with the '*' special character.
pub const CODE39_GUARD: [u8; 12] = [1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1];

/// The Code39 barcode type.
pub struct Code39 {
    data: Vec<char>,
    pub checksum: bool,
}

impl Code39 {
   fn init(data: String, checksum: bool) -> Result<Code39, String> {
        match Code39::parse(data) {
            Ok(d) => Ok(Code39{data: d.chars().collect(), checksum: checksum}),
            Err(e) => Err(e),
        }
    }

    /// Creates a new barcode.
    /// Returns Result<Code39, String> indicating parse success.
    pub fn new(data: String) -> Result<Code39, String> {
        Code39::init(data, false)
    }

    /// Creates a new barcode with an appended check-digit, calculated using modulo-43..
    /// Returns Result<Code39, String> indicating parse success.
    pub fn with_checksum(data: String) -> Result<Code39, String> {
        Code39::init(data, true)
    }

    /// Returns the data as was passed into the constructor.
    pub fn raw_data(&self) -> &[char] {
        &self.data[..]
    }

    /// Calculates the checksum character using a modulo-43 algorithm.
    pub fn checksum_char(&self) -> Option<char> {
        let get_char_pos = |&c| CODE39_CHARS.iter().position(|t| t.0 == c).unwrap();
        let indices = self.data.iter().map(&get_char_pos);
        let index = indices.fold(0, |acc, i| acc + i) % CODE39_CHARS.len();

        match CODE39_CHARS.get(index) {
            Some(&(c, _)) => Some(c),
            None => None,
        }
    }

    fn checksum_encoding(&self) -> [u8; 12] {
        match self.checksum_char() {
            Some(c) => self.char_encoding(&c),
            None => panic!("Cannot compute checksum"),
        }
    }

    fn char_encoding(&self, c: &char) -> [u8; 12] {
         match CODE39_CHARS.iter().find(|&ch| ch.0 == *c) {
            Some(&(_, enc)) => enc,
            None => panic!(format!("Unknown char: {}", c)),
        }
    }

    // Encoded characters are separated by a single "narrow" bar in
    // Code39 barcodes.
    fn push_encoding(&self, into: &mut Vec<u8>, from: [u8; 12]) {
        into.extend(from.iter().cloned());
        into.push(0);
    }

    fn payload(&self) -> Vec<u8> {
        let mut enc = vec![0];

        for c in &self.data {
            self.push_encoding(&mut enc, self.char_encoding(&c));
        }

        if self.checksum {
            self.push_encoding(&mut enc, self.checksum_encoding());
        }

        enc
    }

    /// Encodes the barcode.
    /// Returns an EncodedBarcode (wrapper type of Vec<u8>) of binary digits.
    pub fn encode(&self) -> EncodedBarcode {
        helpers::join_vecs(&[
            CODE39_GUARD.to_vec(), self.payload(), CODE39_GUARD.to_vec()][..])
    }
}

impl Parse for Code39 {
    /// Returns the valid length of data acceptable in this type of barcode.
    /// Code-39 is variable-length.
    fn valid_len() -> Range<u32> {
        1..256
    }

    /// Returns the set of valid characters allowed in this type of barcode.
    fn valid_chars() -> Vec<char> {
        let (chars, _): (Vec<_>, Vec<_>) = CODE39_CHARS.iter().cloned().unzip();
        chars
    }
}

#[cfg(test)]
mod tests {
    use ::sym::code39::*;
    use std::char;

    fn collapse_vec(v: Vec<u8>) -> String {
        let chars = v.iter().map(|d| char::from_digit(*d as u32, 10).unwrap());
        chars.collect()
    }

    #[test]
    fn new_code39() {
        let code39 = Code39::new("12345".to_string());

        assert!(code39.is_ok());
    }

    #[test]
    fn invalid_data_code39() {
        let code39 = Code39::new("1212s".to_string());

        assert!(code39.is_err());
    }

    #[test]
    fn invalid_len_code39() {
        let code39 = Code39::new("".to_string());

        assert!(code39.is_err());
    }

    #[test]
    fn code39_raw_data() {
        let code39 = Code39::new("12345".to_string()).unwrap();

        assert_eq!(code39.raw_data(), &['1', '2', '3', '4', '5']);
    }

    #[test]
    fn code39_encode() {
        let code391 = Code39::new("1234".to_string()).unwrap();
        let code392 = Code39::new("983RD512".to_string()).unwrap();
        let code393 = Code39::new("TEST8052".to_string()).unwrap();

        assert_eq!(collapse_vec(code391.encode()), "10010110110101101001010110101100101011011011001010101010011010110100101101101".to_string());
        assert_eq!(collapse_vec(code392.encode()), "100101101101010110010110101101001011010110110010101011010101100101010110010110110100110101011010010101101011001010110100101101101".to_string());
        assert_eq!(collapse_vec(code393.encode()), "100101101101010101101100101101011001010101101011001010101101100101101001011010101001101101011010011010101011001010110100101101101".to_string());
    }

    #[test]
    fn code39_encode_with_checksum() {
        let code391 = Code39::with_checksum("1234".to_string()).unwrap();
        let code392 = Code39::with_checksum("983RD512".to_string()).unwrap();

        assert_eq!(collapse_vec(code391.encode()), "100101101101011010010101101011001010110110110010101010100110101101101010010110100101101101".to_string());
        assert_eq!(collapse_vec(code392.encode()), "1001011011010101100101101011010010110101101100101010110101011001010101100101101101001101010110100101011010110010101101011011010010100101101101".to_string());
    }

    #[test]
    fn code39_checksum_calculation() {
        let code391 = Code39::new("1234".to_string()).unwrap(); // Check char: 'A'
        let code392 = Code39::new("159AZ".to_string()).unwrap(); // Check char: 'H'

        assert_eq!(code391.checksum_char().unwrap(), 'A');
        assert_eq!(code392.checksum_char().unwrap(), 'H');
    }
}
