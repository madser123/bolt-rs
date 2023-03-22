use super::*;

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Plain;
impl parsing::SerializeDefaultPhantomData for Plain {}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Markdown;
impl parsing::SerializeDefaultPhantomData for Markdown {}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Any;
impl parsing::SerializeDefaultPhantomData for Any {}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Text<T: parsing::SerializeDefaultPhantomData = Any> {
    #[serde(default, deserialize_with = "parsing::default_phantomdata", skip_serializing)]
    t: std::marker::PhantomData<T>,

    #[serde(default)]
    r#type: String,
    #[serde(default)]
    text: String,

    emoji: Option<bool>,
    verbatim: Option<bool>,
}
// Element and ContextElement are not really related to Text-objects,
// but this is needed to associate the type with the 'Context' layout-block.
impl<T: parsing::SerializeDefaultPhantomData> element::Element for Text<T> {}
impl<T: parsing::SerializeDefaultPhantomData> element::ContextElement for Text<T> {}
impl<T: parsing::SerializeDefaultPhantomData> Composition for Text<T> {}
impl<T: parsing::SerializeDefaultPhantomData> Default for Text<T> {
    fn default() -> Self {
        Text::<T> {
            t: std::marker::PhantomData::<T>,

            r#type: String::default(),
            text: String::default(),
            emoji: None,
            verbatim: None,
        }
    }
}
impl Text {
    pub fn mrkdwn(text: &str) -> Text<Markdown> {
        Text::<Markdown> {
            r#type: "mrkdwn".to_string(),
            text: text.to_string(),
            emoji: None,
            verbatim: Some(false),
            ..Default::default()
        }
    }

    pub fn plain(text: &str) -> Text<Plain> {
        Text::<Plain> {
            r#type: "plain_text".to_string(),
            text: text.to_string(),
            emoji: Some(false),
            verbatim: None,
            ..Default::default()
        }
    }
}
impl<T: parsing::SerializeDefaultPhantomData> Text<T> {
    pub fn len(&self) -> usize {
        self.text.len()
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}
impl Text<Markdown> {
    pub fn verbatim(mut self) -> Self {
        self.verbatim = Some(true);
        self
    }
}
impl Text<Plain> {
    pub fn emoji(mut self) -> Self {
        self.emoji = Some(true);
        self
    }
}
impl<T: parsing::SerializeDefaultPhantomData> Build for Text<T> {
    fn get_type(&self) -> String {
        "text".to_string()
    }
}

impl From<Text<Plain>> for Text<Any> {
    fn from(value: Text<Plain>) -> Self {
        Text::<Any> {
            t: std::marker::PhantomData::<Any>,
            r#type: value.r#type,
            text: value.text,
            emoji: value.emoji,
            verbatim: value.verbatim,
        }
    }
}

impl From<Text<Markdown>> for Text<Any> {
    fn from(value: Text<Markdown>) -> Self {
        Text::<Any> {
            t: std::marker::PhantomData::<Any>,
            r#type: value.r#type,
            text: value.text,
            emoji: value.emoji,
            verbatim: value.verbatim,
        }
    }
}
