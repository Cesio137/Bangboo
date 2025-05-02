use once_cell::sync::Lazy;
use std::env;

pub struct EnvSchema {
    pub bot_token: String,
}

impl EnvSchema {
    pub fn new() -> Self {
        dotenvy::dotenv().ok(); // Carrega o .env, mas não falha se não for encontrado
        Self {
            bot_token: env::var("BOT_TOKEN").expect("BOT_TOKEN is required in .env"),
        }
    }
}

pub static ENV_SCHEMA: Lazy<EnvSchema> = Lazy::new(EnvSchema::new);