use super::{element, parsing, skip_serializing_none, Build, Debug, Deserialize, Serialize, Style};

mod confirmation;
mod dispatchactionconfig;
mod filter;
mod text;

pub mod option;

pub use confirmation::Confirmation;
pub use dispatchactionconfig::DispatchActionConfig;
pub use filter::Filter;
pub use text::{Any, Markdown, Plain, Text};

pub trait Composition {}
