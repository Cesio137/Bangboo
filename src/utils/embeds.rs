use twilight_model::channel::message::Embed;
use twilight_util::builder::embed;
use crate::settings::global::Colors;

pub fn res(color: u32, content: String) -> Embed {
    embed::EmbedBuilder::new()
    .color(color)
    .description(content)
    .validate()
    .unwrap()
    .build()
}