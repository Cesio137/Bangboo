// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::guild;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: guild = serde_json::from_str(&json).unwrap();
// }

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
