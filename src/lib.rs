#![warn(clippy::all, rust_2018_idioms)]

mod pre;
mod tests;

mod app;
mod core;

pub mod conversation;
pub mod file;
pub mod layout;
pub mod message;
pub mod user;
pub mod view;

pub use layout::{
    block,
    comp as composition,
    element,
};

pub use crate::core::{
    Error,
    BoltResult,
    request::Request,
    response::Response,
    payload,
};
