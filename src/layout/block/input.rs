use super::{
    comp::{Plain, Text},
    element::{Element, InputElement},
    skip_serializing_none, Block, Build, Serialize,
};
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

    /// Sets the input to dispatch a `block_actions` payload after submission.
    #[must_use]
    pub const fn dispatch_action(mut self) -> Self {
        self.dispatch_action = Some(true);
        self
    }

    /// Add a block-id
    #[must_use]
    pub fn id(mut self, id: &str) -> Self {
        self.block_id = Some(id.to_string());
        self
    }

    /// Add a hint to the user.
    #[must_use]
    pub fn hint(mut self, hint: Text<Plain>) -> Self {
        self.hint = Some(hint);
        self
    }

    /// Sets the input to be optional.
    #[must_use]
    pub const fn optional(mut self) -> Self {
        self.optional = Some(true);
        self
    }
}
impl<E: InputElement> Build for Input<E> {
    fn get_type(&self) -> String {
        "input".to_string()
    }
}
