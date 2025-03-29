use std::io::{Error, ErrorKind};
use image::load_from_memory;
use tiny_skia::Pixmap;
use crate::utils::skia::convert_image_to_pixmap;

pub fn display_avatar_url(user_id: u64, hash: &str, size: u16) -> String {
    let ext = if hash.starts_with("a_") { "gif" } else { "png" };
    format!("https://cdn.discordapp.com/avatars/{}/{}.{}?size={}", user_id, hash, ext, size)
}

pub async fn load_image_from_cdn(url: String) -> Result<Pixmap, Error> {
    let data = reqwest::get(url).await.map_err(|err| {
        Error::new(ErrorKind::InvalidInput, err.to_string())
    })?;
    let bytes = data.bytes().await.map_err(|err| {
        Error::new(ErrorKind::InvalidData, err.to_string())
    })?;
    let img = load_from_memory(bytes.as_ref()).map_err(|err| {
        let msg = format!("Failed to load image from memory. \n{}", err);
        Error::new(ErrorKind::InvalidData, msg)
    })?;
    let pixmap = convert_image_to_pixmap(img)?;
    Ok(pixmap)
}