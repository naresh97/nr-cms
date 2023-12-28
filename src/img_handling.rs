use image::GenericImageView;
use image::imageops::FilterType;

pub fn get_img_as_b64(path: &std::path::Path) -> Result<_, _> {
    let img = image::io::Reader::open(path)?.decode()?;
    let dims = img.dimensions();
    const TARGET_HEIGHT: u32 = 200;
    let scale = (TARGET_HEIGHT as f32) / (dims.0 as f32);
    let height = dims.1 as f32 * scale;
    let height = height as u32;
    let img = img.resize(TARGET_HEIGHT, height, FilterType::Nearest);
}