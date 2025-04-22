use super::cdn::{display_avatar_url, load_image_from_cdn};
use super::skia::{draw_circle, draw_image, draw_text_with_font, resize_image};
use crate::settings::global::{ADD_ICON, DEFAULT_AVATAR, HAMMER_ICON, JOIN_IMG, LATO, LEAVE_IMG, MINUS_ICON, MOD_IMG, POPPINS, RUBIK};
use crate::utils::skia::load_image_from_bytes;
use anyhow::{anyhow, Result};
use serenity::all::User;
use skia_safe::{EncodedImageFormat, ISize, Point};

#[derive(Eq, PartialEq)]
pub enum EventType {
    MemberAdd,
    MemberRemove,
    BanAdd
}

pub async fn global_message(event: EventType, user: &User) -> Result<Vec<u8>> {
    // Defaults
    let mut user_avatar: Vec<u8> = vec![];
    let mut is_animated = false;

    if let Some(avatar_hash) = user.avatar.as_ref() {
        let (url, is_anim) = display_avatar_url(user.id.get(), &avatar_hash.to_string(), 256);
        is_animated = is_anim;
        if let Ok(res) = reqwest::get(url).await {
            if let Ok(bytes) = res.bytes().await {
                user_avatar = bytes.to_vec();
            }
        }
    } else if user_avatar.is_empty() {
        user_avatar = DEFAULT_AVATAR.to_vec();
    }
    
    let (background, icon_action) = match event {
        EventType::MemberAdd => (JOIN_IMG, ADD_ICON),
        EventType::MemberRemove => (LEAVE_IMG, MINUS_ICON),
        EventType::BanAdd => (MOD_IMG, HAMMER_ICON)
    };

    let user_block = user.clone();
    let data = tokio::task::spawn_blocking(move || -> Result<Vec<u8>> {
        let mut surface = skia_safe::surfaces::raster_n32_premul(ISize {width: 1024, height: 260})
            .ok_or(anyhow!("Failed to create surface"))?;
        let canvas = surface.canvas();

        let background_image = load_image_from_bytes(background)?;
        canvas.draw_image(&background_image, Point {x: 0.0, y: 0.0 }, None);
        canvas.save();

        // Avatar
        let cdn_avatar = load_image_from_cdn(&user_avatar, is_animated)?;
        let avatar = resize_image(cdn_avatar, 180, 180)?;
        draw_circle(canvas, avatar, 90.0 + 68.0, 90.0 + 40.0, 90.0)?;

        // Action icon
        let action_icon = load_image_from_bytes(icon_action)?;
        draw_image(canvas, action_icon, 205.0, 179.0)?;

        // Welcome message
        if event == EventType::MemberAdd {
            let welcome_text = "WELCOME ABOARD";
            draw_text_with_font(canvas, welcome_text, POPPINS, 16.0, 522.0, 68.0 - 6.0)?;
        }

        draw_text_with_font(canvas, &user_block.name, RUBIK, 60.0, 300.0, 100.0)?;

        let undefined_nick = "Undefined".to_string();
        let nickname = user_block.global_name.as_ref().unwrap_or(&undefined_nick);
        draw_text_with_font(canvas, &format!("@{}", nickname), LATO, 32.0, 300.0, 164.0)?;
        
        let image = surface.image_snapshot();
        let encoded_data = image.encode(None, EncodedImageFormat::PNG, Some(100)).unwrap();

        let data = encoded_data.to_vec();
        
        Ok(data)
    }).await??;
    
    Ok(data)
}