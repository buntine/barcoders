//! Encoder for Code128 barcodes.
//!
//! Code128 is a popular, high-density symbology that allows for the encoding of alphanumeric
//! data along with many special characters by utilising three separate character-sets.
//!
//! Code128 also offers double-density encoding of digits.
//!
//! ## Character sets
//!
//! Barcoders provides special Unicode syntax for specifying the character set(s) which should be
//! used in the barcode:
//!
//! <ul><li>\u{00C0} = Switch to character-set A (À)</li>
//! <li>\u{0181} = Switch to character-set B (Ɓ)</li>
//! <li>\u{0106} = Switch to character-set C (Ć)</li></ul>
//!
//! You must provide both the starting character-set along with any changes during the data. This
//! means all Code128 barcodes must start with either "À", "Ɓ" or "Ć". Simple alphanumeric data
//! can generally use character-set A solely.
//!
//! As an example, this barcode uses character-set B:
//!
//! <ul><li>\u{0181}HE1234A*1</li></ul>
//!
//! Or:
//!
//! <ul><li>ƁHE1234A*1</li></ul>
//!
//! And this one starts at character-set A (the default) and then switches to C to encode the digits more
//! effectively:
//!
//! <ul><li>\u{00C0}HE@$A\u{0106}123456</li></ul>
//!
//! Or:
//!
//! <ul><li>ÀHE@$AĆ123456</li></ul>
//!
//! ## Unicode characters
//!
//! The invisible unicode characters that are available in character set A should be represented as
//! their Unicode sequences. For example, to represent the 'ACK' character:
//!
//! <ul><li>À\u{0006}</li></ul>
//!
//! ## Special-purpose function characters (FNC1 - 4)
//!
//! The function sequences can be represented via the following unicode characters:
//!
//! - FNC1: ```Ź``` (```\u{0179}```)
//! - FNC2: ```ź``` (```\u{017A}```)
//! - FNC3: ```Ż``` (```\u{017B}```)
//! - FNC4: ```ż``` (```\u{017C}```)
//! - SHIFT: ```Ž``` (```\u{017D}```)

use super::*;

mod a;
mod b;
mod c;

pub use a::Code128A;
pub use b::Code128B;
pub use c::Code128C;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Code128Inner<'a> {
    A(Code128A<'a>),
    B(Code128B<'a>),
    C(Code128C<'a>),
}

// For Blitz
// use core::mem::ManuallyDrop;
// union Code128ImplInner<'a> {
//     a: ManuallyDrop<a::Code128A<'a>>,
//     b: ManuallyDrop<b::Code128B<'a>>,
//     c: ManuallyDrop<c::Code128C<'a>>,
// }
// 
// #[repr(u8)]
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// enum Code128ImplId {
//     A = 0,
//     B = 1,
//     C = 2,
// }
// 
// #[repr(packed)]
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// struct Code128Impl<'a> {
//     id: Code128ImplId,
//     inner: Code128ImplInner<'a>,
// }

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Code128<'a> {
    inner: Code128Inner<'a>,
}

impl<'a> Barcode<'a> for Code128<'a> {
    fn new(data: &'a [u8]) -> Result<Self> {
        todo!()
    }
    fn encode_in_place(&self, buffer: &mut [u8]) -> Option<()> {
        todo!()
    }
    fn encode(&self) -> Vec<u8> {
        todo!()
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::error::Error;
//     use crate::sym::code128::*;
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
//     fn new_code128() {
//         let code128_a = Code128::new("À !! Ć0201");
//         let code128_b = Code128::new("À!!  \" ");
// 
//         assert!(code128_a.is_ok());
//         assert!(code128_b.is_ok());
//     }
// 
//     #[test]
//     fn invalid_length_code128() {
//         let code128_a = Code128::new("");
// 
//         assert_eq!(code128_a.err().unwrap(), Error::Length);
//     }
// 
//     #[test]
//     fn invalid_data_code128() {
//         let code128_a = Code128::new("À☺ "); // Unknown character.
//         let code128_b = Code128::new("ÀHELLOĆ12352"); // Trailing carry at the end.
//         let code128_c = Code128::new("HELLO"); // No Character-Set specified.
// 
//         assert_eq!(code128_a.err().unwrap(), Error::Character);
//         assert_eq!(code128_b.err().unwrap(), Error::Character);
//         assert_eq!(code128_c.err().unwrap(), Error::Character);
//     }
// 
//     #[test]
//     fn code128_encode() {
//         let code128_a = Code128::new("ÀHELLO").unwrap();
//         let code128_b = Code128::new("ÀXYĆ2199").unwrap();
//         let code128_c = Code128::new("ƁxyZÀ199!*1").unwrap();
// 
//         assert_eq!(collapse_vec(code128_a.encode()), "110100001001100010100010001101000100011011101000110111010001110110110100010001100011101011");
//         assert_eq!(collapse_vec(code128_b.encode()), "110100001001110001011011101101000101110111101101110010010111011110100111011001100011101011");
//         assert_eq!(collapse_vec(code128_c.encode()), "1101001000011110010010110110111101110110001011101011110100111001101110010110011100101100110011011001100100010010011100110100101111001100011101011");
//     }
// 
//     #[test]
//     fn code128_encode_special_chars() {
//         let code128_a = Code128::new("ÀB\u{0006}").unwrap();
// 
//         assert_eq!(
//             collapse_vec(code128_a.encode()),
//             "110100001001000101100010110000100100110100001100011101011"
//         );
//     }
// 
//     #[test]
//     fn code128_encode_fnc_chars() {
//         let code128_a = Code128::new("ĆŹ4218402050À0").unwrap();
// 
//         assert_eq!(collapse_vec(code128_a.encode()), "110100111001111010111010110111000110011100101100010100011001001110110001011101110101111010011101100101011110001100011101011");
//     }
// 
//     #[test]
//     fn code128_encode_longhand() {
//         let code128_a = Code128::new("\u{00C0}HELLO").unwrap();
//         let code128_b = Code128::new("\u{00C0}XY\u{0106}2199").unwrap();
//         let code128_c = Code128::new("\u{0181}xyZ\u{00C0}199!*1").unwrap();
// 
//         assert_eq!(collapse_vec(code128_a.encode()), "110100001001100010100010001101000100011011101000110111010001110110110100010001100011101011");
//         assert_eq!(collapse_vec(code128_b.encode()), "110100001001110001011011101101000101110111101101110010010111011110100111011001100011101011");
//         assert_eq!(collapse_vec(code128_c.encode()), "1101001000011110010010110110111101110110001011101011110100111001101110010110011100101100110011011001100100010010011100110100101111001100011101011");
//     }
// }
