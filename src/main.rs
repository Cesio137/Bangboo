mod discord;
mod settings;
mod utils;

use crate::discord::app::App;

#[tokio::main]
async fn main() {
    let _ = App::bootstrap().await;
}
