use super::{
    parsing, skip_serializing_none, Any, Composition, Debug, Deserialize, Plain, Serialize, Text,
};

/// A composition-object
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Object<T: parsing::SerializeDefaultPhantomData = Any> {
    text: Text<T>,
    value: String,
    description: Option<Text<Plain>>,
    url: Option<String>,
}
impl Composition for Object {}
impl<T: Default + parsing::SerializeDefaultPhantomData> Object<T> {
    /// Creates a new [`OptionObject`] composition-block
    #[must_use]
    pub fn new(text: Text<T>, value: &str) -> Self {
        Self {
            text,
            value: value.to_string(),
            ..Default::default()
        }
    }

    /// Adds a description to the [`OptionObject`]
    #[must_use]
    pub fn description(mut self, text: Text<Plain>) -> Self {
        self.description = Some(text);
        self
    }

    /// Adds a url
    #[must_use]
    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Group {
    label: Text<Plain>,
    options: Vec<Object>,
}
impl Composition for Group {}
impl Group {
    #[must_use]
    pub fn new(label: Text<Plain>, options: Vec<Object>) -> Self {
        Self { label, options }
    }
}
