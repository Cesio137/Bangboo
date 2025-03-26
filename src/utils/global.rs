use skia_safe::{Canvas, Paint, Path, Font, FontMgr, Typeface, Color, Rect, EncodedImageFormat, Image, Data, FontStyle, TextBlob};
use skia_safe::surfaces;
use std::fs;
use std::hash::Hash;
use twilight_gateway::EventType;
use twilight_model::user::User;
use twilight_util::snowflake::Snowflake;

const POPPINS: &[u8] = include_bytes!("../../assets/fonts/Poppins-SemiBold.ttf");

pub fn global_message(event: EventType, user: User) {
    let mut surface = match surfaces::raster_n32_premul((1024, 260)) {
        Some(surf) => surf,
        None => {
            eprintln!("Failed to create surface.");
            return ();
        },
    };
    let canvas = surface.canvas();
    // Path
    let (background_path, icon_path) = match event {
        EventType::MemberAdd => {
            ("../../assets/images/join.png", "../../assets/icons/static/add.svg")
        },
        EventType::MemberRemove => {
            ("../../assets/images/leave.png", "../../assets/icons/static/minus.svg")
        }
        _ => { return (); }
    };
    
    // Load background image
    let background = match load_image("assets/images/leave.png") {
        Some(img) => img,
        None => {
            eprintln!("Failed to load background image.");
            return ();
        },
    };
    canvas.draw_image(&background, (0, 0), None);

    // Avatar
    let avatar_hash = match user.avatar {
        Some(avatar) => {
            avatar.to_string()
        },
        None => {
            eprintln!("Failed to load avatar image.");
            return ();
        },
    };
    //eprintln!("Failed to load avatar image.");
    //return ();
    let avatar = {
        let url = display_avatar_url(user.id.id(), avatar_hash.as_str(), 256);
        println!("avatar size: {}", &url);
        let image = match load_image_from_url(url) {
            None => {
                eprintln!("Failed to load avatar image.");
                return ();
            }
            Some(img) => img
        };
        /*let data = Data::new_copy();
        let image = match Image::from_encoded(data) {
            Some(img) => img,
            None => {
                eprintln!("Failed to decode avatar image.");
                return;
            }
        };*/
        image
    };
    let mut path = Path::new();
    path.add_circle((90.0 + 68.0, 90.0 + 41.0), 90.0, None);
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    canvas.clip_path(&path, None, Some(true));
    canvas.draw_image_rect(&avatar, None, Rect::from_xywh(68.0, 41.0, 180.0, 180.0), &paint);
    
    // Welcome text
    let mut paint = Paint::default();
    paint.set_color(Color::WHITE);
    paint.set_anti_alias(true);

    let font_mgr = FontMgr::new();
    let typeface = match font_mgr.new_from_data(POPPINS, None) {
        None => {
            eprintln!("Failed to load poppins font.");
            return ();
        }
        Some(typeface) => typeface
    };
    let font = Font::new(typeface, 16.0);
    
    let text_blob = TextBlob::from_str("FIRST TIME", &font).unwrap();
    canvas.draw_text_blob(&text_blob, (533.0, 66.0 + 8.0), &paint);
    
    // Save image
    let image = surface.image_snapshot();
    //let data = image.encode(None, EncodedImageFormat::PNG, None).expect("Failed to encode image");
    let data = match image.encode(None, EncodedImageFormat::PNG, None) {
        None => {
            eprintln!("Failed to encode image.");
            return ();
        }
        Some(data) => { data }
    };
    fs::write("output.png", data.as_bytes()).expect("Failed to save image");
}

fn load_image(path: &str) -> Option<Image> {
    if let Ok(data) = fs::read(path) {
        Image::from_encoded(Data::new_copy(&data))
    } else {
        None
    }
}

fn load_image_from_url(url: String) -> Option<Image> {
    if let Ok(data) = reqwest::blocking::get(url) {
        if let Ok(bytes) = data.bytes() {
            return Image::from_encoded(Data::new_copy(&bytes))
        }
    }
    None
}

fn display_avatar_url(user_id: u64, image_hash: &str, size: u16) -> String {
    let ext = if image_hash.starts_with("a_") { "gif" } else { "png" };
    format!("https://cdn.discordapp.com/avatars/{}/{}.{}?size={}", user_id, image_hash, ext, size)
}