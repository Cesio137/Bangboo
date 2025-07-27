use crate::data::settings::EColors;
use crate::menus::moderate::close::close_menu;
use crate::menus::moderate::message_delete::message_delete_menu;
use crate::settings::logger::error;
use crate::utils::interaction::{
    edit_component_reply, edit_reply, reply, reply_component, reply_with_embed,
};
use serenity::all::{
    CacheHttp, CommandInteraction, ComponentInteraction, ComponentInteractionCollector,
    ComponentInteractionDataKind, Context, CreateEmbed, CreateEmbedAuthor, Member, UserId,
};
use serenity::futures::StreamExt;
use serenity::nonmax::NonMaxU8;
use std::time::Duration;

pub async fn delete_message_action(
    ctx: &Context,
    interaction: &ComponentInteraction,
    ids: &[UserId],
) {
    let mut success: Vec<UserId> = Vec::new();
    let mut failed: Vec<UserId> = Vec::new();

    let user = &interaction.user;

    let max_msg = NonMaxU8::new(255);
    let message_ids = match ctx
        .http
        .get_messages(interaction.channel_id, None, max_msg)
        .await
    {
        Ok(msg) => msg
            .iter()
            .filter(|val| ids.contains(&val.author.id))
            .cloned()
            .map(|val| val.id)
            .collect::<Vec<_>>(),
        Err(_) => {
            let embed_author =
                CreateEmbedAuthor::new(user.global_name.as_ref().unwrap_or(&user.name))
                    .icon_url(user.avatar_url().unwrap_or(user.default_avatar_url()));

            let embed = CreateEmbed::new()
                .color(EColors::royal as u32)
                .author(embed_author)
                .title("**Officer Cui's panel**")
                .thumbnail("https://raw.githubusercontent.com/Cesio137/Bangboo/refs/heads/rust/assets/avatar/Officer.png")
                .description("**Failed to delete message(s): No user(s) selected**");

            edit_component_reply(ctx, interaction, None, Some(vec![embed]), Some(vec![])).await;
            return;
        }
    };

    match interaction
        .channel_id
        .delete_messages(ctx.http(), &message_ids, None)
        .await
    {
        Ok(_) => success.extend(ids),
        Err(err) => {
            error(&format!("Could not delete message(s)\n└ {:?}", err));
            failed.extend(ids)
        }
    }

    let mut description = Vec::new();
    if !success.is_empty() {
        description.push("**Deleted message(s) from users:**".to_string());
        for user_id in success {
            description.push(format!("<@{}>", user_id));
        }
    }
    if !failed.is_empty() {
        description.push("\n**Failed to delete message(s) from user(s):**".to_string());
        for user_id in failed {
            description.push(format!("<@{}>", user_id));
        }
    }
    let description = description.join("\n");

    let embed_author = CreateEmbedAuthor::new(user.global_name.as_ref().unwrap_or(&user.name))
        .icon_url(user.avatar_url().unwrap_or(user.default_avatar_url()));

    let embed = CreateEmbed::new()
        .color(EColors::royal as u32)
        .author(embed_author)
        .title("**Officer Cui's panel**")
        .thumbnail("https://raw.githubusercontent.com/Cesio137/Bangboo/refs/heads/rust/assets/avatar/Officer.png")
        .description(description);

    reply_component(
        ctx,
        &interaction,
        true,
        None,
        Some(vec![embed]),
        Some(vec![]),
    )
    .await;
}

pub async fn delete_message_collector(
    ctx: &Context,
    interaction: &CommandInteraction,
    member: &Box<Member>,
) {
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
    let (embed, components) = message_delete_menu(&member.user, &empty_slice);
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
                _ = i.defer(ctx.http());
                delete_message_action(ctx, &i, &ids).await;
                timeout = false;
                break;
            }
            ComponentInteractionDataKind::UserSelect { values } => {
                ids = values.to_vec();
            }
            _ => {}
        }

        let (embed, components) = message_delete_menu(&member.user, &ids);
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
