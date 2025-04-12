use crate::settings::global::EColor;
use crate::utils::{embeds::interaction_res, interaction::get_options};
use anyhow::Result;
use chrono::DateTime;
use std::sync::Arc;
use twilight_http::Client;
use twilight_model::{
    application::{command::Command, command::CommandType, interaction::application_command::CommandOptionValue},
    gateway::payload::incoming::InteractionCreate,
    http::interaction::InteractionResponseType,
};
use twilight_util::{
    builder::command::{CommandBuilder, UserBuilder},
    snowflake::Snowflake,
};

pub fn command() -> Command {
    let user_option = UserBuilder::new("user", "Select user.").build();
    CommandBuilder::new(
        "age",
        "Displays your or another user's account creation date.",
        CommandType::ChatInput,
    )
        .option(user_option)
        .build()
}

pub async fn run(interaction: Box<InteractionCreate>, client: Arc<Client>) -> Result<()> {
    if interaction.guild_id.is_none() {
        let response = interaction_res(
            EColor::Danger,
            "/age command can only be executed inside a guild.".to_string(),
            InteractionResponseType::ChannelMessageWithSource,
        );

        client.interaction(interaction.application_id)
            .create_response(interaction.id, &interaction.token, &response)
            .await?;

        return Ok(());
    }
    let mut color = EColor::Green;
    let mut age = None;

    // Obtém o usuário do comando, se existir
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
    );

    client.interaction(interaction.application_id)
        .create_response(interaction.id, &interaction.token, &response)
        .await?;

    Ok(())
}
