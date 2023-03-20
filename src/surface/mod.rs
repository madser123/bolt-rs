use serde::{Serialize, Deserialize};

use crate::parsing::SerializeDefaultPhantomData;


#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct HomeTab {}
impl SerializeDefaultPhantomData for HomeTab {}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct Modal {}
impl SerializeDefaultPhantomData for Modal {}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct ModalResponse {}
impl SerializeDefaultPhantomData for ModalResponse {}