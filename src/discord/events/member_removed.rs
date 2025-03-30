use twilight_model::gateway::event::{Event, EventType};
use crate::discord::app::creators::{create_event, EventHandler};
use crate::settings::global::EColor;
use crate::utils::embeds::res;
use crate::utils::global::global_message;

pub fn member_removed() -> EventHandler {
    create_event(
        EventType::MemberRemove,
        |event, client| async move {
            let member = match &event {
                Event::MemberAdd(member) => member,
                _ => return,
            };

            let canvas = global_message(event.kind(), member.user.clone()).await;
            if let Ok(buffer) = canvas {
                let result = client.create_message(member.guild_id.cast()).await;
                if let Err(err) = result {
                    eprintln!("Error trying to responde MemberRemoved event: {:?}", err);
                }
                return;
            }

            let name = &member.user.name;
            let message = format!("{} left the server!", name);
            let embed_res = res(EColor::Danger, message);
            let result = client.create_message(member.guild_id.cast()).embeds(&[embed_res]).await;

            if let Err(why) = result {
                eprintln!("Error trying to responde MemberRemoved event: {:?}", why);
            }
        }
    )
}