use super::tiff::{self, Tags};
use super::ThumbnailImage;

use image::{io::Reader as ImageReader, DynamicImage, ImageError};
use std::fs::File;
use std::io::{BufReader, Cursor, Read};

pub struct Nef {
	buffer: Vec<u8>,
}

enum NefTags {
	JpgFromRawStart = 513,
	JpgFromRawLength = 514,
}

impl Nef {
	fn parse_header(&self) -> tiff::Fileheader {
		tiff::Tiff::parse_header(&self.buffer)
	}

	fn parse_ifd(&self, offset: usize) -> tiff::IFD {
		tiff::Tiff::parse_ifd(&self.buffer, offset)
	}

	pub fn get_jpeg(&self, ifd: tiff::IFD) -> Option<tiff::Thumbnail> {
		let offset_tag = ifd.get_tag(NefTags::JpgFromRawStart as u16);
		let length_tag = ifd.get_tag(NefTags::JpgFromRawLength as u16);

		if offset_tag.is_some() && length_tag.is_some() {
			let offset = u32::from_le_bytes(offset_tag.unwrap().value[0..4].try_into().unwrap());
			let length = u32::from_le_bytes(length_tag.unwrap().value[0..4].try_into().unwrap());

			return Some(tiff::Thumbnail {
				offset: offset as usize,
				length: length as usize,
			});
		}

		return None;
	}
}

impl ThumbnailImage for Nef {
	fn new(file: File) -> Nef {
		let mut reader = BufReader::new(file);
		let mut buffer = Vec::new();
		reader.read_to_end(&mut buffer).unwrap();
		Nef { buffer }
	}

	fn get_thumbnail(&self) -> Result<Option<DynamicImage>, ImageError> {
		let header = self.parse_header();
		let offset = header.ifd_offset as usize;

		let ifd = self.parse_ifd(offset);

		let sub_ifd_tag = ifd.get_tag(Tags::SubIDF as u16);
		if sub_ifd_tag.is_some() {
			let sub_ifd0_offset =
				u32::from_le_bytes(sub_ifd_tag.unwrap().value[0..4].try_into().unwrap());

			let sub_ifd = self.parse_ifd(sub_ifd0_offset as usize);

			let thumbnail = self.get_jpeg(sub_ifd);
			if thumbnail.is_some() {
				let thumb = thumbnail.unwrap();
				let image_data = &self.buffer[thumb.offset..thumb.offset + thumb.length];

				return Ok(Some(
					ImageReader::new(Cursor::new(image_data))
						.with_guessed_format()?
						.decode()?,
				));
			}
		}

		Ok(None)
	}
}
