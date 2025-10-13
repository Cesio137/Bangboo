use crate::constants::*;
use serenity::all::{CreateEmbed, CreateEmbedAuthor, User};

pub fn close_menu(user: &User, timeout: bool) -> CreateEmbed {
    let embed_author = CreateEmbedAuthor::new(user.global_name.as_ref().unwrap_or(&user.name))
        .icon_url(user.avatar_url().unwrap_or(user.default_avatar_url()));

    CreateEmbed::new()
        .color(COLORS.royal)
        .author(embed_author)
        .title("**Officer Cui's panel**")
        .thumbnail("https://raw.githubusercontent.com/Cesio137/Bangboo/refs/heads/rust/assets/avatar/Officer.png")
        .description(if timeout { "⏰ **Timeout!**" } else { "👋 **Bye!**" })
}
