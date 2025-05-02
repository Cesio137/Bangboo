use crate::discord::app::base::App;
use crate::settings::global::EColor;
use crate::utils::embeds::res;
use crate::utils::global::{global_message, EventType};
use serenity::all::{Context, Member};
use serenity::builder::{CreateAttachment, CreateMessage};

pub async fn run(app: &App, ctx: Context, member_added: Member) {
    let guild_id = member_added.guild_id.as_ref();
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


    let user = member_added.user.clone();
    let canvas = global_message(EventType::MemberAdd, &user).await;
    if let Ok(bytes) = canvas {
        let message = CreateMessage::new();
        let attachment = CreateAttachment::bytes(bytes, "Welcome.png");
        if let Err(err) = system_channel_id.send_files(&ctx.http, vec![attachment], message).await {
            tracing::error!("Error when sending welcome image: {:?}", err);
        }
        return;
    } else if let Err(err) = canvas {
        tracing::error!("Failed to create welcome canvas: {:?}", err);
    }

    let id = member_added.user.id.to_string();
    let message = format!("<@{}> join the server!", id);
    let embed = res(EColor::Success, &message);
    let message = CreateMessage::new().embed(embed); 

    if let Err(err) = system_channel_id.send_message(&ctx.http, message).await {
        tracing::error!("Failed to send welcome message: {:?}", err);
    }
}