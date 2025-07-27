use crate::data::guild::ERoles;
use serenity::all::{CacheHttp, Context, GuildId, RoleId, UserId};

pub async fn filter_users(ctx: &Context, guild_id: &GuildId, ids: Vec<UserId>) -> Vec<UserId> {
    let mut filtered_ids = Vec::new();

    for user_id in ids {
        if let Ok(member) = guild_id.member(ctx.http(), user_id).await {
            if member.user.bot() {
                continue;
            }
            if member.roles.contains(&RoleId::new(ERoles::stf as u64))
                || member.roles.contains(&RoleId::new(ERoles::kernel as u64))
            {
                continue;
            }

            filtered_ids.push(user_id.clone());
        }
    }

    filtered_ids
}
