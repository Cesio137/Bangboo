// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::Constants;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: Constants = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Constants {
    pub colors: Colors,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Colors {
    #[serde(rename = "default")]
    pub colors_default: String,

    pub primary: String,

    pub secondary: String,

    pub success: String,

    pub danger: String,

    pub warning: String,

    pub azoxo: String,

    pub green: String,

    pub yellow: String,

    pub fuchsia: String,

    pub magic: String,

    pub developer: String,

    pub balance: String,

    pub brilliance: String,

    pub nitro: String,

    pub bravery: String,

    pub royal: String,
}
