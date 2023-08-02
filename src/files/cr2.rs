use super::{RawImage, RawResult, RawError};

use std::fs::File;
use std::io::{BufReader, Read};
use image::DynamicImage;

pub struct Cr2 {
  buffer: Vec<u8>,
}

struct IFD {
  entry_count: u16,
}

struct Entry {
  tag: u16,
  entry_type: u16,
  count: u32,
  value_offset: u32,
}

#[derive(Debug)]
struct Fileheader {
  byte_order: String,
  number: u16,
  ifd_offset: u16,
}

impl Cr2 {
  fn parse_ifd(&self, offset: usize) -> IFD {
    let buffer = &self.buffer;

    IFD {
      entry_count: u16::from_be_bytes(buffer[offset..offset+4].try_into().unwrap()),
    }
  }

  fn parse_header(&self) -> Fileheader {
    let buffer = &self.buffer;

    Fileheader {
      byte_order: String::from(std::str::from_utf8(buffer[0..2].try_into().unwrap()).unwrap()),
      number: u16::from_be_bytes(buffer[2..4].try_into().unwrap()),
      ifd_offset: u16::from_be_bytes(buffer[4..6].try_into().unwrap()),
    }
  }
}

impl RawImage for Cr2 {
  fn new(file: File) -> Cr2 {
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

		reader.read_to_end(&mut buffer).unwrap();

    Cr2 { buffer: buffer }
  }

  fn get_thumbnail(&self) -> RawResult<DynamicImage> {
    let mut offset: usize = 0;

    let header = self.parse_header();

    println!("header: {:?}", header);

    // for _ in 0..4 {
    //   let mut index = offset;
    //   let ifd = self.parse_ifd(index);

    //   // offset += ifd.size;
    // }

    Err(RawError::ExtractThumbnail)
  }
}
