//! # Barcoders
//! Barcoders allows you to encode valid data for a chosen barcode symbology into a ```Vec<u8>``` representation
//! of the underlying binary structure. From here, you can take advantage one of optional builtin generators
//! (for exporting to GIF, PNG, etc) or build your own.
//!
//! ## Current Support
//!
//! The ultimate goal of Barcoders is to provide encoding support for all major (and many not-so-major) symbologies.
//!
//! ### Symbologies
//!
//! * EAN-13
//!   * UPC-A
//!   * JAN
//!   * Bookland
//! * EAN-8
//! * EAN Supplementals
//!   * EAN-2
//!   * EAN-5
//! * Code39
//! * Code128
//! * Two-Of-Five
//!   * Interleaved (ITF)
//!   * Standard (STF)
//! * Codabar
//! * More coming!
//!
//! ### Generators
//!
//! Each generator is defined as an optional "feature" that must be opted-into in order for it's
//! functionality to be compiled into your app.
//!
//! * ASCII (feature: `ascii`)
//! * JSON (feature: `json`)
//! * SVG (feature: `svg`)
//! * PNG (feature: `image`)
//! * GIF (feature: `image`)
//! * WEBP (feature: `image`)
//! * Or add your own
//!
//! ## Examples
//!
//! See the Github repository.

#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]
#![cfg_attr(not(feature = "std"), no_std)]

use core::ops::Range;
use error::Result;

/// The Barcode trait.
/// 
/// All barcode symbologies must implement this trait.
pub trait Barcode<'a>: Sized {
    /// The valid data length for the barcode.
    const SIZE: Range<u16>;
    /// The valid data values for the barcode.
    const ALLOWED_VALUES: &'static [u8];

    /// Performs validation on the data.
    /// 
    /// This is provided as a convenience method for the implementor.
    /// 
    /// It is NOT recommended to override nor call this method directly
    /// as a user of the library.
    #[inline]
    #[doc(hidden)]
    fn validate(data: &'a [u8]) -> Result<()> {
        let len = data.len() as u16;
        if len < Self::SIZE.start || len > Self::SIZE.end {
            return Err(error::Error::Length);
        }
        for &byte in data.iter() {
            if !Self::ALLOWED_VALUES.contains(&byte) {
                return Err(error::Error::Character);
            }
        }
        Ok(())
    }

    /// Creates a new barcode.
    fn new(data: &'a [u8]) -> Result<Self>;
    /// Encodes the barcode in-place.
    /// (Without any allocation or copying of data)
    /// 
    /// This method returns None if the buffer size is too small.
    fn encode_in_place(&self, buffer: &mut [u8]) -> Option<()>;
    /// Encodes the barcode.
    #[cfg(feature = "std")]
    fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.encode_in_place(&mut buffer);
        buffer
    }
}

pub mod error;
pub mod generators;
pub mod sym;
