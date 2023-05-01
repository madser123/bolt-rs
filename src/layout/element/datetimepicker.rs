use super::*;

/// Represents an element of type `datetimepicker`
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DatetimePicker {
    r#type: String,
    action_id: String,
    /// UNUIX timestamp in seconds
    initial_date_time: Option<String>,
    confirm: Option<Confirmation>,
    focus_on_load: Option<bool>,
}
impl ActionsElement for DatetimePicker {}
impl InputElement for DatetimePicker {}
impl Element for DatetimePicker {}
impl Default for DatetimePicker {
    fn default() -> Self {
        Self {
            r#type: "datetimepicker".to_string(),
            action_id: String::default(),
            initial_date_time: None,
            confirm: None,
            focus_on_load: None,
        }
    }
}
impl DatetimePicker {
    /// Creates a new [DatetimePicker] element
    pub fn new(action_id: &str) -> Self {
        Self {
            action_id: action_id.to_string(),
            ..Default::default()
        }
    }

    /// Sets the initial datetime selected upon load
    pub fn initial_datetime(mut self, date: &str) -> Self {
        self.initial_date_time = Some(date.to_string());
        self
    }

    /// Adds a confirmation-dialogue to the form
    pub fn confirm(mut self, confirm: Confirmation) -> Self {
        self.confirm = Some(confirm);
        self
    }

    /// Forces the element to be focused upon load
    pub fn focus_on_load(mut self, focus: bool) -> Self {
        self.focus_on_load = Some(focus);
        self
    }
}
impl Build for DatetimePicker {
    fn get_type(&self) -> String {
        "datetimepicker".to_string()
    }
}
