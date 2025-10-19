use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename = "colors")]
pub struct Colors {
    #[serde(rename = "default")]
    pub colors_default: u32,

    pub primary: u32,

    pub secondary: u32,

    pub success: u32,

    pub danger: u32,

    pub warning: u32,

    pub azoxo: u32,

    pub green: u32,

    pub yellow: u32,

    pub fuchsia: u32,

    pub magic: u32,

    pub developer: u32,

    pub balance: u32,

    pub brilliance: u32,

    pub nitro: u32,

    pub bravery: u32,

    pub royal: u32,
}
