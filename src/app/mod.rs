use crate::Error;

mod app;
mod auth;

pub struct App {
    bot_token: Option<String>,
    user_token: Option<String>,

    auth: Auth
}

pub struct Auth {
    signing_secret: String,
}