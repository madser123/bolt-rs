use super::*;
use std::fmt::Display;
use colored::Colorize;
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

        // Never leak the actual error to the endpoint.
        (StatusCode::INTERNAL_SERVER_ERROR, "An error occurred.").into_response()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parsing(error) => {
                let banner = "[ERROR][Parsing]".red();
                write!(f, "{banner} {error}")
            },
            Self::Authentication(error) => {
                let banner = "[ERROR][Authentication]".red();
                write!(f, "{banner} {error}")
            },
            Self::BlockAction(error) => {
                let banner = "[ERROR][BlockAction]".red();
                write!(f, "{banner} {error}")
            },
            Self::MessageAction(error) => {
                let banner = "[ERROR][MessageAction]".red();
                write!(f, "{banner} {error}")
            },
            Self::Shortcut(error) => {
                let banner = "[ERROR][Shortcut]".red();
                write!(f, "{banner} {error}")
            },
            Self::ViewClosed(error) => {
                let banner = "[ERROR][ViewClosed]".red();
                write!(f, "{banner} {error}")
            },
            Self::ViewSubmission(error) => {
                let banner = "[ERROR][ViewSubmission]".red();
                write!(f, "{banner} {error}")
            },
        }
    }
}

impl From<json::Error> for Error {
    fn from(value: json::Error) -> Self {
        Self::Parsing(value.to_string())
    }
}