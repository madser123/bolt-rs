use super::*;

/// A composition-object 
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct OptionObject<T: parsing::SerializeDefaultPhantomData = Any> {
    text: Text<T>,
    value: String,
    description: Option<Text<Plain>>,
    url: Option<String>,
}
impl Composition for OptionObject {}
impl<T: Default + parsing::SerializeDefaultPhantomData> OptionObject<T> {
    /// Creates a new [OptionObject] composition-block
    pub fn new(text: Text<T>, value: &str) -> Self {
        Self {
            text,
            value: value.to_string(),
            ..Default::default()
        }
    }

    /// Adds a description to the [OptionObject]
    pub fn description(mut self, text: Text<Plain>) -> Self {
        self.description = Some(text);
        self
    }

    /// Adds a url 
    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct OptionGroup {
    label: Text<Plain>,
    options: Vec<OptionObject>,
}
impl Composition for OptionGroup {}
impl OptionGroup {
    pub fn new(label: Text<Plain>, options: Vec<OptionObject>) -> Self {
        Self { label, options }
    }
}
