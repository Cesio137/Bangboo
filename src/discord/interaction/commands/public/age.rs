use crate::discord::app::creators::{create_slash_command, SlashCommand};
use crate::settings::global::EColor;
use crate::utils::{embeds::res, interaction::reply_with_embed};
use serenity::all::{CommandDataOptionValue, CommandOptionType, CommandType, CreateCommand, InteractionContext};
use serenity::builder::CreateCommandOption;

pub fn command() -> SlashCommand {
    let option = CreateCommandOption::new(CommandOptionType::User, "user", "Select an user.")
        .required(false);
    
    create_slash_command(
        CreateCommand::new("age")
            .description("Displays your or another user's account creation date.")
            .kind(CommandType::ChatInput)
            .add_option(option)
            .add_context(InteractionContext::Guild),
        |ctx, interaction| async move {
            let user_id = match interaction.data.options.first() {
                None => interaction.user.id,
                Some(option) => {
                    if let CommandDataOptionValue::User(user_id) = option.value {
                        user_id
                    } else {
                        interaction.user.id
                    }
                }
            };

            let user = match ctx.http.get_user(user_id).await {
                Ok(user) => user,
                Err(err) => {
                    let embed = res(EColor::Danger, "An error occured while fetching the user.".to_string());
                    if let Err(err) = reply_with_embed(&ctx, &interaction, embed, false).await {
                        tracing::error!("Failed to reply /age command.\n{}", err);
                    }
                    tracing::error!("{}", err);
                    return
                }
            };
            
            let user_name = user.global_name.as_ref().unwrap_or(&user.name);
            let formatted_date = user.created_at().format("%a, %Hh%Mmin, %d/%b/%Y").to_string();

            let content = format!(
                "***{}***'s account was created at {}.",
                user_name,
                formatted_date,
            );
            
            let embed = res(EColor::Green, content);
            
            if let Err(err) = reply_with_embed(&ctx, &interaction, embed, false).await {
                tracing::error!("Failed to reply /age command.\n{}", err);
            }
        }
    )
}