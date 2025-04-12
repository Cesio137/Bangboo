use anyhow::Result;
use std::{pin::Pin, sync::Arc};
use twilight_http::Client;
use twilight_model::{application::command::Command, gateway::payload::incoming::{InteractionCreate, MessageCreate}};

pub type PrefixCommandCallback = Box<dyn Fn(Box<MessageCreate>, Arc<Client>) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> + Send + Sync>;

pub struct PrefixCommand {
    pub name: String,
    pub run: PrefixCommandCallback,
}

pub fn create_prefix_command<F, Fut>(name: &str, run: F) -> PrefixCommand
where
    F: Fn(Box<MessageCreate>, Arc<Client>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<()>> + Send + 'static,
{
    PrefixCommand {
        name: format!("!{name}"),
        run: Box::new(move |message, client| Box::pin(run(message, client))),
    }
}

pub type SlashCommandCallback = Box<dyn Fn(Box<InteractionCreate>, Arc<Client>) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> + Send + Sync>;

pub struct SlashCommand {
    pub command: Command,
    pub run: SlashCommandCallback,
}

pub fn create_slash_command<F, Fut>(command: Command, reply: F) -> SlashCommand
where
    F: Fn(Box<InteractionCreate>, Arc<Client>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<()>> + Send + 'static,
{
    SlashCommand {
        command,
        run: Box::new(move |interaction, client| Box::pin(reply(interaction, client))),
    }
}