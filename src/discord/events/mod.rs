mod message_create;
mod interaction_create;
mod member_added;
mod member_removed;
mod ban_added;

#[cfg(target_env = "gnu")]
use crate::utils::malloc::*;

use super::app::base::App;
use colored::Colorize;
use serenity::all::{async_trait, Context, EventHandler, GuildId, Interaction, Member, Message, Ready, User};
use serenity::model::application;

#[async_trait]
impl EventHandler for App {
    async fn ready(&self, ctx: Context, data: Ready) {
        let mut len = self.commands.len();
        match application::Command::set_global_commands(&ctx.http, self.commands.clone()).await {
            Ok(commands) => {
                for command in commands {
                    println!(
                        "{} {} {} {}",
                        "✔".bright_green(),
                        "[/]".green(),
                        command.name.bright_cyan(),
                        "command loaded!".bright_green()
                    );
                }
            }
            Err(_) => {
                len = 0;
                println!(
                    "{} {}",
                    "✖".bright_red(),
                    "Failed to register global commands!".red(),
                );
            },
        };
        let name = &data.user.name;
        println!("\n{} {}", "➡ Online as".green(), name.bright_green());
        if len > 0 {
            println!("{} {} {}", "⤿".bright_green(), len.to_string().green(), "command(s) successfully registered globally!".green());
        }

        #[cfg(target_env = "gnu")]
        malloc::trim();
    }

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        if !new_member.user.bot {
            member_added::run(self, ctx, new_member).await;
        }
        #[cfg(target_env = "gnu")]
        malloc::trim();
    }

    async fn guild_member_removal(&self, ctx: Context, guild_id: GuildId, user: User, member_data_if_available: Option<Member>) {
        if !user.bot {
            member_removed::run(self, ctx, guild_id, user).await;
        }
        #[cfg(target_env = "gnu")]
        malloc::trim();
    }

    async fn guild_ban_addition(&self, ctx: Context, guild_id: GuildId, banned_user: User) {
        if !banned_user.bot {
            ban_added::run(self, ctx, guild_id, banned_user).await;
        }
        #[cfg(target_env = "gnu")]
        malloc::trim();
    }
    
    async fn message(&self, ctx: Context, new_message: Message) {
        if !new_message.author.bot || new_message.guild_id.is_some() {
            message_create::run(self, ctx, new_message).await;
        }
        #[cfg(target_env = "gnu")]
        malloc::trim();
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        interaction_create::run(self, ctx, interaction).await;
        #[cfg(target_env = "gnu")]
        malloc::trim();
    }
}