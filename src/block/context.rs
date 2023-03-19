use super::*;

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
    pub fn new() -> Self {
        Self::default()
    }

    /// Add multiple elements
    pub fn elements(mut self, elements: &mut Vec<impl ContextElement>) -> Result<Self, Error> {
        for e in elements {
            self.elements.push(e.build()?);
        }
        Ok(self)
    }

    pub fn element(mut self, element: impl ContextElement) -> Result<Self, Error> {
        self.elements.push(element.build()?);
        Ok(self)
    }

    /// Add a block-id
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
