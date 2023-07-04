use super::*;
use serde::de::DeserializeOwned;
use reqwest::{Client, multipart::Form};

/// A request to the Slack-API.
pub struct Request(reqwest::RequestBuilder);

impl Request {

    /// Creates a new [Request] with the `POST` http-method.
    pub fn post(endpoint: &str, token: &str) -> Self {
        let client = Client::new();
        Self(client.post(format!("https://slack.com/api/{endpoint}")).bearer_auth(token))
    }

    /// Creates a new [Request] with the `GET` http-method.
    pub fn get(endpoint: &str, token: &str) -> Self {
        let client = Client::new();
        Self(client.get(format!("https://slack.com/api/{endpoint}")).bearer_auth(token))
    }

    /// Adds a json-body to the request.
    /// 
    /// # Errors
    /// 
    /// Serialization can fail if T's implementation of Serialize decides to fail, or if T contains a map with non-string keys
    pub fn json<T: Serialize + ?Sized>(mut self, json: &T ) -> Self {
        self.0 = self.0.json(json);
        self
    }

    /// Adds a multipart form to the request
    pub fn multipart(mut self, form: Form) -> Self {
        self.0 = self.0.multipart(form);
        self
    }

    /// Adds a header to the request
    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.0 = self.0.header(key, value);
        self
    }

    /// Sends the request
    /// 
    /// # Errors
    /// 
    /// If the request can't be sent, an error will be returned.
    /// Any other API-related errors will be included in the [Response].
    pub async fn send<V: DeserializeOwned>(self) -> BoltResult<Response<V>> {
        Response::from_response(self.0.send().await?).await
    }
}