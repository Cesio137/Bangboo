use twilight_model::gateway::event::{Event, EventType};
use crate::discord::app::creators::{create_event, EventHandler};
use crate::settings::global::EColor;
use crate::utils::embeds::res;

pub fn member_removed() -> EventHandler {
    create_event(
        EventType::MemberRemove,
        |event, client| async move {
            let member = match event {
                Event::MemberAdd(member) => member,
                _ => return (),
            };

            let name = &member.user.name;
            let message = format!("{} left the server!", name);
            let channel_id = match client.guild(member.guild_id).await {
                Ok(guild) => {
                    match guild.model().await {
                        Ok(channel) => {
                            match channel.system_channel_id {
                                Some(system_channel) => system_channel,
                                None => {
                                    eprintln!("Error trying to responde MemberAdd event: Can not get system channel fom guild.");
                                    return
                                },
                            }
                        },
                        Err(err) => {
                            eprintln!("Error trying to responde MemberAdd event: Can not get system channel fom guild.\n{}", err);
                            return
                        },
                    }
                },
                Err(err) => {
                    eprintln!("Error trying to responde MemberAdd event: Can not get guild from id.\n{}", err);
                    return
                }
            };
            let embed_res = res(EColor::Success, message);
            let result = client.create_message(channel_id).embeds(&[embed_res]).await;

            if let Err(why) = result {
                eprintln!("Error trying to responde MemberAdd event: {:?}", why);
            }
        }
    )
}