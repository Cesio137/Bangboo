use crate::discord::app::context::AppContext;
use crate::settings::global::EColor;
use crate::utils::{embeds::res, global::global_message, logger::*};
use std::sync::Arc;
use twilight_model::gateway::payload::incoming::MemberAdd;
use twilight_model::{
    gateway::event::EventType,
    http::attachment::Attachment,
};

pub async fn run(member: Box<MemberAdd>, context: Arc<AppContext>) {
    if member.user.bot { return; }

    let channel_id = match context.client.guild(member.guild_id).await {
        Ok(response) => {
            match response.model().await.ok().and_then(|guild| guild.system_channel_id) {
                Some(channel_id) => channel_id,
                None => {
                    error("Error getting system message channel".to_string().as_str());
                    return
                }
            }
        },
        Err(err) => {
            error(format!("Error getting system message channel\n{:?}", err).as_str());
            return
        },
    };

    let canvas = global_message(EventType::MemberAdd, &member.user, member.joined_at).await;
    if let Ok(buffer) = canvas {
        let result = context.client
            .create_message(channel_id)
            .attachments(&vec![Attachment::from_bytes(
                "welcome.png".to_string(),
                buffer,
                1,
            )])
            .await;

        if let Err(err) = result {
            error(format!("Error trying to responde MemberAdd event: {:?}", err).as_str());
        }
        return;
    }

    let name = &member.user.name;
    let message = format!("{} join the server!", name);
    let embed_res = res(EColor::Success, message);
    let result = context.client
        .create_message(channel_id)
        .embeds(&[embed_res])
        .await;

    if let Err(err) = result {
        error(format!("Error trying to responde MemberAdd event: {:?}", err).as_str());
    }
}
