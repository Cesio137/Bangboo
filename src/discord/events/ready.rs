use colored::Colorize;
use crate::discord::*;
use serenity::all::{ActivityData, Context, Ready};
use serenity::model::application::Command;

pub async fn run(app: &App, ctx: &Context, ready: &Ready) {
    log(&format!("● {} online ✓", ready.user.name.underline()).bright_green());
    let result = Command::set_global_commands(&ctx.http, &app.commands).await;
    let mut commands_len: usize = 0;
    match result {
        Ok(commands) => {
            commands_len = commands.len();
            log(&format!("└ {} command(s) successfully registered globally!",
                    commands_len
                ).bright_green(),
            );
            for command in commands {
                log(&format!("{{/}} Slash command > {} ✓",
                             command.name.bright_blue()).bright_green(),
                );
            }
        }
        Err(err) => {
            error(&format!("└ {} command(s) successfully registered globally!",
                    commands_len
                )
            );
            error(&format!("{:?}", err));
        }
    }

    ctx.set_activity(Some(ActivityData::custom(
        "Rust-powered bot.\nHosted by discloud.\nJoin in: .gg/DBNATxA6Jx",
    )));
}
