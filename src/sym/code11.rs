//! Encoder for Code11 (USD-8) barcodes.
//!
//! Code11 is able to encode all of the decimal digits and the dash character. It is mainly
//! used in the telecommunications industry.
//!
//! Code11 is a discrete symbology. This encoder always provides a C checksum. For barcodes longer
//! than 10 characters, a second checksum digit (K) is appended.

use super::*;

/// Maps an unicode character to its value in the Code11 encoding.
/// 
/// `'0'`-`'9'` -> `0`-`9` and `'-'` -> `10`
fn char_lookup(c: &u8) -> usize {
    match c {
        b'0'..=b'9' => (c - b'0') as usize,
        b'-' => 10,
        _ => unreachable!("No position of illegal character"),
    }
}

/// Calculates a checksum character using a weighted modulo-11 algorithm.
fn checksum_char(data: &[u8], weight_threshold: usize, c_checksum: Option<u8>) -> u8 {
    let weight = |i| {
        let n = i % weight_threshold;
        if n == 0 {
            return weight_threshold;
        }
        n
    };

    let positions = data.iter().map(char_lookup);
    let weight_mod = if c_checksum.is_some() { 2 } else { 1 };
    let mut index = positions
        .rev()
        .enumerate()
        .fold(0, |acc, (i, pos)| acc + (weight(i + weight_mod) * pos));
    if let Some(c) = c_checksum {
        index = index + char_lookup(&c);
    }

    // Some sources suggest that the C checksum should use modulo-11, whilst the K
    // checksum should use modulo-9. But most generators always use modulo-11.
    // This algorithm currently just uses 11 for both checksums, but can be easily
    // changed at a later date.
    let index = index % 11; // 11 is the modulo value
    if index == 10 {
        return b'-';
    }
    index as u8 + b'0'
}

// Code11 barcodes must start and end with a special character.
const GUARD_LENGTH: usize = 7;
const GUARD: [u8; GUARD_LENGTH] = [1, 0, 1, 1, 0, 0, 1];
const PADDING: usize = 1;
const SEPARATOR: u8 = 0;

/// The Code11 barcode type.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Code11<'a>(&'a [u8]);

impl<'a> Code11<'a> {
    #[inline]
    fn calc_c_checksum(&self) -> u8 {
        checksum_char(self.0, 10, None)
    }

    #[inline]
    fn calc_k_checksum(&self, c_checksum: u8) -> u8 {
        checksum_char(self.0, 9, Some(c_checksum))
    }

    fn calc_sum_and_checksums(&self) -> (usize, u8, Option<u8>) {
        let mut payload_len = 0;
        for byte in self.0.iter() {
            match byte {
                b'0' | b'9' | b'-' => payload_len += 6,
                b'1'..=b'8' => payload_len += 7,
                #[cfg(not(feature = "blitz"))]
                _ => unreachable!("Validation did not catch an illegal character"),
                #[cfg(feature = "blitz")]
                _ => unsafe { core::hint::unreachable_unchecked() },
            }
            payload_len += PADDING;
        }

        let c_checksum = self.calc_c_checksum();
        match c_checksum {
            b'0' | b'9' | b'-' => payload_len += 6,
            b'1'..=b'8' => payload_len += 7,
            #[cfg(not(feature = "blitz"))]
            _ => unreachable!("Checksum C is not a valid character"),
            #[cfg(feature = "blitz")]
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
        payload_len += PADDING; // Padding after checksum C

        let mut k_checksum = u8::MAX; // Used to represent absence of K-checksum
        if self.0.len() > 10 { // K-checksum is only appended on barcodes greater than 10 characters.
            k_checksum = self.calc_k_checksum(c_checksum);
            match k_checksum {
                b'0' | b'9' | b'-' => payload_len += 6,
                b'1'..=b'8' => payload_len += 7,
                #[cfg(not(feature = "blitz"))]
                _ => unreachable!("Checksum K is not a valid character"),
                #[cfg(feature = "blitz")]
                _ => unsafe { core::hint::unreachable_unchecked() },
            }
            payload_len += PADDING; // Padding after checksum K
        }

        // The total length of the barcode is the sum of the payload length, the guard lengths
        let sum = GUARD_LENGTH + PADDING + payload_len + GUARD_LENGTH;

        // Do we actually have a K-checksum?
        if k_checksum != u8::MAX {
            return (sum, c_checksum, Some(k_checksum));
        }

        (sum, c_checksum, None)
    }

    fn encode_into(&self, buffer: &mut [u8], c: u8, k: Option<u8>) {
        let mut i = 0;
        // Start guard
        for &byte in GUARD.iter() {
            buffer[i] = byte;
            i += 1;
        }

        macro_rules! enc {
            ($v:ident) => ( __encode!((buffer, i) $v {
                b'0' => ([1, 0, 1, 0, 1, 1]),
                b'1' => ([1, 1, 0, 1, 0, 1, 1]),
                b'2' => ([1, 0, 0, 1, 0, 1, 1]),
                b'3' => ([1, 1, 0, 0, 1, 0, 1]),
                b'4' => ([1, 0, 1, 1, 0, 1, 1]),
                b'5' => ([1, 1, 0, 1, 1, 0, 1]),
                b'6' => ([1, 0, 0, 1, 1, 0, 1]),
                b'7' => ([1, 0, 1, 0, 0, 1, 1]),
                b'8' => ([1, 1, 0, 1, 0, 0, 1]),
                b'9' => ([1, 1, 0, 1, 0, 1]),
                b'-' => ([1, 0, 1, 1, 0, 1]),
            }) );
        }

        // Padding
        buffer[i] = SEPARATOR;
        i += PADDING;

        // Payload
        for byte in self.0.iter() {
            enc!(byte);

            // Padding
            buffer[i] = SEPARATOR;
            i += PADDING;
        }

        // C checksum
        enc!(c);

        // Padding
        buffer[i] = SEPARATOR;
        i += PADDING;

        // K checksum
        if let Some(k) = k {
            enc!(k);

            // Padding
            buffer[i] = SEPARATOR;
            i += PADDING;
        }

        // End guard
        for &byte in GUARD.iter() {
            buffer[i] = byte;
            i += 1;
        }
    }
}

impl<'a> BarcodeDevExt<'a> for Code11<'a> {
    const SIZE: Range<u16> = 1..256;
    const CHARS: &'static [u8] = b"0123456789-";
}

impl<'a> Barcode<'a> for Code11<'a> {
    fn new(data: &'a [u8]) -> Result<Self> {
        Self::validate(data).map(Self)
    }

    fn encode_in_place(&self, buffer: &mut [u8]) -> Option<()> {
        let (sum, c, k) = self.calc_sum_and_checksums();
        if buffer.len() < sum {
            return None;
        }
        self.encode_into(buffer, c, k);
        Some(())
    }

    #[cfg(feature = "alloc")]
    fn encode(&self) -> Vec<u8> {
        let (sum, c, k) = self.calc_sum_and_checksums();
        let mut buffer = vec![0; sum];
        self.encode_into(&mut buffer, c, k);
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
    fn invalid_length_code11() {
        let code11 = Code11::new(b"");

        assert_eq!(code11.err().unwrap(), Error::Length);
    }

    #[test]
    fn invalid_data_code11() {
        let code11 = Code11::new(b"NOTDIGITS");

        assert_eq!(code11.err().unwrap(), Error::Character);
    }

    #[test]
    fn code11_encode_less_than_10_chars() {
        let code111 = Code11::new(b"123-45").unwrap();
        let code112 = Code11::new(b"666").unwrap();
        let code113 = Code11::new(b"12-9").unwrap();

        let code111encoded = code111.encode();
        let expected = vec![
            1, 0, 1, 1, 0, 0, 1, // Start guard
            0, // Padding
            1, 1, 0, 1, 0, 1, 1, // 1
            0, // Padding
            1, 0, 0, 1, 0, 1, 1, // 2
            0, // Padding
            1, 1, 0, 0, 1, 0, 1, // 3
            0, // Padding
            1, 0, 1, 1, 0, 1, // -
            0, // Padding
            1, 0, 1, 1, 0, 1, 1, // 4
            0, // Padding
            1, 1, 0, 1, 1, 0, 1, // 5
            0, // Padding
            1, 1, 0, 1, 1, 0, 1, // C Checksum
            0, // Padding
            1, 0, 1, 1, 0, 0, 1, // End guard
        ];
        assert_eq!(expected, code111encoded);
        assert_eq!(
            "10110010100110101001101010011010110010101011001",
            collapse_vec(code112.encode())
        );
        assert_eq!(
            "10110010110101101001011010110101101010100110101011001",
            collapse_vec(code113.encode())
        );
    }

    #[test]
    fn code11_encode_more_than_10_chars() {
        let code111 = Code11::new(b"1234-5678-4321").unwrap();

        assert_eq!(
            "101100101101011010010110110010101011011010110101101101010011010101001101101001010110101011011011001010100101101101011011011010100110101011001",
            collapse_vec(code111.encode()),
        );
    }
}
