use crate::helpers::*;
use serenity::all::{
    ButtonStyle, CreateActionRow, CreateButton, CreateComponent, CreateEmbed, CreateEmbedAuthor,
    CreateSelectMenu, CreateSelectMenuKind, User, UserId,
};
use std::borrow::Cow;

pub fn kick_menu<'a>(
    user: &'a User,
    ids: &'a [UserId],
) -> (CreateEmbed<'a>, Vec<CreateComponent<'a>>) {
    let embed_author = CreateEmbedAuthor::new(user.global_name.as_ref().unwrap_or(&user.name))
        .icon_url(user.avatar_url().unwrap_or(user.default_avatar_url()));

    let embed = CreateEmbed::new()
        .color(str_hex_to_u32(&CONSTANTS.colors.royal))
        .author(embed_author)
        .title("**Officer Cui's panel**")
        .thumbnail("https://raw.githubusercontent.com/Cesio137/Bangboo/refs/heads/rust/assets/avatar/Officer.png")
        .description("🖱️ **Select user(s) to kick!**");

    let user_row = CreateActionRow::SelectMenu(
        CreateSelectMenu::new(
            "mod/select-user",
            CreateSelectMenuKind::User {
                default_users: Some(Cow::Borrowed(ids)),
            },
        )
        .max_values(25)
        .min_values(0)
        .placeholder("Select user(s)"),
    );

    let confirm_row = CreateActionRow::Buttons(Cow::Owned(vec![
        CreateButton::new("mod/btn-cancel")
            .label("Cancel")
            .style(ButtonStyle::Danger),
        CreateButton::new("mod/btn-confirm")
            .label("Confirm")
            .style(ButtonStyle::Success)
            .disabled(ids.is_empty()),
    ]));

    let components = vec![
        CreateComponent::ActionRow(user_row),
        CreateComponent::ActionRow(confirm_row),
    ];

    (embed, components)
}
