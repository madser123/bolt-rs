#![warn(clippy::all, rust_2018_idioms)]

mod pre;
mod tests;

mod core;
mod app;

pub mod block;
pub mod conversation;
pub mod element;
pub mod file;
pub mod message;
pub mod user;
pub mod comp;
pub mod view;

pub use crate::app::{App, Auth};
pub use crate::core::Error;
pub use crate::core::parsing;
pub use crate::core::payload;