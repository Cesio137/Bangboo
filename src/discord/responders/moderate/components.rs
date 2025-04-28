use serenity::all::{ButtonStyle, CreateActionRow, CreateButton, CreateEmbed, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption, ReactionType, UserId};

pub fn close_panel(embed: CreateEmbed, timeout: bool) -> (CreateEmbed, Vec<CreateActionRow>) {
    let embed = embed.description(if timeout { "⏰ ***Timeout!***" } else { "👋 ***Bye!***" });
    (embed, vec![])
}

pub fn main_components(embed: CreateEmbed) -> (CreateEmbed, Vec<CreateActionRow>) {
    let embed = embed.description("🖱️ ***Select an action!***");

    let btn_timeout = CreateButton::new("moderate/btn-timeout")
        .emoji(ReactionType::Unicode("⏰".to_string()))
        .label("Timeout")
        .style(ButtonStyle::Secondary);

    let btn_kick = CreateButton::new("moderate/btn-kick")
        .emoji(ReactionType::Unicode("👋".to_string()))
        .label("Kick")
        .style(ButtonStyle::Secondary);

    let btn_ban = CreateButton::new("moderate/btn-ban")
        .emoji(ReactionType::Unicode("🛡️".to_string()))
        .label("Ban")
        .style(ButtonStyle::Secondary);

    let action_row = CreateActionRow::Buttons(vec![btn_timeout, btn_kick, btn_ban]);

    let btn_close = CreateButton::new("moderate/btn-close")
        .label("Close")
        .style(ButtonStyle::Primary);

    let close_row = CreateActionRow::Buttons(vec![btn_close]);

    (embed, vec![action_row, close_row])
}

pub fn timeout_panel(embed: CreateEmbed, user_ids: &[UserId], duration: &str) -> (CreateEmbed, Vec<CreateActionRow>) {
    let embed = embed.description("🖱️ ***Select user(s) and timeout duration!***");

    let select_user = CreateSelectMenu::new(
        "moderate/timeout-select-user",
        CreateSelectMenuKind::User {
            default_users: Some(user_ids.to_vec()),
        }
    )
        .max_values(25)
        .min_values(1)
        .placeholder("Select user(s)");

    let select_user_row = CreateActionRow::SelectMenu(select_user);

    let select_user = CreateSelectMenu::new(
        "moderate/timeout-select-duration",
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
        .placeholder("Select duration");

    let select_duration_row = CreateActionRow::SelectMenu(select_user);

    let btn_confirm = CreateButton::new("moderate/btn-confirm")
        .label("Confirm")
        .style(ButtonStyle::Success)
        .disabled(user_ids.is_empty() || duration.is_empty());
    let btn_cancel = CreateButton::new("moderate/btn-cancel")
        .label("Cancel")
        .style(ButtonStyle::Danger);

    let confirm_row = CreateActionRow::Buttons(vec![btn_cancel, btn_confirm]);
    (embed, vec![select_user_row, select_duration_row, confirm_row])
}

pub fn kick_panel(embed: CreateEmbed, user_ids: &[UserId]) -> (CreateEmbed, Vec<CreateActionRow>) {
    let embed = embed.description("🖱️ ***Select user(s) to kick!***");

    let select_user = CreateSelectMenu::new(
        "moderate/kick-select-user",
        CreateSelectMenuKind::User {
            default_users: Some(user_ids.to_vec()),
        }
    )
        .max_values(25)
        .min_values(1)
        .placeholder("Select user(s)");

    let select_user_row = CreateActionRow::SelectMenu(select_user);

    let btn_confirm = CreateButton::new("moderate/btn-confirm")
        .label("Confirm")
        .style(ButtonStyle::Success)
        .disabled(user_ids.is_empty());
    let btn_cancel = CreateButton::new("moderate/btn-cancel")
        .label("Cancel")
        .style(ButtonStyle::Danger);

    let confirm_row = CreateActionRow::Buttons(vec![btn_cancel, btn_confirm]);
    (embed, vec![select_user_row, confirm_row])
}

pub fn ban_panel(embed: CreateEmbed, user_ids: &[UserId]) -> (CreateEmbed, Vec<CreateActionRow>) {
    let embed = embed.description("🖱️ ***Select user(s) to ban!***");
    let select_user = CreateSelectMenu::new(
        "moderate/ban-select-user",
        CreateSelectMenuKind::User {
            default_users: Some(user_ids.to_vec()),
        }
    )
        .max_values(25)
        .min_values(1)
        .placeholder("Select user(s)");

    let select_user_row = CreateActionRow::SelectMenu(select_user);

    let btn_confirm = CreateButton::new("moderate/btn-confirm")
        .label("Confirm")
        .style(ButtonStyle::Success)
        .disabled(user_ids.is_empty());
    let btn_cancel = CreateButton::new("moderate/btn-cancel")
        .label("Cancel")
        .style(ButtonStyle::Danger);

    let confirm_row = CreateActionRow::Buttons(vec![btn_cancel, btn_confirm]);
    (embed, vec![select_user_row, confirm_row])
}