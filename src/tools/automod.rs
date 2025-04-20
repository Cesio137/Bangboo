use crate::settings::global::{EColor, REGEX};
use crate::utils::embeds::res;
use anyhow::{anyhow, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serenity::all::{CreateMessage, Message};
use serenity::client::Context;

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

    pub async fn handle_spam(&self, ctx: Context, message: Message) {
        let username = match &message.author.global_name {
            Some(name) => name,
            None => &message.author.name,
        };

        let channel_id = &message.channel_id;
        let message_id = &message.id;

        let warn = format!("{username} sent a message that was flagged as a scam. Messages containing ***[text](hyperlink)*** are strictly prohibited. Bangboo (me) will presume his/her account has been compromised, leading to a server kick!");
        let embed = res(EColor::Warning, warn);
        
        // Corrigido: usar ctx.http para operações HTTP
        if let Err(err) = channel_id.send_message(
            ctx.http.as_ref(),
            CreateMessage::new()
                .embed(embed)
        ).await {
            tracing::error!("Failed to send warning message: {}", err);
            return;
        }
        
        // Corrigido: usar ctx.http para deletar a mensagem
        channel_id.delete_message(ctx.http.as_ref(), message_id).await;
        
        // Corrigido: passar o ctx em vez de client
        self.kick_member(ctx, message).await;
    }
    /*
    fn check_scamlinks(&self, msg: &str) -> Option<Report> {
        self.reports.iter()
            .find_map(|(link, report)| msg.contains(link).then(|| report.clone()))
    }
    */
    async fn kick_member(&self, ctx: Context, message: Message) {
        let guild_id = match message.guild_id.as_ref().ok_or_else(|| anyhow!("Message is not from a guild")) {
            Ok(guild_id) => guild_id,
            Err(err) => {
                tracing::error!("Failed to get guild ID: {}", err);
                return;
            }       
        };
        let user_id = message.author.id.as_ref();
        let guild = match guild_id.to_guild_cached(ctx.cache.as_ref()) {
            Some(guild) => guild.clone(),
            None => {
                tracing::error!("Failed to get guild");
                return;
            }      
        };

        if guild.owner_id.get() == user_id.get() {
            tracing::error!("Tried to kick the owner of the guild: {}", user_id);
            return;
        }

        if let Err(err) = guild_id.kick(ctx.http.as_ref(), user_id).await {
            tracing::error!("Failed to kick member: {}", err);
        };
        
        
        // Corrigido: passar o ctx em vez de client
        self.send_dm(ctx, message, "It look like you probably got hacked and sent a message that was flagged as scam containing ***[text](hyperlink)***. You were just kicked from the server, but feel free to come back as soon as you resolve the issue with your account.").await;
    }

    async fn send_dm(&self, ctx: Context, message: Message, content: &str) {
        // Corrigido: criar canal privado usando ctx
        let user_id = &message.author.id;
        let channel = match user_id.create_dm_channel(&ctx.http).await {
            Ok(channel) => channel,
            Err(err) => {
                tracing::error!("Failed to create DM channel: {}", err);
                return;
            }       
        };

        let embed = res(EColor::Warning, content.to_string());
        
        // Corrigido: enviar mensagem usando ctx.http
        if let Err(err) = channel.send_message(
            &ctx.http,
            CreateMessage::new()
                .embed(embed)
        ).await {
            tracing::error!("Failed to send warning message: {}", err);
            return;
        };
    }
}