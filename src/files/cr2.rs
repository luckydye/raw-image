use super::RawImage;

pub struct Cr2 {
  buffer: Vec<u8>,
}

impl RawImage for Cr2 {
  fn new(buffer: Vec<u8>) -> Cr2 {
    Cr2 { buffer: buffer }
  }

  fn get_thumbnail(&self) -> RawResult<image::DynamicImage> {
    let img = image::RgbImage::from_vec(0, 0, Vec::new()).unwrap();
    image::DynamicImage::ImageRgb8(img)
  }
}
