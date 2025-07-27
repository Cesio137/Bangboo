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
use serenity::all::Token;
use std::str::FromStr;
use std::sync::Arc;
use crate::discord::app::bootstrap;

#[tokio::main]
async fn main() {
    #[cfg(target_env = "gnu")]
    configure_malloc();
/*
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILDS
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MODERATION;*/

    let intents = GatewayIntents::all();
    bootstrap(intents).await;
}
