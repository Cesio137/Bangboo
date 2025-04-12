mod prefix;
mod public;

use crate::discord::commands::{prefix::ping, public::{age, canvas}};
use anyhow::Result;
use std::sync::Arc;
use twilight_http::Client;
use twilight_model::{
    application::command::Command,
    gateway::payload::incoming::{InteractionCreate, MessageCreate}
};

pub fn commands() -> Vec<Command> {
    let mut commands: Vec<Command> = Vec::new();
    // Debug commands
    if cfg!(debug_assertions) {
        commands.push(canvas::command());
    }
    // Add more commands here...
    commands.push(age::command());

    commands
}

pub async fn slash_commands(name: &str, interaction: Box<InteractionCreate>, client: Arc<Client>) -> Result<()> {
    match name {
        "age" => age::run(interaction, client).await?,
        "canvas" => if cfg!(debug_assertions) { canvas::run(interaction, client).await? },
        _ => {}
    }
    Ok(())
}

pub async fn prefix_commands(name: &str, message: Box<MessageCreate>, client: Arc<Client>) -> Result<()> {
    match name {
        "!ping" => ping::run(message, client).await?,
        _ => {}
    }
    Ok(())
}
