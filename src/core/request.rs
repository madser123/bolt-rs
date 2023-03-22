use serde::de::DeserializeOwned;

use super::*;

pub struct Request(reqwest::RequestBuilder);

impl Request {
    pub fn post(endpoint: &str, token: &str) -> Self {
        let client = reqwest::Client::new();
        Self(client.post(format!("https://slack.com/api/{endpoint}")).bearer_auth(token))
    }

    pub fn get(endpoint: &str, token: &str) -> Self {
        let client = reqwest::Client::new();
        Self(client.get(format!("https://slack.com/api/{endpoint}")).bearer_auth(token))
    }

    pub fn json<T: Serialize + ?Sized>(mut self, json: &T ) -> Self {
        self.0 = self.0.json(json);
        self
    }

    pub fn multipart(mut self, form: reqwest::multipart::Form) -> Self {
        self.0 = self.0.multipart(form);
        self
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.0 = self.0.header(key, value);
        self
    }

    pub async fn send<V: DeserializeOwned>(self) -> BoltResult<Response<V>> {
        Response::from_json(self.0.send().await?).await
    }
}