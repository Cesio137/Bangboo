use colored::Colorize;
use once_cell::sync::Lazy;
use std::env::var;

pub struct EnvSchema {
    pub bot_token: String,
}

impl EnvSchema {
    pub fn new() -> EnvSchema {
        dotenvy::dotenv().unwrap_or_else(|_| {
            panic!(
                "{} {}",
                " FATAL ".on_red().bold(),
                ".env was not found.".bright_red()
            )
        });
        let envschema = EnvSchema {
            bot_token: var("BOT_TOKEN").unwrap_or_else(|_| {
                panic!(
                    "{} {} {}",
                    " ENV VAR ".on_red().bold(),
                    "BOT_TOKEN".bright_red().bold(),
                    "is required".bright_red()
                )
            }),
        };
        println!("{}", "✔ Env vars loaded successfully!".bright_purple());
        envschema
    }
}

pub static ENV_SCHEMA: Lazy<EnvSchema> = Lazy::new(EnvSchema::new);
