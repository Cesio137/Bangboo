use std::time::Duration;
use std::vec;

use async_trait::async_trait;
use serenity::all::{ButtonStyle, CacheHttp, CommandInteraction, CommandOptionType, CommandType, ComponentInteraction, ComponentInteractionCollector, ComponentInteractionDataKind, Context, CreateActionRow, CreateButton, CreateCommand, CreateCommandOption, CreateEmbed, CreateEmbedAuthor, CreateInteractionResponse, CreateInteractionResponseMessage, CreateQuickModal, EditInteractionResponse, Guild, GuildId, InteractionContext, Member, PartialGuild, ReactionType, RoleId, Timestamp, User, UserId};
use serenity::futures::StreamExt;
use crate::discord::app::base::App;
use crate::discord::app::creators::SlashCommandHandler;
use crate::menus::moderate::{ban_menu, close_menu, kick_menu, load_menu, timeout_menu};
use crate::settings::global::{EColor, APP_RULE_ID, KERNEL_RULE_ID, STF_RULE_ID};
use crate::utils::embeds::res;
use crate::utils::interaction::reply_with_embed;

pub struct Moderate;

#[async_trait]
impl SlashCommandHandler for Moderate {
    fn command(&self) -> CreateCommand {
        CreateCommand::new("moderate")
            .description("Equality before the law is the cornerstone of justice ⚖.")
            .kind(CommandType::ChatInput)
            .add_context(InteractionContext::Guild)
            .add_option(
                CreateCommandOption::new(CommandOptionType::String, "action", "Select an action.")
                .required(true)
                    .add_string_choice("timeout", "timeout")
                    .add_string_choice("kick", "kick")
                    .add_string_choice("ban", "ban")
            )
    }

    async fn run(&self, app: &App, ctx: Context, interaction: CommandInteraction) {
        let member = match interaction.member.as_ref() {
            Some(member) => member,
            None => {
                let embed = res(EColor::Danger, "Interaction member is none.");
                let _ = reply_with_embed(&ctx, &interaction, embed, false).await;
                return;
            },
        };
        
        let permissions = match member.permissions.as_ref() {
            Some(permissions) => permissions,
            None => {
                let embed = res(EColor::Danger, "Interaction member has no permission.");
                let _ = reply_with_embed(&ctx, &interaction, embed, false).await;
                return;
            }
        };

        if !permissions.administrator() {
            let embed = res(EColor::Danger, "You don't have **ADMINISTRATOR** permission.");
            let _ = reply_with_embed(&ctx, &interaction, embed, false).await;
            return;
        }

        if interaction.data.options.is_empty() {
            let embed = res(EColor::Danger, "Interaction options is empty.");
            let _ = reply_with_embed(&ctx, &interaction, embed, false).await;
            return;
        }

        let action = interaction.data.options[0].value.as_str().unwrap();
        
        match action {
            "timeout" => timeout_collector(&ctx, &interaction, member).await,
            "kick" => kick_collector(&ctx, &interaction, member).await,
            "ban" => ban_collector(&ctx, &interaction, member).await,
            _ => {}
        }

    }
}

pub async fn filter_users(ctx: &Context, guild_id: &GuildId, ids: Vec<UserId>) -> Vec<UserId> {
    let mut filtered_ids = Vec::new();

    for user_id in ids {
        if let Ok(member) = guild_id.member(ctx.http(), user_id).await {
            if member.user.bot {
                continue;
            }

            if let Ok(guild) = guild_id.to_partial_guild(ctx.http()).await {
                if guild.owner_id == user_id {
                    continue;
                }
            }

            if member.roles.contains(&RoleId::new(KERNEL_RULE_ID)) || 
            member.roles.contains(&RoleId::new(STF_RULE_ID)) || 
            member.roles.contains(&RoleId::new(APP_RULE_ID)) {
                continue;
            }

            filtered_ids.push(user_id);
        }
    }

    filtered_ids
}

pub async fn timeout_action(ctx: &Context, interaction: &ComponentInteraction, member: &Box<Member>, guild_id: &GuildId, ids: &[UserId], duration: &str) {
    let duration = Duration::from_secs(duration.parse::<u64>().unwrap());

    let mut success = Vec::new();
    let mut failed = Vec::new();

    let duration = Timestamp::from_unix_timestamp((chrono::Utc::now() + duration).timestamp()).unwrap();
    for id in ids {
        if let Ok(mut member) = guild_id.member(ctx.http(), id).await {
            match member.disable_communication_until_datetime(ctx.http(), duration).await {
                Ok(_) => success.push(id),
                Err(_) => failed.push(id),
            }
        } else {
            failed.push(id);
        }
    }

    let mut description = Vec::new();
    description.push("***Timeouted users:***".to_string());
    for user_id in success {
        description.push(format!("<@{}>", user_id));
    }
    if !failed.is_empty() {
        description.push("\n***Untimeouted users:***".to_string());
        for user_id in failed {
            description.push(format!("<@{}>", user_id));
        }
    }
    let description = description.join("\n");

    let embed = CreateEmbed::new()
        .color(EColor::Royal as u32)
        .author(
            CreateEmbedAuthor::new(member.user.global_name.as_ref().unwrap_or(&member.user.name))
                .icon_url(member.user.avatar_url().as_ref().unwrap_or(&"https://raw.githubusercontent.com/Cesio137/Bangboo/refs/heads/master/assets/member/default_avatar.png".to_string()))
        )
        .title("**Officer Cui's panel**")
        .thumbnail("https://raw.githubusercontent.com/Cesio137/Bangboo/refs/heads/master/assets/avatar/Officer.png")
        .description(description);

    let _ = interaction.edit_response(
        ctx.http(), 
        EditInteractionResponse::new().add_embed(embed).components(vec![])
    ).await;
}

pub async fn timeout_collector(ctx: &Context, interaction: &CommandInteraction, member: &Box<Member>) {
    let guild = match interaction.guild_id.as_ref() {
        Some(guild_id) => guild_id,
        None => {
            let embed = res(EColor::Danger, "Guild id is none.");
            let _ = reply_with_embed(ctx, interaction, embed, false).await;
            return;
        },
    };

    let (embed, components) = timeout_menu(&member.user, &vec![], "");
    let res = interaction.create_response(
        ctx.http(), 
        CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .embed(embed).components(components).ephemeral(true)
        )
    ).await;

    if res.is_err() {return}

    let message_id = match interaction.get_response(ctx.http()).await {
        Ok(msg) => msg.id,
        Err(_) => return,
    };
    let user_id = member.user.id;
    let filter = move |i: &ComponentInteraction| i.message.id == message_id && i.member.as_ref().unwrap().user.id == user_id;
    let mut collector = ComponentInteractionCollector::new(&ctx.shard)
        .filter(filter)
        .author_id(interaction.user.id)
        .timeout(Duration::from_secs(300))
        .stream();

    // Data
    let mut ids: Vec<UserId> = Vec::new();
    let mut duration = String::new();
    let mut timeout = true;

    while let Some(i) = collector.next().await {
        let _ = i.defer(ctx.http()).await;

        match &i.data.kind {
            ComponentInteractionDataKind::Button => {
                if &i.data.custom_id == "moderate/btn-cancel" {
                    timeout = false;
                    let close_embed = close_menu(&member.user, timeout);
                    let _ = i.edit_response(
                        ctx.http(), 
                        EditInteractionResponse::new().add_embed(close_embed).components(vec![])
                    ).await;
                    return;
                }
                timeout_action(ctx, &i, member, guild, &ids, &duration).await;
                return ;
            },
            ComponentInteractionDataKind::StringSelect { values } => {
                duration = values.first().cloned().unwrap_or("".to_string());
            },
            ComponentInteractionDataKind::UserSelect { values } => {
                let load = load_menu(&member.user);
                let _ = i.edit_response(
                    ctx.http(), 
                    EditInteractionResponse::new().add_embed(load)
                ).await;
                ids = filter_users(ctx, guild, values.clone()).await;
            },
            _ => {}
        }

        let menu = timeout_menu(&member.user, &ids, &duration);
        let _ = i.edit_response(
            ctx.http(), 
            EditInteractionResponse::new()
                .add_embed(menu.0).components(menu.1)
        ).await;
    }
    let close_embed = close_menu(&member.user, timeout);
    let _ = interaction.edit_response(
        ctx.http(), 
        EditInteractionResponse::new().add_embed(close_embed)
    ).await;
}

pub async fn kick_action(ctx: &Context, interaction: &ComponentInteraction, member: &Box<Member>, guild_id: &GuildId, ids: &[UserId]) -> bool {
    let res = interaction.quick_modal(
        ctx,
        CreateQuickModal::new("What's the reason?")
            .paragraph_field("Reason")
            .timeout(Duration::from_secs(120))
    ).await;

    let reason = match res {
        Ok(modal) => {
            if let Some(modal_submit) = modal {
                let _ = modal_submit.interaction.create_response(ctx.http(), CreateInteractionResponse::Acknowledge).await;
                modal_submit.inputs[0].clone()
            } else { 
                "".to_string()
            }
        },
        Err(_) => { let _ = interaction.edit_response(ctx.http(), EditInteractionResponse::new()).await; return false; },
    };
    let mut success = Vec::new();
    let mut failed = Vec::new();

    for id in ids {
        if let Ok(member) = guild_id.member(ctx.http(), id).await {
            match member.kick_with_reason(ctx.http(), &reason).await {
                Ok(_) => success.push(id),
                Err(_) => failed.push(id),
            }
        } else {
            failed.push(id);
        }
    }

    let mut description = Vec::new();
    description.push("***Timeouted users:***".to_string());
    for user_id in success {
        description.push(format!("<@{}>", user_id));
    }
    if !failed.is_empty() {
        description.push("\n***Untimeouted users:***".to_string());
        for user_id in failed {
            description.push(format!("<@{}>", user_id));
        }
    }
    let description = description.join("\n");

    let embed = CreateEmbed::new()
        .color(EColor::Royal as u32)
        .author(
            CreateEmbedAuthor::new(member.user.global_name.as_ref().unwrap_or(&member.user.name))
                .icon_url(member.user.avatar_url().as_ref().unwrap_or(&"https://raw.githubusercontent.com/Cesio137/Bangboo/refs/heads/master/assets/member/default_avatar.png".to_string()))
        )
        .title("**Officer Cui's panel**")
        .thumbnail("https://raw.githubusercontent.com/Cesio137/Bangboo/refs/heads/master/assets/avatar/Officer.png")
        .description(description);

    let _ = interaction.edit_response(
        ctx.http(), 
        EditInteractionResponse::new().add_embed(embed).components(vec![])
    ).await;

    true
}

pub async fn kick_collector(ctx: &Context, interaction: &CommandInteraction, member: &Box<Member>) {
    let guild = match interaction.guild_id.as_ref() {
        Some(guild_id) => guild_id,
        None => {
            let embed = res(EColor::Danger, "Guild id is none.");
            let _ = reply_with_embed(ctx, interaction, embed, false).await;
            return;
        },
    };

    let (embed, components) = kick_menu(&member.user, &vec![]);
    let res = interaction.create_response(
        ctx.http(), 
        CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .embed(embed).components(components).ephemeral(true)
        )
    ).await;

    if res.is_err() {return}

    let message_id = match interaction.get_response(ctx.http()).await {
        Ok(msg) => msg.id,
        Err(_) => return,
    };
    let user_id = member.user.id;
    let filter = move |i: &ComponentInteraction| i.message.id == message_id && i.member.as_ref().unwrap().user.id == user_id;
    let mut collector = ComponentInteractionCollector::new(&ctx.shard)
        .filter(filter)
        .author_id(interaction.user.id)
        .timeout(Duration::from_secs(300))
        .stream();

    // Data
    let mut ids: Vec<UserId> = Vec::new();
    let mut timeout = true;

    while let Some(i) = collector.next().await {
        match &i.data.kind {
            ComponentInteractionDataKind::Button => {
                if &i.data.custom_id == "moderate/btn-cancel" {
                    timeout = false;
                    let close_embed = close_menu(&member.user, timeout);
                    let _ = i.create_response(
                        ctx.http(), 
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                            .embed(close_embed)
                            .components(vec![])
                            .ephemeral(true)
                        )
                    ).await;
                    return;
                }
                if kick_action(ctx, &i, member, guild, &ids).await {return ;}
                continue;
            },
            ComponentInteractionDataKind::UserSelect { values } => {
                let _ = i.defer(ctx.http()).await;
                let load = load_menu(&member.user);
                let _ = i.edit_response(
                    ctx.http(), 
                    EditInteractionResponse::new().add_embed(load)
                ).await;
                ids = filter_users(ctx, guild, values.clone()).await;
            },
            _ => {}
        }

        let menu = kick_menu(&member.user, &ids);
        let _ = i.edit_response(
            ctx.http(), 
            EditInteractionResponse::new()
                .add_embed(menu.0).components(menu.1)
        ).await;
    }
    let close_embed = close_menu(&member.user, timeout);
    let _ = interaction.edit_response(
        ctx.http(), 
        EditInteractionResponse::new().add_embed(close_embed)
    ).await;
}

pub async fn ban_action(ctx: &Context, interaction: &ComponentInteraction, member: &Box<Member>, guild_id: &GuildId, ids: &[UserId]) -> bool {
    let res = interaction.quick_modal(
        ctx,
        CreateQuickModal::new("What's the reason?")
            .paragraph_field("Reason")
            .timeout(Duration::from_secs(120))
    ).await;

    let reason = match res {
        Ok(modal) => {
            if let Some(modal_submit) = modal {
                let _ = modal_submit.interaction.create_response(ctx.http(), CreateInteractionResponse::Acknowledge).await;
                modal_submit.inputs[0].clone()
            } else { 
                "".to_string()
            }
        },
        Err(_) => { let _ = interaction.edit_response(ctx.http(), EditInteractionResponse::new()).await; return false; },
    };
    let mut success = Vec::new();
    let mut failed = Vec::new();

    for id in ids {
        if let Ok(member) = guild_id.member(ctx.http(), id).await {
            match member.ban_with_reason(ctx.http(), 255, &reason).await {
                Ok(_) => success.push(id),
                Err(_) => failed.push(id),
            }
        } else {
            failed.push(id);
        }
    }

    let mut description = Vec::new();
    description.push("***Timeouted users:***".to_string());
    for user_id in success {
        description.push(format!("<@{}>", user_id));
    }
    if !failed.is_empty() {
        description.push("\n***Untimeouted users:***".to_string());
        for user_id in failed {
            description.push(format!("<@{}>", user_id));
        }
    }
    let description = description.join("\n");

    let embed = CreateEmbed::new()
        .color(EColor::Royal as u32)
        .author(
            CreateEmbedAuthor::new(member.user.global_name.as_ref().unwrap_or(&member.user.name))
                .icon_url(member.user.avatar_url().as_ref().unwrap_or(&"https://raw.githubusercontent.com/Cesio137/Bangboo/refs/heads/master/assets/member/default_avatar.png".to_string()))
        )
        .title("**Officer Cui's panel**")
        .thumbnail("https://raw.githubusercontent.com/Cesio137/Bangboo/refs/heads/master/assets/avatar/Officer.png")
        .description(description);

    let _ = interaction.edit_response(
        ctx.http(), 
        EditInteractionResponse::new().add_embed(embed).components(vec![])
    ).await;

    true
}

pub async fn ban_collector(ctx: &Context, interaction: &CommandInteraction, member: &Box<Member>) {
    let guild = match interaction.guild_id.as_ref() {
        Some(guild_id) => guild_id,
        None => {
            let embed = res(EColor::Danger, "Guild id is none.");
            let _ = reply_with_embed(ctx, interaction, embed, false).await;
            return;
        },
    };

    let (embed, components) = ban_menu(&member.user, &vec![]);
    let res = interaction.create_response(
        ctx.http(), 
        CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .embed(embed).components(components).ephemeral(true)
        )
    ).await;

    if res.is_err() {return}

    let message_id = match interaction.get_response(ctx.http()).await {
        Ok(msg) => msg.id,
        Err(_) => return,
    };
    let user_id = member.user.id;
    let filter = move |i: &ComponentInteraction| i.message.id == message_id && i.member.as_ref().unwrap().user.id == user_id;
    let mut collector = ComponentInteractionCollector::new(&ctx.shard)
        .filter(filter)
        .author_id(interaction.user.id)
        .timeout(Duration::from_secs(300))
        .stream();

    // Data
    let mut ids: Vec<UserId> = Vec::new();
    let mut timeout = true;

    while let Some(i) = collector.next().await {
        match &i.data.kind {
            ComponentInteractionDataKind::Button => {
                if &i.data.custom_id == "moderate/btn-cancel" {
                    timeout = false;
                    let close_embed = close_menu(&member.user, timeout);
                    let _ = i.create_response(
                        ctx.http(), 
                        CreateInteractionResponse::UpdateMessage(
                            CreateInteractionResponseMessage::new()
                            .embed(close_embed)
                            .components(vec![])
                            .ephemeral(true)
                        )
                    ).await;
                    return;
                }
                if ban_action(ctx, &i, member, guild, &ids).await {return ;}
                continue;
            },
            ComponentInteractionDataKind::UserSelect { values } => {
                let _ = i.defer(ctx.http()).await;
                let load = load_menu(&member.user);
                let _ = i.edit_response(
                    ctx.http(), 
                    EditInteractionResponse::new().add_embed(load)
                ).await;
                ids = filter_users(ctx, guild, values.clone()).await;
            },
            _ => {}
        }

        let menu = ban_menu(&member.user, &ids);
        let _ = i.edit_response(
            ctx.http(), 
            EditInteractionResponse::new()
                .add_embed(menu.0).components(menu.1)
        ).await;
    }
    let close_embed = close_menu(&member.user, timeout);
    let _ = interaction.edit_response(
        ctx.http(), 
        EditInteractionResponse::new().add_embed(close_embed)
    ).await;
}