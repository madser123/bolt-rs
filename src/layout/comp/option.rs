use super::*;

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
    pub fn new(text: Text<T>, value: &str) -> Self {
        Self {
            text,
            value: value.to_string(),
            ..Default::default()
        }
    }

    pub fn description(mut self, text: Text<Plain>) -> Self {
        self.description = Some(text);
        self
    }

    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }
}
impl Build for OptionObject {
    fn get_type(&self) -> String {
        "composition:optionobject".to_string()
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
impl Build for OptionGroup {
    fn get_type(&self) -> String {
        "optiongroup".to_string()
    }
}
