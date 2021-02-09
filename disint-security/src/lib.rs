use std::fmt;

fn parse_hex(hex: impl AsRef<str>) -> Option<Vec<u8>> {
    let hex = hex.as_ref().as_bytes();
    if hex.len() % 2 != 0 {
        return None;
    }

    hex.chunks_exact(2)
        .map(|byte| {
            let (a, b) = match byte {
                &[a, b] => (a, b),
                _ => unreachable!(),
            };
            let a = match a {
                b'0'..=b'9' => a - b'0',
                b'A'..=b'F' => a - b'A' + 10,
                b'a'..=b'f' => a - b'a' + 10,
                _ => return None,
            };
            let b = match b {
                b'0'..=b'9' => b - b'0',
                b'A'..=b'F' => b - b'A' + 10,
                b'a'..=b'f' => b - b'a' + 10,
                _ => return None,
            };
            Some(a << 4 | b)
        })
        .collect::<Option<Vec<_>>>()
}

#[derive(Clone)]
pub struct Application {
    public_key: ring::signature::UnparsedPublicKey<Vec<u8>>,
}

impl Application {
    pub fn from_public_key(public_key: impl AsRef<str>) -> crate::Result<Self> {
        let public_key = parse_hex(public_key).ok_or(crate::Error::KeyFormat)?;
        let public_key =
            ring::signature::UnparsedPublicKey::new(&ring::signature::ED25519, public_key);
        Ok(Self { public_key })
    }

    pub fn verify(&self, body: &[u8], timestamp: &str, signature: &str) -> crate::Result<()> {
        let timestamp = timestamp.as_bytes().to_vec();
        let signature = parse_hex(signature);
        let signature = if let Some(v) = signature {
            v
        } else {
            return Err(crate::Error::SignatureFormat);
        };

        let mut message = timestamp;
        message.extend_from_slice(body);
        self.public_key
            .verify(&message, &signature)
            .map_err(|_| crate::Error::Verification)
    }
}

impl fmt::Debug for Application {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Application")
            .field("public_key", &format_args!("(...)"))
            .finish()
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    KeyFormat,
    SignatureFormat,
    Verification,
}

impl Error {
    pub fn http_status(&self) -> u16 {
        match self {
            Error::KeyFormat | Error::SignatureFormat => 400,
            Error::Verification => 401,
        }
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::KeyFormat => f.write_str("invalid public key format"),
            Error::SignatureFormat => f.write_str("invalid signature format"),
            Error::Verification => f.write_str("verification failed"),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
