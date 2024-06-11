//! Encoder for UPC and EAN barcodes.
//!
//! EAN13 barcodes are very common in retail. 90% of the products you purchase from a supermarket
//! will use EAN13.
//!
//! This module defines types for:
//!   * UPC-A
//!   * EAN-13
//!   * Bookland
//!   * JAN

use super::*;

/// Left side A (odd parity) encoding mapping for EAN barcodes.
/// 1 = bar, 0 = no bar.
pub const ENCODING_LEFT_A: [[u8; 7]; 10] = [
    [0, 0, 0, 1, 1, 0, 1],
    [0, 0, 1, 1, 0, 0, 1],
    [0, 0, 1, 0, 0, 1, 1],
    [0, 1, 1, 1, 1, 0, 1],
    [0, 1, 0, 0, 0, 1, 1],
    [0, 1, 1, 0, 0, 0, 1],
    [0, 1, 0, 1, 1, 1, 1],
    [0, 1, 1, 1, 0, 1, 1],
    [0, 1, 1, 0, 1, 1, 1],
    [0, 0, 0, 1, 0, 1, 1],
];
/// Left side B (even parity) encoding mapping for EAN barcodes.
/// 1 = bar, 0 = no bar.
pub const ENCODING_LEFT_B: [[u8; 7]; 10] = [
    [0, 1, 0, 0, 1, 1, 1],
    [0, 1, 1, 0, 0, 1, 1],
    [0, 0, 1, 1, 0, 1, 1],
    [0, 1, 0, 0, 0, 0, 1],
    [0, 0, 1, 1, 1, 0, 1],
    [0, 1, 1, 1, 0, 0, 1],
    [0, 0, 0, 0, 1, 0, 1],
    [0, 0, 1, 0, 0, 0, 1],
    [0, 0, 0, 1, 0, 0, 1],
    [0, 0, 1, 0, 1, 1, 1],
];
/// Right side encoding mapping for EAN barcodes.
/// 1 = bar, 0 = no bar.
pub const ENCODING_RIGHT: [[u8; 7]; 10] = [
    [1, 1, 1, 0, 0, 1, 0],
    [1, 1, 0, 0, 1, 1, 0],
    [1, 1, 0, 1, 1, 0, 0],
    [1, 0, 0, 0, 0, 1, 0],
    [1, 0, 1, 1, 1, 0, 0],
    [1, 0, 0, 1, 1, 1, 0],
    [1, 0, 1, 0, 0, 0, 0],
    [1, 0, 0, 0, 1, 0, 0],
    [1, 0, 0, 1, 0, 0, 0],
    [1, 1, 1, 0, 1, 0, 0],
];

/// Maps parity (odd/even) for the left-side digits based on the first digit in
/// the number system portion of the barcode data.
const PARITY: [[bool; 5]; 10] = [
    [false, false, false, false, false],
    [false,  true, false,  true,  true],
    [false,  true,  true, false,  true],
    [false,  true,  true,  true, false],
    [ true, false, false,  true,  true],
    [ true,  true, false, false,  true],
    [ true,  true,  true, false, false],
    [ true, false,  true, false,  true],
    [ true, false,  true,  true, false],
    [ true,  true, false,  true, false],
];

/// The left-hand guard pattern.
pub const LEFT_GUARD: [u8; 3] = [1, 0, 1];
/// The middle guard pattern.
pub const MIDDLE_GUARD: [u8; 5] = [0, 1, 0, 1, 0];
/// The right-hand guard pattern.
pub const RIGHT_GUARD: [u8; 3] = [1, 0, 1];

/// Calculate the modulo-10 checksum for the given data.
/// The data should be the digits of the barcode, excluding the checksum digit.
/// The checksum digit is returned (0-9).
/// The `even_start` parameter is used to determine whether the first digit in the
/// data is an even or odd digit. This is used to deferentiate between EAN-13 and
/// EAN-8 barcodes.
pub fn modulo_10_checksum(data: &[u8], even_start: bool) -> u8 {
    let mut odds = 0;
    let mut evens = 0;

    for (i, d) in data.iter().enumerate() {
        match i % 2 {
            1 => odds += *d,
            _ => evens += *d,
        }
    }

    // EAN-13 (and some others?) barcodes use EVEN-first weighting to maintain
    // backwards compatibility.
    if even_start {
        odds *= 3;
    } else {
        evens *= 3;
    }

    match 10 - ((odds + evens) % 10) {
        10 => 0,
        n => n,
    }
}

/// The EAN-13 barcode type.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EAN13([u8; 12]);

const OUTPUT_SIZE: usize = 95;

impl EAN13 {
    fn encode_into(&self, buffer: &mut [u8]) {
        let mut i = 0;

        // Identify the parity for the left-side digits.
        let parity = PARITY[self.0[0] as usize];

        // Left guard
        for bit in LEFT_GUARD {
            buffer[i] = bit;
            i += 1;
        }

        // Number system
        for bit in ENCODING_LEFT_A[self.0[1] as usize] {
            buffer[i] = bit;
            i += 1;
        }

        // Left Payload
        let mut j = 0;
        for digit in &self.0[2..7] {
            let bits =
            if parity[j] {
                ENCODING_LEFT_B[*digit as usize]
            } else {
                ENCODING_LEFT_A[*digit as usize]
            }
            ;
            for bit in bits {
                buffer[i] = bit;
                i += 1;
            }
            j += 1;
        }

        // Middle guard
        for bit in MIDDLE_GUARD {
            buffer[i] = bit;
            i += 1;
        }

        // Right payload
        for digit in &self.0[7..] {
            for bit in ENCODING_RIGHT[*digit as usize] {
                buffer[i] = bit;
                i += 1;
            }
        }

        // Checksum
        let checksum = self.checksum();
        for bit in ENCODING_RIGHT[checksum as usize] {
            buffer[i] = bit;
            i += 1;
        }

        // Right guard
        for bit in RIGHT_GUARD {
            buffer[i] = bit;
            i += 1;
        }
    }

    fn checksum(&self) -> u8 {
        modulo_10_checksum(&self.0, true)
    }
}

impl<'a> Barcode<'a> for EAN13 {
    fn new(data: &'a [u8]) -> Result<Self> {
        if data.len() != 12 && data.len() != 13 {
            return Err(Error::Length);
        }
        let mut digits = [0; 12];
        for i in 0..12 {
            let byte = data[i];
            if byte < b'0' || byte > b'9' {
                return Err(Error::Character);
            }
            digits[i] = byte - b'0';
        }
        let this = Self(digits);

        // If checksum digit is provided, check the checksum.
        if data.len() == 13 {
            if this.checksum() != data[12] - b'0' {
                return Err(Error::Checksum);
            }
        }

        Ok(this)
    }

    fn encode_in_place(&self, buffer: &mut [u8]) -> Option<()> {
        if buffer.len() < OUTPUT_SIZE {
            return None;
        }
        self.encode_into(buffer);
        Some(())
    }

    #[cfg(feature = "alloc")]
    fn encode(&self) -> Vec<u8> {
        let mut buffer = vec![0; OUTPUT_SIZE];
        self.encode_into(&mut buffer);
        buffer
    }
}

/// The Bookland barcode type.
/// Bookland are EAN-13 that use number system 978.
pub type Bookland = EAN13;

/// The UPC-A barcode type.
/// UPC-A are EAN-13 that start with a 0.
pub type UPCA = EAN13;

/// The JAN barcode type.
/// JAN are EAN-13 that use number system 49.
pub type JAN = EAN13;

#[cfg(test)]
mod tests {
    use super::*;

    fn collapse_vec(v: Vec<u8>) -> String {
        let chars = v.iter().map(|d| char::from_digit(*d as u32, 10).unwrap());
        chars.collect()
    }

    #[test]
    fn new_ean13() {
        let ean13 = EAN13::new(b"123456123456");

        assert!(ean13.is_ok());
    }

    #[test]
    fn new_bookland() {
        let bookland = Bookland::new(b"978456123456");

        assert!(bookland.is_ok());
    }

    #[test]
    fn invalid_data_ean13() {
        let ean13 = EAN13::new(b"1234er123412");

        assert_eq!(ean13.err().unwrap(), Error::Character)
    }

    #[test]
    fn invalid_len_ean13() {
        let ean13 = EAN13::new(b"1111112222222333333");

        assert_eq!(ean13.err().unwrap(), Error::Length)
    }

    #[test]
    fn invalid_checksum_ean13() {
        let ean13 = EAN13::new(b"8801051294881");

        assert_eq!(ean13.err().unwrap(), Error::Checksum)
    }

    #[test]
    fn ean13_encode_as_upca() {
        let ean131 = UPCA::new(b"012345612345").unwrap(); // Check digit: 8
        let ean132 = UPCA::new(b"000118999561").unwrap(); // Check digit: 3

        assert_eq!(collapse_vec(ean131.encode()), "10100110010010011011110101000110110001010111101010110011011011001000010101110010011101001000101");
        assert_eq!(collapse_vec(ean132.encode()), "10100011010001101001100100110010110111000101101010111010011101001001110101000011001101000010101");
    }

    #[test]
    fn ean13_encode_as_bookland() {
        let bookland1 = Bookland::new(b"978345612345").unwrap(); // Check digit: 5
        let bookland2 = Bookland::new(b"978118999561").unwrap(); // Check digit: 5

        assert_eq!(collapse_vec(bookland1.encode()), "10101110110001001010000101000110111001010111101010110011011011001000010101110010011101001110101");
        assert_eq!(collapse_vec(bookland2.encode()), "10101110110001001011001100110010001001000101101010111010011101001001110101000011001101001110101");
    }

    #[test]
    fn ean13_encode() {
        let ean131 = EAN13::new(b"750103131130").unwrap(); // Check digit: 5
        let ean132 = EAN13::new(b"983465123499").unwrap(); // Check digit: 5

        assert_eq!(collapse_vec(ean131.encode()), "10101100010100111001100101001110111101011001101010100001011001101100110100001011100101110100101");
        assert_eq!(collapse_vec(ean132.encode()), "10101101110100001001110101011110111001001100101010110110010000101011100111010011101001000010101");
    }
}
