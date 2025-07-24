use once_cell::sync::Lazy;
use std::env;

pub struct EnvSchema {
    pub bot_token: String,
    pub discloud_token: String,
    pub gemini_api_key: String,
}

impl EnvSchema {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();
        Self {
            bot_token: env::var("BOT_TOKEN").expect("Discord Bot Token is required"),
            discloud_token: env::var("DISCLOUD_TOKEN").expect("Discloud token is required"),
            gemini_api_key: env::var("GEMINI_API_KEY").expect("Gemini API key is required"),
        }
    }
}

pub static ENV_SCHEMA: Lazy<EnvSchema> = Lazy::new(EnvSchema::new);
