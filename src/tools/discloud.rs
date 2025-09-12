use discloud_rs::Discloud;
use once_cell::sync::Lazy;
use regex::Regex;
use crate::env::ENV;

pub const APPID: &str = "1757608889661";

pub static ASCII_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\x1b\[[0-9;]*m").unwrap());

pub static DISCLOUD: Lazy<Discloud> = Lazy::new(|| {
    Discloud::new(&ENV.DISCLOUD_TOKEN)
});
