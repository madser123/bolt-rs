use super::{
    skip_serializing_none, Build, Debug, Deserialize, DispatchActionConfig, Element, InputElement,
    Plain, Serialize, Text,
};

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Url {
    r#type: String,
    action_id: String,
    inital_value: Option<String>,
    dispatch_action_config: Option<DispatchActionConfig>,
    focus_on_load: Option<bool>,
    placeholder: Option<Text<Plain>>,
}
impl InputElement for Url {}
impl Element for Url {}
impl Default for Url {
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
impl Url {
    /// Creates a new [`Url`] element
    #[must_use]
    pub fn new(action_id: &str) -> Self {
        Self {
            action_id: action_id.to_string(),
            ..Default::default()
        }
    }

    /// Sets the initial value of the element
    #[must_use]
    pub fn initial_value(mut self, value: &str) -> Self {
        self.inital_value = Some(value.to_string());
        self
    }

    /// Applies a dispatch-action config to the element
    #[must_use]
    pub fn dispatch_action_config(mut self, config: DispatchActionConfig) -> Self {
        self.dispatch_action_config = Some(config);
        self
    }

    /// Sets the element to be focused on load
    #[must_use]
    pub const fn focus_on_load(mut self) -> Self {
        self.focus_on_load = Some(true);
        self
    }

    /// Sets the placeholder text for the element.
    #[must_use]
    pub fn placeholder(mut self, text: Text<Plain>) -> Self {
        self.placeholder = Some(text);
        self
    }
}
impl Build for Url {
    fn get_type(&self) -> String {
        "url_text_input".to_string()
    }
}
