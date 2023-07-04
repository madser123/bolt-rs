use super::{
    skip_serializing_none, ActionsElement, Build, Confirmation, Debug, Deserialize, Element,
    InputElement, Plain, SectionElement, Serialize, Text,
};

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TimePicker {
    r#type: String,
    action_id: String,
    initial_time: Option<String>,
    confirm: Option<Confirmation>,
    focus_on_load: Option<bool>,
    placeholder: Option<Text<Plain>>,
    timezone: Option<String>,
}
impl SectionElement for TimePicker {}
impl ActionsElement for TimePicker {}
impl InputElement for TimePicker {}
impl Element for TimePicker {}
impl Default for TimePicker {
    fn default() -> Self {
        Self {
            r#type: "timepicker".to_string(),
            action_id: String::default(),
            initial_time: None,
            confirm: None,
            focus_on_load: None,
            placeholder: None,
            timezone: None,
        }
    }
}
impl TimePicker {
    /// Creates a new [`TimePicker`] element
    #[must_use]
    pub fn new(action_id: &str) -> Self {
        Self {
            action_id: action_id.to_string(),
            ..Default::default()
        }
    }

    /// Sets the initial time selected.
    #[must_use]
    pub fn initial_time(mut self, hour: i8, minute: i8) -> Self {
        self.initial_time = Some(format!("{hour:02}:{minute:02}"));
        self
    }

    /// Applies confirmation to the element
    #[must_use]
    pub fn confirm(mut self, confirm: Confirmation) -> Self {
        self.confirm = Some(confirm);
        self
    }

    /// Sets the element to be focused on load.
    #[must_use]
    pub const fn focus_on_load(mut self) -> Self {
        self.focus_on_load = Some(true);
        self
    }

    /// Sets the placeholder text for the element
    #[must_use]
    pub fn placeholder(mut self, text: Text<Plain>) -> Self {
        self.placeholder = Some(text);
        self
    }

    /// Sets the timezone in which the date is to be selected for.
    #[must_use]
    pub fn timezone(mut self, iana: &str) -> Self {
        self.timezone = Some(iana.to_string());
        self
    }
}
impl Build for TimePicker {
    fn get_type(&self) -> String {
        "timepicker".to_string()
    }
}
