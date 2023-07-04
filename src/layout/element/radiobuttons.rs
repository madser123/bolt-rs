use super::{
    option, skip_serializing_none, ActionsElement, Build, Confirmation, Debug, Deserialize,
    Element, InputElement, SectionElement, Serialize,
};

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RadioButtons {
    r#type: String,
    action_id: String,
    options: Vec<option::Object>,
    initial_option: Option<option::Object>,
    confirm: Option<Confirmation>,
    focus_on_load: Option<bool>,
}
impl SectionElement for RadioButtons {}
impl ActionsElement for RadioButtons {}
impl InputElement for RadioButtons {}
impl Element for RadioButtons {}
impl Default for RadioButtons {
    fn default() -> Self {
        Self {
            r#type: "radio_buttons".to_string(),
            action_id: String::default(),
            options: Vec::new(),
            initial_option: None,
            confirm: None,
            focus_on_load: None,
        }
    }
}
impl RadioButtons {
    /// Creates a new [`RadioButtons`] element.
    #[must_use]
    pub fn new(action_id: &str, options: Vec<option::Object>) -> Self {
        Self {
            action_id: action_id.to_string(),
            options,
            ..Default::default()
        }
    }

    /// Sets the initial options selected
    #[must_use]
    pub fn initial_option(mut self, option: option::Object) -> Self {
        self.initial_option = Some(option);
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
}
impl Build for RadioButtons {
    fn get_type(&self) -> String {
        "radio_buttons".to_string()
    }
}
