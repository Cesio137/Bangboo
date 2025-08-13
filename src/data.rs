use std::str::FromStr;
use once_cell::sync::Lazy;
use crate::data::constants::Constants;
use crate::data::emojis::Emojis;
use crate::data::fab::Fab;
use crate::data::guild::Guild;

pub mod constants;
pub mod fab;
pub mod guild;
pub mod emojis;

const CONSTANTSJSON: &str = include_str!("../data/constants.json");
pub static CONSTANTS: Lazy<Constants> = Lazy::new(|| {
    serde_json::from_str(&CONSTANTSJSON).unwrap()
});

const EMOJISJSON: &str = include_str!("../data/emojis.json");
pub static EMOJIS: Lazy<Emojis> = Lazy::new(|| {
    serde_json::from_str(&EMOJISJSON).unwrap()
});

const FABJSON: &str = include_str!("../data/fab.json");
pub static FAB: Lazy<Fab> = Lazy::new(|| {
    serde_json::from_str(&FABJSON).unwrap()
});

const GUILDJSON: &str = include_str!("../data/guild.json");
pub static GUILD: Lazy<Guild> = Lazy::new(|| {
    serde_json::from_str(&GUILDJSON).unwrap()
});

pub fn str_hex_to_u32(val: &str) -> u32 {
    let hex_str = val.trim_start_matches('#');
    let int_value = u32::from_str_radix(hex_str, 16).expect("Invalid hex");
    int_value
}

pub fn str_to_u32(val: &str) -> u32 {
    let int_value = u32::from_str(val).expect("Invalid id");
    int_value
}

pub fn str_to_u64(val: &str) -> u64 {
    let int_value = u64::from_str(val).expect("Invalid id");
    int_value
}