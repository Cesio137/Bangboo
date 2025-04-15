use crate::settings::global::{EColor, REGEX};
use crate::utils::embeds::res;
use anyhow::{anyhow, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use twilight_http::Client;
use twilight_model::{
    gateway::payload::incoming::MessageCreate,
    id::{marker::{GuildMarker, UserMarker}, Id}
};

#[derive(Debug, Clone)]
pub enum DangerLevel {
    Safe,
    High,
}

pub struct ScamFilter {
    regex: Regex,
    //keywords: HashSet<String>,
    //scamlinks: HashSet<String>,
    //reports: HashMap<String, Report>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub desc: String,
    pub report: String,
}

impl ScamFilter {
    pub fn new() -> Result<Self> {
        /*
        let json_data: Value = serde_json::from_str(SHORTLINKS)?;
        let shortlinks = json_data.as_object().context("Failed to parse JSON as object.")?;
        
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
        */
        let regex = Regex::new(REGEX)?;
        Ok(Self { regex })
    }

    pub fn filter_message(&self, message: &str) -> DangerLevel {
        if self.regex.is_match(message) { return DangerLevel::High; }
        /*
        if let Some(report) = self.check_scamlinks(message) {
            return DangerLevel::HighReport(report);
        }

        if self.keywords.iter().any(|kw| message.starts_with(kw)) {
            return DangerLevel::High;
        }
        */
        DangerLevel::Safe
    }

    pub async fn handle_spam(&self, client: &Client, message: Box<MessageCreate>) -> Result<()> {
        let username = match &message.author.global_name {
            Some(name) => name.clone(),
            None => message.author.name.clone(),
        };

        let channel_id = message.channel_id;
        let message_id = message.id;
        
        let warn = format!("{username} sent a message that was flagged as a scam. Messages containing ***[text](hyperlink)*** are strictly prohibited. Bangboo (me) will presume his/her account has been compromised, leading to a server kick!");
        let embed = res(EColor::Warning, warn);

        client.create_message(channel_id).reply(message_id).embeds(&vec![embed]).await?;

        client.delete_message(channel_id, message_id).await?;

        let guild_id = match message.guild_id {
            Some(guild_id) => guild_id,
            None => {
                return Err(anyhow!("Message is not from a guild"));
            }
        };
        let user_id = message.author.id;

        self.kick_member(client, guild_id, user_id).await?;

        Ok(())
    }
    /*
    fn check_scamlinks(&self, msg: &str) -> Option<Report> {
        self.reports.iter()
            .find_map(|(link, report)| msg.contains(link).then(|| report.clone()))
    }
    */
    async fn kick_member(&self, client: &Client, guild_id: Id<GuildMarker>, user_id: Id<UserMarker>) -> Result<()> {
        let guild = client.guild(guild_id).await?.model().await?;

        if guild.owner_id == user_id {
            return Err(anyhow!("Tried to kick the owner of the guild: {}", user_id));
        }

        client.remove_guild_member(guild_id, user_id).await?;

        self.send_dm(client, user_id, "It look like you probably got hacked and sent a message that was flagged as scam containing ***[text](hyperlink)***. You were just kicked from the server, but feel free to come back as soon as you resolve the issue with your account.").await?;
        self.send_dm(client, user_id, "").await?;
        Ok(())
    }

    async fn send_dm(&self, client: &Client, user_id: Id<UserMarker>, content: &str) -> Result<()> {
        let channel = client.create_private_channel(user_id).await?;
        let channel_id = channel.model().await?.id;

        let embed = res(EColor::Warning, content.to_string());
        client.create_message(channel_id).embeds(&vec![embed]).await?;
        Ok(())
    }
}