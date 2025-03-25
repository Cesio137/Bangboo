use twilight_model::channel::message::Embed;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType};
use twilight_util::builder::embed;
use crate::settings::global::EColor;

pub fn res(color: EColor, content: String) -> Embed {
    embed::EmbedBuilder::new()
        .color(color as u32)
        .description(content)
        .validate()
        .unwrap()
        .build()
}

pub fn interaction_res(color: EColor, content: String) -> InteractionResponse {
    let embed = embed::EmbedBuilder::new()
    .color(color as u32)
    .description(content)
    .validate()
    .unwrap()
    .build();

    InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: InteractionResponseData {
            embeds: vec![embed].into(),
            ..Default::default()
        }.into(),
    }
}