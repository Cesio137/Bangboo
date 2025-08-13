use crate::discord::*;
use crate::data::*;
use crate::utils::*;
use serenity::all::{CacheHttp, Context, GuildMemberUpdateEvent, Member, RoleId};

pub async fn run(
    ctx: &Context,
    old_if_available: &Option<Member>,
    new: &Option<Member>,
    event: &GuildMemberUpdateEvent,
) {
    let old_was_premium = {
        if let Some(old) = old_if_available {
            old.premium_since.is_some()
        } else {
            false
        }
    };
    let new_was_premium = {
        if let Some(new_member) = new {
            new_member.premium_since.is_some()
        } else {
            false
        }
    };

    let member = new.as_ref().unwrap().clone();
    let role_id = RoleId::new(str_to_u64(&GUILD.roles.boosters));
    if !old_was_premium && new_was_premium {
        if let Err(err) = member
            .add_role(ctx.http(), role_id, Some("Became a booster!"))
            .await
        {
            error(&format!("Failed to set member role!\n└ {:?}", err));
            return;
        }

        global_boost(ctx, &event.user, &event.guild_id).await;
    } else {
        if let Err(err) = member
            .remove_role(ctx.http(), role_id, Some("Boost is over!"))
            .await
        {
            error(&format!("Failed to remove member role!\n└ {:?}", err));
        }
    }
}
