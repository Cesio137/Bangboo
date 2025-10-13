use crate::constants::*;
use serenity::all::{
    Colour, CreateActionRow, CreateButton, CreateComponent, CreateContainer, CreateSeparator,
    CreateTextDisplay, EmojiId, ReactionType, Spacing,
};
use std::borrow::Cow;

pub fn fab_component<'a>() -> CreateComponent<'a> {
    let accent_color = Colour::new(COLORS.green);
    let text_display = CreateTextDisplay::new("**### CHECK OUT MY FAB PRODUCTS**");
    let separator = CreateSeparator::new(true).spacing(Spacing::Small);
    let eus_row = CreateActionRow::Buttons(Cow::Owned(vec![
        CreateButton::new_link(&FAB.engine_user_setings.product_url)
            .label("Engine User Settings")
            .emoji(ReactionType::Custom {
                id: EmojiId::from(EMOJIS.emojis_static.icons_f),
                name: None,
                animated: false,
            }),
        CreateButton::new_link(&FAB.engine_user_setings.doc_url)
            .label("Documentation")
            .emoji(ReactionType::Custom {
                id: EmojiId::from(EMOJIS.emojis_static.icons_d),
                name: None,
                animated: false,
            }),
    ]));
    let eus_desc = CreateTextDisplay::new(&FAB.engine_user_setings.product_desc);

    let ip_row = CreateActionRow::Buttons(Cow::Owned(vec![
        CreateButton::new_link(&FAB.internet_protocol.product_url)
            .label("Internet Protocol")
            .emoji(ReactionType::Custom {
                id: EmojiId::from(EMOJIS.emojis_static.icons_f),
                name: None,
                animated: false,
            }),
        CreateButton::new_link(&FAB.internet_protocol.doc_url)
            .label("Documentation")
            .emoji(ReactionType::Custom {
                id: EmojiId::from(EMOJIS.emojis_static.icons_d),
                name: None,
                animated: false,
            }),
    ]));
    let ip_desc = CreateTextDisplay::new(&FAB.internet_protocol.product_desc);

    CreateComponent::Container(
        CreateContainer::new(vec![
            CreateComponent::TextDisplay(text_display),
            CreateComponent::Separator(separator.clone()),
            CreateComponent::ActionRow(eus_row),
            CreateComponent::TextDisplay(eus_desc),
            CreateComponent::Separator(separator),
            CreateComponent::ActionRow(ip_row),
            CreateComponent::TextDisplay(ip_desc),
        ])
        .accent_color(accent_color),
    )
}
