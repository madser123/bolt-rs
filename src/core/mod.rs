use crate::pre::*;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

pub mod payload;
pub mod parsing;
pub mod response;
pub mod request;
pub mod state;

pub type BoltResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Block(String, String),
    Building(String, json::Error),
    Conversation(String),
    Composition(String, String),
    Element(String, String),
    File(String),
    Message(String),
    Parsing(String, String),
    Request(reqwest::Error),
    Response(String, String),
    User(String),
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

pub trait Build: Serialize {
    fn build(&self) -> BoltResult<json::Value> {
        match json::to_value(self) {
            Ok(json) => Ok(json),
            Err(error) => Err(Error::Building(self.get_type(), error)),
        }
    }

    fn get_type(&self) -> String;
}
