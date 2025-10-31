use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::env;
use serde::Deserialize;
use crate::discord::error;

#[derive(Deserialize)]
pub struct EnvSchema {
    pub BOT_TOKEN: String,
    pub DISCLOUD_TOKEN: String,
    pub GEMINI_API_KEY: String,
}

pub static ENV: Lazy<EnvSchema> = Lazy::new(|| {
    _ = dotenvy::dotenv().expect("Failed to load .env file");
    let env_vars = env::vars().collect::<HashMap<String, String>>();

    let env: EnvSchema = match serde_json::to_string(&env_vars) {
        Ok(env_str) => match serde_json::from_str(&env_str) {
            Ok(schema) => schema,
            Err(err) => {
                error(&format!("Failed to parse environment variables\n└{}", err));
                panic!();
            }
        },
        Err(err) => {
            error(&format!("Failed to parse environment variables\n└{}", err));
            panic!();
        }
    };

    env
});
