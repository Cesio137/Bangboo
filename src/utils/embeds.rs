use crate::settings::global::EColor;
use twilight_model::{
    channel::message::Embed,
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType}
};
use twilight_util::builder::embed;

pub fn res(color: EColor, content: String) -> Embed {
    embed::EmbedBuilder::new()
        .color(color as u32)
        .description(content)
        .validate()
        .unwrap()
        .build()
}

pub fn interaction_res(color: EColor, content: String, interaction_type: InteractionResponseType) -> InteractionResponse {
    let embed = embed::EmbedBuilder::new()
        .color(color as u32)
        .description(content)
        .validate()
        .unwrap()
        .build();

    InteractionResponse {
        kind: interaction_type,
        data: InteractionResponseData {
            embeds: vec![embed].into(),
            ..Default::default()
        }
        .into(),
    }
}
