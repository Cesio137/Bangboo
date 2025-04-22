use super::cdn::{display_avatar_url, load_image_from_cdn};
use super::skia::{draw_circle, draw_image, draw_text_with_font, resize_image, wrap_sendable_surface};
use crate::settings::global::{ADD_ICON, DEFAULT_AVATAR, HAMMER_ICON, JOIN_IMG, LATO, LEAVE_IMG, MINUS_ICON, MOD_IMG, POPPINS, RUBIK};
use crate::utils::skia::load_image_from_bytes;
use anyhow::{anyhow, Result};
use serenity::all::User;
use skia_safe::{EncodedImageFormat, ISize};
#[cfg(target_env = "gnu")]
use crate::utils::malloc::malloc;


#[derive(Eq, PartialEq)]
pub enum EventType {
    MemberAdd,
    MemberRemove,
    BanAdd
}

pub async fn global_message(event: EventType, user: &User) -> Result<Vec<u8>> {
    let mut surf = skia_safe::surfaces::raster_n32_premul(ISize {width: 1024, height: 260, })
        .ok_or(anyhow!("Failed to create surface."))?;
    
    let mut surface = wrap_sendable_surface(surf)?;
    
    
    // Path
    let (background, icon_action) = match event {
        EventType::MemberAdd => (JOIN_IMG, ADD_ICON),
        EventType::MemberRemove => (LEAVE_IMG, MINUS_ICON),
        EventType::BanAdd => (MOD_IMG, HAMMER_ICON)
    };

    let background_image = load_image_from_bytes(background)?;
    surface = draw_image(surface, background_image, 0.0, 0.0)?;
    
    // Avatar
    let avatar_hash = user
        .avatar
        .as_ref();
    
    let cdn_avatar = {
        if avatar_hash.is_some() {
            let avatar_hash = avatar_hash.unwrap();
            let is_animated = avatar_hash.is_animated();
            let (url, _) = display_avatar_url(user.id.get(), &avatar_hash.to_string(), 0);
            match load_image_from_cdn(&url, is_animated).await {
                Ok(image) => image,
                Err(_) => load_image_from_bytes(DEFAULT_AVATAR)?
            }
        }
        else {
            load_image_from_bytes(DEFAULT_AVATAR)?
        }
    };
    
    let avatar = resize_image(cdn_avatar, 180, 180)?;
    surface = draw_circle(surface, avatar, 90.0 + 68.0, 90.0 + 40.0, 90.0)?;

    // Action icon
    let action_icon = load_image_from_bytes(icon_action)?;
    surface = draw_image(surface, action_icon, 205.0, 179.0)?;

    // Welcome message
    if event == EventType::MemberAdd {
        let welcome_text = "WELCOME ABOARD";
        surface = draw_text_with_font(surface, welcome_text, POPPINS, 16.0, 522.0, 68.0 - 6.0)?;
    }

    surface = draw_text_with_font(surface, &user.name, RUBIK, 60.0, 300.0, 100.0)?;

    let undefined_nick = "Undefined".to_string();
    let nickname = user.global_name.as_ref().unwrap_or(&undefined_nick);
    surface = draw_text_with_font(surface, &format!("@{}", nickname), LATO, 32.0, 300.0, 164.0)?;

    let mut surf = surface.into_inner();
    let image = surf.image_snapshot();
    let encoded_data = image.encode(None, EncodedImageFormat::PNG, Some(100))
        .ok_or(anyhow!("Failed to encode image."))?;
    
    let data = encoded_data.to_vec();
    
    drop(surf);
    drop(image);
    drop(encoded_data);

    #[cfg(target_env = "gnu")]
    malloc::trim();
    
    Ok(data)
}