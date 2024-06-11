//! Supported barcode symbologies.
//!
//! Symbologies are separated into logical modules and thus you must `use` the appropriate one(s).
//!
//! For example:
//!
//! ```rust
//! use barcoders::sym::ean13::*;
//!
//! let barcode = EAN13::new(b"750103131130").unwrap();
//! let encoded = barcode.encode();
//! ```
//! Each encoder accepts a `String` to be encoded. Valid data is barcode-specific and thus
//! constructors return an Option<T>.

use crate::*;

pub mod codabar;
pub mod code11;
pub mod code39;
pub mod code93;
#[cfg(feature = "alloc")]
pub mod code128;
pub mod ean13;
pub mod ean8;
pub mod ean_supp;
pub mod tf;

/// An extension trait for barcode symbologies.
/// 
/// This trait provides a default implementation for the `validate` method
/// commonly used in the `new` method of barcode symbologies.
pub trait BarcodeDevExt<'a> {
    /// The valid data length for the barcode.
    const SIZE: Range<u16>;
    /// The valid data values for the barcode.
    const CHARS: &'static [u8];
    /// Performs validation on the data.
    /// 
    /// Users of the library should not need to call this method directly.
    /// This method is provided for potential implementors of new barcode symbologies.
    fn validate(data: &'a [u8]) -> Result<&'a [u8]> {
        let len = data.len() as u16;
        if len < Self::SIZE.start || len > Self::SIZE.end {
            return Err(Error::Length);
        }
        for byte in data {
            if !Self::CHARS.contains(byte) {
                return Err(Error::Character);
            }
        }
        Ok(data)
    }
}

/// A helper macro for encoding data into a buffer.
/// 
/// Example usage:
/// ```rust
/// # struct Dummy([u8; 0]);
/// # macro_rules! encode { ($($t:tt)*) => () }
/// # impl Dummy {
/// fn encode_into(&self, buffer: &mut [u8]) {
///     let mut i = 0;
///     for byte in self.0.iter() {
///         encode!((buffer, i) byte {
///             b'0' => ([1, 0, 1, 0, 1, 0, 0, 1, 1]),
///             b'1' => ([1, 0, 1, 0, 1, 1, 0, 0, 1]),
///             b'2' => ([1, 0, 1, 0, 0, 1, 0, 1, 1]),
///             b'3' => ([1, 1, 0, 0, 1, 0, 1, 0, 1]),
///             b'4' => ([1, 0, 1, 1, 0, 1, 0, 0, 1]),
///             b'5' => ([1, 1, 0, 1, 0, 1, 0, 0, 1]),
///             b'6' => ([1, 0, 0, 1, 0, 1, 0, 1, 1]),
///             b'7' => ([1, 0, 0, 1, 0, 1, 1, 0, 1]),
///             b'8' => ([1, 0, 0, 1, 1, 0, 1, 0, 1]),
///             b'9' => ([1, 1, 0, 1, 0, 0, 1, 0, 1]),
///             b'-' => ([1, 0, 1, 0, 0, 1, 1, 0, 1]),
///             b'$' => ([1, 0, 1, 1, 0, 0, 1, 0, 1]),
///             b':' => ([1, 1, 0, 1, 0, 1, 1, 0, 1, 1]),
///             b'/' => ([1, 1, 0, 1, 1, 0, 1, 0, 1, 1]),
///             b'.' => ([1, 1, 0, 1, 1, 0, 1, 1, 0, 1]),
///             b'+' => ([1, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1]),
///             b'A' => ([1, 0, 1, 1, 0, 0, 1, 0, 0, 1]),
///             b'B' => ([1, 0, 1, 0, 0, 1, 0, 0, 1, 1]),
///             b'C' => ([1, 0, 0, 1, 0, 0, 1, 0, 1, 1]),
///             b'D' => ([1, 0, 1, 0, 0, 1, 1, 0, 0, 1]),
///         });
///         // Don't forget the padding
///         if i < buffer.len() {
///             buffer[i] = 0;
///             i += 1;
///         }
///     }
/// }
/// # }
/// ```
#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __encode {
    (@VALUE ($buffer:ident, $i:ident) [$($bit:literal),+]) => (
        $(
            $buffer[$i] = $bit;
            $i += 1;
        )+
    );
    (@VALUE ($buffer:ident, $i:ident) $val:expr) => (
        for value in $val {
            $buffer[$i] = value;
            $i += 1;
        }
    );
    (($buffer:ident, $i:ident) $v:ident {
        $($pat:pat => ($($t:tt)*),)+
    }) => (
        match $v {
            $($pat => {
                $crate::__encode!(@VALUE ($buffer, $i) $($t)+);
            },)*
            #[cfg(not(feature = "blitz"))]
            _ => ::core::unreachable!("Validation did not catch an illegal character"),
            #[cfg(feature = "blitz")]
            _ => unsafe { ::core::hint::unreachable_unchecked() },
        }
    );
}
