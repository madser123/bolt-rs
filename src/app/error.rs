use super::*;
use std::fmt::Display;
use axum::{
    response::IntoResponse,
    http::StatusCode,
};

pub enum Error {
    Parsing(String),

    BlockAction(String),
    MessageAction(String),
    Shortcut(String),
    ViewClosed(String),
    ViewSubmission(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parsing(error) => {
                write!(f, "[ERROR][Parsing] {error}")
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