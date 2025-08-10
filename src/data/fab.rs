// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::fab;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: fab = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Fab {
    pub engine_user_setings: ProductInfos,

    pub internet_protocol: ProductInfos,
}

#[derive(Serialize, Deserialize)]
pub struct ProductInfos {
    pub product_name: String,

    pub product_desc: String,

    pub thumb_link: String,

    pub product_url: String,

    pub doc_url: String,
}
