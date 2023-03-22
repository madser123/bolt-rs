#![warn(clippy::all, rust_2018_idioms)]

mod pre;
mod tests;

mod core;
mod app;

pub mod block;
pub mod comp;
pub mod element;
pub mod user;
pub mod file;
pub mod view;
pub mod message;
pub mod surface;

pub use crate::file::{File, Upload};
pub use crate::view::View;
pub use crate::message::{Message, MessageAble};

pub use crate::app::{App, Auth};
pub use crate::core::parsing;
pub use crate::core::payload;
pub use crate::core::Error;
pub use crate::core::SlackResult;
