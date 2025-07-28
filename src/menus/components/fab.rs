use crate::data::emojis::EStatic;
use crate::data::settings::EColors;
use serenity::all::{
    Colour, CreateActionRow, CreateButton, CreateComponent, CreateContainer,
    CreateSeparator, CreateTextDisplay, EmojiId, ReactionType, Spacing,
};
use std::borrow::Cow;
use crate::data::fab::EProduct;

pub fn fab_component<'a>() -> CreateComponent<'a> {
    let eus = EProduct::engine_user_setings.info();
    let ip = EProduct::internet_protocol.info();

    let accent_color = Colour::new(EColors::green as u32);
    let text_display = CreateTextDisplay::new("**### CHECK OUT MY FAB PRODUCTS**");
    let separator = CreateSeparator::new(true).spacing(Spacing::Small);

    let eus_row = CreateActionRow::Buttons(Cow::Owned(vec![
        CreateButton::new_link(eus.product_url)
            .label("Engine User Settings")
            .emoji(ReactionType::Custom {
                id: EmojiId::from(EStatic::icons_f as u64),
                name: None,
                animated: false,
            }),
        CreateButton::new_link(eus.doc_url)
            .label("Documentation")
            .emoji(ReactionType::Custom {
                id: EmojiId::from(EStatic::icons_d as u64),
                name: None,
                animated: false,
            }),
    ]));
    let eus_desc = CreateTextDisplay::new(eus.product_desc);

    let ip_row = CreateActionRow::Buttons(Cow::Owned(vec![
        CreateButton::new_link(ip.product_url)
            .label("Internet Protocol")
            .emoji(ReactionType::Custom {
                id: EmojiId::from(EStatic::icons_f as u64),
                name: None,
                animated: false,
            }),
        CreateButton::new_link(ip.doc_url)
            .label("Documentation")
            .emoji(ReactionType::Custom {
                id: EmojiId::from(EStatic::icons_d as u64),
                name: None,
                animated: false,
            }),
    ]));
    let ip_desc = CreateTextDisplay::new(ip.product_desc);

    CreateComponent::Container(
        CreateContainer::new(vec![
            CreateComponent::TextDisplay(text_display),
            CreateComponent::Separator(separator.clone()),

            CreateComponent::ActionRow(eus_row),
            CreateComponent::TextDisplay(eus_desc),
            CreateComponent::Separator(separator),

            CreateComponent::ActionRow(ip_row),
            CreateComponent::TextDisplay(ip_desc),
        ]).accent_color(accent_color)
    )
}
