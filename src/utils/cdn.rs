use crate::utils::skia::{load_image_from_bytes, wrap_sendable_image};
use anyhow::{anyhow, Result};
use skia_safe::{codec::{Options, ZeroInitialized}, surfaces, Codec, Data, Image, Sendable};
#[cfg(target_env = "gnu")]
use crate::utils::malloc::malloc;

pub fn display_avatar_url(user_id: u64, hash: &str, size: u16) -> (String, bool) {
    let mut is_animated = false;
    let ext = if hash.starts_with("a_") { is_animated = true; "gif" } else { "png" };
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
        is_animated
    )
}

pub async fn load_image_from_cdn(url: &str, is_animated: bool) -> Result<Sendable<Image>> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;

    if is_animated {
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
        let image = surface.image_snapshot();
        
        drop(codec);
        drop(image_info);
        drop(bytes);
        drop(surface);
        #[cfg(target_env = "gnu")]
        malloc::trim();
        return Ok(wrap_sendable_image(image)?)
    }
    // PNG
    let image = load_image_from_bytes(&bytes)?;
    drop(bytes);
    #[cfg(target_env = "gnu")]
    malloc::trim();
    Ok(image)
}