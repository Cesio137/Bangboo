use crate::discord::*;
use crate::functions::*;
use crate::menus::*;
use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, CommandType, Context, CreateCommand, InteractionContext, MessageFlags,
};

pub struct Social;

#[async_trait]
impl SlashCommandHandler for Social {
    fn command(&self) -> CreateCommand<'static> {
        CreateCommand::new("social")
            .description("Social medias")
            .kind(CommandType::ChatInput)
            .add_context(InteractionContext::Guild)
    }

    async fn run(&self, app: &App, ctx: &Context, interaction: &CommandInteraction) {
        let component = social_component();
        let payload = ReplyPayload {
            components: Some(vec![component]),
            ..ReplyPayload::default()
        };
        reply(ctx, interaction, MessageFlags::IS_COMPONENTS_V2, &payload).await;
    }
}
