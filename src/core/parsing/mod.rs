use super::*;
use serde::{de::IgnoredAny, Deserialize, Deserializer};
use url::Url;

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct SlackResponse<V> {
    ok: bool,

    #[serde(alias = "view")]
    #[serde(alias = "user")]
    #[serde(alias = "members")]
    value: Option<V>,

    cache_ts: Option<i64>,
    error: Option<String>,
    response_metadata: Option<HashMap<String, Vec<String>>>,
}

impl<V: serde::de::DeserializeOwned> SlackResponse<V> {
    pub async fn from_json(
        resp: reqwest::Response,
    ) -> SlackResult<Self> {
        // This implementation should be rewritten
        // I think we could check the response for the user, before returning.
        // I just don't know how to construct the error yet. 
        // How do you access an enum variant through generics?
        match resp.json::<SlackResponse<V>>().await {
            Ok(t) => Ok(t),
            Err(error) => Err(Error::Parsing(
                std::any::type_name::<V>().to_string(),
                error.to_string(),
            )),
        }
    }
}

impl<V> SlackResponse<V> {
    pub fn is_ok(&self) -> bool {
        self.ok
    }

    pub fn error(self) -> String {
        format!("{:?}\nMetadata: {:#?}\nTimestamp: {:?}", self.error, self.response_metadata, self.cache_ts)
    } 

    pub fn value(self) -> Option<V> {
        self.value
    }
}

pub fn parse_url(url: &str) -> Result<(), Error> {
    if let Err(error) = Url::parse(url) {
        return Err(Error::Parsing("url".to_string(), error.to_string()));
    }
    Ok(())
}

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
