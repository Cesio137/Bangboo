use crate::data::*;
use serenity::all::{
    Colour, CreateActionRow, CreateButton, CreateComponent, CreateContainer, CreateSeparator,
    CreateTextDisplay, EmojiId, ReactionType, Spacing,
};
use std::borrow::Cow;

pub fn social_component<'a>() -> CreateComponent<'a> {
    let accent_color = Colour::new(str_hex_to_u32(&CONSTANTS.colors.green));
    let text_display = CreateTextDisplay::new("**FOLLOW ME ON SOCIAL MEDIA**");
    let separator = CreateSeparator::new(false).spacing(Spacing::Small);

    let social_row = CreateActionRow::Buttons(Cow::Owned(vec![
        CreateButton::new_link("https://nathan-miguel.vercel.app/")
            .label("Portifolio")
            .emoji(ReactionType::Custom {
                id: EmojiId::from(str_to_u64(&EMOJIS.emojis_static.icons_p)),
                name: None,
                animated: false,
            }),
        CreateButton::new_link("https://www.youtube.com/@NathanMiguel1")
            .label("Youtube")
            .emoji(ReactionType::Custom {
                id: EmojiId::from(str_to_u64(&EMOJIS.emojis_static.icons_youtube)),
                name: None,
                animated: false,
            }),
        CreateButton::new_link("https://www.instagram.com/nathan_cmiguel/")
            .label("Instagram")
            .emoji(ReactionType::Custom {
                id: EmojiId::from(str_to_u64(&EMOJIS.emojis_static.icons_instagram)),
                name: None,
                animated: false,
            }),
        CreateButton::new_link("https://github.com/Cesio137")
            .label("GitHub")
            .emoji(ReactionType::Custom {
                id: EmojiId::from(str_to_u64(&EMOJIS.emojis_static.icons_github)),
                name: None,
                animated: false,
            }),
        CreateButton::new_link("https://x.com/NathanCmig")
            .label("X/Twitter")
            .emoji(ReactionType::Custom {
                id: EmojiId::from(str_to_u64(&EMOJIS.emojis_static.icons_x)),
                name: None,
                animated: false,
            }),
    ]));

    let fab_separator = CreateSeparator::new(true).spacing(Spacing::Large);
    let fab_text_display = CreateTextDisplay::new("**VISIT MY FAB STORE**");

    let fab_row = CreateActionRow::Buttons(Cow::Owned(vec![
        CreateButton::new_link("https://www.fab.com/sellers/Nathan%20Miguel")
            .label("Fab")
            .emoji(ReactionType::Custom {
                id: EmojiId::from(str_to_u64(&EMOJIS.emojis_static.icons_f)),
                name: None,
                animated: false,
            }),
    ]));

    CreateComponent::Container(
        CreateContainer::new(vec![
            CreateComponent::TextDisplay(text_display),
            CreateComponent::Separator(separator.clone()),
            CreateComponent::ActionRow(social_row),
            CreateComponent::Separator(fab_separator.clone()),
            CreateComponent::TextDisplay(fab_text_display),
            CreateComponent::Separator(fab_separator.divider(false).spacing(Spacing::Small)),
            CreateComponent::ActionRow(fab_row),
        ])
        .accent_color(accent_color),
    )
}
