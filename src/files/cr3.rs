use super::{RawError, RawResult, ThumbnailImage};

use std::{
	fs::File,
	io::{BufReader, Cursor, Read},
};

use image::io::Reader as ImageReader;

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
const UUID_MOOV: &str = "85c0b68782f11e08111f4ce462b6a48";

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

	fn prase_box_uuid(&self, offset: usize) -> String {
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
		// offset += 16;

		return uuid;
	}

	fn parse_box_preview(&self, offset: usize) -> RawResult<image::DynamicImage> {
		let buffer = &self.buffer;

		let mut offset = offset + 11 + 1 + 4 + 2 + 2 + 2;

		// let width = u16::from_be_bytes(buffer[offset..offset+2].try_into().unwrap());
		// let height = u16::from_be_bytes(buffer[offset+2..offset+4].try_into().unwrap());
		offset += 4;
		offset += 2;

		let byte_size = u32::from_be_bytes(buffer[offset..offset + 4].try_into().unwrap());
		offset += 4;

		let image_data = &buffer[offset..offset + usize::try_from(byte_size).unwrap()];

		let img = ImageReader::new(Cursor::new(image_data))
			.with_guessed_format()?
			.decode()?;

		Ok(img)
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

		for _ in 0..4 {
			let mut index = offset;
			let cr3_box = self.parse_box(index);
			index += 8;

			if cr3_box.box_type == "uuid" {
				let uuid = self.prase_box_uuid(index);
				index += 16;

				if uuid == UUID_MOOV {
					// skip
				}

				if uuid == UUID_PRVW {
					return Ok(self.parse_box_preview(index)?);
				}
			}

			offset += cr3_box.size;
		}

		Err(RawError::ExtractThumbnail)
	}
}
