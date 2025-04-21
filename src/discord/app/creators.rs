use async_trait::async_trait;
use serenity::all::{CommandInteraction, Message};
use serenity::builder::CreateCommand;
use serenity::client::Context;

#[async_trait]
pub trait PrefixCommandHandler: Send + Sync {
    fn name(&self) -> String;
    async fn run(&self, ctx: Context, message: Message);
}

#[async_trait]
pub trait SlashCommandHandler: Send + Sync {
    fn command(&self) -> CreateCommand;
    async fn run(&self, ctx: Context, interaction: CommandInteraction);
}