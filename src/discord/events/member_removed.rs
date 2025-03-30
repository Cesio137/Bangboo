use crate::discord::app::creators::{create_event, EventHandler};
use crate::settings::global::EColor;
use crate::utils::{embeds::res, global::global_message, logger::*};
use twilight_model::{
    gateway::event::{Event, EventType},
    http::attachment::Attachment,
};

pub fn member_removed() -> EventHandler {
    create_event(EventType::MemberRemove, |event, client| async move {
        let member = match &event {
            Event::MemberAdd(member) => member,
            _ => return,
        };
        if member.user.bot {
            return;
        }

        let canvas = global_message(event.kind(), &member.member).await;
        if let Ok(buffer) = canvas {
            let result = client
                .create_message(member.guild_id.cast())
                .attachments(&vec![Attachment::from_bytes(
                    "welcome.png".to_string(),
                    buffer,
                    1,
                )])
                .await;

            if let Err(err) = result {
                error(format!("Error trying to responde MemberRemoved event: {:?}", err).as_str());
            }
            return;
        }

        let name = &member.user.name;
        let message = format!("{} left the server!", name);
        let embed_res = res(EColor::Danger, message);
        let result = client
            .create_message(member.guild_id.cast())
            .embeds(&[embed_res])
            .await;

        if let Err(err) = result {
            error(format!("Error trying to responde MemberRemoved event: {:?}", err).as_str());
        }
    })
}
