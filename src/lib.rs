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
//! * JPEG (feature: `image`)
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
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

#[cfg(feature = "image")]
extern crate image;

pub mod error;
pub mod generators;
pub mod sym;
