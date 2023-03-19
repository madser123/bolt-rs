use crate::core::Build;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{Display, Formatter};

mod confirmation;
mod dispatchactionconfig;
mod filter;
mod option;
mod text;

pub use confirmation::Confirmation;
pub use dispatchactionconfig::DispatchActionConfig;
pub use filter::Filter;
pub use option::{OptionGroup, OptionObject};
pub use text::{Any, Markdown, Plain, Text};

pub trait Composition: Build {}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub enum Style {
    #[default]
    Default,
    Danger,
    Primary,
}

impl Display for Style {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Style::Default => write!(f, "default"),
            Style::Danger => write!(f, "danger"),
            Style::Primary => write!(f, "primary"),
        }
    }
}
