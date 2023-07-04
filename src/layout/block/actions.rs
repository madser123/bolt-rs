use super::{
    element::ActionsElement, json, skip_serializing_none, Block, BoltResult, Build, Debug,
    Deserialize, Serialize,
};

/// A block of type `actions`
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
pub struct Actions {
    r#type: String,
    elements: Vec<json::Value>,
    block_id: Option<String>,
}
impl Block for Actions {}
impl Default for Actions {
    fn default() -> Self {
        Self {
            r#type: "actions".to_string(),
            elements: Vec::new(),
            block_id: None,
        }
    }
}
impl Actions {
    /// Creates a new [Actions] block.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add multiple elements
    ///
    /// # Errors
    ///
    /// An error will occur if one or more of the supplied elements fails serializing.
    ///
    pub fn elements(mut self, elements: Vec<impl ActionsElement>) -> BoltResult<Self> {
        for e in elements {
            self.elements.push(e.build()?);
        }
        Ok(self)
    }

    /// Add a block-id
    #[must_use]
    pub fn id(mut self, id: &str) -> Self {
        self.block_id = Some(id.to_string());
        self
    }
}
impl Build for Actions {
    fn get_type(&self) -> String {
        "actions".to_string()
    }
}
