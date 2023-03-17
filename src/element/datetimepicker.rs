use super::*;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DatetimePicker {
    r#type: String,
    action_id: String,
    // UNUIX timestamp in seconds
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
    pub fn new(action_id: &str) -> Self {
        Self {
            action_id: action_id.to_string(),
            ..Default::default()
        }
    }

    pub fn initial_datetime(mut self, date: &str) -> Self {
        self.initial_date_time = Some(date.to_string());
        self
    }

    pub fn confirm(mut self, confirm: Confirmation) -> Self {
        self.confirm = Some(confirm);
        self
    }

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
