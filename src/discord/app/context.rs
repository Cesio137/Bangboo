use crate::discord::app::{commands::AppCommands, events::AppEvents};
use crate::settings::env::ENV_SCHEMA;
use crate::tools::automod::ScamFilter;
use anyhow::{Error, Result};
use std::sync::Arc;
use tokio::sync::Mutex;
use twilight_http::Client;
use twilight_model::gateway::payload::incoming::MessageCreate;

#[derive(Clone)]
pub struct AppContext {
    pub client: Arc<Client>,
    pub commands: Arc<Mutex<AppCommands>>,
    pub events: Arc<Mutex<AppEvents>>,
    pub scam_filter: Arc<Mutex<ScamFilter>>,
}

impl AppContext {
    pub fn new() -> Result<Self, Error> {
        let client = Arc::new(Client::new(ENV_SCHEMA.bot_token.clone()));
        let commands = Arc::new(Mutex::new(AppCommands::new()));
        let events = Arc::new(Mutex::new(AppEvents::new()));
        let scam_filter = Arc::new(Mutex::new(ScamFilter::new()?));

        Ok(Self {
            client,
            commands,
            events,
            scam_filter
        })
    }
}

#[derive(Clone)]
pub struct PrefixCommandContext {
    pub message: Box<MessageCreate>,
    pub client: Arc<Client>,
}

impl PrefixCommandContext {
    pub fn new(message: Box<MessageCreate>, client: Arc<Client>) -> Result<Self, Error> {
        Ok(Self {
            message,
            client,
        })
    }
}