pub mod discord;
mod menus;
mod settings;
mod tools;
mod utils;
mod env;
mod assets;
mod data;

#[cfg(target_env = "gnu")]
use settings::malloc::malloc::configure_malloc;

use serenity::{all::GatewayIntents};
use crate::discord::base::bootstrap;

#[tokio::main]
async fn main() {
    #[cfg(target_env = "gnu")]
    configure_malloc();
    
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILDS
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MODERATION;
        
    bootstrap(intents).await;
}
