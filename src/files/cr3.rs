use super::{RawError, RawResult, ThumbnailImage};

use image::io::Reader as ImageReader;
use std::{
	fs::File,
	io::{BufReader, Cursor, Read},
};

//
// Reference: https://github.com/lclevy/canon_cr3
//

pub struct Cr3 {
	buffer: Vec<u8>,
}

struct Cr3Box {
	size: usize,
	box_type: String,
}

const UUID_PRVW: &str = "eaf42b5e1c984b88b9fbb7dc406e4d16";

impl Cr3 {
	fn parse_box(&self, offset: usize) -> Cr3Box {
		let buffer = &self.buffer;

		let file_type_box_size = u32::from_be_bytes(buffer[offset..offset + 4].try_into().unwrap());
		let box_type = std::str::from_utf8(buffer[offset + 4..offset + 8].try_into().unwrap());

		Cr3Box {
			size: usize::try_from(file_type_box_size).unwrap(),
			box_type: String::from(box_type.unwrap()),
		}
	}

	fn prase_uuid(&self, offset: usize) -> String {
		let buffer = &self.buffer;

		let mut uuid = String::new();
		let mut uuid_offset = offset;
		for _ in 0..16 {
			let hex = format!(
				"{:x}",
				u8::from_be_bytes(buffer[uuid_offset..uuid_offset + 1].try_into().unwrap())
			);
			uuid_offset += 1;
			uuid += &String::from(hex);
		}

		return uuid;
	}
}

impl ThumbnailImage for Cr3 {
	fn new(file: File) -> Cr3 {
		let mut reader = BufReader::new(file);
		let mut buffer = Vec::new();

		reader.read_to_end(&mut buffer).unwrap();

		Cr3 { buffer: buffer }
	}

	fn get_thumbnail(&self) -> RawResult<image::DynamicImage> {
		let mut offset: usize = 0;

		for _ in 0..6 {
			if offset >= self.buffer.len() && offset > 0 {
				break;
			}

			let mut sub_offset = offset;
			let cr3_box = self.parse_box(sub_offset);
			sub_offset += 8;

			if cr3_box.box_type == "uuid" {
				let uuid = self.prase_uuid(sub_offset);

				if uuid == UUID_PRVW {
					let mut offset = offset + 11 + 1 + 4 + 2 + 2 + 2 + 4 + 2;
					let byte_size =
						u32::from_be_bytes(self.buffer[offset..offset + 4].try_into().unwrap());
					offset += 4;

					let image_data =
						&self.buffer[offset..offset + usize::try_from(byte_size).unwrap()];

					let img = ImageReader::new(Cursor::new(image_data))
						.with_guessed_format()?
						.decode()?;

					// small pp
					return Ok(img);
				}
			}

			offset += cr3_box.size;
		}

		Err(RawError::ExtractThumbnail)
	}
}
