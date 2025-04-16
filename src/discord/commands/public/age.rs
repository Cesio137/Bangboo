use crate::discord::app::creators::{create_slash_command, SlashCommand};
use crate::settings::global::EColor;
use crate::utils::{embeds::interaction_res, interaction::get_options};
use chrono::DateTime;
use twilight_model::{
    application::{command::CommandType, interaction::{application_command::CommandOptionValue, InteractionContextType}},
    http::interaction::InteractionResponseType,
    oauth::ApplicationIntegrationType
};
use twilight_util::{
    builder::command::{CommandBuilder, UserBuilder},
    snowflake::Snowflake,
};

pub fn age_command() -> SlashCommand {
    let user_option = UserBuilder::new("user", "Select user.").build();
    create_slash_command(
        CommandBuilder::new(
            "age",
            "Displays your or another user's account creation date.",
            CommandType::ChatInput,
            ).integration_types(vec![ApplicationIntegrationType::GuildInstall])
            .contexts(vec![InteractionContextType::Guild])
            .option(user_option)
            .build(),
        |interaction, client| async move {
            let mut color = EColor::Green;
            let mut age = None;
        
            // Try get user
            let user_id = get_options(&interaction)
                .first()
                .and_then(|opt| match &opt.value {
                    CommandOptionValue::User(user) => Some(*user),
                    _ => None,
                })
                .or_else(|| interaction.member.as_ref()?.user.as_ref().map(|u| u.id));
        
            if let Some(user_id) = user_id {
                let timestamp = user_id.timestamp();
                if let Some(datetime) = DateTime::from_timestamp_millis(timestamp as i64) {
                    let user = client.user(user_id).await;
                    let username = match user {
                        Ok(user) => {
                            let model = user.model().await.ok()
                                .map(|user| user.name)
                                .unwrap_or_else(|| "Unknown user".to_string());
                            model
                        }
                        Err(_) => {"Unknown".to_string()}
                    };
                    age = Some(format!(
                        "{}'s account was created at {}.",
                        username,
                        datetime.format("%a, %Hh%Mmin, %d/%b/%Y")
                    ));
                } else {
                    color = EColor::Warning;
                    age = Some("Account creation date unknown.".to_string());
                }
            }
        
            let response_text = age.unwrap_or_else(|| {
                color = EColor::Danger;
                "Error trying to respond to /age command: Can't find a user ID.".to_string()
            });
        
            let response = interaction_res(
                color,
                response_text,
                InteractionResponseType::ChannelMessageWithSource,
                false
            );
        
            client.interaction(interaction.application_id)
                .create_response(interaction.id, &interaction.token, &response)
                .await?;
        
            Ok(())
        },
    )
}