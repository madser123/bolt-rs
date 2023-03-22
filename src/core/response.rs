use super::*;

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Response<V> {
    #[serde(alias = "view")]
    #[serde(alias = "user")]
    #[serde(alias = "members")]
    #[serde(alias = "message")] 
    #[serde(alias = "channel")]
    value: Option<V>,
    ok: bool,

    // Conversation specific
    no_op: Option<bool>,
    already_open: Option<bool>,

    // Message specific
    channel_id: Option<String>,
    ts: Option<String>,

    // Misc
    cache_ts: Option<String>,
    error: Option<String>,
    response_metadata: Option<HashMap<String, Vec<String>>>,
}

impl<V: serde::de::DeserializeOwned> Response<V> {
    pub async fn from_json(
        resp: reqwest::Response,
    ) -> BoltResult<Self> {
        // This implementation should be rewritten
        // I think we could check the response for the user, before returning.
        // I just don't know how to construct the error yet. 
        // How do you access an enum variant through generics?
        match resp.json::<Response<V>>().await {
            Ok(t) => Ok(t),
            Err(error) => Err(Error::Response(std::any::type_name::<V>().to_string(), error.to_string())),
        }
    }
}

impl<V> Response<V> {
    pub fn is_ok(&self) -> bool {
        self.ok
    }

    pub fn format_error(self) -> String {
        format!("{:?}\nMetadata: {:#?}\nTimestamp: {:?}", self.error, self.response_metadata, self.cache_ts)
    }

    pub fn value(self) -> Option<V> {
        self.value
    }

    pub fn unpack(self) -> BoltResult<V> {
        // Check for errors
        if !self.is_ok() {
            return Err(Error::Response(std::any::type_name::<V>().to_string(), self.format_error()))
        }

        if let Some(message) = self.value() {
            return Ok(message)
        }

        Err(Error::Response(std::any::type_name::<V>().to_string(), "Recieved an OK response and an empty value?!".to_string()))
    }
}