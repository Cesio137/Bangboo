use crate::discord::*;
use serenity::all::{Context, GuildId, User};

pub async fn run(app: &App, ctx: &Context, guild_id: &GuildId, user: &User) {
    if user.bot() {
        return;
    }

    let guild = match guild_id.to_guild_cached(&ctx.cache) {
        Some(guild) => guild.clone(),
        None => {
            error("Failed to load guild data from cache.");
            return;
        }
    };

    if guild_id.get_ban(ctx.http.clone(), user.id).await.is_ok() {
        return;
    }
    user.id;
    let system_channel_id = match guild.system_channel_id {
        Some(channel_id) => channel_id,
        None => {
            error("System channel not set for guild.");
            return;
        }
    };

    global_message(ctx, &system_channel_id, EventType::MemberRemoved, None, user).await;
}
