use crate::discord::app::creators::{create_event, EventHandler};
use crate::settings::global::EColor;
use crate::utils::embeds::res;
use crate::utils::global::global_message;
use crate::utils::logger::error;
use twilight_model::gateway::event::{Event, EventType};
use twilight_model::http::attachment::Attachment;

pub fn ban_add() -> EventHandler {
    create_event(
        EventType::BanAdd,
        |event, context| async move {
            let ban = match &event {
                Event::BanAdd(ban) => ban,
                _ => return,
            };
            if ban.user.bot { return; }

            let channel_id = match context.client.guild(ban.guild_id).await {
                Ok(response) => {
                    match response.model().await.ok().and_then(|guild| guild.system_channel_id) {
                        Some(channel_id) => channel_id,
                        None => {
                            error("Error getting system message channel".to_string().as_str());
                            return
                        }
                    }
                },
                Err(err) => {
                    error(format!("Error getting system message channel\n{:?}", err).as_str());
                    return
                },
            };

            let canvas = global_message(event.kind(), &ban.user, None).await;
            if let Ok(buffer) = canvas {
                let result = context.client
                    .create_message(channel_id)
                    .attachments(&vec![Attachment::from_bytes(
                        "welcome.png".to_string(),
                        buffer,
                        1,
                    )])
                    .await;

                if let Err(err) = result {
                    error(format!("Error trying to responde BanAdd event: {:?}", err).as_str());
                }
                return;
            }

            let name = &ban.user.name;
            let message = format!("{} join the server!", name);
            let embed_res = res(EColor::Success, message);
            let result = context.client
                .create_message(channel_id)
                .embeds(&[embed_res])
                .await;

            if let Err(err) = result {
                error(format!("Error trying to responde BanAdd event: {:?}", err).as_str());
            }
        }
    )
}