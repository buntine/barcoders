use crate::*;

pub mod codabar;

pub mod code11;

pub mod code39;

pub mod code93;

// pub mod code128;

pub mod ean8;
// pub use ean8::EAN8;

pub mod ean13;
// pub use ean13::EAN13;

// mod tf;
// pub use tf::TF;

/// An extension trait for barcode symbologies.
/// 
/// This trait provides a default implementation for the `validate` method
/// commonly used in the `new` method of barcode symbologies.
pub trait BarcodeDevExt<'a, SegmentSize: 'static + PartialEq = u8>: Barcode<'a> {
    /// The valid data length for the barcode.
    const SIZE: Range<u16>;
    /// The valid data values for the barcode.
    const CHARS: &'static [SegmentSize];
    /// Performs validation on the data.
    /// 
    /// Users of the library should not need to call this method directly.
    /// This method is provided for potential implementors of new barcode symbologies.
    fn validate(data: &'a [SegmentSize]) -> Result<&'a [SegmentSize]> {
        let len = data.len() as u16;
        if len < Self::SIZE.start || len > Self::SIZE.end {
            return Err(error::Error::Length);
        }
        for byte in data {
            if !Self::CHARS.contains(byte) {
                return Err(error::Error::Character);
            }
        }
        Ok(data)
    }
}

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
