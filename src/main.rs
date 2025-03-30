mod discord;
mod settings;
mod tools;
mod utils;
use crate::discord::app::App;

#[tokio::main]
async fn main() {
    let _ = App::bootstrap().await;
}
