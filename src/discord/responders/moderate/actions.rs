use std::time::Duration;
use anyhow::Result;
use serenity::all::{CacheHttp, ComponentInteraction, Context, CreateEmbed, EditInteractionResponse, GuildId, Timestamp, UserId};

pub async fn timeout_action(ctx: &Context, component: &ComponentInteraction, guild_id: &GuildId, user_ids: &[UserId], duration: &str, embed: CreateEmbed) -> Result<()> {
    let duration = match duration.parse::<u64>() {
        Ok(secs) => Duration::from_secs(secs),
        Err(_) => {
            let embed = embed.description("Invalid timeout duration specified.");
            component.edit_response(ctx.http(), EditInteractionResponse::new().add_embed(embed)).await?;
            return Ok(());
        }
    };

    let mut success = Vec::new();
    let mut failed = Vec::new();

    let duration = Timestamp::from_unix_timestamp((chrono::Utc::now() + duration).timestamp())?;
    for user_id in user_ids {
        if let Ok(mut member) = guild_id.member(ctx.http(), user_id).await {
            match member.disable_communication_until_datetime(ctx.http(), duration).await {
                Ok(_) => success.push(user_id),
                Err(_) => failed.push(user_id),
            }
        } else {
            failed.push(user_id);
        }
    }

    let mut description = Vec::new();
    description.push("***Timeouted users:***".to_string());
    for user_id in success {
        description.push(format!("<@{}>", user_id));
    }
    if !failed.is_empty() {
        description.push("\n***Unimeouted users:***".to_string());
        for user_id in failed {
            description.push(format!("<@{}>", user_id));
        }
    }
    let description = description.join("\n");

    let embed = embed.description(description);

    component.edit_response(ctx.http(), EditInteractionResponse::new().add_embed(embed).components(vec![])).await?;
    Ok(())
}

pub async fn kick_action(ctx: &Context, component: &ComponentInteraction, guild_id: &GuildId, user_ids: &[UserId], reason: &str, embed: CreateEmbed) -> Result<()> {
    let mut success = Vec::new();
    let mut failed = Vec::new();
    
    for user_id in user_ids {
        if let Ok(member) = guild_id.member(ctx.http(), user_id).await {
            match member.kick_with_reason(ctx.http(), reason).await {
                Ok(_) => success.push(user_id),
                Err(_) => failed.push(user_id),
            }
        } else {
            failed.push(user_id);
        }
    }

    let mut description = Vec::new();
    description.push("***Kicked users:***".to_string());
    for user_id in success {
        description.push(format!("<@{}>", user_id));
    }
    if !failed.is_empty() {
        description.push("\n***Unkicked users:***".to_string());
        for user_id in failed {
            description.push(format!("<@{}>", user_id));
        }
    }
    description.push(format!("\n***Reason: {}***", reason));
    
    let description = description.join("\n");

    let embed = embed.description(description);

    component.edit_response(ctx.http(), EditInteractionResponse::new().add_embed(embed).components(vec![])).await?;
    Ok(())
}

pub async fn ban_action(ctx: &Context, component: &ComponentInteraction, guild_id: &GuildId, user_ids: &[UserId], reason: &str, embed: CreateEmbed) -> Result<()> {
    let mut success = Vec::new();
    let mut failed = Vec::new();
    
    for user_id in user_ids {
        if let Ok(member) = guild_id.member(ctx.http(), user_id).await {
            match member.ban_with_reason(ctx.http(), 255, reason).await {
                Ok(_) => success.push(user_id),
                Err(_) => failed.push(user_id),
            }
        } else {
            failed.push(user_id);
        }
    }

    let mut description = Vec::new();
    description.push("***Banned users:***".to_string());
    for user_id in success {
        description.push(format!("<@{}>", user_id));
    }
    if !failed.is_empty() {
        description.push("\n***Unbanned users:***".to_string());
        for user_id in failed {
            description.push(format!("<@{}>", user_id));
        }
    }
    description.push(format!("\n***Reason: {}***", reason));
    
    let description = description.join("\n");

    let embed = embed.description(description);

    component.edit_response(ctx.http(), EditInteractionResponse::new().add_embed(embed).components(vec![])).await?;
    Ok(())
}