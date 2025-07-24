mod data;
mod discord;
mod menus;
mod settings;
mod tools;
mod utils;

#[cfg(target_env = "gnu")]
use settings::malloc::malloc::configure_malloc;

use crate::settings::env::ENV_SCHEMA;
use anyhow::Result;
use discord::app::base::App;
use serenity::{all::GatewayIntents, Client};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(target_env = "gnu")]
    configure_malloc();

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILDS
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MODERATION;

    let app = Arc::new(App::new());
    let mut client = Client::builder(&ENV_SCHEMA.bot_token, intents)
        .event_handler_arc(app)
        .await
        .expect("Error when trying to create gateway client");

    if let Err(err) = client.start_autosharded().await {
        eprint!("Client error: {err:?}");
    }

    Ok(())
}
