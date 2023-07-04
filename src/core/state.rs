use super::*;
use std::collections::HashMap;

/// A state-object from Slack.
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct State {
    pub values: HashMap<String, HashMap<String, Value>>,
}

/// A value from a [State] object
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Value {
    pub r#type: String,

    #[serde(alias = "selected_conversation")]
    #[serde(alias = "selected_user")]
    pub value: Option<String>,
}


impl State {
    /// Tries to extract the value from a specified state-object.
    pub fn get_value(&self, block_id: &str, action_id: &str) -> Result<&String, String> {
        let block = match self.values.get(block_id) {
            Some(b) => b,
            None => {
                return Err(format!("Couldn't get state value of block: '{block_id}'"))
            }
        };

        if let Some(value) = block.get(action_id) {
            if let Some(v) = &value.value {
                return Ok(v);
            }
        }

        Err(format!("Couldn't get state value of block: '{block_id}' action: '{action_id}'"))
    }
}