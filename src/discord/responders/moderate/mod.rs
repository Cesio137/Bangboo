mod actions;
mod components;
mod modal;

use actions::*;
use components::*;
use modal::*;
use crate::discord::app::creators::ResponderHandler;
use crate::settings::global::EColor;
use anyhow::Result;
use async_trait::async_trait;
use serenity::all::{CreateEmbed, CreateEmbedAuthor};
use serenity::{all::{CacheHttp, CommandInteraction, ComponentInteraction, ComponentInteractionCollector, ComponentInteractionDataKind, Context, EditInteractionResponse, GuildId, RoleId, UserId}, futures::StreamExt};
use std::cmp::PartialEq;
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModerateAction {
    None,
    Close,
    Timeout,
    Kick,
    Ban,
}

impl From<&str> for ModerateAction {
    fn from(value: &str) -> Self {
        match value {
            // Global
            "btn-close" => Self::Close,
            // Timeout
            "btn-timeout" => Self::Timeout,
            "timeout-select-user" => Self::Timeout,
            "timeout-select-duration" => Self::Timeout,
            // Kick
            "btn-kick" => Self::Kick,
            "kick-select-user" => Self::Kick,
            // Ban
            "btn-ban" => Self::Ban,
            "ban-select-user" => Self::Ban,
            _ => Self::None,
        }
    }
}

pub struct Moderate;

#[async_trait]
impl ResponderHandler for Moderate {
    fn custom_id(&self) -> String { String::from("moderate") }
    async fn run(&self, ctx: &Context, interaction: &CommandInteraction) {
        let guild_id = interaction.guild_id.as_ref().unwrap();
        let member = interaction.member.as_ref().unwrap();
        let user = &member.user;
        let user_name = user.global_name.as_ref().unwrap_or(&user.name);
        let mut author = CreateEmbedAuthor::new(user_name);
        if let Some(avatar_url) = user.avatar_url() {
            author = author.icon_url(avatar_url);
        }

        let main_embed = CreateEmbed::new()
            .color(EColor::Royal as u32)
            .author(author)
            .title("**Officer Cui's panel**")
            .thumbnail("https://raw.githubusercontent.com/Cesio137/Bangboo/refs/heads/master/assets/avatar/Officer.png");

        let mut component_collector = ComponentInteractionCollector::new(&ctx.shard)
            .author_id(interaction.user.id)
            .timeout(Duration::from_secs(300))
            .stream();

        // Data
        let mut mod_action = ModerateAction::None;
        let mut ids: Vec<UserId> = Vec::new();
        let mut duration = String::new();
        let mut timeout = true;

        while let Some(component) = component_collector.next().await {
            if let Err(err) = component.defer(ctx.http()).await {
                tracing::error!("Failed to reply moderate component.\n{}", err);
                continue;
            }

            let (prefix, action) = component.data.custom_id.split_once('/').unwrap_or(("", ""));
            if prefix != "moderate" || action.is_empty() { return; }

            if action == "btn-confirm" {
                match mod_action {
                    ModerateAction::Timeout => {
                        if let Err(err) = timeout_action(ctx, &component, guild_id, &ids, &duration, main_embed).await {
                            tracing::error!("Failed to reply moderate component.\n{}", err);
                        }
                    },
                    ModerateAction::Kick => {
                        let reason = modal_panel(ctx, &component).await;
                        if let Err(err) = kick_action(ctx, &component, guild_id, &ids, &reason, main_embed).await {
                            tracing::error!("Failed to reply moderate component.\n{}", err);
                        }
                    },
                    ModerateAction::Ban => {
                        let reason = modal_panel(ctx, &component).await;
                        if let Err(err) = ban_action(ctx, &component, guild_id, &ids, &reason, main_embed).await {
                            tracing::error!("Failed to reply moderate component.\n{}", err);
                        }
                    },
                    _ => {}
                }
                return;
            }

            mod_action = action.into();

            let (embed, components) = match mod_action {
                ModerateAction::None => {
                    ids.clear(); duration.clear();
                    main_components(main_embed.clone())
                }
                ModerateAction::Close => {
                    ids.clear(); duration.clear(); timeout = false;
                    close_panel(main_embed.clone(), timeout)
                }
                ModerateAction::Timeout => {
                    match &component.data.kind {
                        ComponentInteractionDataKind::UserSelect{ values } => {
                            if let Err(err) = load_embed(ctx, &component,"👥 ***Filtering selected users...***", main_embed.clone()).await {
                                tracing::error!("Failed to reply moderate panel.\n{}", err);
                                continue;
                            }
                            ids = filter_ids(ctx, guild_id, values.clone()).await;
                        }
                        ComponentInteractionDataKind::StringSelect { values } => { duration = values.first().cloned().unwrap_or("".to_string()) }
                        _ => {}
                    }
                    timeout_panel(main_embed.clone(), &ids, &duration)
                }
                ModerateAction::Kick => {
                    match &component.data.kind {
                        ComponentInteractionDataKind::UserSelect{ values } => {
                            if let Err(err) = load_embed(ctx, &component,"👥 ***Filtering selected users...***", main_embed.clone()).await {
                                tracing::error!("Failed to reply moderate panel.\n{}", err);
                                continue;
                            }
                            ids = filter_ids(ctx, guild_id, values.clone()).await;
                        }
                        _ => {}
                    }
                    kick_panel(main_embed.clone(), &ids)
                }
                ModerateAction::Ban => {
                    match &component.data.kind {
                        ComponentInteractionDataKind::UserSelect{ values } => {
                            if let Err(err) = load_embed(ctx, &component,"👥 ***Filtering selected users...***", main_embed.clone()).await {
                                tracing::error!("Failed to reply moderate panel.\n{}", err);
                                continue;
                            }
                            ids = filter_ids(ctx, guild_id, values.clone()).await;
                        }
                        _ => {}
                    }
                    ban_panel(main_embed.clone(), &ids)
                }
            };

            let message = EditInteractionResponse::new()
                .add_embed(embed)
                .components(components);

            let result = component.edit_response(
                ctx.http(),
                message
            ).await;

            if let Err(err) = result {
                tracing::error!("Failed to reply {} component.\n{}", "close", err);
                return;
            }

            if mod_action == ModerateAction::Close {
                timeout = false;
                break;
            }
        }
        if timeout {
            let (embed, _) = close_panel(main_embed.clone(), timeout);
            let message = EditInteractionResponse::new()
                .add_embed(embed)
                .components(vec![]);
            let result = interaction.edit_response(
                ctx.http(),
                message
            ).await;
            if let Err(err) = result {
                tracing::error!("Failed to reply {} component.\n{}", "close", err);
                return;
            }
        }
    }
}

pub async fn load_embed(ctx: &Context, interaction: &ComponentInteraction, description: &str, embed: CreateEmbed) -> Result<()> {
    let embed = embed.description(description);
    interaction.edit_response(
        ctx.http(),
        EditInteractionResponse::new().add_embed(embed)
    ).await?;
    Ok(())
}

pub async fn filter_ids(ctx: &Context, guild_id: &GuildId, ids: Vec<UserId>) -> Vec<UserId> {
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

            if member.roles.contains(&RoleId::new(1254154469428691035)) {
                continue;
            }

            filtered_ids.push(user_id);
        }
    }

    filtered_ids
}





