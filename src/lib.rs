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

#![cfg_attr(
    all(
        feature = "nightly",
        not(feature = "std"),
    ),
    feature(error_in_core)
)] // Enable error_in_core feature for nightly builds without std

#![warn(
    missing_debug_implementations,
    missing_copy_implementations,
    missing_docs,
    unused
)]

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc as __alloc;
#[cfg(feature = "std")]
use std as __alloc;

#[cfg(feature = "alloc")]
use __alloc::{
    string::ToString,
    string::String,
    vec::Vec,
    format,
    vec,
};

use core::ops::Range;
use error::Result;

/// The Barcode trait.
/// 
/// All barcode symbologies must implement this trait.
pub trait Barcode<'a>: Sized {
    /// The valid data length for the barcode.
    const SIZE: Range<u16>;
    /// The valid data values for the barcode.
    const CHARS: &'static [u8];
    /// Creates a new barcode.
    fn new(data: &'a [u8]) -> Result<Self>;
    /// Encodes the barcode in-place.
    /// (Without any allocation or copying of data)
    /// 
    /// This method returns None if the buffer size is too small.
    fn encode_in_place(&self, buffer: &mut [u8]) -> Option<()>;
    /// Encodes the barcode.
    #[cfg(feature = "alloc")]
    fn encode(&self) -> Vec<u8>;
}

pub mod error;
// pub mod generators;

#[doc(hidden)]
pub mod sym;
pub use sym::{
    Codabar,
    Code11,
};
