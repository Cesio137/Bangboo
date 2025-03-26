mod discord;
mod settings;
mod tools;
mod utils;
use crate::discord::app::App;
use crate::utils::global::global_message;

#[tokio::main]
async fn main() {
    let _ = App::bootstrap().await;
}
