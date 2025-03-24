use std::string::String;
use chrono::DateTime;
use twilight_model::application::{
    command::CommandType,
    interaction::{application_command::CommandOptionValue, InteractionData}
};
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType};
use twilight_util::{
    builder::command::{CommandBuilder, UserBuilder}, 
    snowflake::Snowflake
};
use crate::{discord::app::creators::{create_slash_command, SlashCommand}, utils::embeds::res};
use crate::settings::global::COLORS;

pub fn age_command() -> SlashCommand {
    let user_option = UserBuilder::new("user", "Select user.")
        .build();

    create_slash_command(
        CommandBuilder::new("age", "Displays your or another user's account creation date.", CommandType::ChatInput)
            .option(user_option)
            .build(),
        |interaction, client| async move {
            let mut age: String = String::new();
            let error_message = String::from("Error trying to responde /age command: Can't find an user ID.");
            
            if let Some(data) = &interaction.data {
                match data {
                    InteractionData::ApplicationCommand(command_data) => {
                        match command_data.options.first() {
                            Some(option) => {
                                if option.name == "user" {
                                    if let CommandOptionValue::User(id) = &option.value {
                                        let user_id = Some(*id);
                                        match client.user(*id).await {
                                            Ok(user_response) => {
                                                let username = match user_response.model().await {
                                                    Ok(user) => user.name ,
                                                    Err(_) => "Unknow".to_string(),
                                                };
                                                let timestamp = match user_id {
                                                    Some(id) => id.timestamp(),
                                                    None => 0,
                                                };
                                                // convert timestamp to readble data
                                                match DateTime::from_timestamp_millis(timestamp as i64) {
                                                    Some(datetime) => {
                                                        // format data to a readble string
                                                        age = format!("{}'s account was created at {}.", username, datetime.format("%a, %Hh%Mmin, %d/%b/%Y").to_string());
                                                    },
                                                    None => {
                                                        age = format!("{}'s account was created at NONE.", username);
                                                    },
                                                }
                                            },
                                            Err(_) => {}
                                        };
                                        
                                    }
                                }
                            },
                            None => {},
                        }
                    }
                    _ => {}
                }
            }

            if age.is_empty() {
                match &interaction.member {
                    Some(member) => {
                        match &member.user {
                            Some(user) => {
                                let username = user.name.clone();
                                let timestamp = user.id.timestamp();
                                // convert timestamp to readble data
                                match DateTime::from_timestamp_millis(timestamp as i64) {
                                    Some(datetime) => {
                                        age = format!("{}'s account was created at {}.", username, datetime.format("%a, %Hh%Mmin, %d/%b/%Y").to_string());
                                    },
                                    None => {
                                        age = format!("{}'s account was created at NONE.", username);
                                    },
                                };
                                // format data to a readble string
                                
                            },
                            None => {},
                        }
                    },
                    None => {},
                }
            }

            let color = if !age.is_empty() { COLORS.green } else { COLORS.danger };
            let embed = res(color, age);
            
            let response = InteractionResponse {
                kind: InteractionResponseType::ChannelMessageWithSource,
                data: InteractionResponseData {
                    embeds: vec![embed].into(),
                    ..Default::default()
                }.into(),
            };
            let result = client.interaction(interaction.application_id)
                .create_response(interaction.id, &interaction.token, &response).await;

            if let Err(why) = result {
                eprintln!("Error trying to responde /age command: {:?}", why);
            }
        }
    )
}