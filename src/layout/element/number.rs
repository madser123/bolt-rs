use super::{
    skip_serializing_none, Build, Debug, Deserialize, DispatchActionConfig, Element, InputElement,
    Plain, Serialize, Text,
};

/// Creates a new [Number] element with static options.
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Number<T: std::cmp::PartialOrd> {
    r#type: String,
    is_decimal_allowed: bool,
    action_id: String,
    initial_value: Option<T>,
    min_value: Option<T>,
    max_value: Option<T>,
    dispatch_action_config: Option<DispatchActionConfig>,
    focus_on_load: Option<bool>,
    placeholder: Option<Text<Plain>>,
}
impl<T: std::cmp::PartialOrd + Serialize> InputElement for Number<T> {}
impl<T: std::cmp::PartialOrd + Serialize> Element for Number<T> {}
impl<T: std::cmp::PartialOrd + Serialize> Default for Number<T> {
    fn default() -> Self {
        Self {
            r#type: "number_input".to_string(),
            is_decimal_allowed: false,
            action_id: String::default(),
            initial_value: None,
            min_value: None,
            max_value: None,
            dispatch_action_config: None,
            focus_on_load: None,
            placeholder: None,
        }
    }
}
impl<T: std::cmp::PartialOrd + Serialize> Number<T> {
    /// Creates a new [`Number`] element
    #[must_use]
    pub fn new(action_id: &str, is_decimal_allowed: bool) -> Self {
        Self {
            is_decimal_allowed,
            action_id: action_id.to_string(),
            ..Default::default()
        }
    }

    /// Sets the inital value to be selected upon load
    #[must_use]
    pub fn initial_value(mut self, value: T) -> Self {
        self.initial_value = Some(value);
        self
    }

    /// Sets a minimum value to select
    #[must_use]
    pub fn min(mut self, value: T) -> Self {
        self.min_value = Some(value);
        self
    }

    /// Sets a maximum value to select
    #[must_use]
    pub fn max(mut self, value: T) -> Self {
        self.max_value = Some(value);
        self
    }

    /// Provides a dispatch-action configuration to the element
    #[must_use]
    pub fn dispatch_action_config(mut self, config: DispatchActionConfig) -> Self {
        self.dispatch_action_config = Some(config);
        self
    }

    /// Forces the element to be focused on load
    #[must_use]
    pub fn focus_on_load(mut self, focus: bool) -> Self {
        self.focus_on_load = Some(focus);
        self
    }

    /// Adds placeholder-text to the field
    #[must_use]
    pub fn placeholder(mut self, text: Text<Plain>) -> Self {
        self.placeholder = Some(text);
        self
    }
}
impl<T: std::cmp::PartialOrd + Serialize> Build for Number<T> {
    fn get_type(&self) -> String {
        "number_input".to_string()
    }
}
