use std::env::var;
use once_cell::sync::Lazy;

pub struct EnvSchema {
    pub bot_token: String,
}

impl EnvSchema {
    pub fn new() -> EnvSchema {
        dotenvy::dotenv().expect(".env file not found.");
        Self {
            bot_token: var("BOT_TOKEN").expect("Discord Bot Token is required"),
        }
    }
}

pub static ENV_SCHEMA: Lazy<EnvSchema> = Lazy::new(EnvSchema::new);