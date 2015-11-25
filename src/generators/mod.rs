//! Supported methods of barcode generation.
//! 
//! Each generation option is an optionally-compiled feature, which you must opt-into when
//! compiling.

#[cfg(feature = "ascii")]
pub mod ascii;

#[cfg(feature = "image")]
pub mod image;
