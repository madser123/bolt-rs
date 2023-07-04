use super::{
    skip_serializing_none,
    text::{Any, Plain, Text},
    Composition, Debug, Deserialize, Serialize, Style,
};

/// A composition-block of type `confirmation`
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Confirmation {
    title: Text<Plain>,
    text: Text<Any>,
    confirm: Text<Plain>,
    deny: Text<Plain>,
    style: String,
}
impl Composition for Confirmation {}
impl Confirmation {
    /// Creates a new [Confirmation] composition-block
    #[must_use]
    pub fn new(
        title: Text<Plain>,
        text: Text<Any>,
        confirm: Text<Plain>,
        deny: Text<Plain>,
    ) -> Self {
        Self {
            title,
            text,
            confirm,
            deny,
            ..Default::default()
        }
    }

    /// Sets the style of the confirmation-buttons
    #[must_use]
    pub fn style(mut self, style: &Style) -> Self {
        self.style = style.to_string();
        self
    }
}
