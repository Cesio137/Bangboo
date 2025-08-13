use once_cell::sync::Lazy;
use std::env;

pub struct Env {
    pub BOT_TOKEN: String,
    pub DISCLOUD_TOKEN: String,
    pub GEMINI_API_KEY: String,
}

pub static ENV: Lazy<Env> = Lazy::new(|| {
    _ = dotenvy::dotenv().expect("Failed to load .env file");

    Env {
        BOT_TOKEN: env::var("BOT_TOKEN").expect("Discord Bot Token is required"),
        DISCLOUD_TOKEN: env::var("DISCLOUD_TOKEN").expect("Discloud token is required"),
        GEMINI_API_KEY: env::var("GEMINI_API_KEY").expect("Gemini API key is required"),
    }
});
