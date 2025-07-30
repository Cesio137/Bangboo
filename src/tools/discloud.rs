use discloud_rs::Discloud;
use once_cell::sync::Lazy;
use regex::Regex;
use std::env;

pub const APPID: &str = "1748902511250";

pub static ASCII_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[[0-9;]+m").unwrap());

pub static DISCLOUD: Lazy<Discloud> = Lazy::new(|| {
    dotenvy::dotenv().ok();
    Discloud::new(&env::var("DISCLOUD_TOKEN").expect("Discloud token is required"))
});
