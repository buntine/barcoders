//! Supported methods of barcode generation.
//!
//! Each generation option is an optionally-compiled feature, which you must opt-into when
//! compiling.
//!
//! For example:
//!
//! ```toml
//! [dependencies]
//! barcoders = {version = "*", features = ["image"]}
//! ```

#[cfg(feature = "ascii")]
pub mod ascii;

#[cfg(feature = "json")]
pub mod json;

#[cfg(feature = "image")]
pub mod image;

#[cfg(feature = "svg")]
pub mod svg;
