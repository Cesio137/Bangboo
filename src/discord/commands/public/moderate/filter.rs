use crate::helpers::*;
use serenity::all::{CacheHttp, Context, GuildId, RoleId, UserId};

pub async fn filter_users(ctx: &Context, guild_id: &GuildId, ids: Vec<UserId>) -> Vec<UserId> {
    let mut filtered_ids = Vec::new();

    for user_id in ids {
        if let Ok(member) = guild_id.member(ctx.http(), user_id).await {
            if member.user.bot() {
                continue;
            }
            if member.roles.contains(&RoleId::new(str_to_u64(&GUILD.roles.stf)))
                || member.roles.contains(&RoleId::new(str_to_u64(&GUILD.roles.kernel)))
            {
                continue;
            }

            filtered_ids.push(user_id.clone());
        }
    }

    filtered_ids
}
