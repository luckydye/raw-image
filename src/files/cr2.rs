use super::tiff;
use super::ThumbnailImage;

use image::{io::Reader as ImageReader, DynamicImage, ImageError};
use std::fs::File;
use std::io::{BufReader, Cursor, Read};

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

	fn get_thumbnail(&self) -> Result<Option<DynamicImage>, ImageError> {
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

					return Ok(Some(
						ImageReader::new(Cursor::new(image_data))
							.with_guessed_format()?
							.decode()?,
					));
				}
			}

			offset = ifd.next;
			ifd_index += 1;
		}

		Ok(None)
	}
}
