use super::*;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Checkboxes {
    r#type: String,
    action_id: String,
    options: Vec<OptionObject<Any>>,
    initial_options: Option<Vec<OptionObject<Any>>>,
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
    pub fn new(options: Vec<OptionObject<Any>>, action_id: &str) -> Self {
        Self {
            options,
            action_id: action_id.to_string(),
            ..Default::default()
        }
    }

    pub fn initial_options(mut self, options: Vec<OptionObject<Any>>) -> Self {
        self.initial_options = Some(options);
        self
    }

    pub fn comfirm(mut self, confirm: Confirmation) -> Self {
        self.confirm = Some(confirm);
        self
    }

    pub fn focus_on_load(mut self, focus: bool) -> Self {
        self.focus_on_load = Some(focus);
        self
    }

}
impl Build for Checkboxes {
    fn get_type(&self) -> String {
        "checkboxes".to_string()
    }
}