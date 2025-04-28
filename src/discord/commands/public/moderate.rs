use async_trait::async_trait;
use serenity::all::{ButtonStyle, CommandInteraction, CommandType, Context, CreateActionRow, CreateButton, CreateCommand, CreateEmbed, CreateEmbedAuthor, CreateInteractionResponse, CreateInteractionResponseMessage, InteractionContext, ReactionType, User};
use crate::discord::app::base::App;
use crate::discord::app::creators::SlashCommandHandler;
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
    }

    async fn run(&self, app: &App, ctx: Context, interaction: CommandInteraction) {
        let member = match interaction.member.as_ref() {
            Some(member) => member,
            None => {
                let embed = res(EColor::Danger, "Failed to fetch member info.".to_string());
                if let Err(err) = reply_with_embed(&ctx, &interaction, embed, false).await {
                    tracing::error!("Failed to reply /moderate command.\n{}", err);
                }
                return;
            },
        };
        
        let permissions = match member.permissions.as_ref() {
            Some(permissions) => permissions,
            None => {
                let embed = res(EColor::Danger, "Failed to fetch permissions from member.".to_string());
                if let Err(err) = reply_with_embed(&ctx, &interaction, embed, false).await {
                    tracing::error!("Failed to reply /moderate command.\n{}", err);
                }
                return;
            }
        };
        
        if !permissions.administrator() {
            let embed = res(EColor::Danger, "You don't have **ADMINISTRATOR** permission.".to_string());
            if let Err(err) = reply_with_embed(&ctx, &interaction, embed, false).await {
                tracing::error!("Failed to reply /moderate command.\n{}", err);
            }
            return;
        }
        
        let (embed, components) = main_components(&member.user);
        let message = CreateInteractionResponseMessage::new()
            .add_embed(embed)
            .components(components)
            .ephemeral(true);
        let result = ctx.http.create_interaction_response(
            interaction.id,
            &interaction.token,
            &CreateInteractionResponse::Message(message),
            vec![]
        ).await;
        if let Err(err) = result {
            tracing::error!("Failed to reply /moderate command.\n{}", err);
            return;
        }

        if let Some(callback) = app.responder_handlers.get("moderate") {
            callback.run(&ctx, &interaction).await;
        }
    }
}

pub fn main_components(user: &User) -> (CreateEmbed, Vec<CreateActionRow>) {
    let mut author = CreateEmbedAuthor::new(user.global_name.as_ref().unwrap_or(&user.name));
    if let Some(avatar_url) = user.avatar_url().as_ref() {
        author = author.icon_url(avatar_url);
    }
    let embed = CreateEmbed::new()
        .color(EColor::Royal as u32)
        .author(author)
        .title("**Officer Cui's panel**")
        .thumbnail("https://raw.githubusercontent.com/Cesio137/Bangboo/refs/heads/master/assets/avatar/Officer.png")
        .description("🖱️ ***Select an action!***");

    let btn_timeout = CreateButton::new("moderate/btn-timeout")
        .emoji(ReactionType::Unicode("⏰".to_string()))
        .label("Timeout")
        .style(ButtonStyle::Secondary);

    let btn_kick = CreateButton::new("moderate/btn-kick")
        .emoji(ReactionType::Unicode("👋".to_string()))
        .label("Kick")
        .style(ButtonStyle::Secondary);

    let btn_ban = CreateButton::new("moderate/btn-ban")
        .emoji(ReactionType::Unicode("🛡️".to_string()))
        .label("Ban")
        .style(ButtonStyle::Secondary);

    let action_row = CreateActionRow::Buttons(vec![btn_timeout, btn_kick, btn_ban]);

    let btn_close = CreateButton::new("moderate/btn-close")
        .label("Close")
        .style(ButtonStyle::Primary);

    let close_row = CreateActionRow::Buttons(vec![btn_close]);

    (embed, vec![action_row, close_row])
}