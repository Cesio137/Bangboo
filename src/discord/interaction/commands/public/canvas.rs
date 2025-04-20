use crate::discord::app::creators::{create_slash_command, SlashCommand};
use crate::settings::global::EColor;
use crate::utils::embeds::res;
use crate::utils::global::{global_message, EventType};
use crate::utils::interaction::{defer_reply, reply_with_attachment, reply_with_embed};
use serenity::all::{CommandType, CreateCommand, InteractionContext};
use serenity::builder::CreateAttachment;

pub fn command() -> SlashCommand {
    create_slash_command(
        CreateCommand::new("canvas")
            .description("Canvas creator test command.")
            .kind(CommandType::ChatInput)
            .add_context(InteractionContext::Guild),
        |ctx, interaction| async move {
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

            let canvas = global_message(EventType::MemberAdd, &user).await.unwrap_or(vec![]);

            if !canvas.is_empty() {
                let attachment = CreateAttachment::bytes(canvas.as_slice(), "Welcome.png");
                if let Err(err) = reply_with_attachment(&ctx, &interaction, attachment, true).await {
                    tracing::error!("Error trying to responde /canvas command: {}", err);
                }

                return 
            }

            let embed = res(EColor::Danger, "Error trying to create canvas.".to_string());
            if let Err(err) = reply_with_embed(&ctx, &interaction, embed, true).await {
                tracing::error!("Error trying to responde /canvas command: {}", err);
            };
        }
    )
}