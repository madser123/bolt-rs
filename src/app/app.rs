use super::*;

impl App {
    pub fn new(
        auth: Auth, 
        bot_token: Option<String>, 
        user_token: Option<String>
    ) -> Self {
        if bot_token.is_none() && user_token.is_none() {
            panic!("No tokens defined!")
        }
        Self {
            auth,
            bot_token,
            user_token,
        }
    }
}