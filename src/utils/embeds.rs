use crate::settings::global::EColor;
use serenity::all::CreateEmbed;

pub fn res(color: EColor, content: String) -> CreateEmbed {
    CreateEmbed::new()
        .color(color as u32)
        .description(content)
}
/*
pub fn interaction_res(color: EColor, content: String, interaction_type: InteractionResponseType, ephemeral: bool) -> InteractionResponse {
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
            flags: if ephemeral { Some(MessageFlags::EPHEMERAL) } else { None },
            ..Default::default()
        }
        .into(),
    }
}
*/