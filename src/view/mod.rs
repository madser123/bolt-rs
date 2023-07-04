use crate::pre::{
    block,
    comp::{Plain, Text},
    parsing, skip_serializing_none, state, BoltResult, Build, Deserialize, Error, HomeTab, Modal,
    ModalResponse, Request, Serialize,
};
use serde::de::DeserializeOwned;
use std::fmt::Debug;

mod controller;

pub use controller::Controller;

/// Convert any type into a view
#[allow(clippy::module_name_repetitions)]
pub trait AsView<T: parsing::SerializeDefaultPhantomData> {
    /// Turns `self` into a view.
    ///
    /// # Errors
    ///
    /// An error should occur if any blocks in the view fail to serialize.
    ///
    fn as_view(&self) -> BoltResult<View<T>>;
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct View<T: parsing::SerializeDefaultPhantomData = ModalResponse> {
    #[serde(
        default,
        deserialize_with = "parsing::default_phantomdata",
        skip_serializing
    )]
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
    /// Creates a new [`View`] for a home-tab
    #[must_use]
    pub fn home(blocks: block::Blocks) -> View<HomeTab> {
        View::<HomeTab> {
            r#type: "home".to_string(),
            blocks: Some(blocks),
            ..Default::default()
        }
    }

    /// Creates a new [`View`] as a Modal
    #[must_use]
    pub fn modal(title: Text<Plain>, blocks: block::Blocks) -> View<Modal> {
        View::<Modal> {
            r#type: "modal".to_string(),
            title: Some(title),
            blocks: Some(blocks),
            ..Default::default()
        }
    }
}
impl<
        T: DeserializeOwned + Serialize + parsing::SerializeDefaultPhantomData + Debug + Send + Sync,
    > View<T>
{
    /// Sets the private-metadata of the view
    #[must_use]
    pub fn private_metadata(mut self, data: &str) -> Self {
        self.private_metadata = Some(data.to_string());
        self
    }

    /// Sets the callback-id of the view
    #[must_use]
    pub fn callback_id(mut self, id: &str) -> Self {
        self.callback_id = Some(id.to_string());
        self
    }

    /// Sets the external-id of the view
    #[must_use]
    pub fn external_id(mut self, id: &str) -> Self {
        self.external_id = Some(id.to_string());
        self
    }

    /// Opens a new view for the supplied trigger-id
    ///
    /// # Errors
    ///
    /// An error will occur if the request fails to be sent, or if slack reports any errors back.
    ///
    pub async fn open(self, trigger_id: &str, token: &str) -> BoltResult<Self> {
        Request::post("views.open", token)
            .json(&Controller::trigger(trigger_id, self))
            .send()
            .await?
            .unpack()
    }

    /// Updates the current view
    ///
    /// # Errors
    ///
    /// An error will occur if the request fails to be sent, or if slack reports any errors back.
    ///
    pub async fn update(self, token: &str) -> BoltResult<Self> {
        Request::post("views.update", token)
            .json(&Controller::update(self))
            .send()
            .await?
            .unpack()
    }
}
impl View<HomeTab> {}
impl View<Modal> {
    /// Sets the text for the "close" button
    #[must_use]
    pub fn close(mut self, text: Text<Plain>) -> Self {
        self.close = Some(text);
        self
    }

    /// Sets the text for the "submit" button
    #[must_use]
    pub fn submit(mut self, text: Text<Plain>) -> Self {
        self.submit = Some(text);
        self
    }

    /// Clears the view on close
    #[must_use]
    pub const fn clear_on_close(mut self) -> Self {
        self.clear_on_close = Some(true);
        self
    }

    /// Notifies the server/app on close
    #[must_use]
    pub const fn notify_on_close(mut self) -> Self {
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
    /// Gets the callback-id from the View
    #[must_use]
    pub const fn get_callback_id(&self) -> Option<&String> {
        self.callback_id.as_ref()
    }

    /// Gets the private-metadata from the view
    #[must_use]
    pub const fn get_private_metadata(&self) -> Option<&String> {
        self.private_metadata.as_ref()
    }

    /// Gets a state-value from the view
    ///
    /// # Errors
    ///
    /// An error will occur if the state is not found.
    ///
    pub fn get_state_value(&self, block_id: &str, action_id: &str) -> BoltResult<String> {
        if let Some(s) = &self.state {
            return Ok(s.get_value(block_id, action_id)?.to_string());
        }

        Err(Error::View(
            "Tried to get state from view without state.".to_string(),
        ))
    }
}
