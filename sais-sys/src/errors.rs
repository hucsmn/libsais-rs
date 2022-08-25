pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IllegalArguments,
    Uncategorized(i32),
}

impl Error {
    fn name(&self) -> &'static str {
        match &self {
            Error::IllegalArguments => "IllegalArguments",
            Error::Uncategorized(_) => "Uncategorized",
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        self.name()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
