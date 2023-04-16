use super::*;
use std::fmt::Display;
use axum::{
    response::IntoResponse,
    http::StatusCode,
};

/// A bolt-rs App-related error
pub enum Error {
    /// Errors regarding parsing of requests, interactions or alike.
    Parsing(String),

    /// Problems regarding Authentification of incoming requests.
    Authentication(String),


    /// Errors relating to `block-actions` interactions from slack.
    BlockAction(String),

    /// Errors relating to `message-actions` interactions from slack.
    MessageAction(String),

    /// Errors relating to `shortcuts` interactions from slack.
    Shortcut(String),

    /// Errors relating to `view-closed` interactions from slack.
    ViewClosed(String),

    /// Errors relating to `view-submission` interactions from slack.
    ViewSubmission(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("{self}");
        (StatusCode::INTERNAL_SERVER_ERROR, "An error occurred.").into_response()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parsing(error) => {
                write!(f, "[ERROR][Parsing] {error}")
            },
            Self::Authentication(error) => {
                write!(f, "[ERROR][Authentication] {error}")
            },
            Self::BlockAction(error) => {
                write!(f, "[ERROR][BlockAction] {error}")
            },
            Self::MessageAction(error) => {
                write!(f, "[ERROR][MessageAction] {error}")
            },
            Self::Shortcut(error) => {
                write!(f, "[ERROR][Shortcut] {error}")
            },
            Self::ViewClosed(error) => {
                write!(f, "[ERROR][ViewClosed] {error}")
            },
            Self::ViewSubmission(error) => {
                write!(f, "[ERROR][ViewSubmission] {error}")
            },
        }
    }
}

impl From<json::Error> for Error {
    fn from(value: json::Error) -> Self {
        Self::Parsing(value.to_string())
    }
}