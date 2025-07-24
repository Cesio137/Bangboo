use crate::discord::app::base::App;
use crate::settings::logger::error;
use crate::utils::global::{global_message, EventType};
use serenity::all::{Context, Member};

pub async fn run(app: &App, ctx: Context, member_added: Member) {
    let guild_id = member_added.guild_id.as_ref();
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

    let user = member_added.user.clone();
    global_message(
        &ctx,
        &system_channel_id,
        EventType::MemberAdd,
        Some(&member_added),
        &user,
    )
    .await;
}
