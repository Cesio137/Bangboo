use crate::settings::global::EColor;
use crate::utils::embeds::res;
use crate::utils::logger::error;
use anyhow::{Context, Error, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::sync::Arc;
use twilight_http::Client;
use twilight_model::gateway::payload::incoming::MessageCreate;
use twilight_model::id::marker::{GuildMarker, UserMarker};
use twilight_model::id::Id;

const SHORTLINKS: &str = include_str!("../../resources/shortlinks.json");
const REGEX: &str = r"(https?://(?:www\.)?(surl\.li|u\.to|t\.co|gclnk\.com|qptr\.ru|uclck\.ru|go-link\.ru|envs\.sh|shorter\.me|sc\.link|goo\.su|plhn\.pw|ej136\.cfd|f-link\.me|lnky\.ru|bitly\.cx))";

#[derive(Debug, Clone)]
pub enum DangerLevel {
    Safe,
    High,
    HighReport(Report),
}

pub struct ScamFilter {
    regex: Regex,
    keywords: HashSet<String>,
    scamlinks: HashSet<String>,
    reports: HashMap<String, Report>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub desc: String,
    pub report: String,
}

impl ScamFilter {
    pub fn new() -> Result<Self, Error> {
        let json_data: Value = serde_json::from_str(SHORTLINKS)?;
        let shortlinks = json_data.as_object().context("Failed to parse JSON as object.")?;

        let regex = Regex::new(REGEX)?;

        let keywords: HashSet<String> = shortlinks.get("keywords_scams")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();

        let reports: HashMap<String, Report> = shortlinks.get("shortlinks_scams")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|obj| {
                obj.as_object().and_then(|o| {
                    let link = o.get("link")?.as_str()?;
                    let report = o.get("report")?.as_str()?;
                    let desc = o.get("desc")?.as_str()?;
                    Some((link.to_string(), Report { desc: desc.into(), report: report.into() }))
                })
            }).collect())
            .unwrap_or_default();

        let scamlinks: HashSet<String> = reports.keys().cloned().collect();

        Ok(Self { regex, keywords, scamlinks, reports })
    }

    pub fn filter_message(&self, message: &str) -> DangerLevel {
        if let Some(cap) = self.regex.find(message) {
            if let Some(report) = self.reports.get(cap.as_str()) {
                return DangerLevel::HighReport(report.clone());
            }
            return DangerLevel::High;
        }

        if let Some(report) = self.check_scamlinks(message) {
            return DangerLevel::HighReport(report);
        }

        /*if self.keywords.iter().any(|kw| message.contains(kw)) {
            return DangerLevel::High;
        }*/

        DangerLevel::Safe
    }

    pub async fn handle_spam(&self, client: Arc<Client>, message: Box<MessageCreate>, report: Option<Report>) {
        let username = match &message.author.global_name {
            Some(name) => name.clone(),
            None => message.author.name.clone(),
        };

        let channel_id = message.channel_id;
        let message_id = message.id;

        let mut warn = format!("{username} probably got hacked and sent a message that was flagged as scam.");
        if let Some(report) = report {
            warn = format!("{warn}\n\nDescription: {}\n[Report about url]({})", report.desc, report.report);
        }
        let embed = res(EColor::Warning, warn);

        if let Err(err) = client.create_message(channel_id).reply(message_id).embeds(&vec![embed]).await {
            error(&format!("Error trying to reply scam message.\n{:?}", err));
        }
        if let Err(err) = client.delete_message(channel_id, message_id).await {
            error(&format!("Error trying to delete scam message.\n{:?}", err));
        }

        let guild_id = match message.guild_id {
            Some(guild_id) => guild_id,
            None => {
                error("Message is not from a guild");
                return;
            }
        };
        let user_id = message.author.id;

        self.kick_member(client, guild_id, user_id).await;
    }

    fn check_scamlinks(&self, msg: &str) -> Option<Report> {
        self.reports.iter()
            .find_map(|(link, report)| msg.contains(link).then(|| report.clone()))
    }

    async fn kick_member(&self, client: Arc<Client>, guild_id: Id<GuildMarker>, user_id: Id<UserMarker>) {
        let guild = match client.guild(guild_id).await {
            Ok(guild) => guild.model().await.unwrap(),
            Err(err) => {
                error(&format!("Error getting guild: {:?}", err));
                return;
            }
        };

        if guild.owner_id == user_id {
            error(&format!("Tried to kick the owner of the guild: {}", user_id));
            return;
        }

        if let Err(err) = client.remove_guild_member(guild_id, user_id).await {
            error(&format!("Error kicking member: {:?}", err));
        }

        self.send_dm(client, user_id, "It look like you probably got hacked and sent a message that was flagged as scam. You were just kicked from the server, but feel free to come back as soon as you resolve the issue with your account.").await;
    }

    async fn send_dm(&self, client: Arc<Client>, user_id: Id<UserMarker>, content: &str) {
        let channel = match client.create_private_channel(user_id).await {
            Ok(channel) => channel.model().await.unwrap(),
            Err(err) => {
                error(&format!("Error creating DM channel: {:?}", err));
                return;
            }
        };

        let embed = res(EColor::Warning, content.to_string());
        if let Err(err) = client.create_message(channel.id).embeds(&vec![embed]).await {
            error(&format!("Error sending DM: {:?}", err));
        }
    }
}