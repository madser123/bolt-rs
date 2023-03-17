use super::*;

#[skip_serializing_none]
#[derive(Serialize)]
pub struct Section {
    r#type: String,
    text: Option<Text<Any>>,
    block_id: Option<String>,
    fields: Option<Vec<Text<Any>>>,
    accessory: Option<json::Value>,
}
impl Block for Section {}
impl Default for Section {
    fn default() -> Self {
        Self {
            r#type: "section".to_string(),
            text: None,
            block_id: None,
            fields: None,
            accessory: None,
        }
    }
}
impl Section {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add text
    pub fn text(mut self, text: Text<Any>) -> Self {
        self.text = Some(text);
        self
    }

    /// Add a block-id
    pub fn id(mut self, id: &str) -> Self {
        self.block_id = Some(id.to_string());
        self
    }

    /// Adds a plaintext field to the section
    pub fn field(self, text: Text<Any>) -> Self {
        self.fields(vec![text])
    }

    /// Add a vec of fields to the section
    pub fn fields(mut self, mut objects: Vec<Text<Any>>) -> Self {
        match self.fields.as_mut() {
            None => self.fields = Some(objects),
            Some(fields) => fields.append(&mut objects),
        }
        self
    }

    pub fn accessory(mut self, element: impl SectionElement) -> Result<Self, Error> {
        self.accessory = Some(element.build()?);
        Ok(self)
    }
}
impl Build for Section {
    fn get_type(&self) -> String {
        "section".to_string()
    }
}
