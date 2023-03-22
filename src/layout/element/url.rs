use super::*;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UrlInput {
    r#type: String,
    action_id: String,
    inital_value: Option<String>,
    dispatch_action_config: Option<DispatchActionConfig>,
    focus_on_load: Option<bool>,
    placeholder: Option<Text<Plain>>,
}
impl InputElement for UrlInput {}
impl Element for UrlInput {}
impl Default for UrlInput {
    fn default() -> Self {
        Self {
            r#type: "url_text_input".to_string(),
            action_id: String::default(),
            inital_value: None,
            dispatch_action_config: None,
            focus_on_load: None,
            placeholder: None,
        }
    }
}
impl UrlInput {
    pub fn new(action_id: &str) -> Self {
        Self {
            action_id: action_id.to_string(),
            ..Default::default()
        }
    }

    pub fn initial_value(mut self, value: &str) -> Self {
        self.inital_value = Some(value.to_string());
        self
    }

    pub fn dispatch_action_config(mut self, config: DispatchActionConfig) -> Self {
        self.dispatch_action_config = Some(config);
        self
    }

    pub fn focus_on_load(mut self) -> Self {
        self.focus_on_load = Some(true);
        self
    }

    pub fn placeholder(mut self, text: Text<Plain>) -> Self {
        self.placeholder = Some(text);
        self
    }
}
impl Build for UrlInput {
    fn get_type(&self) -> String {
        "url_text_input".to_string()
    }
}
