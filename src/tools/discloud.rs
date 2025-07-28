use std::env;
use discloud_rs::Discloud;
use once_cell::sync::Lazy;

pub static DISCLOUD: Lazy<Discloud> = Lazy::new(|| {
    dotenvy::dotenv().ok();
    Discloud::new(&env::var("DISCLOUD_TOKEN").expect("Discloud token is required"))
});