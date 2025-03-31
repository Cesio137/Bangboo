use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::str::FromStr;
use regex::Regex;

const SHORTLINKS: &str = include_str!("../../resources/shortlinks.json");
const REGEX: &str = r"(https:\\/\\/www\\.|http:\\/\\/www\\.|https:\\/\\/|http:\\/\\/)(surl\\.li|u\\.to|t\\.co|gclnk\\.com|qptr\\.ru|uclck\\.ru|go-link\\.ru|envs\\.sh|shorter\\.me|sc\\.link|goo\\.su|plhn\\.pw|ej136\\.cfd|f-link\\.me|lnky\\.ru|bitly\\.cx)";

#[derive(Debug, Clone)]
pub enum DangerLevel {
    Safe,
    Medium,
    High,
    HighReport(Report)
}

pub struct MessageFilter {
    regex: Regex,
    keywords: Vec<String>,
    scamlinks: HashMap<String, Report>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub desc: String,
    pub report: String,
}

impl MessageFilter {
    pub fn new() -> Result<Self, Error> {
        let json_data = Value::from_str(SHORTLINKS)?;
        let shortlinks = json_data.as_object()
            .context("Failed to pass json data to object.")?;

        let regex = Regex::new(REGEX)?;

        let keywords_value = shortlinks.get_key_value("keywords_scams")
            .context("'keywords_scams' field does not exist.")?
            .1;
        let scamlinks_value = shortlinks.get_key_value("shortlinks_scams")
            .context("'shortlinks_scams' field does not exist.")?
            .1;

        let keywords: Vec<String> = keywords_value
            .as_array().context("Failed to get array content from 'keywords_scams' field.")?
            .iter().filter_map(|value| value.as_str().map(|str| str.to_string()))
            .collect();

        let scamlinks: HashMap<String, Report> = scamlinks_value
            .as_array().context("Failed to get array content from 'shortlinks_scams' field.")?
            .iter().filter_map(|value| {
                if let Value::Object(obj) = value {
                    if let (Some(link), Some(report), Some(desc)) = (
                        obj.get("link").and_then(|v| v.as_str()),
                        obj.get("report").and_then(|v| v.as_str()),
                        obj.get("desc").and_then(|v| v.as_str()),
                    ) {
                        return Some((
                            link.to_string(),
                            Report {
                                desc: desc.to_string(),
                                report: report.to_string(),
                            },
                        ));
                    }
                }
                None
            })
            .collect();

        Ok(Self {
            regex, 
            keywords,
            scamlinks
         })
    }

    pub fn filter_message(&self, message: &str) -> DangerLevel {
        let captures = self.regex.captures(message);
        if let Some(caps) = captures {
            for cap in caps.iter() {
                if let Some(mat) = cap {
                    println!("{}", mat.as_str());
                    if let Some(report) = self.scamlinks.get(mat.as_str()).cloned() {
                        return DangerLevel::HighReport(report);
                    }
                }
            }
            return DangerLevel::High;
        }

        let mut level = DangerLevel::Safe;
        let msg = message;

        let has_keyword = self.keywords.iter().any(|kw| msg.contains(kw));
        if has_keyword {
            level = DangerLevel::Medium;
        }

        let (has_scamlink, report) = self.check_scamlinks(msg);
        if has_scamlink {
            if let Some(report) = report {
                return DangerLevel::HighReport(report);
            }
            return DangerLevel::High;
        }

        level
    }

    fn check_scamlinks(&self, msg: &str) -> (bool, Option<Report>) {
        for (link, report) in &self.scamlinks {
            if msg.contains(link) {
                return (true, Some(report.clone()));
            }
        }
        (false, None)
    }
}