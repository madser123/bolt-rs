use crate::pre::*;
use reqwest::Url;
use serde::{Deserializer, de::IgnoredAny};

pub fn default_phantomdata<'de, D, T>(
    deserializer: D,
) -> Result<std::marker::PhantomData<T>, D::Error>
where
    D: Deserializer<'de>,
    T: SerializeDefaultPhantomData,
{
    // Ignore the data in the input.
    IgnoredAny::deserialize(deserializer)?;
    Ok(T::new())
}

pub trait SerializeDefaultPhantomData {
    fn new() -> std::marker::PhantomData<Self> {
        std::marker::PhantomData::<Self>
    }
}

pub fn parse_url(url: &str) -> BoltResult<Null> {
    if let Err(error) = Url::parse(url) {
        return Err(Error::Parsing("url".to_string(), error.to_string()));
    }
    Ok(Null::Null)
}
