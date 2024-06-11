//! Encoder for Code39 barcodes.
//!
//! Code39 is a discrete, variable-length barcode. They are often referred to as "3-of-9".
//!
//! Code39 is the standard barcode used by the United States Department of Defense and is also
//! popular in non-retail environments. It was one of the first symbologies to support encoding
//! of the ASCII alphabet.

use super::*;

const CHARS_COUNT: usize = 43;
const CHAR_SIZE: usize = 12;
const CHARS: [(u8, [u8; CHAR_SIZE]); CHARS_COUNT] = [
    (b'0', [1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1]),
    (b'1', [1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1]),
    (b'2', [1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1]),
    (b'3', [1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1]),
    (b'4', [1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1]),
    (b'5', [1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1]),
    (b'6', [1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 1]),
    (b'7', [1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1]),
    (b'8', [1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1]),
    (b'9', [1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1]),
    (b'A', [1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1]),
    (b'B', [1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1]),
    (b'C', [1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1]),
    (b'D', [1, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1]),
    (b'E', [1, 1, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1]),
    (b'F', [1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1]),
    (b'G', [1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 1]),
    (b'H', [1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1]),
    (b'I', [1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1]),
    (b'J', [1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1]),
    (b'K', [1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 1]),
    (b'L', [1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1]),
    (b'M', [1, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1]),
    (b'N', [1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1]),
    (b'O', [1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1]),
    (b'P', [1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1]),
    (b'Q', [1, 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 1]),
    (b'R', [1, 1, 0, 1, 0, 1, 0, 1, 1, 0, 0, 1]),
    (b'S', [1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1]),
    (b'T', [1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1]),
    (b'U', [1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1]),
    (b'V', [1, 0, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1]),
    (b'W', [1, 1, 0, 0, 1, 1, 0, 1, 0, 1, 0, 1]),
    (b'X', [1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1]),
    (b'Y', [1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1]),
    (b'Z', [1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1]),
    (b'-', [1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 1, 1]),
    (b'.', [1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 1]),
    (b' ', [1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1]),
    (b'$', [1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1]),
    (b'/', [1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1]),
    (b'+', [1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1]),
    (b'%', [1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1]),
];

const PADDING_SIZE: usize = 1;
const PADDING: u8 = 0;

// Code39 barcodes must start and end with the '*' special character.
const GUARD: [u8; CHAR_SIZE] = [1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1];

/// The Code39 barcode type.
// #[cfg_attr(feature = "nightly", repr(packed))] // May be useful for embedded systems.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Code39<'a> {
    /// Indicates whether to encode a checksum digit.
    pub checksum: bool,
    data: &'a [u8],
}

fn char2id(c: &u8) -> usize {
    #[cfg(not(feature = "blitz"))]
    {
        CHARS.iter().position(|t| t.0 == *c).unwrap()
    }
    #[cfg(feature = "blitz")]
    unsafe {
        CHARS.iter().position(|t| t.0 == *c)
            .unwrap_unchecked()
    }
}

impl<'a> Code39<'a> {
    fn calc_sum(&self) -> usize {
        let mut payload = self.data.len() * (CHAR_SIZE + PADDING_SIZE);
        if self.checksum {
            payload += CHAR_SIZE + PADDING_SIZE;
        }
        // Guards at the beginning and end
        return CHAR_SIZE + PADDING_SIZE + payload + CHAR_SIZE;
    }

    fn calc_checksum(&self) -> [u8; 12] {
        let indices = self.data.iter().map(char2id);
        let index = indices.sum::<usize>() % CHARS_COUNT;
        CHARS[index].1
    }

    fn encode_into(&self, buffer: &mut [u8]) {
        let mut i = 0;
        for &bit in &GUARD {
            buffer[i] = bit;
            i += 1;
        }

        buffer[i] = PADDING;
        i += 1;
        
        for byte in self.data {
            let index = char2id(byte);
            for &bit in &CHARS[index].1 {
                buffer[i] = bit;
                i += 1;
            }
            
            // Padding
            buffer[i] = PADDING;
            i += 1;
        }

        if self.checksum {
            let checksum = self.calc_checksum();
            for &bit in &checksum {
                buffer[i] = bit;
                i += 1;
            }
            buffer[i] = PADDING;
            i += 1;
        }

        for &bit in &GUARD {
            buffer[i] = bit;
            i += 1;
        }
    }

    /// Creates a new Code39 barcode, with the checksum enabled.
    pub fn with_checksum(data: &'a [u8]) -> Result<Self> {
        Self::validate(data).map(|data| Self {
            checksum: true,
            data,
        })
    }
}

impl<'a> BarcodeDevExt<'a> for Code39<'a> {
    const SIZE: Range<u16> = 1..256;
    const CHARS: &'static [u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ-. $/+%";
}

impl<'a> Barcode<'a> for Code39<'a> {
    fn new(data: &'a [u8]) -> Result<Self> {
        Self::validate(data).map(|data| Self {
            checksum: false,
            data,
        })
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

// impl Parse for Code39 {
//     fn valid_len() -> Range<u32> {
//         1..256
//     }
// 
//     fn valid_chars() -> Vec<char> {
//         let (chars, _): (Vec<_>, Vec<_>) = CHARS.iter().cloned().unzip();
//         chars
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn collapse_vec(v: Vec<u8>) -> String {
        let chars = v.iter().map(|d| char::from_digit(*d as u32, 10).unwrap());
        chars.collect()
    }

    #[test]
    fn new_code39() {
        let code39 = Code39::new(b"12345");

        assert!(code39.is_ok());
    }

    #[test]
    fn invalid_data_code39() {
        let code39 = Code39::new(b"1212s");

        assert_eq!(code39.err().unwrap(), Error::Character);
    }

    #[test]
    fn invalid_len_code39() {
        let code39 = Code39::new(b"");

        assert_eq!(code39.err().unwrap(), Error::Length);
    }

    #[test]
    fn code39_encode() {
        let code391 = Code39::new(b"1234").unwrap();
        let code392 = Code39::new(b"983RD512").unwrap();
        let code393 = Code39::new(b"TEST8052").unwrap();

        assert_eq!(
            "10010110110101101001010110101100101011011011001010101010011010110100101101101",
            collapse_vec(code391.encode())
        );
        assert_eq!(
            "100101101101010110010110101101001011010110110010101011010101100101010110010110110100110101011010010101101011001010110100101101101",
            collapse_vec(code392.encode())
        );
        assert_eq!(
            "100101101101010101101100101101011001010101101011001010101101100101101001011010101001101101011010011010101011001010110100101101101",
            collapse_vec(code393.encode())
        );
    }

    #[test]
    fn code39_encode_with_checksum() {
        let code391 = Code39::with_checksum(b"1234").unwrap();
        let code392 = Code39::with_checksum(b"983RD512").unwrap();

        assert_eq!(
            "100101101101011010010101101011001010110110110010101010100110101101101010010110100101101101",
            collapse_vec(code391.encode())
        );
        assert_eq!(
            "1001011011010101100101101011010010110101101100101010110101011001010101100101101101001101010110100101011010110010101101011011010010100101101101",
            collapse_vec(code392.encode())
        );
    }
}
