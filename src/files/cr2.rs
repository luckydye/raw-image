use super::tiff::{self, Tags};
use super::{RawError, RawResult, ThumbnailImage};

use image::io::Reader as ImageReader;
use image::DynamicImage;
use std::fs::File;
use std::io::Cursor;
use std::io::{BufReader, Read};

pub struct Cr2 {
	buffer: Vec<u8>,
}

impl Cr2 {
	fn parse_header(&self) -> tiff::Fileheader {
		tiff::Tiff::parse_header(&self.buffer)
	}

	fn parse_ifd(&self, offset: usize) -> tiff::IFD {
		tiff::Tiff::parse_ifd(&self.buffer, offset)
	}
}

impl ThumbnailImage for Cr2 {
	fn new(file: File) -> Cr2 {
		let mut reader = BufReader::new(file);
		let mut buffer = Vec::new();
		reader.read_to_end(&mut buffer).unwrap();
		Cr2 { buffer }
	}

	fn get_thumbnail(&self) -> RawResult<DynamicImage> {
		let header = self.parse_header();
		let mut offset = header.ifd_offset as usize;

		let mut ifd_index = 0;
		while offset > 0 && offset < self.buffer.len() {
			let ifd = self.parse_ifd(offset);

			if ifd_index == 1 {
				let strips = ifd.get_strips();
				if strips.len() > 0 {
					let strip = &strips[0];
					let image_data = &self.buffer[strip.offset..strip.offset + strip.length];

					return Ok(ImageReader::new(Cursor::new(image_data))
						.with_guessed_format()?
						.decode()?);
				}
			}

			offset = ifd.next;
			ifd_index += 1;
		}

		Err(RawError::ExtractThumbnail)
	}
}
