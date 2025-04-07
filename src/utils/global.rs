use crate::settings::global::{ADD_ICON, DEFAULT_AVATAR, HAMMER_ICON, JOIN_IMG, LATO, LEAVE_IMG, MINUS_ICON, MOD_IMG, POPPINS, RUBIK};
use crate::utils::{cdn::*, skia::*};
use anyhow::{anyhow, Result};
use fontdue::layout::VerticalAlign;
use image::codecs::png::{CompressionType, FilterType, PngEncoder};
use std::time::{SystemTime, UNIX_EPOCH};
use tiny_skia::*;
use twilight_gateway::EventType;
use twilight_model::user::User;
use twilight_model::util::Timestamp;
use twilight_util::snowflake::Snowflake;

pub async fn global_message(event: EventType, user: &User, joined_at: Option<Timestamp>) -> Result<Vec<u8>> {
    let mut pixmap = Pixmap::new(1024, 260).ok_or_else(|| anyhow!("Failed to create pixelmap."))?;
    // Path
    let (background, icon_action) = match event {
        EventType::MemberAdd => (JOIN_IMG, ADD_ICON),
        EventType::MemberRemove => (LEAVE_IMG, MINUS_ICON),
        EventType::BanAdd => (MOD_IMG, HAMMER_ICON),
        _ => {
            return Err(anyhow!("Event is not MemberAdd or MemberRemove."));
        }
    };

    // Load background image
    let background_image = load_image_from_bytes(background)?;
    pixmap.draw_pixmap(
        0,
        0,
        background_image.as_ref(),
        &Default::default(),
        Transform::identity(),
        None,
    );

    // Avatar
    let avatar_hash = user
        .avatar
        .ok_or_else(|| anyhow!("User does not have an avatar."))?
        .to_string();

    let (url, is_gif) = display_avatar_url(user.id.id(), avatar_hash.as_str(), 256);
    let avatar = match load_image_from_cdn(url, is_gif).await {
        Ok(avatar) => {
            resize_image(&avatar, 180, 180)?
        },
        Err(_) => {
            load_image_from_bytes(DEFAULT_AVATAR)?
        },
    };
    let avatar_image = draw_circle_image(&avatar, 90)?;
    pixmap.draw_pixmap(
        61,
        40,
        avatar_image.as_ref(),
        &Default::default(),
        Transform::identity(),
        None,
    );

    // Avatar action icon
    let action_icon = load_image_from_bytes(icon_action)?;
    pixmap.draw_pixmap(
        197,
        171,
        action_icon.as_ref(),
        &Default::default(),
        Transform::identity(),
        None,
    );

    // Welcome message
    if event == EventType::MemberAdd {
        let mut welcome_text = "FIRST TIME".to_string();
        if let Ok(duration) = SystemTime::now().duration_since(UNIX_EPOCH) {
            let now = duration.as_millis();
            let timestamp = user.id.timestamp() as u128;
            if now >= timestamp {
                let account_age = now - timestamp;

                if let Some(joined_at) = joined_at {
                    let join_age = now.saturating_sub(joined_at.as_micros() as u128 / 1000);
                    let timelimit = 60 * 1000;
                    if join_age > timelimit && account_age > timelimit {
                        welcome_text = "WELCOME BACK".to_string();
                    }
                }
            }
        }

        let welcome_pixmap = draw_text(&welcome_text, 16f32, POPPINS, VerticalAlign::Middle)?;
        let x_pos = if welcome_text == "FIRST TIME" {
            556
        } else {
            533
        };
        pixmap.draw_pixmap(
            x_pos,
            68,
            welcome_pixmap.as_ref(),
            &Default::default(),
            Transform::identity(),
            None,
        );
    }

    // Render text
    let name_pixmap = draw_text(&user.name, 60f32, RUBIK, VerticalAlign::Bottom)?;
    pixmap.draw_pixmap(
        300,
        110,
        name_pixmap.as_ref(),
        &Default::default(),
        Transform::identity(),
        None,
    );
    let undefined_nick = "Undefined".to_string();
    let nickname = user.global_name.as_ref().unwrap_or(&undefined_nick);
    let nick_pixmap = draw_text(&format!("@{}", nickname), 32f32, LATO, VerticalAlign::Bottom)?;
    pixmap.draw_pixmap(
        300,
        170,
        nick_pixmap.as_ref(),
        &Default::default(),
        Transform::identity(),
        None,
    );

    // Save as PNG buffer
    let buffer = image::RgbaImage::from_raw(1024, 260, pixmap.data().to_vec())
        .ok_or_else(|| anyhow!("Image buffer allocation failed"))?;

    let mut png_buffer: Vec<u8> = Vec::new();
    let encoder =
        PngEncoder::new_with_quality(&mut png_buffer, CompressionType::Best, FilterType::Adaptive);
    buffer
        .write_with_encoder(encoder)
        .map_err(|err| anyhow!("Failed to encode PNG: {}", err))?;

    Ok(png_buffer)
}
