use super::{RawImage, RawResult, RawError};

use std::io::Cursor;
use image::io::Reader as ImageReader;

//
// Reference: https://github.com/lclevy/canon_cr3
//

pub struct Cr3 {
  buffer: Vec<u8>,
}

const UUID_PRVW: &str = "eaf42b5e1c984b88b9fbb7dc406e4d16";

impl Cr3 {}

impl RawImage for Cr3 {
  fn new(buffer: Vec<u8>) -> Cr3 {
    Cr3 { buffer: buffer }
  }

  fn get_thumbnail(&self) -> RawResult<image::DynamicImage> {
    let buffer = &self.buffer;

    let mut offset: usize = 0;

    let file_type_box_size = u32::from_be_bytes(buffer[offset..offset+4].try_into().unwrap());
    // let box_type = std::str::from_utf8(buffer[offset+4..offset+8].try_into().unwrap());

    offset = offset + usize::try_from(file_type_box_size).unwrap();

    let moov_box_size = u32::from_be_bytes(buffer[offset..offset+4].try_into().unwrap());

    offset = offset + usize::try_from(moov_box_size).unwrap();

    let xpacket_box_size = u32::from_be_bytes(buffer[offset..offset+4].try_into().unwrap());

    offset = offset + usize::try_from(xpacket_box_size).unwrap();

    // let preview_box_size = u32::from_be_bytes(buffer[offset..offset+4].try_into().unwrap());
    // let preview_type = std::str::from_utf8(buffer[offset+4..offset+8].try_into().unwrap());

    offset += 8;

    let mut uuid = String::new();
    let mut uuid_offset = offset;
    for _ in 0..16 {
      let hex = format!("{:x}", u8::from_be_bytes(buffer[uuid_offset..uuid_offset+1].try_into().unwrap()));
      uuid_offset += 1;
      uuid += &String::from(hex);
    }
    offset += 16;

    if uuid == UUID_PRVW {
      offset += 11 + 1 + 4 + 2 + 2 + 2;

      let width = u16::from_be_bytes(buffer[offset..offset+2].try_into().unwrap());
      let height = u16::from_be_bytes(buffer[offset+2..offset+4].try_into().unwrap());
      offset += 4;
      offset += 2;

      let byte_size = u32::from_be_bytes(buffer[offset..offset+4].try_into().unwrap());
      offset += 4;

      let image_data = &buffer[offset..offset+usize::try_from(byte_size).unwrap()];

      let jpeg_img = ImageReader::new(Cursor::new(image_data)).with_guessed_format()?.decode()?;

      println!("<Thumbnail> width: {:?} height: {:?} size: {:?}", width, height, byte_size);

      return Ok(jpeg_img.rotate270());
    }

    Err(RawError::ExtractThumbnail)
  }
}
