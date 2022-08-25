use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum Error<Int: Copy + Debug + Display> {
    IllegalArguments,
    InternalError,
    Uncategorized(Int),
}

impl <Int: Copy + Debug + Display> Error<Int> {
    fn name(&self) -> &'static str {
        match &self {
            Error::IllegalArguments => "IllegalArguments",
            Error::InternalError => "InternalError",
            Error::Uncategorized(_) => "Uncategorized",
        }
    }
}

impl <Int: Copy + Debug + Display> std::error::Error for Error<Int> {
    fn description(&self) -> &str {
        self.name()
    }
}

impl <Int: Copy + Debug + Display> std::fmt::Display for Error<Int> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
