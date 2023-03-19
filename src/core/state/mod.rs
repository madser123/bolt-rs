use super::*;
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct State {
    pub values: HashMap<String, HashMap<String, StateValue>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct StateValue {
    pub r#type: String,

    #[serde(alias = "selected_conversation")]
    #[serde(alias = "selected_user")]
    pub value: Option<String>,
}
