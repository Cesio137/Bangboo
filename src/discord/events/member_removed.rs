use crate::discord::app::base::App;
use crate::settings::logger::error;
use crate::utils::global::{global_message, EventType};
use serenity::all::{Context, GuildId, User};

pub async fn run(app: &App, ctx: Context, guild_id: GuildId, user: User) {
    if ctx.cache.guild(guild_id).is_none() {
        error("Failed to load guild data from cache.");
        return;
    }
    let guild = ctx.cache.guild(guild_id).unwrap().clone();
    let system_channel_id = match guild.system_channel_id {
        Some(channel_id) => channel_id,
        None => {
            error("System channel not set for guild.");
            return;
        }
    };
    global_message(&ctx, &system_channel_id, EventType::MemberAdd, None, &user).await;
}
