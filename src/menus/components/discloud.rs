use crate::data::settings::EColors;
use serenity::all::{
    ButtonStyle, Colour, CreateActionRow, CreateButton, CreateComponent, CreateContainer,
    CreateSeparator, CreateTextDisplay, Spacing,
};
use std::borrow::Cow;

pub fn discloud_component<'a>(infos: Vec<String>) -> CreateComponent<'a> {
    let accent_color = Colour::new(EColors::green as u32);
    let text_display = CreateTextDisplay::new("### BANGBOO'S STATUS");
    let separator = CreateSeparator::new(true).spacing(Spacing::Large);
    let info_display = CreateTextDisplay::new(infos.join("\n"));
    let refresh_row = CreateActionRow::Buttons(Cow::Owned(vec![
        CreateButton::new("discloud/status/refresh")
            .label("Refresh")
            .style(ButtonStyle::Success),
    ]));

    CreateComponent::Container(
        CreateContainer::new(vec![
            CreateComponent::TextDisplay(text_display),
            CreateComponent::Separator(separator.clone()),
            CreateComponent::TextDisplay(info_display),
            CreateComponent::Separator(separator),
            CreateComponent::ActionRow(refresh_row),
        ])
        .accent_color(accent_color),
    )
}
