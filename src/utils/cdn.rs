use super::skia::load_image_from_bytes;
use anyhow::Result;
use image::{AnimationDecoder, DynamicImage};
use std::io::Cursor;
use tiny_skia::Pixmap;

pub fn display_avatar_url(user_id: u64, hash: &str, size: u16) -> (String, bool) {
    let mut is_gif = false;
    let ext = if hash.starts_with("a_") { is_gif = true; "gif" } else { "png" };
    (
        format!(
            "https://cdn.discordapp.com/avatars/{}/{}.{}?size={}",
            user_id, hash, ext, size
        ),
        is_gif
    )
}

pub async fn load_image_from_cdn(url: String, is_gif: bool) -> Result<Pixmap> {
    let data = reqwest::get(url)
        .await
        .map_err(|err| anyhow::anyhow!("Failed to fetch image: {}", err))?;
    let bytes = data
        .bytes()
        .await
        .map_err(|err| anyhow::anyhow!("Failed to read image bytes: {}", err))?;
    if is_gif {
        // Decode the GIF and extract the first frame
        let cursor = Cursor::new(bytes);
        let decoder = image::codecs::gif::GifDecoder::new(cursor)
            .map_err(|err| anyhow::anyhow!("Failed to decode GIF: {}", err))?;
        let frames = decoder.into_frames();
        let first_frame = frames
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("No frames found in GIF"))?
            .map_err(|err| anyhow::anyhow!("Failed to extract GIF frame: {}", err))?;
        let image = DynamicImage::ImageRgba8(first_frame.into_buffer());
        let pixmap = super::skia::convert_image_to_pixmap(image)
            .map_err(|err| anyhow::anyhow!("Failed to convert image to Pixmap: {}", err))?;
        Ok(pixmap)
    } else {
        // Handle PNG or other static images
        let pixmap = load_image_from_bytes(bytes.as_ref())
            .map_err(|err| anyhow::anyhow!("Failed to load image from bytes: {}", err))?;
        Ok(pixmap)
    }
}
