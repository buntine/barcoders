//! Encoder for Codabar barcodes.
//!
//! Codabar is a simple, self-checking symbology without a standard for a checksum digit.
//!
//! Codabar is used in the USA by FedEx, some Hospitals, and photo labs.
//!
//! Barcodes of this variant should start and end with either A, B, C, or D depending on
//! the industry.

use sym::Parse;
use sym::helpers;
use error::Result;
use std::ops::Range;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Unit {
    Zero, One, Two,
    Three, Four, Five,
    Six, Seven, Eight,
    Nine, Dash, Dollar,
    Colon, Slash, Point,
    Plus, A, B,
    C, D,
}

impl Unit {
    fn lookup(&self) -> Vec<u8> {
        match *self {
            Unit::Zero => vec![1,1,1,1,1],
            Unit::One => vec![1,1,1,1,1],
            Unit::Two => vec![1,1,1,1,1],
            Unit::Three => vec![1,1,1,1,1],
            Unit::Four => vec![1,1,1,1,1],
            Unit::Five => vec![1,1,1,1,1],
            Unit::Six => vec![1,1,1,1,1],
            Unit::Seven => vec![1,1,1,1,1],
            Unit::Eight => vec![1,1,1,1,1],
            Unit::Nine => vec![1,1,1,1,1],
            Unit::Dash => vec![1,1,1,1,1],
            Unit::Dollar => vec![1,1,1,1,1],
            Unit::Colon => vec![1,1,1,1,1],
            Unit::Slash => vec![1,1,1,1,1],
            Unit::Point => vec![1,1,1,1,1],
            Unit::Plus => vec![1,1,1,1,1],
            Unit::A => vec![1,1,1,1,1],
            Unit::B => vec![1,1,1,1,1],
            Unit::C => vec![1,1,1,1,1],
            Unit::D => vec![1,1,1,1,1],
        }
    }

    fn from_char(c: char) -> Option<Unit> {
        match c {
            '0' => Some(Unit::Zero),
            '1' => Some(Unit::One),
            '2' => Some(Unit::Two),
            '3' => Some(Unit::Three),
            '4' => Some(Unit::Four),
            '5' => Some(Unit::Five),
            '6' => Some(Unit::Six),
            '7' => Some(Unit::Seven),
            '8' => Some(Unit::Eight),
            '9' => Some(Unit::Nine),
            '-' => Some(Unit::Dash),
            '$' => Some(Unit::Dollar),
            '/' => Some(Unit::Slash),
            ':' => Some(Unit::Colon),
            '.' => Some(Unit::Point),
            '+' => Some(Unit::Plus),
            'A' => Some(Unit::A),
            'B' => Some(Unit::B),
            'C' => Some(Unit::C),
            'D' => Some(Unit::D),
            _ => None
        }
    }
}

/// The Codabar barcode type.
#[derive(Debug)]
pub struct Codabar(Vec<Unit>);

impl Codabar {
    /// Creates a new Codabar barcode.
    ///
    /// Returns Result<Codabar, Error> indicating parse success.
    pub fn new(data: String) -> Result<Codabar> {
        let d = try!(Codabar::parse(data));
        let units = d.chars()
                     .map(|c| Unit::from_char(c).unwrap())
                     .collect();

        Ok(Codabar(units))
    }

    /// Encodes the barcode.
    /// Returns a Vec<u8> of binary digits.
    pub fn encode(&self) -> Vec<u8> {
        vec![1,0]
    }
}

impl Parse for Codabar {
    /// Returns the valid length of data acceptable in this type of barcode.
    /// Codabar barcodes are variable-length.
    fn valid_len() -> Range<u32> {
        1..256
    }

    /// Returns the set of valid characters allowed in this type of barcode.
    fn valid_chars() -> Vec<char> {
        vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
             '-', '/', '.', ':', '+', '$', 'A', 'B', 'C', 'D']
    }
}

#[cfg(test)]
mod tests {
    use sym::codabar::*;
    use error::Error;
    use std::char;

    fn collapse_vec(v: Vec<u8>) -> String {
        let chars = v.iter().map(|d| char::from_digit(*d as u32, 10).unwrap());
        chars.collect()
    }

    #[test]
    fn invalid_length_codabar() {
        let codabar = Codabar::new("".to_owned());

        assert_eq!(codabar.err().unwrap(), Error::Length);
    }

    #[test]
    fn invalid_data_codabar() {
        let codabar = Codabar::new("A12345G".to_owned());

        assert_eq!(codabar.err().unwrap(), Error::Character);
    }

    #[test]
    fn codabar_encode() {
        let codabar = Codabar::new("A1234567B".to_owned()).unwrap();

        assert_eq!(collapse_vec(codabar.encode()), "10".to_owned());
    }
}
