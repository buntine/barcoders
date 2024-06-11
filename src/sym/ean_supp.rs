//! Encoders for supplemental 2-digit and 5-digit EAN barcodes.
//!
//! EAN-2 barcodes are used in magazines and newspapers to indicate issue number.
//!
//! EAN-5 barcodes are often used to indicate the suggested retail price of books.
//!
//! These supplemental barcodes never appear without a full EAN-13 barcode alongside them.

use super::*;
use ean13::{
    ENCODING_LEFT_A, ENCODING_LEFT_B
};

const LEFT_GUARD: [u8; 4] = [1, 0, 1, 1];

/// Maps parity (odd/even) for the EAN-5 barcodes based on the check digit.
const EAN5_PARITY: [[usize; 5]; 10] = [
    [0, 0, 1, 1, 1],
    [1, 0, 1, 0, 0],
    [1, 0, 0, 1, 0],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 0, 0],
    [0, 0, 1, 1, 0],
    [0, 0, 0, 1, 1],
    [0, 1, 0, 1, 0],
    [0, 1, 0, 0, 1],
    [0, 0, 1, 0, 1],
];

/// Maps parity (odd/even) for the EAN-2 barcodes based on the check digit.
const EAN2_PARITY: [[usize; 5]; 4] = [
    [0, 0, 0, 0, 0],
    [0, 1, 0, 0, 0],
    [1, 0, 0, 0, 0],
    [1, 1, 0, 0, 0],
];

/// EAN-2 supplemental barcode type.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EAN2([u8; 2]);

fn ean_encode_into(this: &[u8], buffer: &mut [u8], parity: [usize; 5]) {
    let mut i = 0;
    
    // Left guard
    for bit in LEFT_GUARD {
        buffer[i] = bit;
        i += 1;
    }

    for j in 0..this.len() {
        // Potential Separator
        if j > 0 {
            buffer[i] = 0;
            i += 1;
            buffer[i] = 1;
            i += 1;
        }

        // Data
        let d = this[j];
        let s = parity[j];
        let bits = if s == 0 {
            ENCODING_LEFT_A[d as usize]
        } else {
            ENCODING_LEFT_B[d as usize]
        };
        for bit in bits {
            buffer[i] = bit;
            i += 1;
        }
    }
}

impl EAN2 {
    fn encode_into(&self, buffer: &mut [u8]) {
        let modulo = self.0[0] * 10 + self.0[1];
        let parity = EAN2_PARITY[modulo as usize % 4];
        ean_encode_into(&self.0, buffer, parity);
    }
}

impl<'a> Barcode<'a> for EAN2 {
    fn new(data: &'a [u8]) -> Result<Self> where Self: Sized {
        if data.len() != 2 {
            return Err(Error::Length);
        }

        let mut bit1 = data[0];
        let mut bit2 = data[1];
        if bit1 < b'0' || bit2 < b'0' {
            return Err(Error::Character);
        }
        bit1 = data[0] - b'0';
        bit2 = data[1] - b'0';

        if bit1 > 9 || bit2 > 9 {
            return Err(Error::Character);
        }

        Ok(Self([bit1, bit2]))
    }

    fn encode_in_place(&self, buffer: &mut [u8]) -> Option<()> {
        if buffer.len() < 20 {
            return None;
        }
        self.encode_into(buffer);
        Some(())
    }

    #[cfg(feature = "alloc")]
    fn encode(&self) -> Vec<u8> {
        let mut buffer = vec![0; 20];
        self.encode_into(&mut buffer);
        buffer
    }
}

/// EAN-5 supplemental barcode type.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EAN5([u8; 5]);

impl EAN5 {
    fn checksum_index(&self) -> usize {
        let mut odds = 0;
        let mut evens = 0;

        for (i, d) in self.0.iter().enumerate() {
            match i % 2 {
                1 => evens += *d,
                _ => odds += *d,
            }
        }

        match ((odds * 3) + (evens * 9)) % 10 {
            10 => 0,
            n => n as usize,
        }
    }
    fn encode_into(&self, buffer: &mut [u8]) {
        let parity = EAN5_PARITY[self.checksum_index()];
        ean_encode_into(&self.0, buffer, parity);
    }
}

impl<'a> Barcode<'a> for EAN5 {
    fn new(data: &'a [u8]) -> Result<Self> where Self: Sized {
        if data.len() != 5 {
            return Err(Error::Length);
        }

        let mut bits = [0; 5];
        for (i, &bit) in data.iter().enumerate() {
            if bit < b'0' {
                return Err(Error::Character);
            }
            bits[i] = bit - b'0';
        }

        Ok(Self(bits))
    }

    fn encode_in_place(&self, buffer: &mut [u8]) -> Option<()> {
        if buffer.len() < 47 {
            return None;
        }
        self.encode_into(buffer);
        Some(())
    }

    #[cfg(feature = "alloc")]
    fn encode(&self) -> Vec<u8> {
        let mut buffer = vec![0; 47];
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
    fn new_ean2() {
        let ean2 = EAN2::new(b"12");

        assert!(ean2.is_ok());
    }

    #[test]
    fn new_ean5() {
        let ean5 = EAN5::new(b"12345");

        assert!(ean5.is_ok());
    }

    #[test]
    fn invalid_data_ean2() {
        let ean2 = EAN2::new(b"AT");

        assert_eq!(ean2.err().unwrap(), Error::Character);
    }

    #[test]
    fn invalid_len_ean2() {
        let ean2 = EAN2::new(b"123");

        assert_eq!(ean2.err().unwrap(), Error::Length);
    }

    #[test]
    fn ean2_encode() {
        let ean21 = EAN2::new(b"34").unwrap();

        assert_eq!("10110100001010100011", collapse_vec(ean21.encode()));
    }

    #[test]
    fn ean5_encode() {
        let ean51 = EAN5::new(b"51234").unwrap();

        assert_eq!(
            "10110110001010011001010011011010111101010011101",
            collapse_vec(ean51.encode())
        );
    }
}
