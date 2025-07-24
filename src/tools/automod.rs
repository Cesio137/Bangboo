use crate::data::settings::EColors;
use crate::settings::logger;
use once_cell::sync::Lazy;
use regex::Regex;
use serenity::all::{CacheHttp, CreateEmbed, CreateMessage, Message};
use serenity::client::Context;
use crate::settings::logger::error;

static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[([^]]+)]\((https?://[^)]+)\)").unwrap());

pub async fn filter_message(ctx: &Context, message: &Message) -> bool {
    let content = &message.content;
    let channel_id = &message.channel_id;
    if !REGEX.is_match(content) {
        return false;
    }

    let username = message
        .author
        .global_name
        .as_ref()
        .unwrap_or(&message.author.name);

    let warning = format!(
        "{} sent a message that was flagged as a scam. Messages containing **[text](hyperlink)** are strictly prohibited. Bangboo (me) will presume his/her account has been compromised, leading to a server kick!",
        username
    );

    let server_embed = CreateEmbed::new()
        .color(EColors::warning as u32)
        .description(warning);
    if let Err(err) =  channel_id
        .send_message(ctx.http(), CreateMessage::new().embed(server_embed))
        .await {
        error(&format!(
            "Failed to send warning message\nʟ {:?}",
            err
        ));
    }
    if let Err(err) = message.delete(ctx.http()).await {
        error(&format!(
            "Failed to delete scan message\nʟ {:?}",
            err
        ));
    }

    let guild_id = match message.guild_id.as_ref() {
        Some(guild_id) => guild_id,
        None => {
            error("Guild id is none.");
            return true;
        }
    };

    let guild = match guild_id.to_guild_cached(&ctx) {
        Some(guild) => guild.clone(),
        None => {
            error("Guild is none.");
            return true;
        }
    };

    if guild.owner_id == message.author.id {
        error("Tried to kick the owner of the guild");
        return true;
    }

    let _ = guild
        .kick_with_reason(
            ctx.http(),
            message.author.id,
            "Sent a message that was flagged as a scam.",
        )
        .await;

    let private_channel = match message.author.id.create_dm_channel(ctx.http()).await {
        Ok(channel) => channel,
        Err(err) => {
            error(&format!(
                "Failed to create DM channel.\nʟ {:?}",
                err
            ));
            return true;
        }
    };

    let dm_warning = "It look like you probably got hacked and sent a message that was flagged as scam containing ***[text](hyperlink)***. You were just kicked from the server, but feel free to come back as soon as you resolve the issue with your account.";
    let dm_embed = CreateEmbed::new()
        .color(EColors::warning as u32)
        .description(dm_warning);
    if let Err(err) = private_channel
        .send_message(ctx.http(), CreateMessage::new().embed(dm_embed))
        .await {
        error(&format!(
            "Failed to send DM warning.\nʟ {:?}",
            err
        ));
        return true;
    }

    true
}
