use crate::settings::global::EColor;
use crate::utils::{
    embeds::*,
    global::*,
    interaction::*,
    logger::*
};
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

pub async fn run(interaction: Box<InteractionCreate>, client: Arc<Client>) {
    if interaction.guild_id.is_none() {
        let response = interaction_res(
            EColor::Danger,
            "/canvas command can only be executed inside a guild.".to_string(),
            InteractionResponseType::ChannelMessageWithSource,
        );

        if let Err(err) = client
            .interaction(interaction.application_id)
            .create_response(interaction.id, &interaction.token, &response)
            .await
        {
            error(&format!("Error responding to /canvas command: {:?}", err));
        }

        return;
    }

    if let Err(err) = defer_reply(interaction.clone(), &client).await {
        error(format!("Error trying to responde /canvas command: {:?}", err).as_str());
        return;
    };
    let guild_id = match interaction.guild_id {
        None => {
            error(
                format!(
                    "Error trying to responde /canvas command: Failed to get guild id."
                )
                    .as_str(),
            );
            return;
        }
        Some(guild_id) => guild_id,
    };
    let user_id = match &interaction.member {
        None => {
            error(
                format!("Error trying to responde /canvas command: Failed to get member.")
                    .as_str(),
            );
            return;
        }
        Some(member) => match &member.user {
            None => {
                error(format!("Error trying to responde /canvas command: Failed to get user from member.").as_str());
                return;
            }
            Some(user) => user,
        },
    };
    let member = match client
        .guild_member(guild_id, user_id.id)
        .await
    {
        Ok(response) => match response.model().await {
            Ok(member) => member,
            Err(err) => {
                error(
                    format!("Error trying to responde /canvas command: {:?}", err).as_str(),
                );
                return;
            }
        },
        Err(err) => {
            error(format!("Error trying to responde /canvas command: {:?}", err).as_str());
            return;
        }
    };
    let canvas = global_message(EventType::MemberAdd, &member.user, None).await.unwrap_or(vec![]);

    if !canvas.is_empty() {
        let result = client
            .interaction(interaction.application_id)
            .create_followup(&interaction.token)
            .attachments(&vec![Attachment::from_bytes(
                "welcome.png".to_string(),
                canvas,
                1,
            )])
            .await;

        if let Err(why) = result {
            eprintln!("Error trying to responde /canvas command: {:?}", why);
        }

        return;
    }

    let embed = res(EColor::Danger, "Error trying to create canvas.".to_string());
    let result = client
        .interaction(interaction.application_id)
        .create_followup(&interaction.token)
        .embeds(&[embed])
        .await;

    if let Err(why) = result {
        eprintln!("Error trying to responde /canvas command: {:?}", why);
    }
}
