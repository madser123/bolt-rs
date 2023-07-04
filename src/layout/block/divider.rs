use super::{skip_serializing_none, Block, Build, Debug, Deserialize, Serialize};

/// A block of type `divider`
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
pub struct Divider {
    r#type: String,
    block_id: Option<String>,
}
impl Block for Divider {}
impl Default for Divider {
    fn default() -> Self {
        Self {
            r#type: "divider".to_string(),
            block_id: None,
        }
    }
}
impl Divider {
    /// Creates a new [Divider] block
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a block-id
    #[must_use]
    pub fn id(mut self, id: &str) -> Self {
        self.block_id = Some(id.to_string());
        self
    }
}
impl Build for Divider {
    fn get_type(&self) -> String {
        "divider".to_string()
    }
}
