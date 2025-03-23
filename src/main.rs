mod settings;
mod discord;

use crate::discord::app::App;

#[tokio::main]
async fn main() {
    let _ = App::bootstrap().await;
}
