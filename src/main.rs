mod discord;
mod settings;
mod tools;
mod utils;
mod models;

use crate::discord::app::App;
#[cfg(target_env = "gnu")]
use crate::utils::malloc::*;

#[tokio::main]
async fn main() {
    #[cfg(target_env = "gnu")]
    malloc::limit_mmap_threshold();

    if let Err(err) = App::bootstrap().await {
        tracing::error!("{}", err);
    }
}
