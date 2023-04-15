use super::*;
use std::fmt::Display;
use hyper::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    Parsing(String),
    Authentication(String),

    BlockAction(String),
    MessageAction(String),
    Shortcut(String),
    ViewClosed(String),
    ViewSubmission(String),
}

impl Into<hyper::http::Result<hyper::Response<String>>> for Error {
    fn into(self) -> hyper::http::Result<hyper::Response<String>> {
        println!("{self}");
        hyper::Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("An error occurred".to_string())
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