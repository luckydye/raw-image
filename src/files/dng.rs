use super::RawImage;

pub struct Dng {
  buffer: Vec<u8>,
}

impl RawImage for Dng {
  fn new(buffer: Vec<u8>) -> Dng {
    Dng { buffer: buffer }
  }

  fn get_thumbnail(&self) -> RawResult<image::DynamicImage> {
    let img = image::RgbImage::from_vec(0, 0, Vec::new()).unwrap();
    image::DynamicImage::ImageRgb8(img)
  }
}
