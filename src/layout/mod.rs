use crate::pre::{
    json, parsing, skip_serializing_none, BoltResult, Build, Deserialize, Error, Serialize,
};

use serde::de::DeserializeOwned;
use std::fmt::{Debug, Display, Formatter};

pub mod block;
pub mod comp;
pub mod element;

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
            Self::Default => write!(f, "default"),
            Self::Danger => write!(f, "danger"),
            Self::Primary => write!(f, "primary"),
        }
    }
}

pub trait SurfaceType:
    parsing::SerializeDefaultPhantomData + DeserializeOwned + Serialize + Debug
{
}

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
