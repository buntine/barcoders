//! Encoder for Codabar barcodes.
//!
//! Codabar is a simple, self-checking symbology without a standard for a checksum digit.
//!
//! Codabar is used in the USA by FedEx, some Hospitals, and photo labs.
//!
//! Barcodes of this variant should start and end with either A, B, C, or D depending on
//! the industry.

use core::ops::Range;
use super::*;

/// The Codabar barcode type.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Codabar<'a>(&'a [u8]);

impl<'a> Codabar<'a> {
    fn calc_sum(&self) -> usize {
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
        for byte in self.0.iter() {
            __encode!((buffer, i) byte {
                b'0' => ([1, 0, 1, 0, 1, 0, 0, 1, 1]),
                b'1' => ([1, 0, 1, 0, 1, 1, 0, 0, 1]),
                b'2' => ([1, 0, 1, 0, 0, 1, 0, 1, 1]),
                b'3' => ([1, 1, 0, 0, 1, 0, 1, 0, 1]),
                b'4' => ([1, 0, 1, 1, 0, 1, 0, 0, 1]),
                b'5' => ([1, 1, 0, 1, 0, 1, 0, 0, 1]),
                b'6' => ([1, 0, 0, 1, 0, 1, 0, 1, 1]),
                b'7' => ([1, 0, 0, 1, 0, 1, 1, 0, 1]),
                b'8' => ([1, 0, 0, 1, 1, 0, 1, 0, 1]),
                b'9' => ([1, 1, 0, 1, 0, 0, 1, 0, 1]),
                b'-' => ([1, 0, 1, 0, 0, 1, 1, 0, 1]),
                b'$' => ([1, 0, 1, 1, 0, 0, 1, 0, 1]),
                b':' => ([1, 1, 0, 1, 0, 1, 1, 0, 1, 1]),
                b'/' => ([1, 1, 0, 1, 1, 0, 1, 0, 1, 1]),
                b'.' => ([1, 1, 0, 1, 1, 0, 1, 1, 0, 1]),
                b'+' => ([1, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1]),
                b'A' => ([1, 0, 1, 1, 0, 0, 1, 0, 0, 1]),
                b'B' => ([1, 0, 1, 0, 0, 1, 0, 0, 1, 1]),
                b'C' => ([1, 0, 0, 1, 0, 0, 1, 0, 1, 1]),
                b'D' => ([1, 0, 1, 0, 0, 1, 1, 0, 0, 1]),
            });
            // Don't forget the padding
            if i < buffer.len() {
                buffer[i] = 0;
                i += 1;
            }
        }
    }
}

impl<'a> BarcodeDevExt<'a> for Codabar<'a> {
    const SIZE: Range<u16> = 1..256;
    const CHARS: &'static [u8] = b"0123456789-$:/+.ABCD";
}

impl<'a> Barcode<'a> for Codabar<'a> {
    fn new(data: &'a [u8]) -> Result<Self> {
        Self::validate(data).map(Self)
    }

    fn encode_in_place(&self, buffer: &mut [u8]) -> Option<()> {
        let sum = self.calc_sum();
        if buffer.len() < sum {
            return None;
        }
        self.encode_into(buffer);
        Some(())
    }

    #[cfg(feature = "alloc")]
    fn encode(&self) -> Vec<u8> {
        let sum = self.calc_sum();
        let mut buffer = vec![0; sum];
        self.encode_into(&mut buffer);
        buffer
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

        assert_eq!(
            codabar_a.encode(),
            vec![
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
            ]
        );
        assert_eq!(
            collapse_vec(codabar_b.encode()),
            "10110010010101101001010101001101010110010110101001010010101101010010011"
        );
    }
}
