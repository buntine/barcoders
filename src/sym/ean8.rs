//! Encoder for EAN-8 barcodes.
//!
//! EAN-8 barcodes are EAN style barcodes for smaller packages on products like
//! cigaretts, chewing gum, etc where package space is limited.

use super::*;
use ean13::{
    ENCODING_LEFT_A, ENCODING_RIGHT,
    LEFT_GUARD, MIDDLE_GUARD, RIGHT_GUARD,
    modulo_10_checksum
};

/// The EAN-8 barcode type.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EAN8([u8; 7]);

const OUTPUT_SIZE: usize = 67;

impl EAN8 {
    fn encode_into(&self, buffer: &mut [u8]) {
        let mut i = 0;

        // Left guard
        for bit in LEFT_GUARD {
            buffer[i] = bit;
            i += 1;
        }

        // Number system
        for &digit in &self.0[0..2] {
            for bit in ENCODING_LEFT_A[digit as usize] {
                buffer[i] = bit;
                i += 1;
            }
        }

        // Left payload
        for &digit in &self.0[2..4] {
            for bit in ENCODING_LEFT_A[digit as usize] {
                buffer[i] = bit;
                i += 1;
            }
        }

        // Middle guard
        for bit in MIDDLE_GUARD {
            buffer[i] = bit;
            i += 1;
        }

        // Right payload
        for &digit in &self.0[4..] {
            for bit in ENCODING_RIGHT[digit as usize] {
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
        modulo_10_checksum(&self.0, false)
    }
}

impl<'a> Barcode<'a> for EAN8 {
    fn new(data: &'a [u8]) -> Result<Self> {
        if data.len() != 7 && data.len() != 8 {
            return Err(Error::Length);
        }
        let mut digits = [0; 7];
        for i in 0..7 {
            let byte = data[i];
            if byte < b'0' || byte > b'9' {
                return Err(Error::Character);
            }
            digits[i] = byte - b'0';
        }
        let this = Self(digits);

        // If checksum digit is provided, check the checksum.
        if data.len() == 8 {
            if this.checksum() != data[7] - b'0' {
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

#[cfg(test)]
mod tests {
    use crate::Error;
    use crate::sym::ean8::*;
    #[cfg(not(feature = "std"))]
    use alloc::string::String;
    use core::char;

    fn collapse_vec(v: Vec<u8>) -> String {
        let chars = v.iter().map(|d| char::from_digit(*d as u32, 10).unwrap());
        chars.collect()
    }

    #[test]
    fn new_ean8() {
        let ean8 = EAN8::new(b"1234567");

        assert!(ean8.is_ok());
    }

    #[test]
    fn invalid_data_ean8() {
        let ean8 = EAN8::new(b"1234er1");

        assert_eq!(ean8.err().unwrap(), Error::Character);
    }

    #[test]
    fn invalid_len_ean8() {
        let ean8 = EAN8::new(b"1111112222222333333");

        assert_eq!(ean8.err().unwrap(), Error::Length);
    }

    #[test]
    fn invalid_checksum_ean8() {
        let ean8 = EAN8::new(b"88023020");

        assert_eq!(ean8.err().unwrap(), Error::Checksum)
    }

    #[test]
    fn ean8_encode() {
        let ean81 = EAN8::new(b"5512345").unwrap(); // Check digit: 7
        let ean82 = EAN8::new(b"9834651").unwrap(); // Check digit: 3

        assert_eq!(
            "1010110001011000100110010010011010101000010101110010011101000100101",
            collapse_vec(ean81.encode())
        );
        assert_eq!(
            "1010001011011011101111010100011010101010000100111011001101010000101",
            collapse_vec(ean82.encode())
        );
    }

    #[test]
    fn ean8_encode_with_checksum() {
        let ean8 = EAN8::new(b"98346516").unwrap();

        assert_eq!(
            "1010001011011011101111010100011010101010000100111011001101010000101",
            collapse_vec(ean8.encode())
        );
    }
}
