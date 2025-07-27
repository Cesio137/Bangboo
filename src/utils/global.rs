use super::skia::{draw_circle, draw_text_with_font, resize_image};
use crate::settings::assets::{
    CARD_BACK, CARD_LEFT, CARD_MOD, CARD_NEW, FONT_FREDOKA, FONT_ROBOTO, IMG_DEFAULT_AVATAR,
};
use crate::utils::skia::load_image_from_bytes;
use serenity::all::{CacheHttp, ChannelId, CommandInteraction, Context, CreateAttachment, CreateMessage, Member, User};
use skia_safe::{EncodedImageFormat, ISize, Image, Point};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::settings::logger::error;
use crate::utils::cdn::display_avatar_url;
use crate::utils::interaction::reply;

#[derive(Eq, PartialEq)]
pub enum EventType {
    MemberAdd,
    MemberRemove,
    BanAdd,
}

pub async fn global_message(
    ctx: &Context,
    channel_id: &ChannelId,
    event: EventType,
    member: Option<&Member>,
    user: &User,
) {
    // Fetch avatar
    let mut user_avatar: Vec<u8> = vec![];
    
    if let Some(avatar_hash) = user.avatar {
        let avatar_url = display_avatar_url(user.id.get(), &avatar_hash.to_string(), 512);
        if let Ok(res) = reqwest::get(avatar_url).await {
            if let Ok(bytes) = res.bytes().await {
                user_avatar = bytes.to_vec();
            }
        }
        if user_avatar.is_empty() {
            user_avatar = IMG_DEFAULT_AVATAR.to_vec();
        }
    } else {
        user_avatar = IMG_DEFAULT_AVATAR.to_vec();
    }
    
    let background = match event {
        EventType::MemberAdd => {
            let now = SystemTime::now();
            let duration = now.duration_since(UNIX_EPOCH)
                .expect("Time went backwards!");
            let joined_at = member.unwrap().joined_at.unwrap().timestamp_millis();
            let account_age = duration.as_millis() - user.id.created_at().timestamp_millis() as u128;
            const TIME_LIMIT: u16 = 60 * 1000;
            if joined_at < TIME_LIMIT as i64 && account_age > TIME_LIMIT as u128 {
                CARD_NEW
            } else {
                CARD_BACK
            }
        }
        EventType::MemberRemove => CARD_LEFT,
        EventType::BanAdd => CARD_MOD,
    };

    let mut data = vec![];
    // Scope to fix safe issue with skia
    {
        let mut surface = match skia_safe::surfaces::raster_n32_premul(ISize {
            width: 2800,
            height: 560,
        }) {
            Some(surface) => surface,
            None => {
                error("Failed to create canvas surface.");
                return;
            }
        };
        let canvas = surface.canvas();

        let background_image = match load_image_from_bytes(background) {
            Some(image) => image,
            None => {
                error("Failed to load background image.");
                return;
            }
        };
        canvas.draw_image(&background_image, Point { x: 0.0, y: 0.0 }, None);
        canvas.save();

        // Avatar
        let cdn_avatar = match load_image_from_bytes(&user_avatar) {
            Some(image) => image,
            None => {
                error("Failed to decode user avatar image.");
                return;
            }
        };
        let avatar = match resize_image(cdn_avatar, 400, 400) {
            Some(image) => image,
            None => {
                error("Failed to resize user avatar image.");
                return;
            }
        };
        draw_circle(canvas, avatar, 204.0, 200.0 + 160.0, 200.0);

        if !draw_text_with_font(canvas, &user.name, FONT_FREDOKA, 200.0, 530.0, 140.0) {
            error("Failed to resize user avatar image.");
            return;
        }

        let nickname = match user.global_name.as_ref() {
            Some(nickname) => nickname,
            None => "Undefined",
        };
        if !draw_text_with_font(canvas, &format!("@{}", nickname), FONT_ROBOTO, 96.0, 530.0, 380.0) {
            error("Failed to resize user avatar image.");
            return;
        }

        let image = surface.image_snapshot();
        let encoded_data = image
            .encode(None, EncodedImageFormat::PNG, Some(100))
            .unwrap();

        data = encoded_data.to_vec();
    }

    let mut utc = String::new();
    if event == EventType::MemberAdd {
        let joined_at = member.unwrap().joined_at.unwrap().timestamp();
        utc.push_str(&format!("<t:{}:F>", joined_at));
    }
    let attachment = CreateAttachment::bytes(data, "card.png");
    let message = CreateMessage::new()
        .content(utc)
        .add_files(vec![attachment]);

    if let Err(err) = channel_id.widen().send_message(ctx.http(), message).await {
        error(&format!(
            "Error trying to send card to system channel\nʟ {:?}",
            err
        ));
    }
}
