//! This module provides types for Interleaved 2-of-5 barcodes.
//! 2-of-5 barcodes are often used by Airlines and in some industrial settings.
//! they also make an appearance in retail where they are sometimes used for the outer cartons on
//! groups of products (cartons of Cola, etc).

use ::sym::Parse;
use ::sym::EncodedBarcode;
use ::sym::helpers;
use std::ops::Range;
use std::char;

const TF_WIDTHS: [&'static str; 10] = [
    "NNWWN", "WNNNW", "NWNNW",
    "WWNNN", "NNWNW", "WNWNN",
    "NWWNN", "NNNWW", "WNNWN",
    "NWNWN",
];

const IFT_START: [u8; 4] = [1,0,1,0];
const IFT_STOP: [u8; 4] = [1,1,0,1];
const SFT_START: [u8; 8] = [1,1,0,1,1,0,1,0];
const SFT_STOP: [u8; 8] = [1,1,0,1,0,1,1,0];

/// The Interleaved 2-of-5 barcode type.
pub enum TF {
    Standard {
        data: Vec<u8>,
    },
    Interleaved {
        data: Vec<u8>,
    },
}

impl TF {
    /// Creates a new IFT barcode.
    /// If the length of the given data is odd, a checksum value will be computed and appended to
    /// the data for encoding.
    ///
    /// Returns Result<TF::Interleaved, String> indicating parse success.
    pub fn interleaved(data: String) -> Result<TF, String> {
        match TF::parse(data) {
            Ok(d) => {
                let mut digits: Vec<u8> = d.chars().map(|c| c.to_digit(10).expect("Unknown character") as u8).collect();
                let checksum_required = digits.len() % 2 == 1;

                if checksum_required {
                    let check_digit = helpers::modulo_10_checksum(&digits[..], false);
                    digits.push(check_digit);
                }

                Ok(TF::Interleaved{data: digits})
            }
            Err(e) => Err(e),
        }
    }

    /// Creates a new SFT barcode.
    ///
    /// Returns Result<TF::Standard, String> indicating parse success.
    pub fn standard(data: String) -> Result<TF, String> {
        match TF::parse(data) {
            Ok(d) => {
                let digits: Vec<u8> = d.chars().map(|c| c.to_digit(10).expect("Unknown character") as u8).collect();
                Ok(TF::Standard{data: digits})
            }
            Err(e) => Err(e),
        }
    }

    /// Returns the data as was passed into the constructor.
    pub fn raw_data(&self) -> &[u8] {
        match *self {
            TF::Standard{data: ref d} => &d[..],
            TF::Interleaved{data: ref d} => &d[..],
        }
    }

    /// Returns a Option<u8> reference to the checksum digit portion of the data.
    /// The Optional value is because Standard TF barcodes do not have a check digit.
    pub fn checksum_digit(&self) -> Option<&u8> {
        match *self {
            TF::Standard{data: _} => None,
            TF::Interleaved{data: ref d} => d.last(),
        }
    }

    fn interleave(&self, bars: u8, spaces: u8) -> Vec<u8> {
        let bwidths = TF_WIDTHS[bars as usize].chars();
        let swidths = TF_WIDTHS[spaces as usize].chars();
        let mut encoding: Vec<u8> = vec![];

        for (b, s) in bwidths.zip(swidths) {
            for &(c, i) in [(b, 1), (s, 0)].iter() {
                match c {
                    'W' => encoding.extend([i; 3].iter().cloned()),
                    _ => encoding.push(i),
                }
            }
        }

        encoding
    }

    fn char_encoding(&self, d: &u8) -> Vec<u8> {
        let bars: Vec<Vec<u8>> = self.char_widths(d)
            .chars()
            .map(|c| {
                match c {
                    'W' => vec![1,1,1,0],
                    _ => vec![1,0],
                }
            })
            .collect();

        helpers::join_vecs(&bars[..])
    }

    fn char_widths(&self, d: &u8) -> &'static str {
        TF_WIDTHS[*d as usize]
    }

    fn stf_payload(&self) -> Vec<u8> {
        let mut encodings = vec![];

        for d in self.raw_data() {
            encodings.extend(self.char_encoding(d).iter().cloned());
        }

        encodings
    }

    fn itf_payload(&self) -> Vec<u8> {
        let weaves: Vec<Vec<u8>> = self.raw_data()
            .chunks(2)
            .map(|c| self.interleave(c[0], c[1]))
            .collect();

        helpers::join_vecs(&weaves[..])
    }

    /// Encodes the barcode.
    /// Returns a Vec<u8> of binary digits.
    pub fn encode(&self) -> EncodedBarcode {
        match *self {
            TF::Standard{data: _} => {
                helpers::join_slices(
                &[&SFT_START[..],
                  &self.stf_payload()[..],
                  &SFT_STOP[..]][..])
            },
            TF::Interleaved{data: _} => {
                helpers::join_slices(
                &[&IFT_START[..],
                  &self.itf_payload()[..],
                  &IFT_STOP[..]][..])
            },
        }
    }
}

impl Parse for TF {
    /// Returns the valid length of data acceptable in this type of barcode.
    /// 2-of5 barcodes are variable-length.
    fn valid_len() -> Range<u32> {
        1..256
    }

    /// Returns the set of valid characters allowed in this type of barcode.
    fn valid_chars() -> Vec<char> {
        (0..10).into_iter().map(|i| char::from_digit(i, 10).unwrap()).collect()
    }
}

#[cfg(test)]
mod tests {
    use ::sym::tf::*;
    use std::char;

    fn collapse_vec(v: Vec<u8>) -> String {
        let chars = v.iter().map(|d| char::from_digit(*d as u32, 10).unwrap());
        chars.collect()
    }

    #[test]
    fn new_itf() {
        let itf = TF::interleaved("12345679".to_string());

        assert!(itf.is_ok());
    }

    #[test]
    fn new_stf() {
        let stf = TF::standard("12345".to_string());

        assert!(stf.is_ok());
    }

    #[test]
    fn new_itf_with_checksum() {
        let itf = TF::interleaved("1234567".to_string()).unwrap();

        assert!(itf.raw_data().len() % 2 == 0);
        assert_eq!(itf.checksum_digit(), Some(&0));
    }

    #[test]
    fn invalid_data_itf() {
        let itf = TF::interleaved("1234er123412".to_string());

        assert!(itf.is_err());
    }

    #[test]
    fn invalid_data_stf() {
        let stf = TF::standard("WORDUP".to_string());

        assert!(stf.is_err());
    }
 
    #[test]
    fn itf_raw_data() {
        let itf = TF::interleaved("12345679".to_string()).unwrap();

        assert_eq!(itf.raw_data(), &[1,2,3,4,5,6,7,9]);
    }

    #[test]
    fn itf_encode() {
        let itf = TF::interleaved("1234567".to_string()).unwrap(); // Check digit: 0

        assert_eq!(collapse_vec(itf.encode()), "10101110100010101110001110111010001010001110100011100010101010100011100011101101".to_string());
    }

    #[test]
    fn stf_encode() {
        let stf = TF::standard("1234567".to_string()).unwrap();

        assert_eq!(collapse_vec(stf.encode()), "110110101110101010111010111010101110111011101010101010111010111011101011101010101110111010101010101110111011010110".to_string());
    }
}
