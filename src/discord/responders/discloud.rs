use crate::data::emojis::EStatic;
use crate::data::settings::EColors;
use crate::discord::app::creators::ResponderHandler;
use crate::menus::components::discloud::{logs_component, status_component};
use crate::settings::logger::error;
use crate::tools::discloud::{APPID, ASCII_REGEX, DISCLOUD};
use crate::utils::components::{edit_component, update_component};
use crate::utils::interaction::ReplyPayload;
use async_trait::async_trait;
use serenity::all::{CacheHttp, Colour, ComponentInteraction, Context, CreateEmbed, EmojiId, MessageFlags, Timestamp};

pub struct Status;

#[async_trait]
impl ResponderHandler for Status {
    fn custom_id(&self) -> String {
        String::from("discloud/status/refresh")
    }

    async fn run(&self, ctx: &Context, interaction: &ComponentInteraction) {
        _ = interaction.defer(ctx.http()).await;

        let app = match DISCLOUD.get_app(APPID).await {
            Ok(apps) => apps.clone(),
            Err(err) => {
                let embed = CreateEmbed::new()
                    .color(Colour::new(EColors::danger as u32))
                    .description("Failed to fetch app.");
                let payload = ReplyPayload {
                    embeds: Some(vec![embed]),
                    components: Some(vec![]),
                    ..Default::default()
                };
                update_component(ctx, interaction, MessageFlags::EPHEMERAL, &payload).await;
                error(&format!("Failed to fetch app.\n└ {:?}", err));
                return;
            }
        };

        let status = match app.get_status(&DISCLOUD).await {
            Ok(status) => status,
            Err(err) => {
                let embed = CreateEmbed::new()
                    .color(Colour::new(EColors::danger as u32))
                    .description("Failed to fetch app status.");
                let payload = ReplyPayload {
                    embeds: Some(vec![embed]),
                    components: Some(vec![]),
                    ..Default::default()
                };
                edit_component(ctx, interaction, MessageFlags::EPHEMERAL, &payload).await;
                error(&format!("Failed to fetch app status.\n└ {:?}", err));
                return;
            }
        };
        let mut infos = Vec::new();
        infos.push(format!(
            "<:icons_id:{}>`Nome(ID):` **{}({})**",
            EmojiId::from(EStatic::icons_id as u64),
            app.name,
            app.id
        ));
        infos.push(format!(
            "<:cpu:{}>`CPU:` **{}**",
            EmojiId::from(EStatic::cpu as u64).to_string(),
            status.cpu
        ));
        infos.push(format!(
            "<:ram:{}>`RAM:` **{}**",
            EmojiId::from(EStatic::ram as u64).to_string(),
            status.memory
        ));
        infos.push(format!(
            "<:wifi:{}>`Network:`  `⬆`**{}** `⬇`**{}**",
            EmojiId::from(EStatic::wifi as u64).to_string(),
            status.net_io.up,
            status.net_io.down
        ));
        infos.push(format!(
            "<:refresh:{}>`Latest restart:` **<t:{}:R>**",
            EmojiId::from(EStatic::refresh as u64).to_string(),
            Timestamp::parse(&status.started_at).unwrap_or_default().timestamp()
        ));

        let component = status_component(infos);
        let payload = ReplyPayload {
            components: Some(vec![component]),
            ..ReplyPayload::default()
        };

        edit_component(
            ctx,
            interaction,
            MessageFlags::IS_COMPONENTS_V2 | MessageFlags::EPHEMERAL,
            &payload,
        )
        .await;
    }
}

pub struct Logs;

#[async_trait]
impl ResponderHandler for Logs {
    fn custom_id(&self) -> String {
        String::from("discloud/logs/refresh")
    }

    async fn run(&self, ctx: &Context, interaction: &ComponentInteraction) {
        _ = interaction.defer(ctx.http()).await;

        let app_logs = match DISCLOUD.get_app_logs(APPID).await {
            Ok(apps) => apps.clone(),
            Err(err) => {
                let embed = CreateEmbed::new()
                    .color(Colour::new(EColors::danger as u32))
                    .description("Failed to fetch app logs.");
                let payload = ReplyPayload {
                    embeds: Some(vec![embed]),
                    components: Some(vec![]),
                    ..Default::default()
                };
                edit_component(ctx, interaction, MessageFlags::EPHEMERAL, &payload).await;
                error(&format!("Failed to fetch app logs.\n└ {:?}", err));
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
        edit_component(
            ctx,
            interaction,
            MessageFlags::IS_COMPONENTS_V2 | MessageFlags::EPHEMERAL,
            &payload,
        )
        .await;
    }
}
