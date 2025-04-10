use crate::settings::global::EColor;
use crate::utils::{
    embeds::*,
    global::*,
    interaction::*
};
use anyhow::{anyhow, Result};
use std::sync::Arc;
use twilight_http::Client;
use twilight_model::{
    application::command::*,
    gateway::{event::EventType, payload::incoming::InteractionCreate},
    http::{attachment::Attachment, interaction::InteractionResponseType},
};
use twilight_util::builder::command::CommandBuilder;

pub fn command() -> Command {
    CommandBuilder::new(
        "canvas",
        "Canvas creator test command.",
        CommandType::ChatInput,
    )
    .build()
}

pub async fn run(interaction: Box<InteractionCreate>, client: Arc<Client>) -> Result<()> {
    if interaction.guild_id.is_none() {
        let response = interaction_res(
            EColor::Danger,
            "/canvas command can only be executed inside a guild.".to_string(),
            InteractionResponseType::ChannelMessageWithSource,
        );

        client.interaction(interaction.application_id)
            .create_response(interaction.id, &interaction.token, &response)
            .await?;

        return Ok(());
    }

    defer_reply(interaction.clone(), &client).await?;

    let guild_id = match interaction.guild_id {
        None => {
            return Err(anyhow!("Error trying to responde /canvas command: Failed to get guild id."));
        }
        Some(guild_id) => guild_id,
    };
    let user_id = match &interaction.member {
        None => {
            return Err(anyhow!("Error trying to responde /canvas command: Failed to get member."));
        }
        Some(member) => match &member.user {
            None => {
                return Err(anyhow!("Error trying to responde /canvas command: Failed to get user from member."));
            }
            Some(user) => user,
        },
    };

    let member = client.guild_member(guild_id, user_id.id).await?.model().await?;
    let canvas = global_message(EventType::MemberAdd, &member.user, None).await.unwrap_or(vec![]);

    if !canvas.is_empty() {
        client.interaction(interaction.application_id)
            .create_followup(&interaction.token)
            .attachments(&vec![Attachment::from_bytes(
                "welcome.png".to_string(),
                canvas,
                1,
            )])
            .await?;

        return Ok(());
    }

    let embed = res(EColor::Danger, "Error trying to create canvas.".to_string());
    client.interaction(interaction.application_id)
        .create_followup(&interaction.token)
        .embeds(&[embed])
        .await?;

    Ok(())
}
