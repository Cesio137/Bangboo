use crate::data::settings::EColors;
use serenity::all::CreateEmbed;

pub fn res(color: EColors, content: &str) -> CreateEmbed {
    CreateEmbed::new().color(color as u32).description(content)
}
