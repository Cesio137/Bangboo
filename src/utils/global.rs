use tiny_skia::*;
use image::DynamicImage;
use std::hash::Hash;
use twilight_gateway::EventType;
use twilight_model::user::User;
use twilight_util::snowflake::Snowflake;
use crate::settings::global::{ JOIN_IMG, ADD_ICON, LEAVE_IMG, MINUS_ICON, RUBIK, LATO };
use std::io::{Error, ErrorKind};
use image::codecs::png::{CompressionType, FilterType, PngEncoder};
use crate::utils::cdn::*;
use crate::utils::skia::*;

pub async fn global_message(event: EventType, user: User) -> Result<Vec<u8>, Error> {
    let mut pixmap = Pixmap::new(1024, 260)
        .ok_or(Error::new(ErrorKind::InvalidData, "Failed to create pixelmap."))?;

    // Path
    let (background, icon_action) = match event {
        EventType::MemberAdd => {
            (JOIN_IMG, ADD_ICON)
        },
        EventType::MemberRemove => {
            (LEAVE_IMG, MINUS_ICON)
        }
        _ => {
            let msg = format!("Event is not MemberAdd or MemberRemove.");
            return Err(Error::new(ErrorKind::InvalidData, msg));
        }
    };
    
    // Load background image
    let background_image = load_image_from_bytes(background)?;
    pixmap.draw_pixmap(0, 0, background_image.as_ref(), &Default::default(), Transform::identity(), None);

    // Avatar
    let avatar_hash = user.avatar
        .ok_or(Error::new(ErrorKind::InvalidData, "User does not have an avatar."))?
        .to_string();

    let url = display_avatar_url(user.id.id(), avatar_hash.as_str(), 256);
    let avatar = load_image_from_cdn(url).await?;
    let resized_avatar = resize_image(&avatar, 180, 180)?;
    let avatar_image = draw_circle_image(&resized_avatar, 90)?;
    pixmap.draw_pixmap(61, 48, avatar_image.as_ref(), &Default::default(), Transform::default(), None);

    // Avatar action icon
    let action_icon = load_image_from_bytes(icon_action)?;
    pixmap.draw_pixmap(205, 179, action_icon.as_ref(), &Default::default(), Transform::default(), None);

    // Renderizar texto 300, 110, 60f32 | 300, 170, 32f32
    let name_pixmap = draw_text(user.name.as_str(), 60f32, RUBIK)?;
    pixmap.draw_pixmap(300, 110, name_pixmap.as_ref(), &Default::default(), Transform::default(), None);
    let nick_pixmap = draw_text(&format!("@{}", user.name), 32f32, LATO)?;
    pixmap.draw_pixmap(300, 170, nick_pixmap.as_ref(), &Default::default(), Transform::default(), None);

    // Salvar como PNG
    let buffer = image::RgbaImage::from_raw(1024, 260, pixmap.data().to_vec()).ok_or(Error::new(ErrorKind::InvalidData, "Image buffer allocation failed"))?;

    let mut png_buffer: Vec<u8> = Vec::new();
    let encoder = PngEncoder::new_with_quality(&mut png_buffer, CompressionType::Best, FilterType::Adaptive);
    buffer.write_with_encoder(encoder).map_err(|err| { Error::new(ErrorKind::InvalidData, err.to_string()) })?;

    Ok(png_buffer)    
}
