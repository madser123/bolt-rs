use super::*;

/// A composition-block of type `dispatchactionconfig`
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct DispatchActionConfig {
    trigger_actions_on: Vec<String>,
}
impl Composition for DispatchActionConfig {}
impl DispatchActionConfig {
    /// Creates a new [DispatchActionConfig] composition-block
    pub fn new() -> Self {
        Self::default()
    }

    /// Trigger events when enter is pressed
    pub fn on_enter(mut self) -> Self {
        self.trigger_actions_on.push("on_enter_pressed".to_string());
        self
    }

    /// Trigger events on characters entered
    pub fn on_characters(mut self) -> Self {
        self.trigger_actions_on
            .push("on_character_entered".to_string());
        self
    }
}
