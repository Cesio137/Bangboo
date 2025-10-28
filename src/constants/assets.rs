use glob::glob;
use imghash::*;
use once_cell::sync::Lazy;

// Avatar
pub const IMG_BANGBOO: &[u8] = include_bytes!("../../assets/avatar/Bangboo.png");
pub const IMG_OFFICER: &[u8] = include_bytes!("../../assets/avatar/Officer.png");

// Member
pub const IMG_DEFAULT_AVATAR: &[u8] = include_bytes!("../../assets/member/default_avatar.png");

// Fonts
pub const FONT_FREDOKA: &[u8] = include_bytes!("../../assets/fonts/Fredoka-Medium.ttf");
pub const FONT_ROBOTO: &[u8] = include_bytes!("../../assets/fonts/Roboto-Medium.ttf");

// Cards
pub const CARD_BACK: &[u8] = include_bytes!("../../assets/cards/card-back.png");
pub const CARD_LEFT: &[u8] = include_bytes!("../../assets/cards/card-left.png");
pub const CARD_MOD: &[u8] = include_bytes!("../../assets/cards/card-mod.png");
pub const CARD_NEW: &[u8] = include_bytes!("../../assets/cards/card-new.png");

pub static HASHES: Lazy<Vec<ImageHash>> = Lazy::new(|| {
    let mut image_hashes: Vec<ImageHash> = vec![];
    let hasher = average::AverageHasher {
        width: 32,
        height: 32,
        color_space: ColorSpace::REC709,
    };
    for entry in glob("./assets/sample/scam/*").expect("Failed to read glob pattern") {
        if let Ok(path) = entry {
            if path.is_file() {
                let img_hash = match hasher.hash_from_path(path.as_path()) {
                    Ok(hash) => hash,
                    Err(_) => continue,
                };

                image_hashes.push(img_hash);
            }
        }
    }

    image_hashes
});
