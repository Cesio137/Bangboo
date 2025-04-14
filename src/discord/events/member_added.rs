use crate::discord::app::context::AppContext;
use crate::settings::global::EColor;
use crate::utils::{global::global_message, embeds::res};
use anyhow::{anyhow, Result};
use std::sync::Arc;
use twilight_model::{
    gateway::{event::EventType, payload::incoming::MemberAdd},
    http::attachment::Attachment,
};

pub async fn run(member: Box<MemberAdd>, context: Arc<AppContext>) -> Result<()> {
    if member.user.bot { return Ok(()); }

    let channel_id = context.client.guild(member.guild_id).await?;
    let guild = channel_id.model().await?;
    let system_channel_id = match guild.system_channel_id {
        Some(id) => id,
        None => return Err(anyhow!("Error getting system message channel.")),
    };

    if let Ok(canvas) = global_message(EventType::MemberAdd, &member.user, member.joined_at).await {
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

    let name = &member.user.name;
    let message = format!("{} join the server!", name);
    let embed_res = res(EColor::Success, message);
    context.client
        .create_message(system_channel_id)
        .embeds(&[embed_res])
        .await?;

    Ok(())
}
