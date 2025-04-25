mod discord;
mod models;
mod settings;
mod tools;
mod utils;

#[cfg(target_env = "gnu")]
use utils::malloc::malloc::configure_malloc;
use std::sync::Arc;
use discord::app::base::App;
use settings::env::ENV_SCHEMA;
use serenity::prelude::*;

#[tokio::main]
async fn main() { 
    #[cfg(target_env = "gnu")]
    configure_malloc();

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MODERATION;

    let app = Arc::new(App::new());
    let mut client = Client::builder(&ENV_SCHEMA.bot_token, intents)
        .event_handler_arc(app)
        .await
        .expect("Err creating client");

    if let Err(err) = client.start().await {
        println!("Client error: {err:?}");
    }
}