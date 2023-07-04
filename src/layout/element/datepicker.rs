use super::{
    skip_serializing_none, ActionsElement, Build, Confirmation, Debug, Deserialize, Element,
    InputElement, Plain, SectionElement, Serialize, Text,
};

/// Represents an element of type `datepicker`
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DatePicker {
    r#type: String,
    action_id: String,
    // YYYY-MM-DD
    initial_date: Option<String>,
    confirm: Option<Confirmation>,
    focus_on_load: Option<bool>,
    placeholder: Option<Text<Plain>>,
}
impl SectionElement for DatePicker {}
impl ActionsElement for DatePicker {}
impl InputElement for DatePicker {}
impl Element for DatePicker {}
impl Default for DatePicker {
    fn default() -> Self {
        Self {
            r#type: "datepicker".to_string(),
            action_id: String::default(),
            initial_date: None,
            confirm: None,
            focus_on_load: None,
            placeholder: None,
        }
    }
}
impl DatePicker {
    /// Creates a new [`DatePicker`] element
    #[must_use]
    pub fn new(action_id: &str) -> Self {
        Self {
            action_id: action_id.to_string(),
            ..Default::default()
        }
    }

    /// Sets the initial date selected upon load
    #[must_use]
    pub fn initial_date(mut self, date: &str) -> Self {
        self.initial_date = Some(date.to_string());
        self
    }

    /// Adds a confirmation dialogue to the form
    #[must_use]
    pub fn confirm(mut self, confirm: Confirmation) -> Self {
        self.confirm = Some(confirm);
        self
    }

    /// Forces the element to be focused upon load
    #[must_use]
    pub const fn focus_on_load(mut self, focus: bool) -> Self {
        self.focus_on_load = Some(focus);
        self
    }

    /// Adds placeholder text to the datepicker
    #[must_use]
    pub fn placeholder(mut self, text: Text<Plain>) -> Self {
        self.placeholder = Some(text);
        self
    }
}
impl Build for DatePicker {
    fn get_type(&self) -> String {
        "datepicker".to_string()
    }
}
