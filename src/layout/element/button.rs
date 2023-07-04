use super::{
    skip_serializing_none, ActionsElement, Build, Confirmation, Debug, Deserialize, Element, Plain,
    SectionElement, Serialize, Style, Text,
};

/// Represents an element of type `button`s
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Button {
    r#type: String,
    text: Text<Plain>,
    action_id: String,
    url: Option<String>,
    value: Option<String>,
    style: Option<String>,
    confirm: Option<Confirmation>,
    accessibility_label: Option<String>,
}
impl SectionElement for Button {}
impl ActionsElement for Button {}
impl Element for Button {}
impl Default for Button {
    fn default() -> Self {
        Self {
            r#type: "button".to_string(),
            text: Text::default(),
            action_id: String::default(),
            url: None,
            value: None,
            style: None,
            confirm: None,
            accessibility_label: None,
        }
    }
}
impl Button {
    /// Creates a new [`Button`] element
    #[must_use]
    pub fn new(text: Text<Plain>, action_id: &str) -> Self {
        Self {
            text,
            action_id: action_id.to_string(),
            ..Default::default()
        }
    }

    /// Sets a url that is opened upon clicking the button
    #[must_use]
    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }

    /// Sets the value to be returned to the app when interacting with the button.
    #[must_use]
    pub fn value(mut self, value: &str) -> Self {
        self.value = Some(value.to_string());
        self
    }

    /// Sets the [Style] of the button
    #[must_use]
    pub fn style(mut self, style: &Style) -> Self {
        self.style = Some(style.to_string());
        self
    }

    /// Adds a confirmation dialogue that appears when the button has been pressed.
    #[must_use]
    pub fn confirm(mut self, confirm: Confirmation) -> Self {
        self.confirm = Some(confirm);
        self
    }

    /// Adds an accessibility label that appears when hovering over the button.
    #[must_use]
    pub fn accessibility_label(mut self, label: &str) -> Self {
        self.accessibility_label = Some(label.to_string());
        self
    }
}
impl Build for Button {
    fn get_type(&self) -> String {
        "button".to_string()
    }
}
