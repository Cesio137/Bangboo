use serenity::all::{CommandInteraction, Message};
use serenity::builder::CreateCommand;
use serenity::client::Context;
use std::pin::Pin;

pub type PrefixCommandCallback = Box<dyn Fn(Context, Message) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

pub struct PrefixCommand {
    pub name: String,
    pub run: PrefixCommandCallback,
}

pub fn create_prefix_command<F, Fut>(name: &str, run: F) -> PrefixCommand
where
    F: Fn(Context, Message) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    PrefixCommand {
        name: format!("!{name}"),
        run: Box::new(move |message, client| Box::pin(run(message, client))),
    }
}

pub type SlashCommandCallback = Box<dyn Fn(Context, CommandInteraction) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> + Send + Sync >;
pub struct SlashCommand {
    pub command: CreateCommand,
    pub run: SlashCommandCallback,
}

pub fn create_slash_command<F, Fut>(command: CreateCommand, reply: F) -> SlashCommand
where
    F: Fn(Context, CommandInteraction) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + Sync + 'static,
{
    SlashCommand {
        command,
        run: Box::new(move |context, interaction| Box::pin(reply(context, interaction))),
    }
}