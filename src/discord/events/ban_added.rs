use crate::discord::app::base::App;
use crate::settings::global::EColor;
use crate::utils::embeds::res;
use crate::utils::global::{global_message, EventType};
use serenity::all::{Context, CreateAttachment, CreateMessage, GuildId, User};

pub async fn run(app: &App, ctx: Context, guild_id: GuildId, banned_user: User) {
    let guild = match guild_id.to_partial_guild(&ctx.http).await {
        Ok(guild) => guild,
        Err(why) => {
            tracing::error!("Failed to get guild info: {:?}", why);
            return;
        }
    };
    let system_channel_id = match guild.system_channel_id {
        Some(channel_id) => channel_id,
        None => {
            tracing::error!("System channel not set for guild.");
            return;
        }
    };

    let canvas = global_message(EventType::BanAdd, &banned_user).await;
    if let Ok(bytes) = canvas {
        let message = CreateMessage::new();
        let attachment = CreateAttachment::bytes(bytes, "Bye.png");
        if let Err(err) = system_channel_id.send_files(&ctx.http, vec![attachment], message).await {
            tracing::error!("Error when sending bye image: {:?}", err);
        }
        return;
    } else if let Err(err) = canvas {
        tracing::error!("Failed to create ban canvas: {:?}", err);
    }

    let id = banned_user.id.as_ref().to_string();
    let message = format!("<@{}> left the server!", id);
    let embed = res(EColor::Success, &message);
    let message = CreateMessage::new().embed(embed);

    if let Err(err) = system_channel_id.send_message(&ctx.http, message).await {
        tracing::error!("Failed to send bye message: {:?}", err);
    }
}