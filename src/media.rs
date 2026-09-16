use image::{ImageFormat, ImageReader};
use std::io::Cursor;
use std::path::Path;

const MAX_DIMENSION: u32 = 1200;

pub fn compress_image(path: &str) -> Result<Vec<u8>, String> {
    let img = ImageReader::open(path)
        .map_err(|e| e.to_string())?
        .decode()
        .map_err(|e| e.to_string())?;

    let (width, height) = img.dimensions();
    let scale = (MAX_DIMENSION as f32 / width.max(height) as f32).min(1.0);
    let new_width = (width as f32 * scale) as u32;
    let new_height = (height as f32 * scale) as u32;

    let resized = img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3);

    let mut buf = Vec::new();
    let mut cursor = Cursor::new(&mut buf);
    resized
        .write_to(&mut cursor, ImageFormat::Jpeg)
        .map_err(|e| e.to_string())?;

    Ok(buf)
}
