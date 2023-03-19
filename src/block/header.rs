use super::*;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
pub struct Header {
    r#type: String,
    text: Text<Plain>,
    block_id: Option<String>,
}
impl Block for Header {}
impl Default for Header {
    fn default() -> Self {
        Self {
            r#type: "header".to_string(),
            text: Text::default(),
            block_id: None,
        }
    }
}
impl Header {
    pub fn new(text: Text<Plain>) -> Self {
        Self {
            text,
            ..Default::default()
        }
    }

    /// Add a block-id
    pub fn id(mut self, id: &str) -> Self {
        self.block_id = Some(id.to_string());
        self
    }
}
impl Build for Header {
    fn get_type(&self) -> String {
        "header".to_string()
    }
}
