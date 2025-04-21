use super::cdn::{display_avatar_url, load_image_from_cdn};
use super::skia::{draw_text_with_font, resize_image};
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
    let mut surface = skia_safe::surfaces::raster_n32_premul(ISize {width: 1024, height: 260, })
        .ok_or(anyhow!("Failed to create surface."))?;
    let canvas = surface.canvas();

    // Path
    let (background, icon_action) = match event {
        EventType::MemberAdd => (JOIN_IMG, ADD_ICON),
        EventType::MemberRemove => (LEAVE_IMG, MINUS_ICON),
        EventType::BanAdd => (MOD_IMG, HAMMER_ICON)
    };

    // Text paint
    let mut text_paint = Paint::default();
    text_paint.set_anti_alias(true);
    text_paint.set_color(0xFFFFFFFF);

    let background_image = load_image_from_bytes(background)?;
    canvas.draw_image(&background_image, Point {x: 0 as scalar, y: 0 as scalar }, None);
    canvas.save();

    // Avatar
    let avatar_hash = user
        .avatar
        .as_ref();
    //let avatar_url = user.avatar_url();
    
    let cdn_avatar = {
        if avatar_hash.is_some() {
            //let avatar_url = avatar_url.unwrap();
            let avatar_hash = avatar_hash.unwrap();
            let is_animated = avatar_hash.is_animated();
            let (url, _) = display_avatar_url(user.id.get(), &avatar_hash.to_string(), 0);
            match load_image_from_cdn(&url, is_animated) {
                Ok(image) => image,
                Err(_) => load_image_from_bytes(DEFAULT_AVATAR)?
            }
        }
        else {
            load_image_from_bytes(DEFAULT_AVATAR)?
        }
    };
    
    let avatar = resize_image(cdn_avatar, 180, 180)?;

    let mut clip_path = Path::new();
    clip_path.add_circle(Point {x: 90.0 + 68.0, y: 90.0 + 40.0 }, 90 as scalar, None);
    canvas.clip_path(&clip_path, None, true);
    canvas.draw_image(&avatar, Point { x: 68.0, y: 40.0 }, None);
    canvas.restore();

    // Action icon
    let action_icon = load_image_from_bytes(icon_action)?;
    canvas.draw_image(&action_icon, Point { x: 205.0, y: 179.0 }, None);

    // Welcome message
    if event == EventType::MemberAdd {
        let welcome_text = "WELCOME ABOARD";
        draw_text_with_font(&canvas, welcome_text, POPPINS, 16.0, 522.0, 68.0 - 6.0)?;
    }

    draw_text_with_font(&canvas, &user.name, RUBIK, 60.0, 300.0, 100.0)?;

    let undefined_nick = "Undefined".to_string();
    let nickname = user.global_name.as_ref().unwrap_or(&undefined_nick);
    draw_text_with_font(&canvas, &format!("@{}", nickname), LATO, 32.0, 300.0, 164.0)?;

    let image = surface.image_snapshot();
    let encoded_data = image.encode(None, EncodedImageFormat::PNG, Some(100))
        .ok_or(anyhow!("Failed to encode image."))?;
    
    Ok(encoded_data.to_vec())
}