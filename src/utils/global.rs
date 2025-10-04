use crate::discord::*;
use crate::utils::*;
use crate::data::*;
use crate::assets::*;
use serenity::all::{
    CacheHttp, ChannelId, Colour, Context, CreateAttachment, CreateEmbed, CreateEmbedAuthor,
    CreateMessage, GuildId, Member, User,
};
use skia_safe::{Data, EncodedImageFormat, ISize, Point};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Eq, PartialEq)]
pub enum EventType {
    MemberAdded,
    MemberRemoved,
    BanAdded,
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
        EventType::MemberAdded => {
            let date = match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(duration) => duration.as_millis(),
                Err(_) => 0,
            };
            let join_age = match member.unwrap().joined_at {
                Some(joined_at) => date - joined_at.timestamp_millis() as u128,
                None => 0,
            };
            let account_age = date - user.id.created_at().timestamp_millis() as u128;
            const TIME_LIMIT: u128 = 300 * 1000;
            if join_age < TIME_LIMIT {
                CARD_NEW
            } else if account_age < TIME_LIMIT {
                CARD_NEW
            } else {
                CARD_BACK
            }
        }
        EventType::MemberRemoved => CARD_LEFT,
        EventType::BanAdded => CARD_MOD,
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
        draw_circle(canvas, avatar, 204.0, 360.0, 200.0);

        if !draw_text_with_font(canvas, &user.name, FONT_FREDOKA, 200.0, 530.0, 140.0) {
            error("Failed to resize user avatar image.");
            return;
        }

        let nickname = match user.global_name.as_ref() {
            Some(nickname) => nickname,
            None => "Undefined",
        };
        if !draw_text_with_font(
            canvas,
            &format!("@{}", nickname),
            FONT_ROBOTO,
            96.0,
            530.0,
            380.0,
        ) {
            error("Failed to resize user avatar image.");
            return;
        }

        let image = surface.image_snapshot();
        let encoded_data = match image.encode(None, EncodedImageFormat::PNG, Some(100)) {
            Some(data) => data,
            None => {
                error("Failed to encode card image.");
                return;
            }
        };

        data = encoded_data.to_vec();
    }

    let mut utc = String::new();
    if event == EventType::MemberAdded {
        let joined_at = member.unwrap().joined_at.unwrap_or_default().timestamp();
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

pub async fn global_boost(ctx: &Context, user: &User, guild_id: &GuildId) {
    let color = Colour::new(str_hex_to_u32(&CONSTANTS.colors.nitro));
    let avatar_url = user.avatar_url().unwrap_or_default();
    let username = user.global_name.clone().unwrap_or(user.name.clone());
    let description = format!(
        "**<a:boost:{}> <@${}> became a <@&${}>**\n\n🚀 Thanks for boosting the server!",
        &EMOJIS.animated.boost, user.id, &GUILD.roles.boosters
    );

    let author = CreateEmbedAuthor::new(username.as_str()).icon_url(&avatar_url);
    let embed = CreateEmbed::new()
        .color(color)
        .author(author)
        .description(description)
        .thumbnail(&avatar_url);

    let channel = match guild_id.channels(ctx.http()).await {
        Ok(channels) => {
            let id = ChannelId::new(str_to_u64(&GUILD.channels.announcement));
            if let Some(channel) = channels.get(&id).cloned() {
                channel
            } else {
                error(&format!("Guild channel not found!"));
                return;
            }
        }
        Err(err) => {
            error(&format!("Failed to remove member role!\n└ {:?}", err));
            return;
        }
    };

    let payload = CreateMessage::new()
        .content("||@everyone @here||")
        .embed(embed);
    if let Err(err) = channel.send_message(ctx.http(), payload).await {
        error(&format!("Failed to send message!\n└ {:?}", err));
    }
}
