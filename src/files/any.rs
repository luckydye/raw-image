use super::{RawImage, RawResult, RawError};

use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use image::ImageFormat;
use image::DynamicImage;

pub struct Any {
  buffer: Vec<u8>,
}

impl Any {
  pub fn get_thumbnail_array(&self, image_index: usize) -> RawResult<DynamicImage> {
    let data = &self.buffer;
    // Find the start positions of embedded JPEGs within the CR2 data
    let mut jpeg_positions = vec![];
    for (i, window) in data.windows(2).enumerate() {
        if window == b"\xFF\xD8" {
            jpeg_positions.push(i);
        }
    }

    // Extract JPEG data from CR2 based on the identified positions
    let mut jpeg_images = Vec::new();
    for i in 0..jpeg_positions.len() {
      if i < jpeg_positions.len() - 1 {
        let start = jpeg_positions[i];
        let end = jpeg_positions[i + 1];
        let jpeg_data = &data[start..end];

        // Parse the JPEG data into a DynamicImage
        let dynamic_image = image::load_from_memory_with_format(jpeg_data, ImageFormat::Jpeg)?;
        jpeg_images.push(dynamic_image);
      }
    }

    if jpeg_images.len() > 1 {
      return Ok(jpeg_images[image_index].clone());
    }

    Err(RawError::ExtractThumbnail)
  }
}

impl RawImage for Any {
  fn new(file: File) -> Any {
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

		reader.read_to_end(&mut buffer).unwrap();

    Any { buffer: buffer }
  }

  fn get_thumbnail(&self) -> RawResult<image::DynamicImage> {
    Err(RawError::ExtractThumbnail)
  }
}
