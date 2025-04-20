use crate::models::skia::Image as SkImage;
use crate::utils::skia::load_image_from_bytes;
use anyhow::{anyhow, Result};
use skia_safe::{codec::{Options, ZeroInitialized}, surfaces, Codec, Data};

pub fn display_avatar_url(user_id: u64, hash: &str, size: u16) -> (String, bool) {
    let mut is_gif = false;
    let ext = if hash.starts_with("a_") { is_gif = true; "gif" } else { "png" };
    (
        if size == 0 {
            format!(
                "https://cdn.discordapp.com/avatars/{}/{}.{}",
                user_id, hash, ext
            )
        } else {
            format!(
                "https://cdn.discordapp.com/avatars/{}/{}.{}?size={}",
                user_id, hash, ext, size
            )
        },        
        is_gif
    )
}

pub async fn load_image_from_cdn(url: String, is_gif: bool) -> Result<SkImage> {
    let data = reqwest::get(url).await?;
    let bytes = data.bytes().await?;
    if is_gif {
        // Decode the GIF and extract the first frame
        let data = Data::new_copy(&bytes);
        let mut codec = Codec::from_data(data).ok_or(anyhow!("Failed to decode gif."))?;
        let image_info = codec.info();
        let row_bytes = image_info.min_row_bytes();
        let total_bytes = image_info.compute_byte_size(row_bytes);

        let mut pixels = vec![0u8; total_bytes];

        let options = Options {
            zero_initialized: ZeroInitialized::Yes,
            subset: None,
            frame_index: 0,
            prior_frame: None,
        };

        codec.get_pixels_with_options(&image_info, pixels.as_mut_slice(), row_bytes, Some(&options));
        let mut surface = surfaces::wrap_pixels(&image_info, pixels.as_mut_slice(), row_bytes, None)
            .ok_or(anyhow!(""))?;
        let image = SkImage(surface.image_snapshot());
        
        Ok(image)
    } else {
        // Handle PNG or other static images
        let image = load_image_from_bytes(bytes.as_ref())?;
        Ok(image)
    }
}