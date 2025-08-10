use crate::discord::*;
use crate::menus::*;
use crate::tools::*;
use crate::utils::*;
use async_trait::async_trait;
use serenity::all::{
    CacheHttp, CommandInteraction, CommandOptionType, CommandType, Context, CreateCommand,
    CreateCommandOption, EmojiId, InteractionContext, MessageFlags, Timestamp,
};
use crate::data::{str_hex_to_u32, str_to_u64, CONSTANTS, EMOJIS};

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
            "logs" => logs(ctx, interaction).await,
            _ => {}
        }
    }
}

async fn status(ctx: &Context, interaction: &CommandInteraction) {
    let app = match DISCLOUD.get_app(APPID).await {
        Ok(apps) => apps.clone(),
        Err(err) => {
            followup_with_embed(
                ctx,
                interaction,
                MessageFlags::EPHEMERAL,
                str_hex_to_u32(&CONSTANTS.colors.danger),
                "Failed to fetch base.",
            )
            .await;
            error(&format!("Failed to fetch base.\n└ {:?}", err));
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
                str_hex_to_u32(&CONSTANTS.colors.danger),
                "Failed to fetch base status.",
            )
            .await;
            error(&format!("Failed to fetch base status.\n└ {:?}", err));
            return;
        }
    };
    let mut infos = Vec::new();
    infos.push(format!(
        "<:id:{}>`Nome(ID):` **{}({})**",
        &EMOJIS.emojis_static.id,
        app.name,
        app.id
    ));
    infos.push(format!(
        "<:cpu:{}>`CPU:` **{}**",
        &EMOJIS.emojis_static.cpu,
        status.cpu
    ));
    infos.push(format!(
        "<:ram:{}>`RAM:` **{}**",
        &EMOJIS.emojis_static.ram,
        status.memory
    ));
    infos.push(format!(
        "<:wifi:{}>`Network:`  `⬆`**{}** `⬇`**{}**",
        &EMOJIS.emojis_static.wifi,
        status.net_io.up,
        status.net_io.down
    ));
    infos.push(format!(
        "<:refresh:{}>`Latest restart:` **<t:{}:R>**",
        &EMOJIS.emojis_static.refresh,
        Timestamp::parse(&status.started_at)
            .unwrap_or_default()
            .timestamp()
    ));

    let component = status_component(infos);
    let payload = ReplyPayload {
        components: Some(vec![component]),
        ..ReplyPayload::default()
    };
    followup(
        ctx,
        interaction,
        MessageFlags::IS_COMPONENTS_V2 | MessageFlags::EPHEMERAL,
        &payload,
    )
    .await;
}

async fn logs(ctx: &Context, interaction: &CommandInteraction) {
    let app = match DISCLOUD.get_app(APPID).await {
        Ok(apps) => apps.clone(),
        Err(err) => {
            followup_with_embed(
                ctx,
                interaction,
                MessageFlags::EPHEMERAL,
                str_hex_to_u32(&CONSTANTS.colors.danger),
                "Failed to fetch base.",
            )
            .await;
            error(&format!("Failed to fetch base.\n└ {:?}", err));
            return;
        }
    };

    let app_logs = match app.get_logs(&DISCLOUD).await {
        Ok(status) => status,
        Err(err) => {
            followup_with_embed(
                ctx,
                interaction,
                MessageFlags::EPHEMERAL,
                str_hex_to_u32(&CONSTANTS.colors.danger),
                "Failed to fetch base logs.",
            )
            .await;
            error(&format!("Failed to fetch base logs.\n└ {:?}", err));
            return;
        }
    };

    let mut logs = app_logs.terminal.small.unwrap_or("".to_string());
    logs = ASCII_REGEX.replace_all(&logs, "").to_string();

    let component = logs_component(&logs);
    let payload = ReplyPayload {
        components: Some(vec![component]),
        ..ReplyPayload::default()
    };
    followup(
        ctx,
        interaction,
        MessageFlags::IS_COMPONENTS_V2 | MessageFlags::EPHEMERAL,
        &payload,
    )
    .await;
}
