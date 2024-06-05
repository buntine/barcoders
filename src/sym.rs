use crate::*;

mod codabar;
pub use codabar::Codabar;

mod code11;
pub use code11::Code11;

mod code39;
pub use code39::Code39;

mod code93;
pub use code93::Code93;

// mod code128;
// pub use code128::Code128;

// mod ean8;
// pub use ean8::EAN8;

// mod ean13;
// pub use ean13::EAN13;

// mod tf;
// pub use tf::TF;

/// An extension trait for barcode symbologies.
/// 
/// This trait provides a default implementation for the `validate` method
/// commonly used in the `new` method of barcode symbologies.
pub trait BarcodeDevExt<'a>: Barcode<'a> {
    /// Performs validation on the data.
    /// 
    /// Users of the library should not need to call this method directly.
    /// This method is provided for potential implementors of new barcode symbologies.
    fn validate(data: &'a [u8]) -> Result<&'a [u8]> {
        let len = data.len() as u16;
        if len < Self::SIZE.start || len > Self::SIZE.end {
            return Err(error::Error::Length);
        }
        for &byte in data.iter() {
            if !Self::CHARS.contains(&byte) {
                return Err(error::Error::Character);
            }
        }
        Ok(data)
    }
}

#[macro_export(local_inner_macros)]
macro_rules! encode {
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
                $crate::encode!(@VALUE ($buffer, $i) $($t)+);
            },)*
            #[cfg(not(feature = "unsafety"))]
            _ => ::core::unreachable!("Validation did not catch an illegal character"),
            #[cfg(feature = "unsafety")]
            _ => unsafe { ::core::hint::unreachable_unchecked() },
        }
    );
}

impl<'a, B> BarcodeDevExt<'a> for B where B: Barcode<'a> {}
