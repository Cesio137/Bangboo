mod prefix;
mod public;

use twilight_model::gateway::payload::incoming::{InteractionCreate, MessageCreate};
use std::sync::Arc;
use twilight_http::Client;
use twilight_model::application::command::Command;
use crate::discord::commands::{prefix::ping, public::{age, canvas}};

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

pub async fn slash_commands(name: &str, interaction: Box<InteractionCreate>, client: Arc<Client>) {
    match name {
        "age" => age::run(interaction, client).await,
        "canvas" => if cfg!(debug_assertions) { canvas::run(interaction, client).await },
        _ => {}
    }
}

pub async fn prefix_commands(name: &str, message: Box<MessageCreate>, client: Arc<Client>) -> Option<()> {
    match name {
        "!ping" => ping::run(message, client).await,
        _ => {return None}
    }
    Some(())
}
