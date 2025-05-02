use async_trait::async_trait;
use crate::settings::global::EColor;
use crate::utils::embeds::res;
use crate::utils::global::{global_message, EventType};
use crate::utils::interaction::{reply_with_attachment, reply_with_embed};
use serenity::all::{CacheHttp, CommandInteraction, CommandType, Context, CreateCommand, InteractionContext};
use serenity::builder::CreateAttachment;
use crate::discord::app::base::App;
use crate::discord::app::creators::SlashCommandHandler;

pub struct Canvas;

#[async_trait]
impl SlashCommandHandler for Canvas {
    fn command(&self) -> CreateCommand {
        CreateCommand::new("canvas")
            .description("Canvas creator test command.")
            .kind(CommandType::ChatInput)
            .add_context(InteractionContext::Guild)
    }

    async fn run(&self, app: &App, ctx: Context, interaction: CommandInteraction) {
        if interaction.defer(ctx.http()).await.is_err() {return;}

        let user = match interaction.member.as_ref() {
            Some(member) => member.user.clone(),
            None => {return;}
        };

        let canvas = global_message(EventType::MemberAdd, &user).await;
        if canvas.is_err() {
            let embed = res(EColor::Danger, "Error trying to create canvas.");
            let _ = reply_with_embed(&ctx, &interaction, embed, true).await;           
            return;
        }
        
        let canvas = canvas.unwrap();
        let attachment = CreateAttachment::bytes(canvas.as_slice(), "card.png");
        let _ = reply_with_attachment(&ctx, &interaction, attachment, true).await;
    }
}