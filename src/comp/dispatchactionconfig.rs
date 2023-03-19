use super::*;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct DispatchActionConfig {
    trigger_actions_on: Vec<String>,
}
impl Composition for DispatchActionConfig {}
impl DispatchActionConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn on_enter(mut self) -> Self {
        self.trigger_actions_on.push("on_enter_pressed".to_string());
        self
    }

    pub fn on_characters(mut self) -> Self {
        self.trigger_actions_on
            .push("on_character_entered".to_string());
        self
    }
}
impl Build for DispatchActionConfig {
    fn get_type(&self) -> String {
        "dispatchactionconfig".to_string()
    }
}
