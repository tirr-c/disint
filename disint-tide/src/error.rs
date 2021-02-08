use std::fmt;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    KeyFormat,
    NoSignature,
    SignatureFormat,
    Timestamp,
    Verification,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::KeyFormat => f.write_str("invalid key format"),
            Error::NoSignature => f.write_str("missing signature"),
            Error::SignatureFormat => f.write_str("invalid signature format"),
            Error::Timestamp => f.write_str("timestamp is either too old or too new"),
            Error::Verification => f.write_str("signature verification failed"),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
