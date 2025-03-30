use twilight_model::{
    http::attachment::Attachment,
    application::command::CommandType,
    gateway::event::EventType::MemberAdd,
};
use twilight_util::builder::command::CommandBuilder;
use crate::discord::app::creators::{create_slash_command, SlashCommand};
use crate::settings::global::EColor;
use crate::utils::{
    interaction::defer_reply,
    embeds::res,
    global::global_message
};

pub fn canvas_command() -> SlashCommand {
    create_slash_command(
        CommandBuilder::new("canvas", "Canvas creator test command.", CommandType::ChatInput)
            .build(),
        |interaction, client| async move {
            if let Err(err) = defer_reply(interaction.clone(), client.clone()).await {
                eprintln!("Error trying to responde /canvas command: {:?}", err);
                return;
            };

            let user = match interaction.member.clone() {
                Some(member) => {
                    match member.user {
                        None => {
                            eprintln!("Error trying to responde /canvas command: Can not retrive user info.");
                            return;
                        }
                        Some(user) => user
                    }
                },
                None => {
                    eprintln!("Error trying to responde /canvas command: Can not retrive user info.");
                    return;
                },
            };
            let canvas = global_message(MemberAdd, user).await.unwrap_or(vec![]);

            if !canvas.is_empty() {
                let result = client.interaction(interaction.application_id)
                    .create_followup(&interaction.token)
                    .attachments(&vec![Attachment::from_bytes("welcome.png".to_string(), canvas, 1)])
                    .await;

                if let Err(why) = result {
                    eprintln!("Error trying to responde /canvas command: {:?}", why);
                }

                return;
            }

            let embed = res(EColor::Danger, "Error trying to create canvas.".to_string());
            let result = client.interaction(interaction.application_id)
                .create_followup(&interaction.token)
                .embeds(&[embed])
                .await;

            if let Err(why) = result {
                eprintln!("Error trying to responde /canvas command: {:?}", why);
            }
        }
    )
}