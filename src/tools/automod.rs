use crate::constants::*;
use crate::discord::*;
use once_cell::sync::Lazy;
use regex::Regex;
use serenity::all::{CacheHttp, Context, CreateEmbed, CreateMessage, Message};

static REGEXSTEAM: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[\s*(?i:steam)[^\]]*]\((https?://[^)]+)\)").unwrap());
static REGEXATTACHMENT: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?:(?:https?:\/\/)?(?:cdn|media)\.discordapp\.(?:net|com)\/attachments\/\d+\/\d+\/[^\s]+\.(?:png|jpg|jpeg|webp)(?:\?[^\s]*)?\s*){3,}").unwrap());

pub async fn filter_message(ctx: &Context, message: &Message) -> bool {
    let guild_id = match message.guild_id.as_ref() {
        Some(guild_id) => guild_id,
        None => {
            return true;
        }
    };

    let content = &message.content;
    
    let steam_test = REGEXSTEAM.is_match(&content);
    let attatchment_test = REGEXATTACHMENT.is_match(&content);
    if !steam_test && !attatchment_test {
        return false;
    }

    let channel_id = &message.channel_id;
    let user_id = message.author.id.get();

    let warning = format!(
        "<@{}> sent a message that was flagged as a scam. Messages containing **[text](hyperlink)** or **more than 2 image attachments** are strictly prohibited. Bangboo (me) will presume his/her account has been compromised, leading to a server kick!",
        user_id
    );

    let server_embed = CreateEmbed::new()
        .color(COLORS.warning)
        .description(warning);
    let mut warning_msg = CreateMessage::new().embed(server_embed);
    if let Err(err) = channel_id
        .send_message(ctx.http(), warning_msg)
        .await
    {
        error(&format!("Failed to send warning message\nʟ {:?}", err));
    }
    if let Err(err) = message.delete(ctx.http(), Some("Sent a message flagged as a scam")).await {
        error(&format!("Failed to delete scan message\nʟ {:?}", err));
    }

    let guild = match guild_id.to_guild_cached(&ctx.cache) {
        Some(guild) => guild.clone(),
        None => {
            error("Guild is none.");
            return true;
        }
    };

    if guild.owner_id == message.author.id {
        error("Tried to kick the owner of the guild.");
        return true;
    }

    if guild
        .id
        .kick(
            ctx.http(),
            message.author.id,
            Some("Sent a message that was flagged as a scam."),
        )
        .await
        .is_err()
    {
        error("Failed to kick the owner of the scam message.");
        return true;
    }

    let private_channel = match message.author.id.create_dm_channel(ctx.http()).await {
        Ok(channel) => channel,
        Err(err) => {
            error(&format!("Failed to create DM channel.\nʟ {:?}", err));
            return true;
        }
    };

    let dm_warning = "It look like you probably got hacked and sent a message that was flagged as scam containing **[text](hyperlink)** or **more than 2 image attachments**. You were just kicked from the server, but feel free to come back as soon as you resolve the issue with your account.";
    let dm_embed = CreateEmbed::new()
        .color(COLORS.warning)
        .description(dm_warning);

    if let Err(err) = private_channel
        .id
        .widen()
        .send_message(ctx.http(), CreateMessage::new().embed(dm_embed))
        .await
    {
        error(&format!("Failed to send DM warning.\nʟ {:?}", err));
        return true;
    }

    true
}
