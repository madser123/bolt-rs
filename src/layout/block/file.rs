use super::*;

/// A block of type `file`
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
pub struct File {
    r#type: String,
    external_id: String,
    source: String,
    block_id: Option<String>,
}
impl Default for File {
    fn default() -> Self {
        Self {
            r#type: "file".to_string(),
            external_id: String::default(),
            source: "remote".to_string(),
            block_id: None,
        }
    }
}
impl File {
    /// Creates a new [File] block
    pub fn new(external_id: &str) -> Self {
        Self {
            external_id: external_id.to_string(),
            ..Default::default()
        }
    }

    /// Add a block-id
    pub fn id(mut self, id: &str) -> Self {
        self.block_id = Some(id.to_string());
        self
    }
}
impl Block for File {}
impl Build for File {
    fn get_type(&self) -> String {
        "file".to_string()
    }
}
