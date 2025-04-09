use crate::discord::app::context::AppContext;
use crate::settings::global::EColor;
use crate::utils::{
    embeds::*,
    global::*,
    logger::*
};
use std::sync::Arc;
use twilight_model::{
    gateway::{event::EventType, payload::incoming::*},
    http::attachment::Attachment
};

pub async fn run(banadd: BanAdd, context: Arc<AppContext>) {
    if banadd.user.bot { return; }

    let channel_id = match context.client.guild(banadd.guild_id).await {
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

    let canvas = global_message(EventType::BanAdd, &banadd.user, None).await;
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
            error(format!("Error trying to responde BanAdd event: {:?}", err).as_str());
        }
        return;
    }

    let name = &banadd.user.name;
    let message = format!("{} join the server!", name);
    let embed_res = res(EColor::Success, message);
    let result = context.client
        .create_message(channel_id)
        .embeds(&[embed_res])
        .await;

    if let Err(err) = result {
        error(format!("Error trying to responde BanAdd event: {:?}", err).as_str());
    }
}