use super::*;

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
    pub fn new() -> Self {
        Self::default()
    }

    /// Add multiple elements
    pub fn elements(mut self, elements: Vec<impl ActionsElement>) -> Result<Self, Error> {
        for e in elements {
            self.elements.push(e.build()?);
        }
        Ok(self)
    }

    /// Add a block-id
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