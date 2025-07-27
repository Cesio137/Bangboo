pub mod base;
pub mod creators;

use crate::discord::app::base::App;
use crate::discord::app::creators::{PrefixCommandHandler, ResponderHandler, SlashCommandHandler};
use crate::settings::env::ENV_SCHEMA;
use crate::settings::logger::error;
use serenity::all::{GatewayIntents, Token};
use serenity::Client;
use std::str::FromStr;

pub async fn bootstrap(intents: GatewayIntents) {
    let app = App::new();

    let token = match Token::from_str(&ENV_SCHEMA.bot_token) {
        Ok(token) => token,
        Err(err) => {
            error(&format!("Token error.\n{:?}", err));
            return;
        }
    };
    let mut client = match Client::builder(token, intents).event_handler(app).await {
        Ok(client) => client,
        Err(err) => {
            error(&format!(
                "Error when trying to create gateway client.\n{:?}",
                err
            ));
            return;
        }
    };

    if let Err(err) = client.start_autosharded().await {
        error(&format!("Client error.\n{:?}", err));
    }
}
