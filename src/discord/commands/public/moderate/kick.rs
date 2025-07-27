use crate::data::settings::EColors;
use crate::discord::commands::public::moderate::filter::filter_users;
use crate::discord::commands::public::moderate::modal::show_modal;
use crate::menus::moderate::close::close_menu;
use crate::menus::moderate::kick::kick_menu;
use crate::menus::moderate::load::load_menu;
use crate::settings::logger::error;
use crate::utils::interaction::{
    edit_component_reply, edit_reply, reply, reply_component, reply_with_embed,
};
use serenity::all::{
    CacheHttp, CommandInteraction, ComponentInteraction, ComponentInteractionCollector,
    ComponentInteractionDataKind, Context, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter
    , GuildId, Member, UserId,
};
use serenity::futures::StreamExt;
use std::time::{Duration, SystemTime};

pub async fn kick_action(
    ctx: &Context,
    interaction: &ComponentInteraction,
    guild_id: &GuildId,
    ids: &[UserId],
    reason: &str,
) {
    let mut success = Vec::new();
    let mut failed = Vec::new();

    for id in ids {
        match guild_id.kick(ctx.http(), id.clone(), Some(reason)).await {
            Ok(_) => success.push(id),
            Err(_) => failed.push(id),
        }
    }

    let mut description = Vec::new();
    if !success.is_empty() {
        description.push("**Kicked users:**".to_string());
        for user_id in success {
            description.push(format!("<@{}>", user_id));
        }
    }
    if !failed.is_empty() {
        description.push("\n**Failed to kick user(s):**".to_string());
        for user_id in failed {
            description.push(format!("<@{}>", user_id));
        }
    }
    let description = description.join("\n");

    let user = &interaction.user;

    let embed_author = CreateEmbedAuthor::new(user.global_name.as_ref().unwrap_or(&user.name))
        .icon_url(user.avatar_url().unwrap_or(user.default_avatar_url()));
    let footer = format!("Reason: {}", reason);
    let embed_footer = CreateEmbedFooter::new(footer);

    let embed = CreateEmbed::new()
        .color(EColors::royal as u32)
        .author(embed_author)
        .title("**Officer Cui's panel**")
        .thumbnail("https://raw.githubusercontent.com/Cesio137/Bangboo/refs/heads/rust/assets/avatar/Officer.png")
        .description(description)
        .footer(embed_footer);

    edit_component_reply(ctx, interaction, None, Some(vec![embed]), Some(vec![])).await;
}

pub async fn kick_collector(ctx: &Context, interaction: &CommandInteraction, member: &Box<Member>) {
    let guild_id = match interaction.guild_id.as_ref() {
        Some(guild_id) => guild_id,
        None => {
            reply_with_embed(
                &ctx,
                &interaction,
                false,
                EColors::danger,
                "Guild id is none.",
            )
            .await;
            return;
        }
    };

    let empty_slice: Vec<UserId> = Vec::new();
    let (embed, components) = kick_menu(&member.user, &empty_slice);
    if !reply(
        ctx,
        interaction,
        true,
        false,
        None,
        Some(vec![embed]),
        Some(components),
        None,
    )
    .await
    {
        return;
    }

    let message_id = match interaction.get_response(ctx.http()).await {
        Ok(msg) => msg.id,
        Err(_) => {
            error("Failed to get message id.");
            return;
        }
    };
    let user_id = member.user.id;
    let filter = move |i: &ComponentInteraction| {
        i.message.id == message_id && i.member.as_ref().unwrap().user.id == user_id
    };

    let mut collector = ComponentInteractionCollector::new(&ctx)
        .filter(filter)
        .author_id(interaction.user.id)
        .timeout(Duration::from_secs(300))
        .stream();

    let time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => duration.as_secs() + 300,
        Err(err) => {
            error(&format!("Time went backwards!?\n└ {}", err));
            return;
        }
    };

    // Data
    let mut ids: Vec<UserId> = Vec::new();
    let mut cancel = false;
    let mut timeout = true;

    while let Some(i) = collector.next().await {
        let mut edit = false;

        match &i.data.kind {
            ComponentInteractionDataKind::Button => {
                if &i.data.custom_id == "mod/btn-cancel" {
                    timeout = false;
                    cancel = true;
                    break;
                }
                let (is_ok, reason) = show_modal(ctx, &i, time).await;
                if is_ok {
                    kick_action(ctx, &i, guild_id, &ids, &reason).await;
                    timeout = false;
                    break;
                }
                edit = true;
            }
            ComponentInteractionDataKind::UserSelect { values } => {
                let load = load_menu(&member.user, "👥 **Filtering selected users...**");
                reply_component(ctx, &i, true, None, Some(vec![load]), None).await;
                ids = filter_users(ctx, guild_id, values.to_vec()).await;
                edit = true;
            }
            _ => {}
        }

        let (embed, components) = kick_menu(&member.user, &ids);
        if edit {
            edit_component_reply(ctx, &i, None, Some(vec![embed]), Some(components)).await;
            continue;
        }
        reply_component(ctx, &i, true, None, Some(vec![embed]), Some(components)).await;
    }
    if timeout || cancel {
        let close_embed = close_menu(&member.user, timeout);
        edit_reply(
            ctx,
            interaction,
            None,
            Some(vec![close_embed]),
            Some(vec![]),
        )
        .await;
    }
}
