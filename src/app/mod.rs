use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use axum::http::HeaderMap;
use futures::{future::BoxFuture, Future};
use serde::de::DeserializeOwned;
use serde_json as json;
use colored::Colorize;

mod auth;
mod error;

pub(crate) use error::Error;

pub use auth::Auth;
pub use crate::payload::{
    BlockAction,
    MessageAction,
    Shortcut,
    ViewClosed,
    ViewSubmission,
};

/// A result from a bolt-rs App.
pub type AppResult<T> = Result<T, Error>;

/// A collection of closures
type Interactions<T> = HashMap<String, Box<dyn Fn(T) -> BoxFuture<'static, AppResult<()>> + Send + Sync>>;

/// Defines behaviour for an interaction.
/// 
/// The identifier-functions help find the correct closure for the interaction.
/// The error-function is used to define the error-type that the interaction should return.
pub(crate) trait Interaction: DeserializeOwned {
    fn identifier(&self) -> String;
    fn identifier_name() -> String;
    fn error(message: String) -> crate::app::Error; 
}

/// The logging "system" for slack-rs.
/// 
/// This trait is used internally to streamline the logging output.
pub(crate) trait Logger {
    /// Should the name of the object the logger trait is attached to
    fn name() -> String;

    /// A normal informational log:`[INFO][<name>] <message>`
    fn log(message: &str) {
        let banner = format!("[INFO][{}]", Self::name()).green();
        println!("{banner} {message}");
    }

    /// A warning log:`[WARNING][<name>] <message>`
    fn warn(message: &str) {
        let banner = format!("[WARNING][{}]", Self::name()).yellow();
        println!("{banner} {message}");
    }
}

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

impl Logger for App {
    fn name() -> String {
        "App".to_string()
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

    async fn handle_interaction<T: Interaction>(closures: Arc<Interactions<T>>, interaction: String) -> AppResult<()> {
        // Parse interaction
        let interaction = match json::from_str::<T>(&interaction) {
            Ok(i) => i,
            Err(error) => return Err(T::error(format!("Tried to parse JSON to struct: {error}")))
        };

        // Get identifier
        let identifier = interaction.identifier();

        // Get and run closure
        match closures.get(&identifier) {
            Some(closure) => closure(interaction).await,
            None => Err(T::error(format!("Unknown {}: '{}'", T::identifier_name(), identifier)))
        }
    }

    pub async fn start(self) {
        // Check for warnings
        self.run_pre_startup_checks();

        // Closure bindings
        let block_actions = Arc::new(self.block_actions);
        let message_actions = Arc::new(self.message_actions);
        let shortcuts = Arc::new(self.shortcuts);
        let view_closes = Arc::new(self.view_closes);
        let view_submissions = Arc::new(self.view_submissions);

        // HANDLER: Interactions
        let interaction_handler = move | headers: HeaderMap, body: String | async move {
            // Sanitize payload
            let payload = self.auth.sanitize_payload(&body, headers)?;
            
            // Parse json
            let json: json::Map<String, json::Value> = json::from_str(&payload)?;

            // Get type
            let r#type = match json.get("type") {
                Some(t) => t.to_string().replace('"', ""),
                None => return Err(Error::Parsing("Received new interaction without a type!".to_string())),
            };
            
            Self::log(&format!("Recieved a new '{type}' interaction"));

            // Match type of interaction to handle
            match r#type.as_str() {
                "block_actions" 
                | "interactive_message" => Self::handle_interaction(block_actions, payload).await?,
                "message_action"        => Self::handle_interaction(message_actions, payload).await?,
                "shortcut"              => Self::handle_interaction(shortcuts, payload).await?,
                "view_closed"           => Self::handle_interaction(view_closes, payload).await?,
                "view_submission"       => Self::handle_interaction(view_submissions, payload).await?,

                t => return Err(Error::Parsing(format!("'{t}' is not a known interaction type!"))),
            };

            Ok(())
        };

        // Setup routes
        let router = axum::Router::new()
            .route("/", axum::routing::post(interaction_handler));

        // Create server
        let server = axum::Server::bind(&self.address);

        Self::log(&format!("Starting server - Serving on {}", self.address));

        // Run server
        server
            .serve(router.into_make_service())
            .await
            .unwrap();
    }

    fn run_pre_startup_checks(&self) {
        self.auth.run_pre_startup_checks();
    }

    pub fn block_actions<F, Fut>(mut self, action_id: &str, cb: F) -> Self
    where
        Fut: Future<Output = AppResult<()>> + Send + 'static,
        F: Fn(BlockAction) -> Fut + Send + Sync + 'static,
    {
        self.block_actions.insert(
            action_id.to_string(),
            Box::new(move |interaction| Box::pin(cb(interaction))),
        );
        self
    }

    pub fn message_actions<F, Fut>(mut self, callback_id: &str, cb: F) -> Self
    where
        Fut: Future<Output = AppResult<()>> + Send + 'static,
        F: Fn(MessageAction) -> Fut + Send + Sync + 'static,
    {
        self.message_actions.insert(
            callback_id.to_string(),
            Box::new(move |interaction| Box::pin(cb(interaction))),
        );
        self
    }

    pub fn shortcut<F, Fut>(mut self, callback_id: &str, cb: F) -> Self
    where
        Fut: Future<Output = AppResult<()>> + Send + 'static,
        F: Fn(Shortcut) -> Fut + Send + Sync + 'static,
    {
        self.shortcuts.insert(
            callback_id.to_string(),
            Box::new(move |interaction| Box::pin(cb(interaction))),
        );
        self
    }

    pub fn view_close<F, Fut>(mut self, callback_id: &str, cb: F) -> Self
    where
        Fut: Future<Output = AppResult<()>> + Send + 'static,
        F: Fn(ViewClosed) -> Fut + Send + Sync + 'static,
    {
        self.view_closes.insert(
            callback_id.to_string(),
            Box::new(move |interaction| Box::pin(cb(interaction))),
        );
        self
    }

    pub fn view_submission<F, Fut>(mut self, callback_id: &str, cb: F) -> Self
    where
        Fut: Future<Output = AppResult<()>> + Send + 'static,
        F: Fn(ViewSubmission) -> Fut + Send + Sync + 'static,
    {
        self.view_submissions.insert(
            callback_id.to_string(),
            Box::new(move |interaction| Box::pin(cb(interaction))),
        );
        self
    }
}