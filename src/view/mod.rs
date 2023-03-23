use std::fmt::Debug;
use crate::pre::*;
use comp::{Text, Plain};
use serde::de::DeserializeOwned;

mod controller;

pub use controller::Controller;

pub trait AsView<T: parsing::SerializeDefaultPhantomData> {
    fn as_view(&self) -> BoltResult<View<T>>;
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct View<T: parsing::SerializeDefaultPhantomData = ModalResponse> {
    #[serde(default, deserialize_with = "parsing::default_phantomdata", skip_serializing)]
    t: std::marker::PhantomData<T>,

    r#type: String,
    title: Option<Text<Plain>>,
    blocks: Option<block::Blocks>,
    close: Option<Text<Plain>>,
    submit: Option<Text<Plain>>,
    private_metadata: Option<String>,
    callback_id: Option<String>,
    clear_on_close: Option<bool>,
    notify_on_close: Option<bool>,
    external_id: Option<String>,
    submit_disabled: Option<bool>,

    // For usage in submission payload:
    state: Option<state::State>,
}
impl View {
    pub fn home(blocks: block::Blocks) -> View<HomeTab> {
        View::<HomeTab> {
            r#type: "home".to_string(),
            blocks: Some(blocks),
            ..Default::default()
        }
    }

    pub fn modal(title: Text<Plain>, blocks: block::Blocks) -> View<Modal> {
        View::<Modal> {
            r#type: "modal".to_string(),
            title: Some(title),
            blocks: Some(blocks),
            ..Default::default()
        }
    }
}
impl<T: DeserializeOwned + Serialize + parsing::SerializeDefaultPhantomData + Debug> View<T> {
    pub fn private_metadata(mut self, data: &str) -> Self {
        self.private_metadata = Some(data.to_string());
        self
    }

    pub fn callback_id(mut self, id: &str) -> Self {
        self.callback_id = Some(id.to_string());
        self
    }

    pub fn external_id(mut self, id: &str) -> Self {
        self.external_id = Some(id.to_string());
        self
    }

    pub async fn open(self, trigger_id: &str, token: &str) -> BoltResult<Null> {
        Request::post("views.open", token)
            .json(&Controller::trigger(trigger_id, self))
            .send()
            .await?
            .unpack()
    }

    pub async fn update(self, token: &str) -> BoltResult<Null> {
        Request::post("views.update", token)
            .json(&Controller::update(self))
            .send()
            .await?
            .unpack()
    }
}
impl View<HomeTab> {}
impl View<Modal> {
    pub fn close(mut self, text: Text<Plain>) -> Self {
        self.close = Some(text);
        self
    }

    pub fn submit(mut self, text: Text<Plain>) -> Self {
        self.submit = Some(text);
        self
    }

    pub fn clear_on_close(mut self) -> Self {
        self.clear_on_close = Some(true);
        self
    }

    pub fn notify_on_close(mut self) -> Self {
        self.notify_on_close = Some(true);
        self
    }

    //pub fn submit_disabled(mut self) -> Self {
    //    self.submit_disabled = Some(true);
    //    self
    //}
}
impl<T: parsing::SerializeDefaultPhantomData> Build for View<T> {
    fn get_type(&self) -> String {
        self.r#type.clone()
    }
}

impl View<ModalResponse> {
    pub fn get_callback_id(&self) -> &str {
        self.callback_id.as_ref().unwrap()
    }

    pub fn get_private_metadata(&self) -> &str {
        self.private_metadata.as_ref().unwrap()
    }

    pub fn get_state_value(&self, block_id: &str, action_id: &str) -> BoltResult<String> {
        if self.state.is_none() {
            return Err(Error::View(format!("Couldn't get state-value block: '{block_id}' action: '{action_id}'. No state found.")));
        }

        let block = match self.state.as_ref().unwrap().values.get(block_id) {
            Some(b) => b,
            None => {
                return Err(Error::View(format!("Couldn't get state value of block: '{block_id}'")))
            }
        };

        if let Some(value) = block.get(action_id) {
            if let Some(v) = &value.value {
                return Ok(v.to_owned());
            }
        }

        Err(Error::View(format!("Couldn't get state value of block: '{block_id}' action: '{action_id}'")))
    }
}
