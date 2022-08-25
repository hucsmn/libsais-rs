//! Definition of interpreted error codes from sais algorithms.

use std::fmt::{Debug, Display};

/// Interpreted error code from libsais.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error<Int: Copy + Eq + Debug + Display> {
    IllegalArguments,
    InternalError,
    Uncategorized(Int),
}

impl<Int: Copy + Eq + Debug + Display> Error<Int> {
    /// Get kind name of the interpreted libsais error code.
    fn kind_name(&self) -> &'static str {
        match &self {
            Error::IllegalArguments => "IllegalArguments",
            Error::InternalError => "InternalError",
            Error::Uncategorized(_) => "Uncategorized",
        }
    }
}

impl<Int: Copy + Eq + Debug + Display> std::error::Error for Error<Int> {
    fn description(&self) -> &str {
        self.kind_name()
    }
}

impl<Int: Copy + Eq + Debug + Display> std::fmt::Display for Error<Int> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.kind_name())
    }
}
