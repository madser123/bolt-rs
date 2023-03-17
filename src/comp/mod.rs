use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{Display, Formatter};
use crate::core::Build;

mod confirmation;
mod dispatchactionconfig;
mod filter;
mod text;
mod option;

pub use confirmation::Confirmation;
pub use dispatchactionconfig::DispatchActionConfig;
pub use filter::Filter;
pub use text::{Text, Markdown, Plain, Any};
pub use option::{OptionObject, OptionGroup};

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