//! Encoder for Code128 barcodes.
//!
//! Code128 is a high-density symbology that allows for the encoding of alphanumeric data. It's
//! very popular and supported by most scanners.

use sym::Parse;
use sym::helpers;
use error::Result;
use std::ops::Range;

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
    /// Returns a Vec<u8> of binary digits.
    pub fn encode(&self) -> Vec<u8> {
        let guard = &CODE39_GUARD[..];

        helpers::join_slices(&[guard, &self.payload()[..], guard][..])
    }
}

impl Parse for Code39 {
    fn valid_len() -> Range<u32> {
        1..256
    }

    fn valid_chars() -> Vec<char> {
        let (chars, _): (Vec<_>, Vec<_>) = CODE39_CHARS.iter().cloned().unzip();
        chars
    }
}

#[cfg(test)]
mod tests {
    use sym::code39::*;
    use error::Error;
    use std::char;

    fn collapse_vec(v: Vec<u8>) -> String {
        let chars = v.iter().map(|d| char::from_digit(*d as u32, 10).unwrap());
        chars.collect()
    }

    #[test]
    fn new_code39() {
        let code39 = Code39::new("12345".to_owned());

        assert!(code39.is_ok());
    }

    #[test]
    fn invalid_data_code39() {
        let code39 = Code39::new("1212s".to_owned());

        assert_eq!(code39.err().unwrap(), Error::Character);
    }

    #[test]
    fn invalid_len_code39() {
        let code39 = Code39::new("".to_owned());

        assert_eq!(code39.err().unwrap(), Error::Length);
    }

    #[test]
    fn code39_raw_data() {
        let code39 = Code39::new("12345".to_owned()).unwrap();

        assert_eq!(code39.raw_data(), &['1', '2', '3', '4', '5']);
    }

    #[test]
    fn code39_encode() {
        let code391 = Code39::new("1234".to_owned()).unwrap();
        let code392 = Code39::new("983RD512".to_owned()).unwrap();
        let code393 = Code39::new("TEST8052".to_owned()).unwrap();

        assert_eq!(collapse_vec(code391.encode()), "10010110110101101001010110101100101011011011001010101010011010110100101101101".to_owned());
        assert_eq!(collapse_vec(code392.encode()), "100101101101010110010110101101001011010110110010101011010101100101010110010110110100110101011010010101101011001010110100101101101".to_owned());
        assert_eq!(collapse_vec(code393.encode()), "100101101101010101101100101101011001010101101011001010101101100101101001011010101001101101011010011010101011001010110100101101101".to_owned());
    }

    #[test]
    fn code39_encode_with_checksum() {
        let code391 = Code39::with_checksum("1234".to_owned()).unwrap();
        let code392 = Code39::with_checksum("983RD512".to_owned()).unwrap();

        assert_eq!(collapse_vec(code391.encode()), "100101101101011010010101101011001010110110110010101010100110101101101010010110100101101101".to_owned());
        assert_eq!(collapse_vec(code392.encode()), "1001011011010101100101101011010010110101101100101010110101011001010101100101101101001101010110100101011010110010101101011011010010100101101101".to_owned());
    }

    #[test]
    fn code39_checksum_calculation() {
        let code391 = Code39::new("1234".to_owned()).unwrap(); // Check char: 'A'
        let code392 = Code39::new("159AZ".to_owned()).unwrap(); // Check char: 'H'

        assert_eq!(code391.checksum_char().unwrap(), 'A');
        assert_eq!(code392.checksum_char().unwrap(), 'H');
    }
}
