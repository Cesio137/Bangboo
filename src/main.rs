mod discord;
mod settings;
mod tools;
mod utils;

use crate::discord::app::App;
#[cfg(target_env = "gnu")]
use crate::utils::malloc::*;

#[tokio::main]
async fn main() {
    #[cfg(target_env = "gnu")]
    malloc::limit_mmap_threshold();

    let _ = App::bootstrap().await;
}
