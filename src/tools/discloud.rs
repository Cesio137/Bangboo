use std::env;
use discloud_rs::Discloud;
use once_cell::sync::Lazy;
use regex::Regex;

pub static ASCII_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[[0-9;]+m").unwrap());

pub static DISCLOUD: Lazy<Discloud> = Lazy::new(|| {
    dotenvy::dotenv().ok();
    Discloud::new(&env::var("DISCLOUD_TOKEN").expect("Discloud token is required"))
});