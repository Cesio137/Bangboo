use crate::discord::app::base::App;
use crate::discord::app::creators::SlashCommandHandler;
use crate::menus::components::fab::fab_component;
use crate::utils::interaction::{reply, ReplyPayload};
use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, CommandType, Context, CreateCommand, InteractionContext, MessageFlags,
};

pub struct Fab;

#[async_trait]
impl SlashCommandHandler for Fab {
    fn command(&self) -> CreateCommand<'static> {
        CreateCommand::new("fab")
            .description("Products on fab marketplace")
            .kind(CommandType::ChatInput)
            .add_context(InteractionContext::Guild)
    }

    async fn run(&self, app: &App, ctx: &Context, interaction: &CommandInteraction) {
        let component = fab_component();
        let payload = ReplyPayload {
            components: Some(vec![component]),
            ..ReplyPayload::default()
        };
        reply(ctx, interaction, MessageFlags::IS_COMPONENTS_V2, &payload).await;
    }
}
