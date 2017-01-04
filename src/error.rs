//! Basic error handling mechanisms
use std::error::Error;
use std::{fmt, io};

/// The crates result type
pub type RainResult<T> = Result<T, RainError>;

/// Representation for an error of the library
pub struct RainError {
    /// The error variant
    pub code: ErrorType,

    /// Additional description for the error
    pub description: String,

    /// The cause for this error
    pub cause: Option<Box<Error>>,
}

impl fmt::Display for RainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Code: {:?}, Description: {}",
               self.code,
               self.description)
    }
}

impl fmt::Debug for RainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Error for RainError {
    fn description(&self) -> &str {
        &self.description
    }
}

// Error conversion
macro_rules! from_error {
    ($($p:ty,)*) => (
        $(impl From<$p> for RainError {
            fn from(err: $p) -> RainError {
                RainError {
                    code: ErrorType::Other,
                    description: err.description().to_owned(),
                    cause: Some(Box::new(err)),
                }
            }
        })*
    )
}

from_error! {
    io::Error,
}

#[derive(Debug, PartialEq, Eq)]
/// Error codes as indicator what happened
pub enum ErrorType {
    /// The error originates from another error
    Other,

    /// The line does not exist and cannot be removed
    LineDoesNotExist,

    /// Could not retrieve the actual terminal dimensions
    TerminalDimensions,
}

/// Throw an internal error
pub fn bail(code: ErrorType, description: &fmt::Display) -> RainError {
    RainError {
        code: code,
        description: description.to_string(),
        cause: None,
    }
}

macro_rules! bail {($code:expr, $($fmt:tt)*) => (
    return Err(::error::bail($code, &format_args!($($fmt)*)))
)}
