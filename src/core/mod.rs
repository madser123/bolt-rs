use crate::pre::*;
use std::fmt::{Display, Formatter};

pub mod payload;
pub mod parsing;
pub mod response;
pub mod request;
pub mod state;

pub type BoltResult<T> = Result<T, Error>;

/// A Bolt-rs related error.
#[derive(Debug)]
pub enum Error {
    /// Errors regarding blocks.
    Block(String, String),

    /// Errors with building types to JSON.
    Building(String, json::Error),

    /// Errors from conversations.
    Conversation(String),

    /// Errors regarding composition-blocks.
    Composition(String, String),

    /// Errors regarding element-blocks.
    Element(String, String),

    /// Errors regarding files (upload/download).
    File(String),

    /// Errors from messages.
    Message(String),

    /// Errors with parsing certain elements.
    Parsing(String, String),

    /// Errors regarding sending requests to slack.
    Request(reqwest::Error),

    /// Errors originating from slack-responses.
    Response(String, String),

    /// Errors regarding users.
    User(String),

    /// Errors regarding views.
    View(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Block(r#type, error) => {
                write!(f, "Block '{type}': {error}")
            }
            Error::Building(r#type, error) => {
                write!(f, "Error during json-building type '{type}': {error}")
            }
            Error::Conversation(error) => {
                write!(f, "Conversation-error: {error}")
            }
            Error::Composition(r#type, error) => {
                write!(f, "Composition '{type}': {error}")
            }
            Error::Message(error) => {
                write!(f, "Message-error: {error}")
            }
            Error::Element(r#type, error) => {
                write!(f, "Element '{type}': {error}")
            }
            Error::File(error) => {
                write!(f, "File-error: {error}")
            }
            Error::Parsing(object, error) => {
                write!(f, "Parsing-error '{object}': {error}")
            }
            Error::Response(expected, error) => {
                write!(f, "Response-error (expected '{expected}'): {error}")
            }
            Error::Request(error) => {
                write!(f, "Request-error: {error}")
            }
            Error::User(error) => {
                write!(f, "User error: {error}")
            }
            Error::View(error) => {
                write!(f, "View error: {error}")
            }
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Request(value)
    }
}

/// Used internally to build blocks to JSON.
pub trait Build: Serialize {
    fn build(&self) -> BoltResult<json::Value> {
        match json::to_value(self) {
            Ok(json) => Ok(json),
            Err(error) => Err(Error::Building(self.get_type(), error)),
        }
    }

    fn get_type(&self) -> String;
}
