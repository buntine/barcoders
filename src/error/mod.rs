//! Custom error types.

use std::error::Error as StdError;
use std::fmt;

/// The possible errors that can occur during barcode encoding and generation.
#[derive(Copy, Clone, Debug, PartialEq)]
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
pub type Result<T> = ::std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Character => write!(f, "Barcode data is invalid"),
            Error::Length => write!(f, "Barcode data length is invalid"),
            Error::Generate => write!(f, "Could not generate barcode data"),
            Error::Checksum => write!(f, "Invalid checksum"),
        }
    }
}

impl StdError for Error {
    fn cause(&self) -> Option<&dyn StdError> {
        None
    }
}
