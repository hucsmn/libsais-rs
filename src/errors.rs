//! Definition of interpreted error codes from sais algorithms.

use std::fmt::{Debug, Display, Formatter};

/// Raw return code types from libsais.
pub trait ReturnCode: Copy + Eq + Debug + Display {}

#[cfg(any(feature = "sais16", feature = "sais32"))]
impl ReturnCode for i32 {}

#[cfg(feature = "sais64")]
impl ReturnCode for i64 {}

/// Interpreted error code from libsais.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error<I: ReturnCode> {
    IllegalArguments,
    InternalError,
    Uncategorized(I),
}

impl<I: ReturnCode> Error<I> {
    /// Get kind name of the interpreted libsais error code.
    fn kind_name(&self) -> &'static str {
        match &self {
            Error::IllegalArguments => "IllegalArguments",
            Error::InternalError => "InternalError",
            Error::Uncategorized(_) => "Uncategorized",
        }
    }
}

impl<I: ReturnCode> std::error::Error for Error<I> {
    fn description(&self) -> &str {
        self.kind_name()
    }
}

impl<I: ReturnCode> Display for Error<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.kind_name())
    }
}
