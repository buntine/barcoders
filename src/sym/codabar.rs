//! Encoder for Codabar barcodes.
//!
//! Codabar is a simple, self-checking symbology without a standard for a checksum digit.
//!
//! Codabar is used in the USA by FedEx, some Hospitals, and photo labs.
//!
//! Barcodes of this variant should start and end with either A, B, C, or D depending on
//! the industry.

use crate::error::Result;
use core::ops::Range;

/// The Codabar barcode type.
#[derive(Debug)]
pub struct Codabar<'a>(&'a [u8]);

impl<'a> Codabar<'a> {
    fn get_sum(&self) -> usize {
        let mut sum: usize = 0;
        for byte in self.0.iter() {
            match byte {
                b'0'..=b'9' => sum += 9,
                b'-' | b'$' | b':' | b'/' | b'.' => sum += 10,
                b'+' => sum += 12,
                b'A' | b'B' | b'C' | b'D' => sum += 10,
                _ => unreachable!("Validation did not catch an illegal character"),
            }
            sum += 1; // Padding between characters
        }
        sum -= 1; // Remove padding after last character
        sum
    }
    fn encode_into(&self, buffer: &mut [u8]) {
        let mut i = 0;
        macro_rules! encode {
            ($iter:expr => $name:ident {$(
                $pattern:pat => $bits:expr
            ),*}) => (
                for $name in $iter {
                    match $name {
                        $($pattern => {
                            let length = $bits.len();
                            for j in 0..length {
                                buffer[i + j] = $bits[j];
                            }
                            i += length;

                            // Don't forget the padding
                            if i < buffer.len() {
                                buffer[i] = 0;
                                i += 1;
                            }
                        },)*
                        _ => unreachable!("Validation did not catch an illegal character"),
                    }
                }
            );
        }
        encode!(self.0.iter() => byte {
            b'0' => [1, 0, 1, 0, 1, 0, 0, 1, 1],
            b'1' => [1, 0, 1, 0, 1, 1, 0, 0, 1],
            b'2' => [1, 0, 1, 0, 0, 1, 0, 1, 1],
            b'3' => [1, 1, 0, 0, 1, 0, 1, 0, 1],
            b'4' => [1, 0, 1, 1, 0, 1, 0, 0, 1],
            b'5' => [1, 1, 0, 1, 0, 1, 0, 0, 1],
            b'6' => [1, 0, 0, 1, 0, 1, 0, 1, 1],
            b'7' => [1, 0, 0, 1, 0, 1, 1, 0, 1],
            b'8' => [1, 0, 0, 1, 1, 0, 1, 0, 1],
            b'9' => [1, 1, 0, 1, 0, 0, 1, 0, 1],
            b'-' => [1, 0, 1, 0, 0, 1, 1, 0, 1],
            b'$' => [1, 0, 1, 1, 0, 0, 1, 0, 1],
            b':' => [1, 1, 0, 1, 0, 1, 1, 0, 1, 1],
            b'/' => [1, 1, 0, 1, 1, 0, 1, 0, 1, 1],
            b'.' => [1, 1, 0, 1, 1, 0, 1, 1, 0, 1],
            b'+' => [1, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1],
            b'A' => [1, 0, 1, 1, 0, 0, 1, 0, 0, 1],
            b'B' => [1, 0, 1, 0, 0, 1, 0, 0, 1, 1],
            b'C' => [1, 0, 0, 1, 0, 0, 1, 0, 1, 1],
            b'D' => [1, 0, 1, 0, 0, 1, 1, 0, 0, 1]
        });
    }
}

impl<'a> crate::Barcode<'a> for Codabar<'a> {
    const SIZE: Range<u16> = 1..256;
    const ALLOWED_VALUES: &'static [u8] = &[
        b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'-', b'$', b':', b'/', b'.',
        b'+', b'A', b'B', b'C', b'D',
    ];
    fn new(data: &'a [u8]) -> Result<Self> {
        Self::validate(data)?;
        Ok(Self(data))
    }
    fn encode_in_place(&self, buffer: &mut [u8]) -> Option<()> {
        let sum = self.get_sum();
        if buffer.len() < sum {
            return None;
        }
        self.encode_into(buffer);
        Some(())
    }
    fn encode(&self) -> Vec<u8> {
        let sum = self.get_sum();
        let mut buffer = vec![0; sum];
        self.encode_into(&mut buffer);
        buffer
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Error;
    use crate::sym::codabar::*;
    use crate::Barcode;
    #[cfg(not(feature = "std"))]
    use alloc::string::String;
    use core::char;

    fn collapse_vec(v: Vec<u8>) -> String {
        let chars = v.iter().map(|d| char::from_digit(*d as u32, 10).unwrap());
        chars.collect()
    }

    #[test]
    fn invalid_length_codabar() {
        let codabar = Codabar::new(b"");

        assert_eq!(codabar.err().unwrap(), Error::Length);
    }

    #[test]
    fn invalid_data_codabar() {
        let codabar = Codabar::new(b"A12345G");

        assert_eq!(codabar.err().unwrap(), Error::Character);
    }

    #[test]
    fn codabar_encode() {
        let codabar_a = Codabar::new(b"A1234B").unwrap();
        let codabar_b = Codabar::new(b"A40156B").unwrap();

        let encoded = codabar_a.encode();
        let expected = vec![
            1, 0, 1, 1, 0, 0, 1, 0, 0, 1, // A
            0,
            1, 0, 1, 0, 1, 1, 0, 0, 1, // 1
            0,
            1, 0, 1, 0, 0, 1, 0, 1, 1, // 2
            0,
            1, 1, 0, 0, 1, 0, 1, 0, 1, // 3
            0,
            1, 0, 1, 1, 0, 1, 0, 0, 1, // 4
            0,
            1, 0, 1, 0, 0, 1, 0, 0, 1, 1, // B
        ];
        println!("{:?}", encoded);
        assert_eq!(encoded, expected);

        assert_eq!(
            collapse_vec(codabar_a.encode()),
            "1011001001010101100101010010110110010101010110100101010010011"
        );
        assert_eq!(
            collapse_vec(codabar_b.encode()),
            "10110010010101101001010101001101010110010110101001010010101101010010011"
        );
    }
}
