use super::{BoltResult, Deserialize, Error, Serialize};
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
    ///
    /// # Errors
    ///
    /// Errors will occur if the requested state is not found.
    pub fn get_value(&self, block_id: &str, action_id: &str) -> BoltResult<&String> {
        let Some(block) = self.values.get(block_id) else {
            return Err(Error::State(format!("Couldn't get state value of block: '{block_id}'")))
        };

        if let Some(value) = block.get(action_id) {
            if let Some(v) = &value.value {
                return Ok(v);
            }
        }

        Err(Error::State(format!(
            "Couldn't get state value of block: '{block_id}' action: '{action_id}'"
        )))
    }
}
