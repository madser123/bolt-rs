use super::{AppResult, Error, Logger};

use axum::http::HeaderMap;
use chrono::Local;
use hmac_sha256::HMAC;

const SLACK_ENCRYPTION_VERSION: &str = "v0";

/// Authentication context for Bolt-rs powered apps.
#[derive(Default, Clone)]
pub struct Auth {
    // Tokens
    /// A slack-app bot-token.
    bot_token: Option<String>,

    /// A slack-app user-token.
    user_token: Option<String>,

    // Client info
    /// A slack-app signing-secret.
    signing_secret: String,
}

impl Logger for Auth {
    fn name() -> String {
        "Authentication".to_string()
    }
}

impl Auth {
    /// Create a new authentification context
    #[must_use]
    pub const fn new(
        signing_secret: String,
        bot_token: Option<String>,
        user_token: Option<String>,
    ) -> Self {
        Self {
            bot_token,
            user_token,
            signing_secret,
        }
    }

    /// Returns the bot-token, if any.
    #[must_use]
    pub fn bot_token(&self) -> Option<String> {
        self.bot_token.clone()
    }

    /// Returns the user-token, if any.
    #[must_use]
    pub fn user_token(&self) -> Option<String> {
        self.user_token.clone()
    }

    /// Returns the signing-secret.
    #[must_use]
    pub fn signing_secret(&self) -> String {
        self.signing_secret.clone()
    }

    /// Alerts the user of non-registered tokens, which could lead to decreased functionality.
    ///
    /// # Panics
    ///
    /// Panics if the signing-secret supplied is empty, as the signing secret is necessary to authenticate requests.
    ///
    pub fn run_pre_startup_checks(&self) {
        assert!(!self.signing_secret.is_empty(), "Signing secret is empty! If you instantiated your Auth using `Auth::default()` you need to use `Auth::new(signing_secret, Option<bot_token>, Option<user_token>)` instead.");
        if self.bot_token.is_none() {
            Self::warn("No Bot-Token supplied. Some features won't be available or be limited.");
        }
        if self.user_token.is_none() {
            Self::warn("No User-Token supplied. Some features won't be available or be limited.");
        }
    }

    /// Authenticates a slack-payload.
    ///
    /// # Errors
    ///
    /// Errors will occur if the slack-payload is unverifiable, unparseable or alike.
    ///
    pub fn sanitize_payload(&self, payload: &String, headers: &HeaderMap) -> AppResult<String> {
        Self::log("Sanitizing new payload.");

        // Get headers
        let Some(slack_ts_header) = headers.get("X-Slack-Request-Timestamp") else { 
            return Err(Error::Authentication("Missing timestamp header!".to_string())) 
        };
        let Some(slack_signature_header) = headers.get("X-Slack-Signature") else { 
            return Err(Error::Authentication("Missing signature header!".to_string())) 
        };

        // Parse timestamp to integer
        let slack_timestamp = match slack_ts_header.to_str() {
            Ok(slice) => match slice.parse::<i64>() {
                Ok(epoch) => epoch,
                Err(error) => {
                    return Err(Error::Authentication(format!(
                        "Couldn't parse timestamp to integer: {error}"
                    )))
                }
            },
            Err(error) => {
                return Err(Error::Authentication(format!(
                    "Couldn't parse timestamp-header to string: {error}"
                )))
            }
        };

        // Check timestamp age
        if (slack_timestamp - Local::now().timestamp()).abs() > 30 {
            return Err(Error::Authentication(
                "Slack timestamp was over 30 seconds old!".to_string(),
            ));
        };

        // Encypt basestring to HMAC_sha256
        let hmac = HMAC::mac(
            // Construct basestring to encrypt
            format!("{SLACK_ENCRYPTION_VERSION}:{slack_timestamp}:{payload}"),
            // Use signing secret as key
            self.signing_secret.clone(),
        );

        // Get signature
        let signature = match slack_signature_header.to_str() {
            Ok(slice) => slice.to_string(),
            Err(error) => {
                return Err(Error::Authentication(format!(
                    "Couldn't parse signature-header to string: {error}"
                )))
            }
        };

        // Ensure signature is matching hmac
        if format!("{SLACK_ENCRYPTION_VERSION}={}", hex::encode(hmac)) != signature {
            return Err(Error::Authentication("Signatures didn't match".to_string()));
        };

        Self::log("OK");

        // Decode payload
        match urlencoding::decode(&payload.replace('+', " ")) {
            Ok(decoded) => Ok(decoded.into_owned().replace("payload=", "")),
            Err(error) => Err(Error::Parsing(format!(
                "Couldn't parse payload body: {error}"
            ))),
        }
    }
}
