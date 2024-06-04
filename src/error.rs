//! Custom error types.

#[cfg(all(not(feature = "std"), feature = "nightly"))]
use core::error::Error as ErrorTrait;
#[cfg(feature = "std")]
use std::error::Error as ErrorTrait;

use core::fmt;

/// The possible errors that can occur during barcode encoding and generation.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
    /// An invalid character found during encoding.
    Character,
    /// An invalid data length during encoding.
    Length,
    /// An error during barcode generation.
    Generate,
    /// Invalid checksum.
    Checksum,
}

/// Alias-type for Result<T, barcoders::error::Error>.
pub type Result<T> = ::core::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Character => write!(f, "Barcode data is invalid"),
            Error::Length => write!(f, "Barcode data length is invalid"),
            Error::Generate => write!(f, "Could not generate barcode data"),
            Error::Checksum => write!(f, "Invalid checksum"),
        }
    }
}

#[cfg(any(
    feature = "std",
    feature = "nightly"
))]
impl ErrorTrait for Error {}
