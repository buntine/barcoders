//! Encoder for Code93 barcodes.
//!
//! Code93 is intented to improve upon Code39 barcodes by offering a wider array of encodable
//! ASCII characters. It also produces denser barcodes than Code39.
//!
//! Code93 is a continuous, variable-length symbology.
//!
//! NOTE: This encoder currently only supports the basic Code93 implementation and not full-ASCII
//! mode.

use super::*;

const CHARS_COUNT: usize = 47;
const CHAR_SIZE: usize = 9;
type Char = [u8; CHAR_SIZE];
// Character -> Binary mappings for each of the 47 allowable character.
// The special "full-ASCII" characters are represented with (, ), [, ].
const CHARS: [(u8, Char); CHARS_COUNT] = [
    (b'0', [1, 0, 0, 0, 1, 0, 1, 0, 0]),
    (b'1', [1, 0, 1, 0, 0, 1, 0, 0, 0]),
    (b'2', [1, 0, 1, 0, 0, 0, 1, 0, 0]),
    (b'3', [1, 0, 1, 0, 0, 0, 0, 1, 0]),
    (b'4', [1, 0, 0, 1, 0, 1, 0, 0, 0]),
    (b'5', [1, 0, 0, 1, 0, 0, 1, 0, 0]),
    (b'6', [1, 0, 0, 1, 0, 0, 0, 1, 0]),
    (b'7', [1, 0, 1, 0, 1, 0, 0, 0, 0]),
    (b'8', [1, 0, 0, 0, 1, 0, 0, 1, 0]),
    (b'9', [1, 0, 0, 0, 0, 1, 0, 1, 0]),
    (b'A', [1, 1, 0, 1, 0, 1, 0, 0, 0]),
    (b'B', [1, 1, 0, 1, 0, 0, 1, 0, 0]),
    (b'C', [1, 1, 0, 1, 0, 0, 0, 1, 0]),
    (b'D', [1, 1, 0, 0, 1, 0, 1, 0, 0]),
    (b'E', [1, 1, 0, 0, 1, 0, 0, 1, 0]),
    (b'F', [1, 1, 0, 0, 0, 1, 0, 1, 0]),
    (b'G', [1, 0, 1, 1, 0, 1, 0, 0, 0]),
    (b'H', [1, 0, 1, 1, 0, 0, 1, 0, 0]),
    (b'I', [1, 0, 1, 1, 0, 0, 0, 1, 0]),
    (b'J', [1, 0, 0, 1, 1, 0, 1, 0, 0]),
    (b'K', [1, 0, 0, 0, 1, 1, 0, 1, 0]),
    (b'L', [1, 0, 1, 0, 1, 1, 0, 0, 0]),
    (b'M', [1, 0, 1, 0, 0, 1, 1, 0, 0]),
    (b'N', [1, 0, 1, 0, 0, 0, 1, 1, 0]),
    (b'O', [1, 0, 0, 1, 0, 1, 1, 0, 0]),
    (b'P', [1, 0, 0, 0, 1, 0, 1, 1, 0]),
    (b'Q', [1, 1, 0, 1, 1, 0, 1, 0, 0]),
    (b'R', [1, 1, 0, 1, 1, 0, 0, 1, 0]),
    (b'S', [1, 1, 0, 1, 0, 1, 1, 0, 0]),
    (b'T', [1, 1, 0, 1, 0, 0, 1, 1, 0]),
    (b'U', [1, 1, 0, 0, 1, 0, 1, 1, 0]),
    (b'V', [1, 1, 0, 0, 1, 1, 0, 1, 0]),
    (b'W', [1, 0, 1, 1, 0, 1, 1, 0, 0]),
    (b'X', [1, 0, 1, 1, 0, 0, 1, 1, 0]),
    (b'Y', [1, 0, 0, 1, 1, 0, 1, 1, 0]),
    (b'Z', [1, 0, 0, 1, 1, 1, 0, 1, 0]),
    (b'-', [1, 0, 0, 1, 0, 1, 1, 1, 0]),
    (b'.', [1, 1, 1, 0, 1, 0, 1, 0, 0]),
    (b' ', [1, 1, 1, 0, 1, 0, 0, 1, 0]),
    (b'$', [1, 1, 1, 0, 0, 1, 0, 1, 0]),
    (b'/', [1, 0, 1, 1, 0, 1, 1, 1, 0]),
    (b'+', [1, 0, 1, 1, 1, 0, 1, 1, 0]),
    (b'%', [1, 1, 0, 1, 0, 1, 1, 1, 0]),
    (b'(', [1, 0, 0, 1, 0, 0, 1, 1, 0]),
    (b')', [1, 1, 1, 0, 1, 1, 0, 1, 0]),
    (b'[', [1, 1, 1, 0, 1, 0, 1, 1, 0]),
    // https://github.com/buntine/barcoders/issues/37 go BRRR
    (b']', [1, 0, 0, 1, 1, 0, 0, 1, 0]),
];

// Code93 barcodes must start and end with the '*' special character.
const GUARD: [u8; CHAR_SIZE] = [1, 0, 1, 0, 1, 1, 1, 1, 0];
const TERMINATOR: [u8; 1] = [1];

/// The Code93 barcode type.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Code93<'a>(&'a [u8]);

/// Generates a checksum character using a weighted modulo-47 algorithm.
///
/// Returns the Byte representation of the checksum character
fn checksum_char(data: &[u8], weight_threshold: usize, c_checksum: Option<u8>) -> u8 {
    let mut datalen = data.len();
    if c_checksum.is_some() {
        datalen += 1;
    }
    let weight = |i| {
        let n = (datalen - i) % weight_threshold;
        if n == 0 {
            return weight_threshold;
        }
        n
    };
    let mut sum = 0;
    for (index, byte) in data.iter().enumerate() {
        let pos = char2id(byte);
        let weight = weight(index);
        println!("i: {index}, pos: {pos}, weight: {weight}");
        sum += weight * pos;
    }

    if let Some(byte) = c_checksum {
        let index = data.len();
        let pos = char2id(&byte);
        let weight = weight(index);
        println!("i: {index}, pos: {pos}, weight: {weight}");
        sum += weight * pos;
    }

    let sum = sum % CHARS_COUNT;
    CHARS[sum].0
}

fn char2id(c: &u8) -> usize {
    #[cfg(not(feature = "unsafety"))]
    {
        CHARS.iter().position(|t| t.0 == *c).unwrap()
    }
    #[cfg(feature = "unsafety")]
    unsafe {
        CHARS.iter().position(|t| t.0 == *c).unwrap_unchecked()
    }
}

impl<'a> Code93<'a> {
    #[inline]
    fn calc_checksum_c(&self) -> u8 {
        checksum_char(self.0, 20, None)
    }

    #[inline]
    fn calc_checksum_k(&self, c_checksum: u8) -> u8 {
        checksum_char(self.0, 15, Some(c_checksum))
    }

    // I know I can simplify this, but I feel like this makes it WAY more readable.
    fn calc_sum(&self) -> usize {
        let checksum_c = CHAR_SIZE;
        let checksum_k = CHAR_SIZE;
        let payload = self.0.len() * CHAR_SIZE;
        let guard = CHAR_SIZE;
        let terminator = 1;
        guard + payload + checksum_c + checksum_k + guard + terminator
    }

    fn encode_into(&self, buffer: &mut [u8]) {
        let mut i = 0;
        for &bit in &GUARD {
            buffer[i] = bit;
            i += 1;
        }

        for &byte in self.0 {
            let index = CHARS.iter().position(|t| t.0 == byte).unwrap();
            for &bit in &CHARS[index].1 {
                buffer[i] = bit;
                i += 1;
            }
        }

        let c_checksum = self.calc_checksum_c();
        let c_checksum_bits = CHARS.iter().position(|t| t.0 == c_checksum).unwrap();
        let k_checksum = self.calc_checksum_k(c_checksum);
        let k_checksum_bits = CHARS.iter().position(|t| t.0 == k_checksum).unwrap();

        for bit in CHARS[c_checksum_bits].1 {
            buffer[i] = bit;
            i += 1;
        }

        for bit in CHARS[k_checksum_bits].1 {
            buffer[i] = bit;
            i += 1;
        }

        for &bit in &GUARD {
            buffer[i] = bit;
            i += 1;
        }

        for &bit in &TERMINATOR {
            buffer[i] = bit;
            i += 1;
        }
    }
}

impl<'a> Barcode<'a> for Code93<'a> {
    const CHARS: &'static [u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ-. $/+%()[]";
    const SIZE: Range<u16> = 1..256;
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
    fn invalid_length_code93() {
        let code93 = Code93::new(b"");

        assert_eq!(code93.err().unwrap(), error::Error::Length);
    }

    #[test]
    fn invalid_data_code93() {
        let code93 = Code93::new(b"lowerCASE");

        assert_eq!(code93.err().unwrap(), error::Error::Character);
    }

    #[test]
    fn code93_encode() {
        // Tests for data longer than 15, data longer than 20
        let code931 = Code93::new(b"TEST93").unwrap();
        let code932 = Code93::new(b"FLAM").unwrap();
        let code933 = Code93::new(b"99").unwrap();
        let code934 = Code93::new(b"1111111111111111111111").unwrap();

        let expected = vec![
            1, 0, 1, 0, 1, 1, 1, 1, 0, // Start guard
            1, 1, 0, 1, 0, 0, 1, 1, 0, // T
            1, 1, 0, 0, 1, 0, 0, 1, 0, // E
            1, 1, 0, 1, 0, 1, 1, 0, 0, // S
            1, 1, 0, 1, 0, 0, 1, 1, 0, // T
            1, 0, 0, 0, 0, 1, 0, 1, 0, // 9
            1, 0, 1, 0, 0, 0, 0, 1, 0, // 3
            1, 0, 1, 1, 1, 0, 1, 1, 0, // Checksum C
            1, 0, 0, 1, 0, 0, 0, 1, 0, // Checksum K
            1, 0, 1, 0, 1, 1, 1, 1, 0, // End guard
            1, // Terminator
        ];
        assert_eq!(expected, code931.encode());
        assert_eq!(
            collapse_vec(code932.encode()),
            "1010111101100010101010110001101010001010011001001011001010011001010111101"
        );
        assert_eq!(
            collapse_vec(code933.encode()),
            "1010111101000010101000010101101100101000101101010111101"
        );
        assert_eq!(collapse_vec(code934.encode()), "1010111101010010001010010001010010001010010001010010001010010001010010001010010001010010001010010001010010001010010001010010001010010001010010001010010001010010001010010001010010001010010001010010001010010001000101101110010101010111101");
    }
}
