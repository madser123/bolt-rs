
use super::Error;
use serde::{de::IgnoredAny, Deserializer, Deserialize};
use url::Url;

pub async fn parse_response<T: serde::de::DeserializeOwned>(resp: reqwest::Response) -> Result<T, Error> {
    match resp.json::<T>().await {
        Ok(t) => Ok(t),
        Err(error) => Err(Error::Parsing(std::any::type_name::<T>().to_string(), error.to_string()))
    }
}

pub fn parse_url(url: &str) -> Result<(), Error> {
    if let Err(error) = Url::parse(url) {
        return Err(Error::Parsing("url".to_string(), error.to_string()))
    }
    Ok(())
}

pub fn serde_default_skip<'de, D, T>(deserializer: D) -> Result<std::marker::PhantomData<T>, D::Error>
where
    D: Deserializer<'de>,
    T: SerdeDefaultSkip,
{
    // Ignore the data in the input.
    IgnoredAny::deserialize(deserializer)?;
    Ok(T::new())
}

pub trait SerdeDefaultSkip {
    fn new() -> std::marker::PhantomData<Self> {
        std::marker::PhantomData::<Self>
    }
}