use crate::discord::app::context::AppContext;
use crate::settings::global::EColor;
use crate::utils::{
    embeds::*,
    global::*,
};
use anyhow::{anyhow, Result};
use std::sync::Arc;
use twilight_model::{
    gateway::{event::EventType, payload::incoming::*},
    http::attachment::Attachment
};

pub async fn run(banadd: BanAdd, context: Arc<AppContext>) -> Result<()> {
    if banadd.user.bot { return Ok(()); }

    let channel_id = context.client.guild(banadd.guild_id).await?;
    let guild = channel_id.model().await?;
    let system_channel_id = match guild.system_channel_id {
        Some(id) => id,
        None => return Err(anyhow!("Error getting system message channel.")),
    };

    if let Ok(canvas) = global_message(EventType::BanAdd, &banadd.user, None).await {
        context.client
            .create_message(system_channel_id)
            .attachments(&vec![Attachment::from_bytes(
                "welcome.png".to_string(),
                canvas,
                1,
            )])
            .await?;

        return Ok(());
    };

    let name = &banadd.user.name;
    let message = format!("{} join the server!", name);
    let embed_res = res(EColor::Success, message);
    context.client
        .create_message(system_channel_id)
        .embeds(&[embed_res])
        .await?;
    Ok(())
}