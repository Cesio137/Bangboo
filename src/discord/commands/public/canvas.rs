use twilight_model::http::attachment::Attachment;
use twilight_model::application::command::CommandType;
use twilight_model::gateway::event::EventType;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType};
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
            let message = format!("Welcome {}!", user.name.clone());
            let mut response = interaction_res(EColor::Success, message);
            let canvas = match global_message(EventType::MemberAdd, user).await {
                Ok(canvas) => canvas,
                Err(err) => {
                    eprintln!("Error trying to responde /canvas command: {:?}", err);
                    vec![]
                }
            };

            if !canvas.is_empty() {
                response = InteractionResponse {
                    kind: InteractionResponseType::ChannelMessageWithSource,
                    data: InteractionResponseData {
                        attachments: vec![
                            Attachment::from_bytes("welcome.png".to_string(), canvas, 1)
                        ].into(),
                        ..Default::default()
                    }.into(),
                }
            }


            let result = client.interaction(interaction.application_id)
                .create_response(interaction.id, &interaction.token, &response).await;

            if let Err(why) = result {
                eprintln!("Error trying to responde /canvas command: {:?}", why);
            }
        }
    )
}