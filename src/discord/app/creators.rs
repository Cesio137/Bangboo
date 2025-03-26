use std::{sync::Arc, pin::Pin};
use twilight_gateway::Event as GatewayEvent;
use twilight_http::Client;
use twilight_model::{
    application::command::Command,
    gateway::{
        event::EventType,
        payload::incoming::{InteractionCreate, MessageCreate}
    }
};

pub type PrefixCommandCallback = Box<dyn Fn(Box<MessageCreate>, Arc<Client>) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>;

pub struct PrefixCommand {
    pub name: String,
    pub reply: PrefixCommandCallback
}

pub fn create_prefix_command<F, Fut>(name: String, reply: F) -> PrefixCommand
where
    F: Fn(Box<MessageCreate>, Arc<Client>) -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    PrefixCommand {
        name,
        reply: Box::new(move |message, client| Box::pin(reply(message, client)))
    }
}

pub type SlashCommandCallback = Box<dyn Fn(Box<InteractionCreate>, Arc<Client>) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + 'static>;

pub struct SlashCommand {
    pub command: Command,
    pub reply: SlashCommandCallback
}

pub fn create_slash_command<F, Fut>(command: Command, reply: F) -> SlashCommand
where
    F: Fn(Box<InteractionCreate>, Arc<Client>) -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    SlashCommand {
        command,
        reply: Box::new(move |interaction, client| Box::pin(reply(interaction, client)))
    }
}

pub type EventCallback = Box<dyn Fn(GatewayEvent, Arc<Client>) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + 'static>;

pub struct EventHandler {
    pub event: EventType,
    pub reply: EventCallback
}

pub fn create_event<F, Fut>(event: EventType, reply: F) -> EventHandler
where
    F: Fn(GatewayEvent, Arc<Client>) -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    EventHandler {
        event,
        reply: Box::new(move |event, client| Box::pin(reply(event, client)))
    }
}