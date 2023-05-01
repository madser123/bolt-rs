use super::*;
use element::{Element, InputElement};
use comp::{Text, Plain};

/// A block of type `input`
#[skip_serializing_none]
#[derive(Serialize)]
pub struct Input<E: Element> {
    r#type: String,
    label: Text<Plain>,
    element: E,
    dispatch_action: Option<bool>,
    block_id: Option<String>,
    hint: Option<Text<Plain>>,
    optional: Option<bool>,
}
impl<E: InputElement> Block for Input<E> {}
impl<E: InputElement> Input<E> {
    /// Creates a new [Input] block
    pub fn new(element: E, label: Text<Plain>) -> Self {
        Self {
            r#type: "input".to_string(),
            label,
            element,
            dispatch_action: None,
            block_id: None,
            hint: None,
            optional: None,
        }
    }

    /// Sets the input to dispatch a block_actions payload after submission.
    pub fn dispatch_action(mut self) -> Self {
        self.dispatch_action = Some(true);
        self
    }

    /// Add a block-id
    pub fn id(mut self, id: &str) -> Self {
        self.block_id = Some(id.to_string());
        self
    }

    /// Add a hint to the user.
    pub fn hint(mut self, hint: Text<Plain>) -> Self {
        self.hint = Some(hint);
        self
    }

    /// Sets the input to be optional.
    pub fn optional(mut self) -> Self {
        self.optional = Some(true);
        self
    }
}
impl<E: InputElement> Build for Input<E> {
    fn get_type(&self) -> String {
        "input".to_string()
    }
}
