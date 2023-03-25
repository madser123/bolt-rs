use super::*;

#[derive(Default)]
pub struct Auth {
    // Tokens
    bot_token: Option<String>,
    user_token: Option<String>,

    // Client info
    signing_secret: String,
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
}