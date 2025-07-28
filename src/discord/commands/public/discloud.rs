use crate::data::emojis::EStatic;
use crate::data::settings::EColors;
use crate::discord::app::base::App;
use crate::discord::app::creators::SlashCommandHandler;
use crate::menus::components::discloud::discloud_component;
use crate::settings::logger::error;
use crate::tools::discloud::DISCLOUD;
use crate::utils::interaction::{followup, followup_with_embed, ReplyPayload};
use async_trait::async_trait;
use serenity::all::{
    CacheHttp, CommandInteraction, CommandOptionType, CommandType, Context, CreateCommand,
    CreateCommandOption, EmojiId, InteractionContext, MessageFlags,
};

pub struct Discloud;

#[async_trait]
impl SlashCommandHandler for Discloud {
    fn command(&self) -> CreateCommand<'static> {
        CreateCommand::new("discloud")
            .description("Products on fab marketplace")
            .kind(CommandType::ChatInput)
            .add_context(InteractionContext::Guild)
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "fetch",
                    "Select info to fetch",
                )
                .required(true)
                .add_string_choice("status", "status")
                .add_string_choice("logs", "logs"),
            )
    }

    async fn run(&self, app: &App, ctx: &Context, interaction: &CommandInteraction) {
        let option = interaction.data.options[0].value.as_str().unwrap();
        _ = interaction.defer_ephemeral(ctx.http()).await;
        match option {
            "status" => status(ctx, interaction).await,
            //"logs" => ,
            _ => {}
        }
    }
}

async fn status(ctx: &Context, interaction: &CommandInteraction) {
    let app = match DISCLOUD.get_all_apps().await {
        Ok(apps) => apps[0].clone(),
        Err(err) => {
            followup_with_embed(
                ctx,
                interaction,
                MessageFlags::EPHEMERAL,
                EColors::danger,
                "Failed to fetch app.",
            )
            .await;
            error(&format!("Failed to fetch app.\n└ {:?}", err));
            return;
        }
    };

    let status = match app.get_status(&DISCLOUD).await {
        Ok(status) => status,
        Err(err) => {
            followup_with_embed(
                ctx,
                interaction,
                MessageFlags::EPHEMERAL,
                EColors::danger,
                "Failed to fetch app status.",
            )
            .await;
            error(&format!("Failed to fetch app status.\n└ {:?}", err));
            return;
        }
    };
    let mut infos = Vec::new();
    infos.push(format!(
        "{}`Nome(ID):` **{}({})**",
        EmojiId::from(EStatic::icons_id as u64).to_string(),
        app.name,
        app.id
    ));
    infos.push(format!(
        "{}`CPU:` **{}**",
        EmojiId::from(EStatic::cpu as u64).to_string(),
        status.cpu
    ));
    infos.push(format!(
        "{}`RAM:` **{}**",
        EmojiId::from(EStatic::ram as u64).to_string(),
        status.memory
    ));
    infos.push(format!(
        "{}`Network:`  `⬆`**{}** `⬇`**{}**",
        EmojiId::from(EStatic::wifi as u64).to_string(),
        status.net_io.up,
        status.net_io.down
    ));
    infos.push(format!(
        "{}`Latest restart:` **<t:{}:R>**",
        EmojiId::from(EStatic::refresh as u64).to_string(),
        status.last_restart
    ));

    let component = discloud_component(infos);
    let payload = ReplyPayload {
        components: Some(vec![component]),
        ..ReplyPayload::default()
    };
    followup(ctx, interaction, MessageFlags::empty(), &payload).await;
}
