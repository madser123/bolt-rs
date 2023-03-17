use super::*;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Image {
    r#type: String,
    image_url: String,
    alt_text: String,
}
impl SectionElement for Image {}
impl ContextElement for Image {}
impl Element for Image {}
impl Default for Image {
    fn default() -> Self {
        Self {
            r#type: "image".to_string(),
            image_url: String::default(),
            alt_text: String::default(),
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
}
impl Build for Image {
    fn get_type(&self) -> String {
        "email_text_input".to_string()
    }
}
