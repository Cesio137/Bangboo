
use twilight_model::channel::ChannelType;
use twilight_model::gateway::event::{Event, EventType};
use twilight_model::id::Id;
use twilight_model::id::marker::ChannelMarker;
use crate::discord::app::creators::{create_event, EventHandler};
use crate::settings::global::EColor;
use crate::utils::embeds::res;

pub fn member_added() -> EventHandler {
    create_event(
        EventType::MemberAdd,
        |event, client| async move {
            let member = match event {
                Event::MemberAdd(member) => member,
                _ => return (),
            };

            let name = &member.user.name;
            let message = format!("{} joined the server", name);

            let channels = match client.guild_channels(member.guild_id).await {
                Ok(channels) => channels,
                Err(err) => {
                    eprintln!("Error trying to responde MemberAdd event: Can not get channels from guild.\n{}", err);
                    return
                }
            };

            let mut welcome_channel: Option<Id<ChannelMarker>> = None;
            for channel in channels.models().await.unwrap_or_default() {
                if channel.name.as_deref() == Some("😏┊welcome") && channel.kind == ChannelType::GuildText {
                    welcome_channel = channel.id.into()
                }
            };
            if welcome_channel.is_none() {
                eprintln!("Error trying to responde MemberAdd event: Can not get channels from guild.");
                return;
            }
            let embed_res = res(EColor::Success, message);
            let result = client.create_message(welcome_channel.unwrap()).embeds(&[embed_res]).await;

            if let Err(why) = result {
                eprintln!("Error trying to responde MemberAdd event: {:?}", why);
            }
        }
    )
}
