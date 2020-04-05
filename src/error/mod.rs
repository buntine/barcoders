//! Custom error types.

use std::fmt;
use std::error::Error as StdError;

/// The possible errors that can occur during barcode encoding and generation.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Error {
  /// An invalid character found during encoding.
  Character,
  /// An invalid data length during encoding.
  Length,
  /// An error during barcode generation.
  Generate,
}

/// Alias-type for Result<T, barcoders::error::Error>.
pub type Result<T> = ::std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Character => "Barcode data is invalid",
            Error::Length => "Barcode data length is invalid",
            Error::Generate => "Could not generate barcode data",
        }
    }

    fn cause(&self) -> Option<&dyn StdError> {
        None
    }
}
