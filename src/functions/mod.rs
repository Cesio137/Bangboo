
#[cfg(target_env = "gnu")]
pub mod malloc;
pub mod utils;

#[cfg(target_env = "gnu")]
pub use malloc::*;
pub use utils::*;