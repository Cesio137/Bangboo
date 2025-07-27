use crate::data::settings::EColors;
use serenity::all::{CreateEmbed, CreateEmbedAuthor, User};

pub fn load_menu<'a>(user: &'a User, description: &'a str) -> CreateEmbed<'a> {
    let embed_author = CreateEmbedAuthor::new(user.global_name.as_ref().unwrap_or(&user.name))
        .icon_url(user.avatar_url().unwrap_or(user.default_avatar_url()));

    CreateEmbed::new()
        .color(EColors::royal as u32)
        .author(embed_author)
        .title("**Officer Cui's panel**")
        .thumbnail("https://raw.githubusercontent.com/Cesio137/Bangboo/refs/heads/rust/assets/avatar/Officer.png")
        .description(description)
}
