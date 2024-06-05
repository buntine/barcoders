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

/// Encoding mappings for EAN barcodes.
/// 1 = bar, 0 = no bar.
///
/// The three indices are:
/// * Left side A (odd parity).
/// * Left side B (even parity).
/// * Right side encodings.
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
const PARITY: [[usize; 5]; 10] = [
    [0, 0, 0, 0, 0],
    [0, 1, 0, 1, 1],
    [0, 1, 1, 0, 1],
    [0, 1, 1, 1, 0],
    [1, 0, 0, 1, 1],
    [1, 1, 0, 0, 1],
    [1, 1, 1, 0, 0],
    [1, 0, 1, 0, 1],
    [1, 0, 1, 1, 0],
    [1, 1, 0, 1, 0],
];

/// The left-hand guard pattern.
pub const LEFT_GUARD: [u8; 3] = [1, 0, 1];
/// The middle guard pattern.
pub const MIDDLE_GUARD: [u8; 5] = [0, 1, 0, 1, 0];
/// The right-hand guard pattern.
pub const RIGHT_GUARD: [u8; 3] = [1, 0, 1];

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

// #[cfg(test)]
// mod tests {
//     use crate::error::Error;
//     use crate::sym::ean13::*;
//     #[cfg(not(feature = "std"))]
//     use alloc::string::String;
//     use core::char;
// 
//     fn collapse_vec(v: Vec<u8>) -> String {
//         let chars = v.iter().map(|d| char::from_digit(*d as u32, 10).unwrap());
//         chars.collect()
//     }
// 
//     #[test]
//     fn new_ean13() {
//         let ean13 = EAN13::new("123456123456");
// 
//         assert!(ean13.is_ok());
//     }
// 
//     #[test]
//     fn new_bookland() {
//         let bookland = Bookland::new("978456123456");
// 
//         assert!(bookland.is_ok());
//     }
// 
//     #[test]
//     fn invalid_data_ean13() {
//         let ean13 = EAN13::new("1234er123412");
// 
//         assert_eq!(ean13.err().unwrap(), Error::Character)
//     }
// 
//     #[test]
//     fn invalid_len_ean13() {
//         let ean13 = EAN13::new("1111112222222333333");
// 
//         assert_eq!(ean13.err().unwrap(), Error::Length)
//     }
// 
//     #[test]
//     fn invalid_checksum_ean13() {
//         let ean13 = EAN13::new("8801051294881");
// 
//         assert_eq!(ean13.err().unwrap(), Error::Checksum)
//     }
// 
//     #[test]
//     fn ean13_encode_as_upca() {
//         let ean131 = UPCA::new("012345612345").unwrap(); // Check digit: 8
//         let ean132 = UPCA::new("000118999561").unwrap(); // Check digit: 3
// 
//         assert_eq!(collapse_vec(ean131.encode()), "10100110010010011011110101000110110001010111101010110011011011001000010101110010011101001000101");
//         assert_eq!(collapse_vec(ean132.encode()), "10100011010001101001100100110010110111000101101010111010011101001001110101000011001101000010101");
//     }
// 
//     #[test]
//     fn ean13_encode_as_bookland() {
//         let bookland1 = Bookland::new("978345612345").unwrap(); // Check digit: 5
//         let bookland2 = Bookland::new("978118999561").unwrap(); // Check digit: 5
// 
//         assert_eq!(collapse_vec(bookland1.encode()), "10101110110001001010000101000110111001010111101010110011011011001000010101110010011101001110101");
//         assert_eq!(collapse_vec(bookland2.encode()), "10101110110001001011001100110010001001000101101010111010011101001001110101000011001101001110101");
//     }
// 
//     #[test]
//     fn ean13_encode() {
//         let ean131 = EAN13::new("750103131130").unwrap(); // Check digit: 5
//         let ean132 = EAN13::new("983465123499").unwrap(); // Check digit: 5
// 
//         assert_eq!(collapse_vec(ean131.encode()), "10101100010100111001100101001110111101011001101010100001011001101100110100001011100101110100101");
//         assert_eq!(collapse_vec(ean132.encode()), "10101101110100001001110101011110111001001100101010110110010000101011100111010011101001000010101");
//     }
// }
