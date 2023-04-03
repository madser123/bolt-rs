use super::*;

use axum::http::HeaderMap;
use hmac_sha256::HMAC;
use chrono::Local;

#[derive(Default, Clone)]
pub struct Auth {
    // Tokens
    bot_token: Option<String>,
    user_token: Option<String>,

    // Client info
    signing_secret: String,
}

impl Logger for Auth {
    fn log(message: &str) {
        println!("[INFO][Authentication] {message}");
    }

    fn warn(message: &str) {
        println!("[WARNING][Authentication] {message}");
    }
}

impl Auth {
    pub fn new(signing_secret: String, bot_token: Option<String>, user_token: Option<String>) -> Self {
        Self {
            signing_secret,
            bot_token,
            user_token,
        }
    }

    pub fn bot_token(&self) -> Option<String> {
        self.bot_token.clone()
    }

    pub fn user_token(&self) -> Option<String> {
        self.user_token.clone()
    }

    pub fn signing_secret(&self) -> String {
        self.signing_secret.clone()
    }

    pub fn run_pre_startup_checks(&self) {
        if self.signing_secret.is_empty() {
            panic!("Signing secret is empty! If you instantiated your Auth using `Auth::default()` you need to use `Auth::new(signing_secret, Option<bot_token>, Option<user_token>)` instead.")
        }
        if self.bot_token.is_none() {
            Self::warn("No Bot-Token supplied. Some features won't be available or be limited.")
        }
        if self.user_token.is_none() {
            Self::warn("No User-Token supplied. Some features won't be available or be limited.")
        }
    }

    pub fn sanitize_payload(&self, payload: &String, headers: HeaderMap) -> AppResult<String> {
        Self::log("Sanitizing new payload.");

        // Get headers
        let slack_ts_header = match headers.get("X-Slack-Request-Timestamp") {
            Some(ts) => ts,
            None => return Err(Error::Authentication("Missing timestamp header!".to_string())),
        };
        let slack_signature_header = match headers.get("X-Slack-Signature") {
            Some(signature) => signature,
            None => return Err(Error::Authentication("Missing signature header!".to_string())),
        };

        // Parse timestamp to integer
        let slack_timestamp = match slack_ts_header.to_str() {
            Ok(slice) => match slice.parse::<i64>() {
                Ok(epoch) => epoch,
                Err(error) => return Err(Error::Authentication(format!("Couldn't parse timestamp to integer: {error}")))
            }
            Err(error) => return Err(Error::Authentication(format!("Couldn't parse timestamp-header to string: {error}")))
        };

        // Check timestamp age
        if (slack_timestamp - Local::now().timestamp()).abs() > 30 {
            return Err(Error::Authentication("Slack timestamp was over 30 seconds old!".to_string()))
        };

        // Encypt basestring to HMAC_sha256
        let hmac = HMAC::mac(
            // Construct basestring to encrypt
            // Version will always be "v0" for now - Slack might change this later.
            format!("v0:{slack_timestamp}:{payload}"), 
            // Use signing secret as key
            self.signing_secret.clone()
        );

        // Get signature
        let signature = match slack_signature_header.to_str() {
            Ok(slice) => slice.to_string(),
            Err(error) => return Err(Error::Authentication(format!("Couldn't parse signature-header to string: {error}")))
        };

        // Ensure signature is matching
        // Again "v0" is used here - Slack might change this value later.
        if format!("v0={}", hex::encode(hmac)) != signature {
            return Err(Error::Authentication("Signatures didn't match".to_string()))
        };

        Self::log("OK");

        // Decode payload
        match urlencoding::decode(&payload.replace('+', " ")) {
            Ok(decoded) => Ok(decoded.into_owned().replace("payload=", "")),
            Err(error) => Err(Error::Parsing(format!("Couldn't parse payload body: {error}"))),
        }
    }
}