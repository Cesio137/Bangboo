use crate::settings::global::EColor;
use serenity::all::CreateEmbed;

pub fn res(color: EColor, content: &str) -> CreateEmbed {
    CreateEmbed::new()
        .color(color as u32)
        .description(content)
}