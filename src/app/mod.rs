use std::convert::Infallible;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use futures::{future::BoxFuture, Future};
use serde::de::DeserializeOwned;
use serde_json as json;

use hyper::{Body, Request, Response, Server, Method};
use hyper::service::{make_service_fn, service_fn};

mod auth;
mod error;

pub(crate) use error::Error;

pub use auth::Auth;
use tokio::net::TcpListener;
pub use crate::payload::{
    BlockAction,
    MessageAction,
    Shortcut,
    ViewClosed,
    ViewSubmission,
};

pub type AppResult<T> = Result<T, Error>;

type IncomingInteraction<T> = Box<dyn Fn(T) -> BoxFuture<'static, AppResult<()>> + Send + Sync>;
type Interactions<T> = HashMap<String, IncomingInteraction<T>>;

pub trait Interaction: DeserializeOwned {
    fn identifier(&self) -> String;
    fn identifier_name() -> String;
    fn error(message: String) -> crate::app::Error; 
}

pub trait Logger {
    fn log(message: &str);
    fn warn(message: &str);
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
    fn log(message: &str) {
        println!("[INFO][App] {message}");
    }

    fn warn(message: &str) {
        println!("[WARNING][App] {message}");
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

    async fn handle_interaction<T: Interaction>(closures: Arc<Interactions<T>>, interaction: json::Value) -> AppResult<()> {
        // Parse interaction
        let interaction = match json::from_value::<T>(interaction) {
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

    async fn shutdown_signal() {
        // Wait for the CTRL+C signal
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install CTRL+C signal handler");
    }
    

    pub async fn start(self) -> AppResult<()> {
        // Check for warnings
        self.run_pre_startup_checks();

        // We create a TcpListener and bind it to the requested address
        let listener = TcpListener::bind(self.address).await.unwrap();

        Self::log(&format!("Starting server - Serving on {}", self.address));

        let addr = self.address.clone();

        let shortcuts = Arc::new(self.shortcuts);

        let handler = move | req: Request<Body> | async move {
            match (req.method(), req.uri().path()) {
                (&Method::POST, "/") => {
                    // Sanitize payload
                    let payload = self.auth.sanitize_payload(req).await?;
    
                    // Parse intermediate json
                    let intermediate = payload.clone();
                    let json = match intermediate.as_object() {
                        Some(obj) => obj,
                        None => return Err(Error::Parsing("Json was empty - Rejecting payload".to_string())),
                    };
    
                    // Get type
                    let r#type = match json.get("type") {
                        Some(t) => t.to_string().replace('"', ""),
                        None => return Err(Error::Parsing("Received new interaction without a type!".to_string())),
                    };
                    
                    Self::log(&format!("Recieved a new '{type}' interaction"));
    
                    // Match type of interaction to handle
                    match r#type.as_str() {
                        //"block_actions" 
                        //| "interactive_message" => self.handle_interaction(Arc::new(self.block_actions), payload).await,
                        //"message_action"        => self.handle_interaction(Arc::new(self.message_actions), payload).await,
                        "shortcut"              => Self::handle_interaction(shortcuts, payload).await,
                        //"view_closed"           => self.handle_interaction(Arc::new(self.view_closes), payload).await,
                        //"view_submission"       => self.handle_interaction(Arc::new(self.view_submissions), payload).await,
    
                        t => return Err(Error::Parsing(format!("'{t}' is not a known interaction type!"))),
                    };
    
                    Ok(Response::new("OK".to_string()))
                }
            }
        };

        let server = Server::bind(&addr)
            .serve(make_service_fn(|_conn| async {
                // service_fn converts our function into a `Service`
                Ok::<_, Infallible>(service_fn(handler))
            }));

        let graceful = server.with_graceful_shutdown(Self::shutdown_signal());

        // Run this server for... forever!
        if let Err(e) = graceful.await {
            eprintln!("server error: {}", e);
        }

        Ok(())
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