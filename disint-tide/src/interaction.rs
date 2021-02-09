use disint_security::Application;

#[derive(Debug, Clone)]
pub struct DiscordInteractionAuth {
    app: Application,
}

impl DiscordInteractionAuth {
    pub fn from_public_key(public_key: impl AsRef<str>) -> crate::Result<Self> {
        let app = Application::from_public_key(public_key)?;
        Ok(Self { app })
    }
}

impl From<Application> for DiscordInteractionAuth {
    fn from(app: Application) -> Self {
        Self { app }
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
            let timestamp = timestamp.as_str().to_owned();
            let signature = signature.as_str().to_owned();

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

            let body = request.take_body();
            let body_raw = body.into_bytes().await?;
            let result = self
                .app
                .verify(&body_raw, &timestamp, &signature)
                .map_err(|e| tide::Error::new(e.http_status(), e));

            let body = tide::Body::from_bytes(body_raw);
            request.set_body(body);

            result?;
            Ok(next.run(request).await)
        } else {
            Err(tide::Error::new(401, crate::Error::NoSignature))
        }
    }
}
