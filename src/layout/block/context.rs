use super::{
    element::ContextElement, json, skip_serializing_none, Block, BoltResult, Build, Debug,
    Deserialize, Serialize,
};

/// A block of type `context`
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
pub struct Context {
    r#type: String,
    elements: Vec<json::Value>,
    block_id: Option<String>,
}
impl Block for Context {}
impl Default for Context {
    fn default() -> Self {
        Self {
            r#type: "context".to_string(),
            elements: Vec::new(),
            block_id: None,
        }
    }
}
impl Context {
    /// Creates a new [Context] block
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Pushes multiple elements to the context
    ///
    /// # Errors
    ///
    /// An error will occur if one or more the supplied elements fails serializing.
    ///
    pub fn elements(mut self, elements: &mut Vec<impl ContextElement>) -> BoltResult<Self> {
        for e in elements {
            self.elements.push(e.build()?);
        }
        Ok(self)
    }

    /// Pushes a single element to the context
    ///
    /// # Errors
    ///
    /// An error will occur if the supplied element fails serializing.
    ///
    pub fn element(mut self, element: &impl ContextElement) -> BoltResult<Self> {
        self.elements.push(element.build()?);
        Ok(self)
    }

    /// Add a block-id
    #[must_use]
    pub fn id(mut self, id: &str) -> Self {
        self.block_id = Some(id.to_string());
        self
    }
}
impl Build for Context {
    fn get_type(&self) -> String {
        "context".to_string()
    }
}
