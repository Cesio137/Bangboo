use anyhow::{anyhow, Result};
use skia_safe::{scalar, surfaces, textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle, TypefaceFontProvider}, ConditionallySend, CubicResampler, Data, FilterMode, FontMgr, Image, Paint, Path, Point, Rect, SamplingOptions, Sendable, Size, Surface};
#[cfg(target_env = "gnu")]
use crate::utils::malloc::malloc;

pub fn draw_image(surface: Sendable<Surface>, image: Sendable<Image>, x: f32, y: f32) -> Result<Sendable<Surface>> {
    let mut surf = surface.into_inner();
    let canvas = surf.canvas();
    let image = image.into_inner();
    
    canvas.draw_image(&image, Point {x: x as scalar, y: y as scalar }, None);
    canvas.save();
    
    drop(image);
    #[cfg(target_env = "gnu")]
    malloc::trim();
    Ok(wrap_sendable_surface(surf)?)
}

pub fn draw_circle(surface: Sendable<Surface>, image: Sendable<Image>, x: f32, y: f32, radius: f32) -> Result<Sendable<Surface>> {
    let mut surf = surface.into_inner();
    let mut image = image.into_inner();
    let canvas = surf.canvas();
    
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_color(0xFFFFFFFF);
    paint.set_style(skia_safe::PaintStyle::Fill);

    let mut clip_path = Path::new();
    clip_path.add_circle(Point {x, y}, radius, None);

    canvas.clip_path(&clip_path, None, true);
    canvas.draw_image(&image, Point { x: x - radius, y: y - radius }, None);
    canvas.restore();
    
    drop(image);
    drop(paint);
    drop(clip_path);
    #[cfg(target_env = "gnu")]
    malloc::trim();
    Ok(wrap_sendable_surface(surf)?)
}

pub fn load_image_from_bytes(slice: &[u8]) -> Result<Sendable<Image>> {
    let data = Data::new_copy(&slice);
    let image = Image::from_encoded(&data).ok_or(anyhow!("Failed to decode image."))?;
    drop(data);
    #[cfg(target_env = "gnu")]
    malloc::trim();
    Ok(wrap_sendable_image(image)?)
}

pub fn resize_image(image: Sendable<Image>, new_width: u32, new_height: u32) -> Result<Sendable<Image>> {
    let mut surface = surfaces::raster_n32_premul((new_width as i32, new_height as i32))
        .ok_or(anyhow!("Failed to create surface for resizing."))?;
    let canvas = surface.canvas();
    let image = image.into_inner();

    let size = Rect::from_size(Size::new(new_width as f32, new_height as f32));

    let mut sampling = SamplingOptions::default();
    sampling.cubic = CubicResampler::mitchell();
    sampling.filter = FilterMode::Linear;

    let mut paint = Paint::default();
    paint.set_anti_alias(true);

    canvas.draw_image_rect_with_sampling_options(
        image,
        None,
        size,
        sampling,
        &paint,
    );

    let resized_image = surface.image_snapshot();

    drop(paint);
    drop(surface);

    #[cfg(target_env = "gnu")]
    malloc::trim();
    
    Ok(wrap_sendable_image(resized_image)?)
}

pub fn draw_text_with_font(surface: Sendable<Surface>, text: &str, font: &[u8], font_size: f32, x: f32, y: f32) -> Result<Sendable<Surface>> {
    let mut surf = surface.into_inner();
    let canvas = surf.canvas();
    
    let mut font_provider = TypefaceFontProvider::new();
    let font_mrg = FontMgr::new();
    let typeface = font_mrg.new_from_data(font, None).ok_or(anyhow!("Failed to create typeface."))?;
    font_provider.register_typeface(typeface, Some("CanvasFont"));

    let mut font_collection = FontCollection::new();
    font_collection.set_default_font_manager(Some(font_provider.into()), None);
    
    let paragraph_style = ParagraphStyle::new();
    let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);

    let mut text_style = TextStyle::new();
    text_style.set_font_size(font_size);
    text_style.set_color(0xFFFFFFFF);
    text_style.set_font_families(&["CanvasFont"]);

    paragraph_builder.push_style(&text_style);
    paragraph_builder.add_text(text);

    let mut paragraph = paragraph_builder.build();
    paragraph.layout(1024.0);

    paragraph.paint(canvas, Point::new(x as scalar, y as scalar));

    drop(font_mrg);
    drop(paragraph_style);
    drop(text_style);
    drop(paragraph_builder);
    drop(paragraph);


    #[cfg(target_env = "gnu")]
    malloc::trim();
    
    Ok(wrap_sendable_surface(surf)?)
}

pub fn wrap_sendable_surface(surface: Surface) -> Result<Sendable<Surface>> {
    Ok(surface.wrap_send().ok().ok_or(anyhow!("Failed to wrap surface."))?)
}

pub fn wrap_sendable_image(image: Image) -> Result<Sendable<Image>> {
    Ok(image.wrap_send().ok().ok_or(anyhow!("Failed to wrap image."))?)
}
