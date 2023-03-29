use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use futures::{future::BoxFuture, Future};
use serde_json as json;

mod auth;
mod error;

use error::Error;

pub use auth::Auth;
pub use crate::payload::{
    Interaction,
    BlockAction,
    MessageAction,
    Shortcut,
    ViewClosed,
    ViewSubmission,
};

pub type AppResult<T> = Result<T, Error>;

type Interactions<T> = HashMap<String, Box<dyn Fn(T) -> BoxFuture<'static, AppResult<()>> + Send + Sync>>;

pub struct App {
    address: SocketAddr,
    auth: Auth,

    block_actions: Interactions<BlockAction>,
    message_actions: Interactions<MessageAction>,
    shortcuts: Interactions<Shortcut>,
    view_closes: Interactions<ViewClosed>,
    view_submissions: Interactions<ViewSubmission>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            address: SocketAddr::from(([0, 0, 0, 0], 8080)),
            auth: Auth::default(),
            block_actions: HashMap::new(),
            message_actions: HashMap::new(),
            shortcuts: HashMap::new(),
            view_closes: HashMap::new(),
            view_submissions: HashMap::new(),
        }
    }
}

impl App {
    pub fn new(auth: Auth) -> Self {
        Self {
            auth,
            ..Default::default()
        }
    }

    pub fn address(mut self, addr: SocketAddr) -> Self {
        self.address = addr;
        self
    }

    async fn handle_interaction<T: Interaction>(closures: Arc<Interactions<T>>, interaction: T) {
        let closure = closures.get(&interaction.identifier()).unwrap();
        closure(interaction).await;
    }

    pub async fn start(self) {
        let block_actions = Arc::new(self.block_actions);
        let message_actions = Arc::new(self.message_actions);
        let shortcuts = Arc::new(self.shortcuts);
        let view_closes = Arc::new(self.view_closes);
        let view_submissions = Arc::new(self.view_submissions);
        let interaction_handler = move | body: String | async move {
            // Start by printing info-message
            println!("[INFO][Interaction] Recieved new interaction!");

            // Parse json
            let json = json::to_value(&body)?;
            
            let r#type = match json.get("type") {
                Some(t) => t.to_string(),
                None => return Err(Error::Parsing("Received new interaction without a type!".to_string())),
            };

            
            match r#type.as_str() {
                "block_actions" | "interactive_message" => {
                    let interaction = match json::from_str::<BlockAction>(&body) {
                        Ok(i) => i,
                        Err(error) => return Err(Error::BlockAction(format!("Tried to parse JSON to struct: {error}")))
                    };
                    Ok(Self::handle_interaction(block_actions, interaction).await)
                },
                "message_action" => {
                    let interaction = match json::from_str::<MessageAction>(&body) {
                        Ok(i) => i,
                        Err(error) => return Err(Error::MessageAction(format!("Tried to parse JSON to struct: {error}")))
                    };
                    Ok(Self::handle_interaction(message_actions, interaction).await)
                },
                "shortcut" => {
                    let interaction = match json::from_str::<Shortcut>(&body) {
                        Ok(i) => i,
                        Err(error) => return Err(Error::Shortcut(format!("Tried to parse JSON to struct: {error}")))
                    };
                    Ok(Self::handle_interaction(shortcuts, interaction).await)
                },
                "view_closed" => {
                    let interaction = match json::from_str::<ViewClosed>(&body) {
                        Ok(i) => i,
                        Err(error) => return Err(Error::ViewClosed(format!("Tried to parse JSON to struct: {error}")))
                    };
                    Ok(Self::handle_interaction(view_closes, interaction).await)
                },
                "view_submission" => {
                    let interaction = match json::from_str::<ViewSubmission>(&body) {
                        Ok(i) => i,
                        Err(error) => return Err(Error::ViewSubmission(format!("Tried to parse JSON to struct: {error}")))
                    };
                    Ok(Self::handle_interaction(view_submissions, interaction).await)
                }

                t => Err(Error::Parsing(format!("'{t}' is not a known interaction type!")))
            }
        };

        let router = axum::Router::new()
            .route("/", axum::routing::post(interaction_handler));

        let server = axum::Server::bind(&self.address);

        // Middleware

        server
            .serve(router.into_make_service())
            .await
            .unwrap();
    }

    pub fn block_actions<F, Fut>(mut self, identifier: &str, fun: F) -> Self
    where
        Fut: Future<Output = ()> + Send + 'static,
        F: Fn(BlockAction) -> Fut + Send + Sync + 'static,
    {
        todo!()
    }

    pub fn message_actions<F, Fut>(mut self, callback_id: &str, fun: F) -> Self
    where
        Fut: Future<Output = ()> + Send + 'static,
        F: Fn(MessageAction) -> Fut + Send + Sync + 'static,
    {
        todo!()
    }

    pub fn shortcut<F, Fut>(mut self, callback_id: &str, fun: F) -> Self
    where
        Fut: Future<Output = AppResult<()>> + Send + 'static,
        F: Fn(Shortcut) -> Fut + Send + Sync + 'static,
    {
        self.shortcuts.insert(
            callback_id.to_string(),
            Box::new(move |interaction| Box::pin(fun(interaction))),
        );
        self
    }

    pub fn view_close<F, Fut>(mut self, identifier: &str, fun: F) -> Self
    where
        Fut: Future<Output = ()> + Send + 'static,
        F: Fn(ViewClosed) -> Fut + Send + Sync + 'static,
    {
        todo!()
    }

    pub fn view_submission<F, Fut>(mut self, identifier: &str, fun: F) -> Self
    where
        Fut: Future<Output = ()> + Send + 'static,
        F: Fn(ViewSubmission) -> Fut + Send + Sync + 'static,
    {
        todo!()
    }
}