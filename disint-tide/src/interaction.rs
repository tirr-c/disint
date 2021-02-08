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

pub struct DiscordInteractionAuth {
    public_key: ring::signature::UnparsedPublicKey<Vec<u8>>,
}

impl DiscordInteractionAuth {
    pub fn from_public_key(public_key: impl AsRef<str>) -> crate::Result<Self> {
        let public_key = parse_hex(public_key).ok_or(crate::Error::KeyFormat)?;
        let public_key =
            ring::signature::UnparsedPublicKey::new(&ring::signature::ED25519, public_key);
        Ok(Self { public_key })
    }
}

#[async_trait::async_trait]
impl<State: Clone + Send + Sync + 'static> tide::Middleware<State> for DiscordInteractionAuth {
    async fn handle(
        &self,
        mut request: tide::Request<State>,
        next: tide::Next<'_, State>,
    ) -> tide::Result {
        let timestamp = request.header("x-signature-timestamp");
        let signature = request.header("x-signature-ed25519");
        if let (Some(timestamp), Some(signature)) = (timestamp, signature) {
            let timestamp = timestamp.as_str();
            let signature = signature.as_str();

            let parsed_timestamp = timestamp
                .parse::<u64>()
                .map_err(|e| tide::Error::new(400, e))?;
            let current_timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            if current_timestamp >= parsed_timestamp + 5 {
                return Err(tide::Error::new(400, crate::Error::Timestamp));
            }
            if current_timestamp <= parsed_timestamp.saturating_sub(5) {
                return Err(tide::Error::new(400, crate::Error::Timestamp));
            }

            tide::log::info!("Verifying interaction signature", {
                timestamp: timestamp,
                signature: signature,
            });
            let timestamp = timestamp.as_bytes().to_vec();
            let signature = parse_hex(signature);
            let signature = if let Some(v) = signature {
                v
            } else {
                return Err(tide::Error::new(400, crate::Error::SignatureFormat));
            };

            let body = request.take_body();
            let body_raw = body.into_bytes().await?;
            let mut message = timestamp;
            message.extend_from_slice(&body_raw);
            if self.public_key.verify(&message, &signature).is_err() {
                return Err(tide::Error::new(401, crate::Error::Verification));
            }

            let body = tide::Body::from_bytes(body_raw);
            request.set_body(body);

            Ok(next.run(request).await)
        } else {
            Err(tide::Error::new(401, crate::Error::NoSignature))
        }
    }
}
