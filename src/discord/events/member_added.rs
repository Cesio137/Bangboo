use crate::discord::*;
use crate::functions::*;
use serenity::all::{Context, Member};

pub async fn run(app: &App, ctx: &Context, member_added: &Member) {
    if member_added.user.bot() {
        return;
    }

    let guild = match member_added.guild_id.to_guild_cached(&ctx.cache) {
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

    let user = member_added.user.clone();

    global_message(
        &ctx,
        &system_channel_id,
        EventType::MemberAdded,
        Some(&member_added),
        &user,
    )
    .await;
}
