use super::tiff;
use super::{RawError, RawResult, ThumbnailImage};

use image::io::Reader as ImageReader;
use image::DynamicImage;
use std::fs::File;
use std::io::Cursor;
use std::io::{BufReader, Read};

pub struct Arw {
	buffer: Vec<u8>,
}

impl Arw {
	fn parse_header(&self) -> tiff::Fileheader {
		tiff::Tiff::parse_header(&self.buffer)
	}

	fn parse_ifd(&self, offset: usize) -> tiff::IFD {
		tiff::Tiff::parse_ifd(&self.buffer, offset)
	}
}

impl ThumbnailImage for Arw {
	fn new(file: File) -> Arw {
		let mut reader = BufReader::new(file);
		let mut buffer = Vec::new();
		reader.read_to_end(&mut buffer).unwrap();
		Arw { buffer }
	}

	fn get_thumbnail(&self) -> RawResult<DynamicImage> {
		let header = self.parse_header();
		let mut offset = header.ifd_offset as usize;

		while offset > 0 && offset < self.buffer.len() {
			let ifd = self.parse_ifd(offset);

			let thumbnail = ifd.get_thumbnail();
			if thumbnail.is_some() {
				let thumb = thumbnail.unwrap();
				let image_data = &self.buffer[thumb.offset..thumb.offset + thumb.length];

				return Ok(ImageReader::new(Cursor::new(image_data))
					.with_guessed_format()?
					.decode()?);
			}

			offset = ifd.next;
		}

		Err(RawError::ExtractThumbnail)
	}
}
