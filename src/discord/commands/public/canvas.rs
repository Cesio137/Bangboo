use std::sync::Arc;
use tokio::sync::Mutex;
use twilight_model::application::command::CommandType;
use twilight_model::gateway::event::EventType;
use crate::discord::app::creators::{create_slash_command, SlashCommand};
use twilight_util::{
    builder::command::CommandBuilder,
};
use crate::settings::global::EColor;
use crate::utils::embeds::interaction_res;
use crate::utils::global::global_message;

pub fn canvas_command() -> SlashCommand {
    create_slash_command(
        CommandBuilder::new("canvas", "Canvas creator test command.", CommandType::ChatInput)
            .build(),
        |interaction, client| async move {
            let user = match &interaction.member {
                None => {
                    let response = interaction_res(EColor::Danger, "error trying to get member from guild".to_string());

                    let result = client.interaction(interaction.application_id)
                        .create_response(interaction.id, &interaction.token, &response).await;

                    if let Err(why) = result {
                        eprintln!("Error trying to responde /canvas command: {:?}", why);
                    }
                    return ();
                }
                Some(member) => {
                    match &member.user {
                        None => {
                            let response = interaction_res(EColor::Danger, "error trying to get member from guild".to_string());

                            let result = client.interaction(interaction.application_id)
                                .create_response(interaction.id, &interaction.token, &response).await;

                            if let Err(why) = result {
                                eprintln!("Error trying to responde /canvas command: {:?}", why);
                            }
                            return ();
                        }
                        Some(user) => {
                            user.clone()
                        }
                    }
                }
            };
            global_message(EventType::MemberAdd, user);
            let response = interaction_res(EColor::Success, "Check the app!".to_string());

            let result = client.interaction(interaction.application_id)
                .create_response(interaction.id, &interaction.token, &response).await;

            if let Err(why) = result {
                eprintln!("Error trying to responde /canvas command: {:?}", why);
            }
            return ();
        }
    )
}