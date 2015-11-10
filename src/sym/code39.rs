//! This module provides types for encoding Code39 barcodes. Also known as 3-of-9 barcodes.
//! Code39 is the standard barcode used by the United States Department of Defense and is also
//! popular in non-retail environments. 

use ::sym::Encode;
use ::sym::Parse;
use std::ops::Range;
use std::char;

/// Character -> Binary mappings for each of the 43 allowable character.
pub const CODE39_CHARS: [(char, &'static str); 43] = [
    ('0', "101001101101"), ('1', "110100101011"), ('2', "101100101011"),
    ('3', "110110010101"), ('4', "101001101011"), ('5', "110100110101"),
    ('6', "101100110101"), ('7', "101001011011"), ('8', "110100101101"),
    ('9', "101100101101"), ('A', "110101001011"), ('B', "101101001011"),
    ('C', "110110100101"), ('D', "101011001011"), ('E', "110101100101"),
    ('F', "101101100101"), ('G', "101010011011"), ('H', "110101001101"),
    ('I', "101101001101"), ('J', "101011001101"), ('K', "110101010011"),
    ('L', "101101010011"), ('M', "110110101001"), ('N', "101011010011"),
    ('O', "110101101001"), ('P', "101101101001"), ('Q', "101010110011"),
    ('R', "110101011001"), ('S', "101101011001"), ('T', "101011011001"),
    ('U', "110010101011"), ('V', "100110101011"), ('W', "110011010101"),
    ('X', "100101101011"), ('Y', "110010110101"), ('Z', "100110110101"),
    ('-', "100101011011"), ('.', "110010101101"), (' ', "100110101101"),
    ('$', "100100100101"), ('/', "100100101001"), ('+', "100101001001"),
    ('%', "101001001001"),
];

/// Code39 barcodes must start and end with the '*' special character.
pub const CODE39_GUARD: &'static str = "100101101101";

/// The Code39 barcode type.
pub struct Code39 {
    data: String,
    checksum_required: bool,
}

impl Code39 {
   fn init(data: String, checksum_required: bool) -> Result<Code39, String> {
        match Code39::parse(data) {
            Ok(d) => Ok(Code39{data: d, checksum_required: checksum_required}),
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
    pub fn raw_data(&self) -> &str {
        &self.data[..]
    }

    /// Calculates the checksum character using a modulo-43 algorithm.
    pub fn checksum_char(&self) -> Option<char> {
        let get_char_pos = |c| CODE39_CHARS.iter().position(|t| t.0 == c).unwrap();
        let indices = self.data.chars().map(&get_char_pos);
        let index = indices.fold(0, |acc, i| acc + i) % CODE39_CHARS.len();

        match CODE39_CHARS.get(index) {
            Some(&(c, _)) => Some(c),
            None => None,
        }
    }

    fn checksum_encoding(&self) -> &'static str {
        match self.checksum_char() {
            Some(c) => self.char_encoding(&c),
            None => "",
        }
    }

    fn char_encoding(&self, c: &char) -> &'static str {
        match CODE39_CHARS.iter().find(|&ch| ch.0 == *c) {
            Some(&(_, enc)) => enc,
            None => panic!(format!("Unknown char: {}", c)),
        }
    }

    fn payload(&self) -> String {
        let chars = self.data.chars()
                             .map(|c| self.char_encoding(&c))
                             .collect();

        if self.checksum_required {
            format!("{}{}", chars, self.checksum_encoding())
        } else {
            chars
        }
    }
}

impl Parse for Code39 {
    /// Returns the valid length of data acceptable in this type of barcode.
    /// Code-39 is variable-length.
    fn valid_len() -> Range<u32> {
        1..128
    }

    /// Returns the set of valid characters allowed in this type of barcode.
    fn valid_chars() -> Vec<char> {
        let (chars, _): (Vec<_>, Vec<_>) = CODE39_CHARS.iter().cloned().unzip();
        chars
    }
}

impl Encode for Code39 {
    /// Encodes the barcode.
    /// Returns a Vec<u32> of binary digits.
    fn encode(&self) -> Vec<u8> {
        let s = format!("{}{}{}", CODE39_GUARD, self.payload(), CODE39_GUARD);

        s.chars().map(|c| c.to_digit(2).expect("Unknown character") as u8).collect::<Vec<u8>>()
    }
}

#[cfg(test)]
mod tests {
    use ::sym::code39::*;
    use ::sym::Encode;
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

        assert_eq!(code39.raw_data(), "12345");
    }

    #[test]
    fn code39_encode() {
        let code391 = Code39::new("1234".to_string()).unwrap();
        let code392 = Code39::new("983RD512".to_string()).unwrap();

        assert_eq!(collapse_vec(code391.encode()), "100101101101110100101011101100101011110110010101101001101011100101101101".to_string());
        assert_eq!(collapse_vec(code392.encode()), "100101101101101100101101110100101101110110010101110101011001101011001011110100110101110100101011101100101011100101101101".to_string());
    }

    #[test]
    fn code39_encode_with_checksum() {
        let code391 = Code39::with_checksum("1234".to_string()).unwrap();
        let code392 = Code39::with_checksum("983RD512".to_string()).unwrap();

        assert_eq!(collapse_vec(code391.encode()), "100101101101110100101011101100101011110110010101101001101011110101001011100101101101".to_string());
        assert_eq!(collapse_vec(code392.encode()), "100101101101101100101101110100101101110110010101110101011001101011001011110100110101110100101011101100101011101101101001100101101101".to_string());
    }

    #[test]
    fn code39_checksum_calculation() {
        let code391 = Code39::new("1234".to_string()).unwrap(); // Check char: 'A'
        let code392 = Code39::new("159AZ".to_string()).unwrap(); // Check char: 'H'

        assert_eq!(code391.checksum_char().unwrap(), 'A');
        assert_eq!(code392.checksum_char().unwrap(), 'H');
    }
}
