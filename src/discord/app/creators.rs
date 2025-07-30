use crate::discord::app::base::App;
use async_trait::async_trait;
use serenity::all::{CommandInteraction, ComponentInteraction, Context, Message};
use serenity::builder::CreateCommand;

#[async_trait]
pub trait PrefixCommandHandler: Send + Sync {
    fn name(&self) -> String;
    async fn run(&self, app: &App, ctx: &Context, message: &Message);
}

#[async_trait]
pub trait SlashCommandHandler: Send + Sync {
    fn command(&self) -> CreateCommand<'static>;
    async fn run(&self, app: &App, ctx: &Context, interaction: &CommandInteraction);
}

#[async_trait]
pub trait ResponderHandler: Send + Sync {
    fn custom_id(&self) -> String;
    async fn run(&self, ctx: &Context, interaction: &ComponentInteraction);
}
