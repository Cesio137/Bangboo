use crate::settings::global::EColor;
use serenity::all::{ButtonStyle, CreateActionRow, CreateButton, CreateEmbed, CreateEmbedAuthor, CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption, InteractionResponseFlags, User, UserId};

pub fn timeout_menu(user: &User, ids: &[UserId], duration: &str) -> CreateInteractionResponseMessage {
    let embed = CreateEmbed::new()
        .color(EColor::Royal as u32)
        .author(
            CreateEmbedAuthor::new(user.global_name.as_ref().unwrap_or(&user.name))
                .icon_url(user.avatar_url().as_ref().unwrap_or(&"https://raw.githubusercontent.com/Cesio137/Bangboo/refs/heads/master/assets/member/default_avatar.png".to_string()))
        )
        .title("**Officer Cui's panel**")
        .thumbnail("https://raw.githubusercontent.com/Cesio137/Bangboo/refs/heads/master/assets/avatar/Officer.png")
        .description("🖱️ ***Select user(s) and timeout duration!***");

    let select_user_row = CreateActionRow::SelectMenu(
        CreateSelectMenu::new(
            "moderate/select-user",
            CreateSelectMenuKind::User {
                default_users: Some(ids.to_vec()),
            }
        )
        .max_values(25)
        .min_values(0)
        .placeholder("Select user(s)")
    );

    let select_duration_row = CreateActionRow::SelectMenu(
        CreateSelectMenu::new(
            "moderate/select-duration",
            CreateSelectMenuKind::String {
                options: vec![
                    CreateSelectMenuOption::new("60 seconds", "60")
                        .default_selection(if duration == "60" { true } else { false }),
                    CreateSelectMenuOption::new("05 minutes", "300")
                        .default_selection(if duration == "300" { true } else { false }),
                    CreateSelectMenuOption::new("10 minutes", "600")
                        .default_selection(if duration == "600" { true } else { false }),
                    CreateSelectMenuOption::new("01 hour", "3600")
                        .default_selection(if duration == "3600" { true } else { false }),
                    CreateSelectMenuOption::new("01 weak", "604800")
                        .default_selection(if duration == "604800" { true } else { false }),
                ],
    
            }
        )
        .min_values(1)
        .placeholder("Select duration")
    );

    let confirm_row = CreateActionRow::Buttons(vec![
        CreateButton::new("moderate/btn-confirm")
            .label("Confirm")
            .style(ButtonStyle::Success)
            .disabled(ids.is_empty() || duration.is_empty()),

        CreateButton::new("moderate/btn-cancel")
            .label("Cancel")
            .style(ButtonStyle::Danger)
    ]);

    CreateInteractionResponseMessage::new()
        .flags(InteractionResponseFlags::EPHEMERAL)
        .add_embed(embed)
        .components(vec![select_user_row, select_duration_row, confirm_row])
}