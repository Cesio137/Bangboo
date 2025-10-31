use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Guild {
    pub channels: Channels,

    pub roles: Roles,
}

#[derive(Serialize, Deserialize)]
pub struct Channels {
    pub announcement: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Roles {
    pub apps: u64,

    pub kernel: u64,

    pub stf: u64,

    pub boosters: u64,
}
