use super::*;
use comp::{Text, Plain};

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
pub struct Image {
    r#type: String,
    image_url: String,
    alt_text: String,
    title: Option<Text<Plain>>,
    block_id: Option<String>,
}
impl Block for Image {}
impl Default for Image {
    fn default() -> Self {
        Self {
            r#type: "image".to_string(),
            image_url: String::default(),
            alt_text: String::default(),
            title: None,
            block_id: None,
        }
    }
}
impl Image {
    pub fn new(url: &str, alt_text: &str) -> Self {
        Self {
            image_url: url.to_string(),
            alt_text: alt_text.to_string(),
            ..Default::default()
        }
    }

    /// Add a title
    pub fn title(mut self, text: Text<Plain>) -> Self {
        self.title = Some(text);
        self
    }

    /// Add a block-id
    pub fn id(mut self, id: &str) -> Self {
        self.block_id = Some(id.to_string());
        self
    }
}
impl Build for Image {
    fn get_type(&self) -> String {
        "image".to_string()
    }
}

impl From<crate::file::File> for Image {
    fn from(value: crate::file::File) -> Self {
        Self::new(value.permalink_public(), value.name())
    }
}
