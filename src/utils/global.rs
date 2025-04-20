use super::cdn::{display_avatar_url, load_image_from_cdn};
use super::skia::{draw_text_with_font, resize_image};
use crate::models::skia::{Canvas as SkCanvas, Image as SkImage, Surface as SkSurface};
use crate::settings::global::{ADD_ICON, DEFAULT_AVATAR, HAMMER_ICON, JOIN_IMG, LATO, LEAVE_IMG, MINUS_ICON, MOD_IMG, POPPINS, RUBIK};
use crate::utils::skia::load_image_from_bytes;
use anyhow::{anyhow, Result};
use serenity::all::User;
use skia_safe::{scalar, EncodedImageFormat, ISize, Paint, Path, Point};

#[derive(Eq, PartialEq)]
pub enum EventType {
    MemberAdd,
    MemberRemove,
    BanAdd
}

pub async fn global_message(event: EventType, user: &User) -> Result<Vec<u8>> {
    let mut surface = SkSurface(skia_safe::surfaces::raster_n32_premul(ISize {width: 1024, height: 260, })
        .ok_or(anyhow!("Failed to create surface."))?);
    let canvas = SkCanvas(surface.0.canvas());

    // Path
    let (background, icon_action) = match event {
        EventType::MemberAdd => (JOIN_IMG, ADD_ICON),
        EventType::MemberRemove => (LEAVE_IMG, MINUS_ICON),
        EventType::BanAdd => (MOD_IMG, HAMMER_ICON),
        _ => {
            return Err(anyhow!("Event is not MemberAdd or MemberRemove."));
        }
    };
    
    // Text paint
    let mut text_paint = Paint::default();
    text_paint.set_anti_alias(true);
    text_paint.set_color(0xFFFFFFFF);
    
    let background_image = load_image_from_bytes(background)?;
    canvas.0.draw_image(&background_image.0, Point {x: 0 as scalar, y: 0 as scalar }, None);
    canvas.0.save();

    // Avatar
    let avatar_hash = user
        .avatar
        .ok_or_else(|| anyhow!("User does not have an avatar."))?
        .to_string();
    
    let (url, is_gif) = display_avatar_url(user.id.get(), avatar_hash.as_str(), 256);
    let avatar = match load_image_from_cdn(url, is_gif).await {
        Ok(avatar) => {
            resize_image(avatar, 180, 180)?
        },
        Err(_) => {
            load_image_from_bytes(DEFAULT_AVATAR)?
        },
    };
    
    let mut clip_path = Path::new();
    clip_path.add_circle(Point {x: 90.0 + 68.0, y: 90.0 + 40.0 }, 90 as scalar, None);
    canvas.0.clip_path(&clip_path, None, true);
    canvas.0.draw_image(&avatar.0, Point { x: 68.0, y: 40.0 }, None);
    canvas.0.restore();

    // Action icon
    let action_icon = load_image_from_bytes(icon_action)?;
    canvas.0.draw_image(&action_icon.0, Point { x: 205.0, y: 179.0 }, None);
    canvas.0.save();
    
    // Welcome message
    if event == EventType::MemberAdd {
        let mut welcome_text = "WELCOME ABOARD";
        draw_text_with_font(&canvas, &welcome_text, POPPINS, 16.0, 522.0, 68.0 - 6.0)?;
        canvas.0.save();
    }

    draw_text_with_font(&canvas, &user.name, RUBIK, 60.0, 300.0, 100.0)?;
    canvas.0.save();

    let undefined_nick = "Undefined".to_string();
    let nickname = user.global_name.as_ref().unwrap_or(&undefined_nick);
    draw_text_with_font(&canvas, &format!("@{}", nickname), LATO, 32.0, 300.0, 164.0)?;
    canvas.0.save();
    
    let image = SkImage(surface.0.image_snapshot());
    let encoded_data = image.0.encode(None, EncodedImageFormat::PNG, Some(100))
        .ok_or(anyhow!("Failed to encode image."))?;
    Ok(encoded_data.to_vec())
}