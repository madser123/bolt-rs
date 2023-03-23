use crate::pre::*;

use std::fmt::{Debug, Display, Formatter};
use serde::de::DeserializeOwned;


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

pub trait SurfaceType: parsing::SerializeDefaultPhantomData + DeserializeOwned + Serialize + Debug {}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct HomeTab {}
impl parsing::SerializeDefaultPhantomData for HomeTab {}
impl SurfaceType for HomeTab {}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct Modal {}
impl parsing::SerializeDefaultPhantomData for Modal {}
impl SurfaceType for Modal {}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct ModalResponse {}
impl parsing::SerializeDefaultPhantomData for ModalResponse {}
impl SurfaceType for ModalResponse {}