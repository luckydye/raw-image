use super::RawImage;

pub struct Tiff {
  buffer: Vec<u8>,
}

impl RawImage for Tiff {
  fn new(buffer: Vec<u8>) -> Tiff {
    Tiff { buffer: buffer }
  }

  fn get_thumbnail(&self) -> RawResult<image::DynamicImage> {
    let img = image::RgbImage::from_vec(0, 0, Vec::new()).unwrap();
    image::DynamicImage::ImageRgb8(img)
  }
}
