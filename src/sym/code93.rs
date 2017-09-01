//! Encoder for Code93 barcodes.
//!
//! Code93 is intented to improve upon Code39 barcodes by offering a wider array of encodable
//! ASCII characters. It also produces denser barcodes than Code39.
//!
//! Code93 is a continuous, variable-length symbology.
//!
//! NOTE: This encoder currently only supports the basic Code93 implementation and not full-ASCII
//! mode.

use sym::Parse;
use sym::helpers;
use error::Result;
use std::ops::Range;

// Character -> Binary mappings for each of the 47 allowable character.
// The special "full-ASCII" characters are represented with (, ), [, ].
const CHARS: [(char, [u8; 9]); 47] = [
    ('0', [1,0,0,0,1,0,1,0,0]), ('1', [1,0,1,0,0,1,0,0,0]), ('2', [1,0,1,0,0,0,1,0,0]),
    ('3', [1,0,1,0,0,0,0,1,0]), ('4', [1,0,0,1,0,1,0,0,0]), ('5', [1,0,0,1,0,0,1,0,0]),
    ('6', [1,0,0,1,0,0,0,1,0]), ('7', [1,0,1,0,1,0,0,0,0]), ('8', [1,0,0,0,1,0,0,1,0]),
    ('9', [1,0,0,0,0,1,0,1,0]), ('A', [1,1,0,1,0,1,0,0,0]), ('B', [1,1,0,1,0,0,1,0,0]),
    ('C', [1,1,0,1,0,0,0,1,0]), ('D', [1,1,0,0,1,0,1,0,0]), ('E', [1,1,0,0,1,0,0,1,0]),
    ('F', [1,1,0,0,0,1,0,1,0]), ('G', [1,0,1,1,0,1,0,0,0]), ('H', [1,0,1,1,0,0,1,0,0]),
    ('I', [1,0,1,1,0,0,0,1,0]), ('J', [1,0,0,1,1,0,1,0,0]), ('K', [1,0,0,0,1,1,0,1,0]),
    ('L', [1,0,1,0,1,1,0,0,0]), ('M', [1,0,1,0,0,1,1,0,0]), ('N', [1,0,1,0,0,0,1,1,0]),
    ('O', [1,0,0,1,0,1,1,0,0]), ('P', [1,0,0,0,1,0,1,1,0]), ('Q', [1,1,0,1,1,0,1,0,0]),
    ('R', [1,1,0,1,1,0,0,1,0]), ('S', [1,1,0,1,0,1,1,0,0]), ('T', [1,1,0,1,0,0,1,1,0]),
    ('U', [1,1,0,0,1,0,1,1,0]), ('V', [1,1,0,0,1,1,0,1,0]), ('W', [1,0,1,1,0,1,1,0,0]),
    ('X', [1,0,1,1,0,0,1,1,0]), ('Y', [1,0,0,1,1,0,1,1,0]), ('Z', [1,0,0,1,1,1,0,1,0]),
    ('-', [1,0,0,1,0,1,1,1,0]), ('.', [1,1,1,0,1,0,1,0,0]), (' ', [1,1,1,0,1,0,0,1,0]),
    ('$', [1,1,1,0,0,1,0,1,0]), ('/', [1,0,1,1,0,1,1,1,0]), ('+', [1,0,1,1,1,0,1,1,0]),
    ('%', [1,1,0,1,0,1,1,1,0]), ('(', [1,0,0,1,0,0,1,1,0]), (')', [1,1,1,0,1,1,0,1,0]),
    ('[', [1,1,1,0,1,0,1,1,0]), ('[', [1,0,0,1,1,0,0,1,0]),
];

// Code93 barcodes must start and end with the '*' special character.
const GUARD: [u8; 9] = [1,0,1,0,1,1,1,1,0];
const TERMINATOR: [u8; 1] = [1];

/// The Code93 barcode type.
#[derive(Debug)]
pub struct Code93(Vec<char>);

impl Code93 {
    /// Creates a new barcode.
    /// Returns Result<Code93, Error> indicating parse success.
    pub fn new<T: AsRef<str>>(data: T) -> Result<Code93> {
        Code93::parse(data.as_ref()).and_then(|d| {
            Ok(Code93(d.chars()
                       .collect()))
        })
    }

    fn char_encoding(&self, c: &char) -> [u8; 9]{
        match CHARS.iter().find(|&ch| ch.0 == *c) {
            Some(&(_, enc)) => enc,
            None => panic!(format!("Unknown char: {}", c)),
        }
    }

    fn push_encoding(&self, into: &mut Vec<u8>, from: [u8; 9]) {
        into.extend(from.iter().cloned());
    }

    fn payload(&self) -> Vec<u8> {
        let mut enc = vec![];

        for c in &self.0 {
            self.push_encoding(&mut enc, self.char_encoding(&c));
        }

        enc
    }

    /// Encodes the barcode.
    /// Returns a Vec<u8> of encoded binary digits.
    pub fn encode(&self) -> Vec<u8> {
        let guard = &GUARD[..];
        let terminator = &TERMINATOR[..];

        helpers::join_slices(&[guard, &self.payload()[..],
                               guard, terminator][..])
    }
}

impl Parse for Code93 {
    /// Returns the valid length of data acceptable in this type of barcode.
    /// Code93 barcodes are variable-length.
    fn valid_len() -> Range<u32> {
        1..256
    }

    /// Returns the set of valid characters allowed in this type of barcode.
    fn valid_chars() -> Vec<char> {
        let (chars, _): (Vec<_>, Vec<_>) = CHARS.iter().cloned().unzip();
        chars
    }
}

#[cfg(test)]
mod tests {
    use sym::code93::*;
    use error::Error;
    use std::char;

    fn collapse_vec(v: Vec<u8>) -> String {
        let chars = v.iter().map(|d| char::from_digit(*d as u32, 10).unwrap());
        chars.collect()
    }

    #[test]
    fn invalid_length_code93() {
        let code93 = Code93::new("".to_owned());

        assert_eq!(code93.err().unwrap(), Error::Length);
    }

    #[test]
    fn invalid_data_code93() {
        let code93 = Code93::new("lowerCASE".to_owned());

        assert_eq!(code93.err().unwrap(), Error::Character);
    }

    #[test]
    fn code93_encode() {
        let code931 = Code93::new("TEST93".to_owned()).unwrap();

        assert_eq!(collapse_vec(code931.encode()), "1010111101101001101100100101101011001101001101000010101010000101011101101001000101010111101".to_owned());
    }
}
