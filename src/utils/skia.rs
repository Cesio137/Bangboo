use anyhow::{anyhow, Result};
use skia_safe::{
    scalar, surfaces, textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle, TypefaceFontProvider}, Canvas, CubicResampler, Data, FilterMode, FontMgr, Image, Paint, Point, Rect, SamplingOptions, Size
};

pub fn load_image_from_bytes(slice: &[u8]) -> Result<Image> {
    let data = Data::new_copy(&slice);
    let image = Image::from_encoded(&data).ok_or(anyhow!("Failed to decode image."))?;
    Ok(image)
}

pub fn resize_image(image: Image, new_width: u32, new_height: u32) -> Result<Image> {
    let mut surface = surfaces::raster_n32_premul((new_width as i32, new_height as i32))
        .ok_or(anyhow!("Failed to create surface for resizing."))?;
    let canvas = surface.canvas();

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
    Ok(resized_image)
}

pub fn draw_text_with_font(canvas: &Canvas, text: &str, font: &[u8], font_size: f32, x: f32, y: f32) -> Result<()> {
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
    Ok(())
}
