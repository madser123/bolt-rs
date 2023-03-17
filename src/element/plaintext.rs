use super::*;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PlainTextInput {
    r#type: String,
    action_id: String,
    initial_value: Option<String>,
    multiline: Option<bool>,
    min_length: Option<i64>,
    max_length: Option<i64>,
    dispatch_action_config: Option<DispatchActionConfig>,
    focus_on_load: Option<bool>,
    placeholder: Option<Text<Plain>>,
}
impl InputElement for PlainTextInput {}
impl Element for PlainTextInput {}
impl Default for PlainTextInput {
    fn default() -> Self {
        Self {
            r#type: "plain_text_input".to_string(),
            action_id: String::default(),
            initial_value: None,
            multiline: None,
            min_length: None,
            max_length: None,
            dispatch_action_config: None,
            focus_on_load: None,
            placeholder: None,
        }
    }
}
impl PlainTextInput {
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

    pub fn multiline(mut self) -> Self {
        self.multiline = Some(true);
        self
    }

    pub fn min_length(mut self, min: i64) -> Self {
        self.min_length = Some(min);
        self
    }

    pub fn max_length(mut self, max: i64) -> Self {
        self.max_length = Some(max);
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
impl Build for PlainTextInput {
    fn get_type(&self) -> String {
        "plain_text_input".to_string()
    }
}