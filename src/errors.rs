//! Basic error handling mechanisms

use std::error::Error;
use std::fmt;

/// The result type for Peeler
pub type PeelerResult<T> = Result<T, Box<Error>>;

/// Concrete errors
struct PeelerError {
    description: String,
    detail: Option<String>,
    cause: Option<Box<Error + Send>>,
}

impl fmt::Display for PeelerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)?;
        if let Some(ref s) = self.detail {
            write!(f, ": {}", s)?;
        }
        Ok(())
    }
}

impl fmt::Debug for PeelerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Error for PeelerError {
    fn description(&self) -> &str {
        &self.description
    }

    fn cause(&self) -> Option<&Error> {
        self.cause.as_ref().map(|c| {
            let e: &Error = &**c;
            e
        })
    }
}

/// Raise an internal error
pub fn error(description: &str) -> Box<Error> {
    Box::new(PeelerError {
        description: description.to_string(),
        detail: None,
        cause: None,
    })
}

macro_rules! bail {
    ($($fmt:tt)*) => (
        return Err(::errors::error(&format!($($fmt)*)))
    )
}
