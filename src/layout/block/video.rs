use super::{
    comp::{Plain, Text},
    skip_serializing_none, Block, Build, Debug, Deserialize, Serialize,
};

/// A block of type `video`
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
pub struct Video {
    r#type: String,
    alt_text: String,
    author_name: Option<String>,
    block_id: Option<String>,
    description: Option<Text<Plain>>,
    provider_icon_url: Option<String>,
    provider_name: Option<String>,
    title: Text<Plain>,
    title_url: Option<String>,
    thumbnail_url: String,
    video_url: String,
}
impl Block for Video {}
impl Default for Video {
    fn default() -> Self {
        Self {
            r#type: "video".to_string(),
            alt_text: String::default(),
            author_name: None,
            block_id: None,
            description: None,
            provider_icon_url: None,
            provider_name: None,
            title: Text::default(),
            title_url: None,
            thumbnail_url: String::default(),
            video_url: String::default(),
        }
    }
}
impl Video {
    /// Creates a new [Video] block
    #[must_use]
    pub fn new(title: Text<Plain>, video_url: &str, thumbnail_url: &str, alt_text: &str) -> Self {
        Self {
            title,
            video_url: video_url.to_string(),
            thumbnail_url: thumbnail_url.to_string(),
            alt_text: alt_text.to_string(),
            ..Default::default()
        }
    }

    /// Add an author to the video
    #[must_use]
    pub fn author(mut self, name: &str) -> Self {
        self.author_name = Some(name.to_string());
        self
    }

    /// Add a block-id
    #[must_use]
    pub fn id(mut self, id: &str) -> Self {
        self.block_id = Some(id.to_string());
        self
    }

    /// Adds a video description
    #[must_use]
    pub fn description(mut self, text: Text<Plain>) -> Self {
        self.description = Some(text);
        self
    }

    /// Add a provider icon
    #[must_use]
    pub fn provider_icon(mut self, url: &str) -> Self {
        self.provider_icon_url = Some(url.to_string());
        self
    }

    /// Add a provider name
    #[must_use]
    pub fn provider(mut self, name: &str) -> Self {
        self.provider_name = Some(name.to_string());
        self
    }

    /// Specifies a url that should open when the video-title is clicked.
    #[must_use]
    pub fn title_url(mut self, url: &str) -> Self {
        self.title_url = Some(url.to_string());
        self
    }
}
impl Build for Video {
    fn get_type(&self) -> String {
        "video".to_string()
    }
}
