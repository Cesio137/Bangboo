use crate::constants::*;
use crate::discord::*;
use crate::tools::*;
use crate::utils::*;
use serenity::all::{
    CacheHttp, CommandInteraction, CommandOptionType, CommandType, Context, CreateCommand,
    CreateCommandOption, CreateEmbed, InteractionContext, MessageFlags,
};
use serenity::async_trait;
use std::collections::VecDeque;

pub struct Prompt;

#[async_trait]
impl SlashCommandHandler for Prompt {
    fn command(&self) -> CreateCommand<'static> {
        CreateCommand::new("prompt")
            .description("Choose a IA and interact with it!")
            .kind(CommandType::ChatInput)
            .add_context(InteractionContext::Guild)
            .add_option(
                CreateCommandOption::new(CommandOptionType::String, "text", "Enter the text.")
                    .required(true),
            )
    }

    async fn run(&self, app: &App, ctx: &Context, interaction: &CommandInteraction) {
        let option = interaction.data.options[0].clone();
        _ = interaction.defer_ephemeral(ctx.http()).await;

        let question = option.value.as_str().unwrap().to_string();
        let response = match get_text(question).await {
            Ok(response) => response,
            Err(err) => {
                followup_with_embed(
                    ctx,
                    interaction,
                    MessageFlags::EPHEMERAL,
                    COLORS.danger,
                    "Failed to get gemini response.",
                )
                .await;
                error(&format!("Failed to get gemini response.\n└ {:?}", err));
                return;
            }
        };

        let mut texts = VecDeque::new();
        let mut i = 0;
        let max_length = 3000;

        while i < response.len() {
            let end = usize::min(i + max_length, response.len());
            texts.push_back(response[i..end].to_string());
            i += max_length;
        }

        let embed = CreateEmbed::new()
            .color(COLORS.green)
            .description(texts.pop_front().unwrap_or_default());
        let payload = ReplyPayload {
            embeds: Some(vec![embed]),
            ..ReplyPayload::default()
        };
        followup(ctx, interaction, MessageFlags::EPHEMERAL, &payload).await;

        while texts.len() > 0 {
            let embed = CreateEmbed::new()
                .color(COLORS.green)
                .description(texts.pop_front().unwrap_or_default());
            let payload = ReplyPayload {
                embeds: Some(vec![embed]),
                ..ReplyPayload::default()
            };
            followup(ctx, interaction, MessageFlags::EPHEMERAL, &payload).await;
        }
    }
}
