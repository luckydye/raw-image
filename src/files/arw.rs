use super::RawImage;

pub struct Arw {
  buffer: Vec<u8>,
}

impl RawImage for Arw {
  fn new(buffer: Vec<u8>) -> Arw {
    Arw { buffer: buffer }
  }

  fn get_thumbnail(&self) -> RawResult<image::DynamicImage> {
    let img = image::RgbImage::from_vec(0, 0, Vec::new()).unwrap();
    image::DynamicImage::ImageRgb8(img)
  }
}
