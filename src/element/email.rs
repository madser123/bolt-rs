use super::*;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Email {
    r#type: String,
    action_id: String,
    initial_value: Option<String>,
    dispatch_action_config: Option<DispatchActionConfig>,
    focus_on_load: Option<bool>,
    placeholder: Option<Text<Plain>>,
}
impl Default for Email {
    fn default() -> Self {
        Self {
            r#type: "email_text_input".to_string(),
            action_id: String::default(),
            initial_value: None,
            dispatch_action_config: None,
            focus_on_load: None,
            placeholder: None,
        }
    }
}
impl InputElement for Email {}
impl Element for Email {}
impl Email {
    pub fn new(action_id: &str) -> Self {
        Self {
            action_id: action_id.to_string(),
            ..Default::default()
        }
    }

    pub fn initial_value(mut self, value: &str) -> Self {
        self.initial_value = Some(value.to_string());
        self
    }

    pub fn dispatch_action_config(mut self, config: DispatchActionConfig) -> Self {
        self.dispatch_action_config = Some(config);
        self
    }

    pub fn focus_on_load(mut self, focus: bool) -> Self {
        self.focus_on_load = Some(focus);
        self
    }

    pub fn placeholder(mut self, text: Text<Plain>) -> Self {
        self.placeholder = Some(text);
        self
    }
}
impl Build for Email {
    fn get_type(&self) -> String {
        "email_text_input".to_string()
    }
}
