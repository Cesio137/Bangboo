use twilight_model::channel::message::Embed;
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