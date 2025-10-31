pub mod colors;
pub mod emojis;
pub mod fab;
pub mod guild;

use colors::*;
use emojis::*;
use fab::*;
use guild::*;
use config::{Config, File, FileFormat};
use once_cell::sync::Lazy;

const COLORSRON: &str = include_str!("../../data/colors.ron");
pub static COLORS: Lazy<Colors> = Lazy::new(|| ron::from_str(&COLORSRON).unwrap());

const DISCLOUDCONFIG: &str = include_str!("../../discloud.config");
pub static APPID: Lazy<String> = Lazy::new(|| {
    let discloud = match Config::builder()
        .add_source(File::from_str(DISCLOUDCONFIG, FileFormat::Ini))
        .build()
    {
        Ok(config) => config,
        Err(_) => return String::new(),
    };
    discloud.get("ID").unwrap_or_default()
});

const EMOJISRON: &str = include_str!("../../data/emojis.ron");
pub static EMOJIS: Lazy<Emojis> = Lazy::new(|| ron::from_str(&EMOJISRON).unwrap());

const FABRON: &str = include_str!("../../data/fab.ron");
pub static FAB: Lazy<Fab> = Lazy::new(|| ron::from_str(&FABRON).unwrap());

const GUILDRON: &str = include_str!("../../data/guild.ron");
pub static GUILD: Lazy<Guild> = Lazy::new(|| ron::from_str(&GUILDRON).unwrap());