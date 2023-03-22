use std::fmt::{Display, Formatter};

use crate::pre::*;

pub mod block;
pub mod element;
pub mod comp;

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

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct HomeTab {}
impl parsing::SerializeDefaultPhantomData for HomeTab {}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct Modal {}
impl parsing::SerializeDefaultPhantomData for Modal {}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct ModalResponse {}
impl parsing::SerializeDefaultPhantomData for ModalResponse {}