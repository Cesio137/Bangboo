use std::time::Duration;

use async_trait::async_trait;
use serenity::all::{ButtonStyle, CacheHttp, CommandInteraction, CommandOptionType, CommandType, ComponentInteraction, ComponentInteractionCollector, Context, CreateActionRow, CreateButton, CreateCommand, CreateCommandOption, CreateEmbed, CreateEmbedAuthor, CreateInteractionResponse, CreateInteractionResponseMessage, EditInteractionResponse, Guild, GuildId, InteractionContext, Member, ReactionType, User, UserId};
use serenity::futures::StreamExt;
use crate::discord::app::base::App;
use crate::discord::app::creators::SlashCommandHandler;
use crate::menus::moderate::timeout_menu;
use crate::settings::global::EColor;
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
            _ => {}
        }

    }
}

pub async fn filter_users(ctx: &Context, guild: &Guild, ids: Vec<UserId>) -> Vec<UserId> {
    let mut filtered_ids = Vec::new();

    for id in ids {
        if let Ok(member) = guild.member(ctx.http(), id).await {
            if member.user.bot {
                continue;
            }

            if guild.owner_id == id {
                continue;
            }

            if member.permissions.is_none() {
                continue;
            }

            if !member.permissions.unwrap().administrator() {
                continue;
            }

            filtered_ids.push(id);
        }
    }

    filtered_ids
}

pub async fn timeout_collector(ctx: &Context, interaction: &CommandInteraction, member: &Box<Member>) {
    let guild = match interaction.guild_id.as_ref() {
        Some(guild_id) => guild_id.to_guild_cached(&ctx.cache).unwrap().clone(),
        None => {
            let embed = res(EColor::Danger, "Guild id is none.");
            let _ = reply_with_embed(ctx, interaction, embed, false).await;
            return;
        },
    };

    let res = interaction.create_response(
        ctx.http(), 
        CreateInteractionResponse::Message(timeout_menu(&member.user, &vec![], ""))
    ).await;

    if res.is_err() {return}

    let message_id = match interaction.get_response(ctx.http()).await {
        Ok(msg) => msg.id,
        Err(_) => return,
    };
    let user_id = member.user.id;
    let filter = move |i: &ComponentInteraction| i.message.id == message_id && i.user.id == user_id;
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
        i.defer_ephemeral(ctx.http()).await;

        match &i.data.kind {
            serenity::all::ComponentInteractionDataKind::StringSelect { values } => {
                duration = values.first().cloned().unwrap_or("".to_string());
            },
            serenity::all::ComponentInteractionDataKind::UserSelect { values } => {
                ids = filter_users(ctx, &guild, values.clone()).await;
            },
            _ => {}
        }

        ctx.http().edit_original_interaction_response(
            &i.token, 
            &CreateInteractionResponse::Message(timeout_menu(&member.user, &ids, &duration)), 
            vec![]
        ).await;
    }
}