//! Encoder for Code11 (USD-8) barcodes.
//!
//! Code11 is able to encode all of the decimal digits and the dash character. It is mainly
//! used in the telecommunications industry.
//!
//! Code11 is a discrete symbology. This encoder always provides a C checksum. For barcodes longer
//! than 10 characters, a second checksum digit (K) is appended.

use super::*;

// Character -> Binary mappings for each of the allowable characters.
// The special "full-ASCII" characters are represented with (, ), [, ].
const CHARS: [(char, &[u8]); 11] = [
    ('0', &[1, 0, 1, 0, 1, 1]),
    ('1', &[1, 1, 0, 1, 0, 1, 1]),
    ('2', &[1, 0, 0, 1, 0, 1, 1]),
    ('3', &[1, 1, 0, 0, 1, 0, 1]),
    ('4', &[1, 0, 1, 1, 0, 1, 1]),
    ('5', &[1, 1, 0, 1, 1, 0, 1]),
    ('6', &[1, 0, 0, 1, 1, 0, 1]),
    ('7', &[1, 0, 1, 0, 0, 1, 1]),
    ('8', &[1, 1, 0, 1, 0, 0, 1]),
    ('9', &[1, 1, 0, 1, 0, 1]),
    ('-', &[1, 0, 1, 1, 0, 1]),
];

// Code11 barcodes must start and end with a special character.
const GUARD: [u8; 7] = [1, 0, 1, 1, 0, 0, 1];
const SEPARATOR: [u8; 1] = [0];

/// The Code11 barcode type.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Code11<'a>(&'a [u8]);

impl<'a> Barcode<'a> for Code11<'a> {
    const SIZE: Range<u16> = 1..257; // 1..=256
    const CHARS: &'static [u8] = b"0123456789-";

    fn new(data: &'a [u8]) -> Result<Self> {
        Self::validate(data).map(Self)
    }

    fn encode_in_place(&self, buffer: &mut [u8]) -> Option<()> {
        todo!()
    }

    fn encode(&self) -> Vec<u8> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn collapse_vec(v: Vec<u8>) -> String {
        let chars = v.iter().map(|d| char::from_digit(*d as u32, 10).unwrap());
        chars.collect()
    }

    #[test]
    fn invalid_length_code11() {
        let code11 = Code11::new("");

        assert_eq!(code11.err().unwrap(), Error::Length);
    }

    #[test]
    fn invalid_data_code11() {
        let code11 = Code11::new("NOTDIGITS");

        assert_eq!(code11.err().unwrap(), Error::Character);
    }

    #[test]
    fn code11_encode_less_than_10_chars() {
        let code111 = Code11::new("123-45").unwrap();
        let code112 = Code11::new("666").unwrap();
        let code113 = Code11::new("12-9").unwrap();

        assert_eq!(
            collapse_vec(code111.encode()),
            "1011001011010110100101101100101010110101011011011011010110110101011001"
        );
        assert_eq!(
            collapse_vec(code112.encode()),
            "10110010100110101001101010011010110010101011001"
        );
        assert_eq!(
            collapse_vec(code113.encode()),
            "10110010110101101001011010110101101010100110101011001"
        );
    }

    #[test]
    fn code11_encode_more_than_10_chars() {
        let code111 = Code11::new("1234-5678-4321").unwrap();

        assert_eq!(collapse_vec(code111.encode()), "101100101101011010010110110010101011011010110101101101010011010101001101101001010110101011011011001010100101101101011011011010100110101011001");
    }
}
