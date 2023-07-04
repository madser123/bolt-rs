#![warn(clippy::all, rust_2018_idioms)]

mod pre;
mod tests;

#[cfg(feature = "client")]
pub mod app;

pub mod conversation;
pub mod core;
pub mod file;
pub mod layout;
pub mod message;
pub mod user;
pub mod view;

#[cfg(feature = "client")]
pub use app::App;

pub use layout::{block, comp as composition, element};

pub use crate::core::{payload, request::Request, response::Response, BoltResult, Error};
