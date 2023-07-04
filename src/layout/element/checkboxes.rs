use super::{
    option, skip_serializing_none, ActionsElement, Any, Build, Confirmation, Debug, Deserialize,
    Element, InputElement, SectionElement, Serialize,
};

/// Represents an element of type `checkboxes`
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Checkboxes {
    r#type: String,
    action_id: String,
    options: Vec<option::Object<Any>>,
    initial_options: Option<Vec<option::Object<Any>>>,
    confirm: Option<Confirmation>,
    focus_on_load: Option<bool>,
}
impl SectionElement for Checkboxes {}
impl ActionsElement for Checkboxes {}
impl InputElement for Checkboxes {}
impl Element for Checkboxes {}
impl Default for Checkboxes {
    fn default() -> Self {
        Self {
            r#type: "checkboxes".to_string(),
            action_id: String::default(),
            options: Vec::default(),
            initial_options: None,
            confirm: None,
            focus_on_load: None,
        }
    }
}
impl Checkboxes {
    /// Creates a new [Checkboxes] element
    #[must_use]
    pub fn new(options: Vec<option::Object<Any>>, action_id: &str) -> Self {
        Self {
            options,
            action_id: action_id.to_string(),
            ..Default::default()
        }
    }

    /// Provides the inital options to be selected upon loading this element
    #[must_use]
    pub fn initial_options(mut self, options: Vec<option::Object<Any>>) -> Self {
        self.initial_options = Some(options);
        self
    }

    /// Adds a confirmation dialogue to the form
    #[must_use]
    pub fn comfirm(mut self, confirm: Confirmation) -> Self {
        self.confirm = Some(confirm);
        self
    }

    /// Forces the element to be focused upon load
    #[must_use]
    pub const fn focus_on_load(mut self, focus: bool) -> Self {
        self.focus_on_load = Some(focus);
        self
    }
}
impl Build for Checkboxes {
    fn get_type(&self) -> String {
        "checkboxes".to_string()
    }
}
