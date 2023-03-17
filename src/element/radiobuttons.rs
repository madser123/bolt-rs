use super::*;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RadioButtons {
    r#type: String,
    action_id: String,
    options: Vec<OptionObject>,
    initial_option: Option<OptionObject>,
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
    pub fn new(action_id: &str, options: Vec<OptionObject>) -> Self {
        Self {
            action_id: action_id.to_string(),
            options,
            ..Default::default()
        }
    }

    pub fn initial_option(mut self, option: OptionObject) -> Self {
        self.initial_option = Some(option);
        self
    }

    pub fn confirm(mut self, confirm: Confirmation) -> Self {
        self.confirm = Some(confirm);
        self
    }

    pub fn focus_on_load(mut self) -> Self {
        self.focus_on_load = Some(true);
        self
    }
}
impl Build for RadioButtons {
    fn get_type(&self) -> String {
        "radio_buttons".to_string()
    }
}