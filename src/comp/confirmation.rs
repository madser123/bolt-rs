use super::*;
use text::{Any, Plain, Text};

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

    pub fn style(mut self, style: Style) -> Self {
        self.style = style.to_string();
        self
    }
}
impl Build for Confirmation {
    fn get_type(&self) -> String {
        "confirmation".to_string()
    }
}
