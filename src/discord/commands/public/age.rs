use std::borrow::Cow;
use crate::data::settings::EColors;
use crate::discord::app::base::App;
use crate::discord::app::creators::SlashCommandHandler;
use crate::utils::interaction::reply_with_embed;
use async_trait::async_trait;
use serenity::all::{CommandInteraction, CommandOptionType, CommandType, Context, CreateCommand, InteractionContext};
use serenity::builder::CreateCommandOption;

pub struct Age;


#[async_trait]
impl SlashCommandHandler for Age {
    fn command(&self) -> CreateCommand<'static> {
        let option = CreateCommandOption::new(CommandOptionType::User, "user", "Selected user.")
            .required(false);
        CreateCommand::new("age")
            .description("Displays your or another user's account creation date.")
            .kind(CommandType::ChatInput)
            .add_option(option)
            .add_context(InteractionContext::Guild)
    }

    async fn run(&self, app: &App, ctx: &Context, interaction: &CommandInteraction) {
        let options = &interaction.data.options;
        let user_id = if let Some(option) = options.get(0) {
            option.value.as_user_id().unwrap()
        } else {
            interaction.user.id
        };

        let guild_id = match interaction.guild_id {
            Some(id) => id,
            None => {
                reply_with_embed(
                    &ctx,
                    &interaction,
                    false,
                    EColors::danger,
                    "Guild id not found."
                )
                .await;
                return;
            }
        };

        if ctx.cache.guild(guild_id).is_none() {
            reply_with_embed(
                &ctx,
                &interaction,
                false,
                EColors::danger,
                "Failed to fetch guild data."
            )
            .await;
            return;
        }

        let guild = ctx.cache.guild(guild_id).unwrap().clone();

        let member = guild.members.get(&user_id).unwrap();
        let user = &member.user;

        let timestamp = user.id.created_at().timestamp();
        let mut age = String::from("");
        if (interaction.locale == "pt-BR") {
            age = format!(
                "**{}** criou a conta <t:{}:R> em um(a) <t:{}:F> ",
                user.global_name.as_ref().unwrap_or(&user.name),
                timestamp,
                timestamp
            );
        } else {
            age = format!(
                "**{}** account was created <t:{}:R> on <t:{}:F> ",
                user.global_name.as_ref().unwrap_or(&user.name),
                timestamp,
                timestamp
            );
        }

        reply_with_embed(&ctx, &interaction, false, EColors::green, &age).await;
    }
}
