mod discord;
mod models;
mod settings;
mod tools;
mod utils;

use crate::discord::app::base::App;
use crate::settings::env::ENV_SCHEMA;
use serenity::prelude::*;

#[tokio::main]
async fn main() {
    #[cfg(target_env = "gnu")]
    malloc::limit_mmap_threshold();
    
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let app = App::new();
    let mut client = Client::builder(&ENV_SCHEMA.bot_token, intents)
        .event_handler(app)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}