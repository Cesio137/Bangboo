use crate::discord::*;
use crate::data::*;
use crate::menus::*;
use crate::tools::*;
use crate::utils::*;
use async_trait::async_trait;
use serenity::all::{
    CacheHttp, Colour, ComponentInteraction, Context, CreateEmbed, MessageFlags, Timestamp,
};


pub struct Status;

#[async_trait]
impl ResponderHandler for Status {
    fn custom_id(&self) -> String {
        String::from("discloud/status/refresh")
    }

    async fn run(&self, ctx: &Context, interaction: &ComponentInteraction) {
        _ = interaction.defer(ctx.http()).await;

        let app = match DISCLOUD.get_app(&APPID).await {
            Ok(apps) => apps.clone(),
            Err(err) => {
                let embed = CreateEmbed::new()
                    .color(Colour::new(str_hex_to_u32(&CONSTANTS.colors.danger)))
                    .description("Failed to fetch base.");
                let payload = ReplyPayload {
                    embeds: Some(vec![embed]),
                    components: Some(vec![]),
                    ..Default::default()
                };
                update_component(ctx, interaction, MessageFlags::EPHEMERAL, &payload).await;
                error(&format!("Failed to fetch base.\n└ {:?}", err));
                return;
            }
        };

        let status = match app.get_status(&DISCLOUD).await {
            Ok(status) => status,
            Err(err) => {
                let embed = CreateEmbed::new()
                    .color(Colour::new(str_hex_to_u32(&CONSTANTS.colors.danger)))
                    .description("Failed to fetch base status.");
                let payload = ReplyPayload {
                    embeds: Some(vec![embed]),
                    components: Some(vec![]),
                    ..Default::default()
                };
                edit_component(ctx, interaction, MessageFlags::EPHEMERAL, &payload).await;
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
        if let Ok(timestamp) = Timestamp::parse(&status.started_at) {
            infos.push(format!(
                "<:refresh:{}>`Latest restart:` **<t:{}:R>**",
                &EMOJIS.emojis_static.refresh,
                timestamp.timestamp()
            ));
        } else {
            infos.push(format!(
                "<:refresh:{}>`Latest restart:` **{}**",
                &EMOJIS.emojis_static.refresh,
                &status.last_restart
            ));
        }

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

        let app_logs = match DISCLOUD.get_app_logs(&APPID).await {
            Ok(apps) => apps.clone(),
            Err(err) => {
                let embed = CreateEmbed::new()
                    .color(Colour::new(str_hex_to_u32(&CONSTANTS.colors.danger)))
                    .description("Failed to fetch base logs.");
                let payload = ReplyPayload {
                    embeds: Some(vec![embed]),
                    components: Some(vec![]),
                    ..Default::default()
                };
                edit_component(ctx, interaction, MessageFlags::EPHEMERAL, &payload).await;
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
        edit_component(
            ctx,
            interaction,
            MessageFlags::IS_COMPONENTS_V2 | MessageFlags::EPHEMERAL,
            &payload,
        )
        .await;
    }
}
