use crate::settings::env::ENV_SCHEMA;
use crate::tools::automod::ScamFilter;
use anyhow::{Error, Result};
use std::sync::Arc;
use twilight_http::Client;

use super::commands::AppCommands;

pub struct AppContext {
    pub client: Arc<Client>,
    pub commands: AppCommands,
    pub scam_filter: ScamFilter,
}

impl AppContext {
    pub fn new() -> Result<Self, Error> {
        let client = Arc::new(Client::new(ENV_SCHEMA.bot_token.clone()));
        let commands = AppCommands::new();
        let scam_filter = ScamFilter::new()?;

        Ok(Self {
            client,
            commands,
            scam_filter
        })
    }
}