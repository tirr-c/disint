use std::fmt;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    NoSignature,
    Timestamp,
    DisintSecurity(disint_security::Error),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::DisintSecurity(e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NoSignature => f.write_str("missing signature"),
            Error::Timestamp => f.write_str("timestamp is either too old or too new"),
            Error::DisintSecurity(e) => write!(f, "inner error: {}", e),
        }
    }
}

impl From<disint_security::Error> for Error {
    fn from(v: disint_security::Error) -> Self {
        Self::DisintSecurity(v)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
