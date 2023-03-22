use super::*;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Overflow {
    r#type: String,
    action_id: String,
    options: Vec<OptionObject<Plain>>,
    confirm: Option<Confirmation>,
}
impl SectionElement for Overflow {}
impl ActionsElement for Overflow {}
impl Element for Overflow {}
impl Default for Overflow {
    fn default() -> Self {
        Self {
            r#type: "overflow".to_string(),
            action_id: String::default(),
            options: Vec::new(),
            confirm: None,
        }
    }
}
impl Overflow {
    pub fn new(action_id: &str, options: Vec<OptionObject<Plain>>) -> Self {
        Self {
            action_id: action_id.to_string(),
            options,
            ..Default::default()
        }
    }

    pub fn confirm(mut self, confirm: Confirmation) -> Self {
        self.confirm = Some(confirm);
        self
    }
}
impl Build for Overflow {
    fn get_type(&self) -> String {
        "overflow".to_string()
    }
}
