use crate::discord::*;
use crate::data::*;
use crate::menus::*;
use crate::utils::*;
use serenity::all::{
    CacheHttp, CommandInteraction, ComponentInteraction, ComponentInteractionCollector,
    ComponentInteractionDataKind, Context, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter,
    GuildId, Member, MessageFlags, UserId,
};
use serenity::futures::StreamExt;
use std::time::{Duration, SystemTime};
use crate::discord::public::moderate::filter::filter_users;
use crate::discord::public::moderate::modal::show_modal;

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
        .color(str_hex_to_u32(&CONSTANTS.colors.royal))
        .author(embed_author)
        .title("**Officer Cui's panel**")
        .thumbnail("https://raw.githubusercontent.com/Cesio137/Bangboo/refs/heads/rust/assets/avatar/Officer.png")
        .description(description)
        .footer(embed_footer);

    let payload = ReplyPayload {
        embeds: Some(vec![embed]),
        components: Some(vec![]),
        ..Default::default()
    };
    edit_component(ctx, interaction, MessageFlags::EPHEMERAL, &payload).await;
}

pub async fn kick_collector(ctx: &Context, interaction: &CommandInteraction, member: &Box<Member>) {
    let guild_id = match interaction.guild_id.as_ref() {
        Some(guild_id) => guild_id,
        None => {
            reply_with_embed(
                &ctx,
                &interaction,
                MessageFlags::EPHEMERAL,
                str_hex_to_u32(&CONSTANTS.colors.danger),
                "Guild id is none.",
            )
            .await;
            return;
        }
    };

    let mut payload = ReplyPayload::default();
    let empty_slice: Vec<UserId> = Vec::new();
    let (embed, components) = kick_menu(&member.user, &empty_slice);
    payload.embeds = Some(vec![embed]);
    payload.components = Some(components);
    if !reply(ctx, interaction, MessageFlags::EPHEMERAL, &payload).await {
        return;
    }

    let message_id = match interaction.get_response(ctx.http()).await {
        Ok(msg) => msg.id,
        Err(_) => {
            reply_with_embed(
                ctx,
                interaction,
                MessageFlags::EPHEMERAL,
                str_hex_to_u32(&CONSTANTS.colors.danger),
                "No message ID.",
            )
            .await;
            error("Failed to get message id.");
            return;
        }
    };
    let user_id = member.user.id;
    let filter = move |i: &ComponentInteraction| {
        i.message.id == message_id && i.member.as_ref().unwrap().user.id == user_id
    };

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

    let mut collector = ComponentInteractionCollector::new(&ctx)
        .filter(filter)
        .author_id(interaction.user.id)
        .timeout(Duration::from_secs(300))
        .stream();

    while let Some(i) = collector.next().await {
        let mut edit = false;
        payload = ReplyPayload::default();

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
                payload.embeds = Some(vec![load]);
                update_component(ctx, &i, MessageFlags::EPHEMERAL, &payload).await;
                payload = ReplyPayload::default();
                ids = filter_users(ctx, guild_id, values.to_vec()).await;
                edit = true;
            }
            _ => {}
        }

        let (embed, components) = kick_menu(&member.user, &ids);
        payload.embeds = Some(vec![embed]);
        payload.components = Some(components);
        if edit {
            edit_component(ctx, &i, MessageFlags::EPHEMERAL, &payload).await;
            continue;
        }
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
