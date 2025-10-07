use crate::discord::*;
use crate::helpers::*;
use crate::menus::*;
use crate::utils::*;
use serenity::all::{
    CacheHttp, CommandInteraction, ComponentInteraction, ComponentInteractionCollector,
    ComponentInteractionDataKind, Context, CreateEmbed, CreateEmbedAuthor, Member, MessageFlags,
    UserId,
};
use serenity::futures::StreamExt;
use serenity::nonmax::NonMaxU8;
use std::time::Duration;

pub async fn delete_message_action(
    ctx: &Context,
    interaction: &ComponentInteraction,
    ids: &[UserId],
) {
    let mut payload = ReplyPayload::default();
    payload.components = Some(vec![]);

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
                .color(str_hex_to_u32(&CONSTANTS.colors.royal))
                .author(embed_author)
                .title("**Officer Cui's panel**")
                .thumbnail("https://raw.githubusercontent.com/Cesio137/Bangboo/refs/heads/rust/assets/avatar/Officer.png")
                .description("**Failed to delete message(s): No user(s) selected**");

            payload.embeds = Some(vec![embed]);
            update_component(ctx, interaction, MessageFlags::EPHEMERAL, &payload).await;
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
        .color(str_hex_to_u32(&CONSTANTS.colors.royal))
        .author(embed_author)
        .title("**Officer Cui's panel**")
        .thumbnail("https://raw.githubusercontent.com/Cesio137/Bangboo/refs/heads/rust/assets/avatar/Officer.png")
        .description(description);
    payload.embeds = Some(vec![embed]);
    update_component(ctx, &interaction, MessageFlags::EPHEMERAL, &payload).await;
}

pub async fn delete_message_collector(
    ctx: &Context,
    interaction: &CommandInteraction,
    member: &Box<Member>,
) {
    let mut payload = ReplyPayload::default();
    let empty_slice: Vec<UserId> = Vec::new();
    let (embed, components) = message_delete_menu(&member.user, &empty_slice);
    payload.embeds = Some(vec![embed]);
    payload.components = Some(components);
    if !reply(ctx, interaction, MessageFlags::EPHEMERAL, &payload).await {
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

    // Data
    let mut ids: Vec<UserId> = Vec::new();
    let mut cancel = false;
    let mut timeout = true;

    let mut collector = ComponentInteractionCollector::new(&ctx)
        .filter(filter)
        .author_id(interaction.user.id)
        .timeout(Duration::from_secs(300))
        .stream();

    while let Some(i) = collector.next().await {
        payload = ReplyPayload::default();

        match &i.data.kind {
            ComponentInteractionDataKind::Button => {
                if &i.data.custom_id == "mod/btn-cancel" {
                    timeout = false;
                    cancel = true;
                    break;
                }
                _ = i.defer_ephemeral(ctx.http());
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
        payload.embeds = Some(vec![embed]);
        payload.components = Some(components);
        update_component(ctx, &i, MessageFlags::EPHEMERAL, &payload).await;
    }
    if timeout || cancel {
        payload = ReplyPayload::default();
        let close_embed = close_menu(&member.user, timeout);
        payload.embeds = Some(vec![close_embed]);
        payload.components = Some(vec![]);
        edit(ctx, interaction, MessageFlags::EPHEMERAL, &payload).await;
    }
}
