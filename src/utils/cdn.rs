use std::io::{Error, ErrorKind};
use tiny_skia::Pixmap;

use super::skia::load_image_from_bytes;

pub fn display_avatar_url(user_id: u64, hash: &str, size: u16) -> String {
    let ext = if hash.starts_with("a_") { "gif" } else { "png" };
    format!(
        "https://cdn.discordapp.com/avatars/{}/{}.{}?size={}",
        user_id, hash, ext, size
    )
}

pub async fn load_image_from_cdn(url: String) -> Result<Pixmap, Error> {
    let data = reqwest::get(url)
        .await
        .map_err(|err| Error::new(ErrorKind::InvalidInput, err.to_string()))?;
    let bytes = data
        .bytes()
        .await
        .map_err(|err| Error::new(ErrorKind::InvalidData, err.to_string()))?;
    let pixmap = load_image_from_bytes(bytes.as_ref())?;
    Ok(pixmap)
}
