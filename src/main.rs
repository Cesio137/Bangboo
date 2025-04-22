mod discord;
mod models;
mod settings;
mod tools;
mod utils;


use std::sync::Arc;
use discord::app::base::App;
use settings::env::ENV_SCHEMA;
use serenity::prelude::*;

#[tokio::main]
async fn main() {    
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let app = Arc::new(App::new());
    let mut client = Client::builder(&ENV_SCHEMA.bot_token, intents)
        .event_handler_arc(app)
        .await
        .expect("Err creating client");

    if let Err(err) = client.start().await {
        println!("Client error: {err:?}");
    }
}