use super::tiff;
use super::{RawError, RawResult, ThumbnailImage};

use image::io::Reader as ImageReader;
use image::DynamicImage;
use std::fs::File;
use std::io::{BufReader, Cursor, Read};

pub struct Cr2 {
	buffer: Vec<u8>,
}

impl Cr2 {
	fn parse_tag_name(&self, tag: u16) -> &str {
		match tag {
			// Canon Raw (CR2)
			513 => "ThumbnailOffset",
			514 => "ThumbnailLength",

			_ => tiff::Tiff::parse_tag_name(tag),
		}
	}

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

		Cr2 { buffer: buffer }
	}

	fn get_thumbnail(&self) -> RawResult<DynamicImage> {
		let header = self.parse_header();
		let mut offset: usize = usize::from(header.ifd_offset).try_into().unwrap();

		for _ in 0..3 {
			let ifd = self.parse_ifd(offset);

			println!("ifd: {:?} tags: {:?}", offset, ifd.tags.len());

			let thumb_offset = ifd
				.tags
				.iter()
				.find(|tag| self.parse_tag_name(tag.tag) == "ThumbnailOffset");
			let thumb_len = ifd
				.tags
				.iter()
				.find(|tag| self.parse_tag_name(tag.tag) == "ThumbnailLength");

			if thumb_offset.is_some() || thumb_len.is_some() {
				let len =
					u32::from_le_bytes(thumb_len.unwrap().value[0..4].try_into().unwrap()) as usize;
				let offset =
					u32::from_le_bytes(thumb_offset.unwrap().value[0..4].try_into().unwrap())
						as usize;

				let image_data = &self.buffer[offset..offset + len];
				let img = ImageReader::new(Cursor::new(image_data))
					.with_guessed_format()?
					.decode()?;

				return Ok(img);
			}

			// for tag in &ifd.tags {
			// 	if tag.name == "Orientation" {
			// 		println!("tag: {:?}", tag);
			// 	}
			// 	if tag.name == "ImageWidth" {
			// 		println!("tag: {:?}", tag);
			// 	}
			// 	if tag.name == "StripOffsets" {
			// 		println!("tag: {:?}", tag);
			// 	}
			// 	if tag.name == "TileOffsets" {
			// 		println!("tag: {:?}", tag);
			// 	}
			// 	if tag.name == "NewSubfileType" {
			// 		println!("tag: {:?}", tag);
			// 	}
			// 	if tag.name == "Compression" {
			// 		println!("tag: {:?}", tag);
			// 	}
			// }

			offset = ifd.next;
		}

		Err(RawError::ExtractThumbnail)
	}
}
