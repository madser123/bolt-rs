use super::*;
use json::Map;

/// A response from slack containing either an error or a value.
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Response<V> {
    ok: bool,

    #[serde(alias = "view")]
    #[serde(alias = "user")]
    #[serde(alias = "profile")]
    #[serde(alias = "members")]
    #[serde(alias = "message")] 
    #[serde(alias = "file")]
    #[serde(alias = "files")]
    #[serde(alias = "channel")]
    value: Option<V>,

    // Conversation specific
    no_op: Option<bool>,
    already_open: Option<bool>,

    // Misc
    ts: Option<String>,
    cache_ts: Option<i64>,
    error: Option<String>,
    response_metadata: Option<json::Value>,
}

impl<V: serde::de::DeserializeOwned> Response<V> {
    /// Tries to create a new instance of [Response] from a [reqwest::Response]
    pub async fn from_response(
        resp: reqwest::Response,
    ) -> BoltResult<Self> {
        // Stupid fix for channel reappearing in `Response<Message>` with a String value, instead of a struct
        // as it is in `Response<Conversation>`. Here we just remove the key from the root-json, put it into the "message" object 
        // and then deserialize it into the struct.
        // TODO: REWRITE!!!!!!! Find another solution to this - There has to be an easier way.
        let mut map: Map<String, json::Value> = json::from_str(&resp.text().await.unwrap()).unwrap();
        if let Some(c) = map.get("channel") {
            let channel: json::Value;
            if c.is_string() {
                channel = map.remove("channel").unwrap();
                if let Some(m) = map.get_mut("message") {
                    if let Some(message) = m.as_object_mut() {
                        message.insert("channel".to_string(), channel);
                    }
                }
            }
        }
        // This implementation may be rewritten
        // Maybe we could check the response for the user, before returning.
        // Don't know how to construct the error yet. s
        // How do you access an enum variant through generics?
        match json::from_value(json::Value::from(map)) {
            Ok(t) => Ok(t),
            Err(error) => Err(Error::Response(std::any::type_name::<V>().to_string(), error.to_string())),
        }
    }
}



impl<V> Response<V> {
    /// If the request has no errors, this will return `true`
    pub fn is_ok(&self) -> bool {
        self.ok
    }

    /// Formats the error, metadata and timestamp into a readable string.
    fn format_error(self) -> String {
        format!("{:?}\nMetadata: {:#?}\nTimestamp: {:?}", self.error, self.response_metadata, self.cache_ts)
    }

    /// Returns the requested value as an option.
    fn value(self) -> Option<V> {
        self.value
    }

    /// Unwraps the response to a result containing the value, if any.
    pub fn unpack(self) -> BoltResult<V> {
        // Check for errors
        if !self.is_ok() {
            return Err(Error::Response(std::any::type_name::<V>().to_string(), self.format_error()))
        }

        if let Some(value) = self.value() {
            return Ok(value)
        }

        Err(Error::Response(std::any::type_name::<V>().to_string(), "Recieved an OK response and an empty value?!".to_string()))
    }
}