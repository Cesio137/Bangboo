mod ban;
mod delete_message;
mod filter;
mod kick;
mod modal;
mod timeout;

use crate::data::settings::EColors;
use crate::discord::app::base::App;
use crate::discord::app::creators::SlashCommandHandler;
use crate::discord::commands::public::moderate::delete_message::delete_message_collector;
use crate::utils::interaction::reply_with_embed;
use async_trait::async_trait;
use ban::ban_collector;
use kick::kick_collector;
use serenity::all::{
    CacheHttp, CommandInteraction, CommandOptionType, CommandType, Context, CreateCommand,
    CreateCommandOption, InteractionContext, MessageFlags,
};
use timeout::timeout_collector;

pub struct Moderate;

#[async_trait]
impl SlashCommandHandler for Moderate {
    fn command(&self) -> CreateCommand<'static> {
        CreateCommand::new("moderate")
            .description("Equality before the law is the cornerstone of justice ⚖.")
            .kind(CommandType::ChatInput)
            .add_context(InteractionContext::Guild)
            .add_option(
                CreateCommandOption::new(CommandOptionType::String, "action", "Select an action.")
                    .required(true)
                    .add_string_choice("delete messages", "delete_messages")
                    .add_string_choice("timeout", "timeout")
                    .add_string_choice("kick", "kick")
                    .add_string_choice("ban", "ban"),
            )
    }

    async fn run(&self, app: &App, ctx: &Context, interaction: &CommandInteraction) {
        let member = match interaction.member.as_ref() {
            Some(member) => member,
            None => {
                reply_with_embed(
                    &ctx,
                    &interaction,
                    MessageFlags::EPHEMERAL,
                    EColors::danger,
                    "Interaction member is none.",
                )
                .await;
                return;
            }
        };

        _ = match member.permissions.as_ref() {
            Some(permissions) => {
                if !permissions.administrator() {
                    reply_with_embed(
                        &ctx,
                        &interaction,
                        MessageFlags::empty(),
                        EColors::danger,
                        "You don't have **ADMINISTRATOR** permission.",
                    )
                    .await;
                    return;
                }
            }
            None => {
                reply_with_embed(
                    &ctx,
                    &interaction,
                    MessageFlags::empty(),
                    EColors::danger,
                    "Interaction member has no permission.",
                )
                .await;
                return;
            }
        };

        let action = interaction.data.options[0].value.as_str().unwrap();

        match action {
            "delete_messages" => delete_message_collector(&ctx, &interaction, member).await,
            "timeout" => timeout_collector(&ctx, &interaction, member).await,
            "kick" => kick_collector(&ctx, &interaction, member).await,
            "ban" => ban_collector(&ctx, &interaction, member).await,
            _ => {}
        }
    }
}
