use std::io::Cursor;

use base64::{engine::general_purpose, Engine as _};

use image::imageops::FilterType;

pub fn get_img_b64_size(
    path: &std::path::Path,
    size: Option<u32>,
) -> Result<usize, Box<dyn std::error::Error>> {
    let img_b64 = get_img_as_b64_url(path, size)?;
    Ok(img_b64.as_bytes().len())
}

pub fn get_img_as_b64_url(
    path: &std::path::Path,
    size: Option<u32>,
) -> Result<String, Box<dyn std::error::Error>> {
    let size = size.unwrap_or(200);
    let img = image::io::Reader::open(path)?.decode()?;
    let img = img.resize(size, size, FilterType::Nearest);
    let mut buf: Vec<u8> = Vec::new();
    img.write_to(
        &mut Cursor::new(&mut buf),
        image::ImageOutputFormat::Jpeg(70),
    )?;
    let b64 = general_purpose::STANDARD.encode(&buf);
    let b64 = format!("data:image/jpg;base64,{b64}");
    return Ok(b64);
}

pub fn resize_image(
    source: &std::path::Path,
    target: &std::path::Path,
    size: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::io::Reader::open(source)?.decode()?;
    let img = img.resize(size, size, FilterType::Nearest);
    img.save(target)?;
    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resize() {
        let sample_img_path = std::path::Path::new("sample/sample.jpg");
        let target = std::path::Path::new("gen/sample.jpg");
        resize_image(sample_img_path, target, 100).expect("");
    }

    #[test]
    fn test_get_img_as_b64() {
        let sample_img_path: &std::path::Path = std::path::Path::new("sample/sample.jpg");
        let b64 =
            get_img_as_b64_url(sample_img_path, Some(10)).expect("Could not get image as b64");
        assert_eq!(b64, "data:image/jpg;base64,/9j/4AAQSkZJRgABAgAAAQABAAD/wAARCAAKAAoDAREAAhEBAxEB/9sAQwAKBwcIBwYKCAgICwoKCw4YEA4NDQ4dFRYRGCMfJSQiHyIhJis3LyYpNCkhIjBBMTQ5Oz4+PiUuRElDPEg3PT47/9sAQwEKCwsODQ4cEBAcOygiKDs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7/8QAHwAAAQUBAQEBAQEAAAAAAAAAAAECAwQFBgcICQoL/8QAtRAAAgEDAwIEAwUFBAQAAAF9AQIDAAQRBRIhMUEGE1FhByJxFDKBkaEII0KxwRVS0fAkM2JyggkKFhcYGRolJicoKSo0NTY3ODk6Q0RFRkdISUpTVFVWV1hZWmNkZWZnaGlqc3R1dnd4eXqDhIWGh4iJipKTlJWWl5iZmqKjpKWmp6ipqrKztLW2t7i5usLDxMXGx8jJytLT1NXW19jZ2uHi4+Tl5ufo6erx8vP09fb3+Pn6/8QAHwEAAwEBAQEBAQEBAQAAAAAAAAECAwQFBgcICQoL/8QAtREAAgECBAQDBAcFBAQAAQJ3AAECAxEEBSExBhJBUQdhcRMiMoEIFEKRobHBCSMzUvAVYnLRChYkNOEl8RcYGRomJygpKjU2Nzg5OkNERUZHSElKU1RVVldYWVpjZGVmZ2hpanN0dXZ3eHl6goOEhYaHiImKkpOUlZaXmJmaoqOkpaanqKmqsrO0tba3uLm6wsPExcbHyMnK0tPU1dbX2Nna4uPk5ebn6Onq8vP09fb3+Pn6/9oADAMBAAIRAxEAPwCDTtZ1QaykkOoSx2P2pzBa+a0QYNJnjHyMDIzKzMCAAccjAm5aSKg8Wa843RxXrIeVLXVySR2yVcA/gAPQCgCVoYn1IM8aMySXG0lQSuHGMemO1IZBaajfJZwol7cKqxqABKwAGPrTA//Z");
    }

    #[test]
    fn test_get_img_b64_size() {
        let sample_img_path = std::path::Path::new("sample/sample.jpg");
        let size = get_img_b64_size(sample_img_path, Some(10)).expect("Cannot get image size");
        assert_eq!(size, 994);
    }
}
