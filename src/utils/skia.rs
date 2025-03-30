use std::io::{Error, ErrorKind};
use fast_image_resize::images::Image;
use fast_image_resize::{PixelType, Resizer};
use fontdue::{Font, FontSettings};
use fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle};
use image::{load_from_memory, DynamicImage, GenericImageView, ImageReader};
use tiny_skia::{FillRule, IntSize, Mask, MaskType, Paint, PathBuilder, Pixmap, PixmapPaint, Transform};

pub fn convert_image_to_pixmap(img: DynamicImage) -> Result<Pixmap, Error> {
    let rgba = img.to_rgba8();
    let (width, height) = img.dimensions();
    let size = IntSize::from_wh(width, height)
        .ok_or(Error::new(ErrorKind::InvalidData, "Failed to convert image to pixmap."))?;
    let pixmap = Pixmap::from_vec(rgba.to_vec(), size)
        .ok_or(Error::new(ErrorKind::InvalidData, "Failed to convert image to pixmap."))?;
    Ok(pixmap)
}

pub fn resize_image(pixmap: &Pixmap, new_width: u32, new_height: u32) -> Result<Pixmap, Error> {
    let src_image = Image::from_vec_u8(
        pixmap.width(),
        pixmap.height(),
        pixmap.data().to_vec(),
        PixelType::U8x4,
    ).map_err(|err| {
        let msg = format!("Failed to convert image to pixmap. {}", err);
        Error::new(ErrorKind::InvalidData, msg)
    })?;

    let mut dst_image = Image::new(new_width, new_height, PixelType::U8x4);
    let mut resizer = Resizer::new();
    let _ = match resizer.resize(&src_image, &mut dst_image, None) {
        Err(why) => return Err(Error::new(ErrorKind::InvalidData, why.to_string())),
        _ => (),
    };
    let new_size = IntSize::from_wh(new_width, new_height)
        .ok_or(Error::new(ErrorKind::InvalidData, "Failed to convert image to pixmap."))?;
    let pixmap = Pixmap::from_vec(dst_image.into_vec(), new_size)
        .ok_or(Error::new(ErrorKind::InvalidData, "Failed to convert image to pixmap."))?;
    Ok(pixmap)
}

pub fn load_image_from_file(path: &str) -> Result<Pixmap, Error> {
    let img = ImageReader::open(path).map_err(|err| {
        let msg = format!("Failed to load image from file. \n{}", err);
        Error::new(ErrorKind::InvalidInput, msg)
    })?
    .decode()
    .map_err(|err| {
        let msg = format!("Failed to decode image. \n{}", err);
        Error::new(ErrorKind::InvalidData, msg)
    })?;
    Ok(convert_image_to_pixmap(img)?)
}

pub fn load_image_from_bytes(bytes: &[u8]) -> Result<Pixmap, Error> {
    let pixmap = Pixmap::decode_png(bytes).map_err(|err| {
        let msg = format!("Failed to decode image from memory. \n{}", err);
        Error::new(ErrorKind::InvalidData, msg)
    })?;
    Ok(pixmap)
}

pub fn draw_circle_image(image: &Pixmap, radius: u32) -> Result<Pixmap, Error> {
    let size = radius * 2;
    let mut pixelmap_mask = Pixmap::new(size, size)
        .ok_or(Error::new(ErrorKind::InvalidData, "Failed to create pixmap mask."))?;

    let mut path_builder = PathBuilder::new();
    path_builder.push_circle(radius as f32, radius as f32, radius as f32);
    let path = path_builder.finish()
        .ok_or(Error::new(ErrorKind::InvalidData, "Failed to build path"))?;

    let mut paint = Paint::default();
    paint.set_color_rgba8(255, 255, 255, 255);
    paint.anti_alias = true;

    pixelmap_mask.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

    let mask = Mask::from_pixmap(pixelmap_mask.as_ref(), MaskType::Alpha);

    let mut pixmap = Pixmap::new(size, size)
        .ok_or(Error::new(ErrorKind::InvalidData, "Failed to create pixmap."))?;
    pixmap.draw_pixmap(
        0,
        0,
        image.as_ref(),
        &PixmapPaint::default(),
        Transform::identity(),
        (&mask).into(),
    );

    Ok(pixmap)
}

pub fn draw_text(text: &str, font_size: f32, font_bytes: &[u8]) -> Result<Pixmap, Error> {
    // Load font from bytes
    let font = Font::from_bytes(font_bytes, FontSettings::default()).map_err(|err| {
        let msg = format!("Failed to load font from bytes.\n{}", err);
        Error::new(ErrorKind::InvalidData, msg)
    })?;

    // Calculate total width and maximum height of the Pixmap
    let (mut total_width, mut max_ascent, mut max_descent) = (0, 0, 0);
    for c in text.chars() {
        // Rasterize character to get metrics
        let (metrics, _) = font.rasterize(c, font_size);

        // Update total width
        total_width += metrics.advance_width as u32 + 1;

        // Track the highest ascent (above baseline)
        max_ascent = max_ascent.max(metrics.ymin.abs() as u32);

        // Track the largest descent (below baseline)
        max_descent = max_descent.max(metrics.height as u32 - metrics.ymin.abs() as u32);
    }

    // Calculate the height of the Pixmap (ascent + descent)
    let total_height = max_ascent + max_descent;

    // Create the Pixmap based on the calculated dimensions
    let mut pixmap = Pixmap::new(total_width, total_height).ok_or(Error::new(
        ErrorKind::InvalidData,
        "Failed to create text Pixmap.",
    ))?;

    // Use Fontdue's Layout system to calculate glyph positions
    let mut layout = Layout::new(CoordinateSystem::PositiveYDown); // Y grows downwards
    layout.reset(&LayoutSettings {
        x: 0.0,                          // Starting X position
        y: max_ascent as f32,            // Start at the baseline (y = ascent)
        max_width: None,                 // No width limit
        max_height: None,                // No height limit
        ..LayoutSettings::default()
    });

    // Append text to the layout
    layout.append(&[&font], &TextStyle::new(text, font_size, 0));

    // Iterate through the glyphs and draw them onto the Pixmap
    for glyph in layout.glyphs() {
        // Rasterize each glyph to obtain its bitmap and metrics
        let (metrics, bitmap) = font.rasterize_indexed(glyph.key.glyph_index, font_size);

        // Skip glyphs that have no bitmap
        if metrics.width == 0 || metrics.height == 0 {
            continue;
        }

        // Create a Pixmap for the individual glyph
        let mut char_pixmap = Pixmap::new(metrics.width as u32, metrics.height as u32)
            .ok_or(Error::new(ErrorKind::InvalidData, "Failed to create glyph Pixmap."))?;
        let data = char_pixmap.data_mut();

        // Fill the Pixmap with the glyph's RGBA bitmap
        for (i, &alpha) in bitmap.iter().enumerate() {
            let pixel_index = i * 4;   // Each pixel is represented by 4 bytes (RGBA)
            data[pixel_index] = alpha;        // Red channel
            data[pixel_index + 1] = alpha;    // Green channel
            data[pixel_index + 2] = alpha;    // Blue channel
            data[pixel_index + 3] = alpha;    // Alpha channel
        }

        // Draw the glyph on the main Pixmap at the correct position
        let y = (glyph.y - (total_height as f32 / 2f32)) as i32;
        pixmap.draw_pixmap(
            glyph.x as i32,
            y.max(0),
            char_pixmap.as_ref(),
            &Default::default(),
            Transform::identity(),
            None,
        );
    }

    Ok(pixmap.to_owned())
}