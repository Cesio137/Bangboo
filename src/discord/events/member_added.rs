use twilight_model::gateway::event::{Event, EventType};
use twilight_model::http::attachment::Attachment;
use crate::discord::app::creators::{create_event, EventHandler};
use crate::settings::global::EColor;
use crate::utils::embeds::res;
use crate::utils::global::global_message;

pub fn member_added() -> EventHandler {
    create_event(
        EventType::MemberAdd,
        |event, client| async move {
            let member = match &event {
                Event::MemberRemove(member) => member,
                _ => return,
            };

            let canvas = global_message(event.kind(), member.user.clone()).await;
            if let Ok(buffer) = canvas {
                let result = client.create_message(member.guild_id.cast())
                    .attachments(&vec![Attachment::from_bytes("welcome.png".to_string(), buffer, 1)]).await;
                if let Err(err) = result {
                    eprintln!("Error trying to responde MemberAdd event: {:?}", err);
                }
                return;
            }

            let name = &member.user.name;
            let message = format!("{} join the server!", name);
            let embed_res = res(EColor::Success, message);
            let result = client.create_message(member.guild_id.cast()).embeds(&[embed_res]).await;

            if let Err(why) = result {
                eprintln!("Error trying to responde MemberAdd event: {:?}", why);
            }
        }
    )
}
