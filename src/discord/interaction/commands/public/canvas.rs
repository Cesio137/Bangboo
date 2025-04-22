use async_trait::async_trait;
use crate::settings::global::EColor;
use crate::utils::embeds::res;
use crate::utils::global::{global_message, EventType};
use crate::utils::interaction::{defer_reply, reply_with_attachment, reply_with_embed};
use serenity::all::{CommandInteraction, CommandType, Context, CreateCommand, InteractionContext};
use serenity::builder::CreateAttachment;
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

    async fn run(&self, ctx: Context, interaction: CommandInteraction) {
        if let Err(err) = defer_reply(&ctx, &interaction).await {
            tracing::error!("{}", err);
            return
        }

        let user = match interaction.member.as_ref() {
            None => {
                tracing::error!("Error trying to responde /canvas command: Failed to get user from member");
                return
            }
            Some(member) => member.user.clone(),
        };

        let canvas = global_message(EventType::MemberAdd, &user).await;
        if canvas.is_err() {
            let embed = res(EColor::Danger, "Error trying to create canvas.".to_string());
            if let Err(err) = reply_with_embed(&ctx, &interaction, embed, true).await {
                tracing::error!("Error trying to responde /canvas command: {}", err);
            };            
            return;
        }
        
        let canvas = canvas.unwrap();
        let attachment = CreateAttachment::bytes(canvas.as_slice(), "Welcome.png");
        if let Err(err) = reply_with_attachment(&ctx, &interaction, attachment, true).await {
            tracing::error!("Error trying to responde /canvas command: {}", err);
        }
    }
}