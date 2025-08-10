use crate::discord::*;
use crate::utils::*;
use serenity::all::{Context, GuildId, User};

pub async fn run(app: &App, ctx: &Context, guild_id: &GuildId, banned_user: &User) {
    if banned_user.bot() {
        return;
    }

    let guild = match guild_id.to_guild_cached(&ctx.cache) {
        Some(guild) => guild.clone(),
        None => {
            error("Failed to load guild data from cache.");
            return;
        }
    };

    let system_channel_id = match guild.system_channel_id {
        Some(channel_id) => channel_id,
        None => {
            error("System channel not set for guild.");
            return;
        }
    };

    global_message(
        &ctx,
        &system_channel_id,
        EventType::MemberAdd,
        None,
        &banned_user,
    )
    .await;
}
