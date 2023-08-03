use super::ThumbnailImage;

use image::{io::Reader as ImageReader, DynamicImage, ImageError};
use std::fs::File;
use std::io::{BufReader, Cursor, Read};

//
// Reference: https://github.com/lclevy/canon_cr3
//

pub struct Cr3 {
	buffer: Vec<u8>,
}

#[derive(Debug)]
struct Cr3Box {
	size: usize,
	box_type: String,
}

const UUID_PRVW: &str = "eaf42b5e1c984b88b9fbb7dc406e4d16";

impl Cr3 {
	fn parse_box(&self, offset: &mut usize) -> Cr3Box {
		let buffer = &self.buffer;

		let file_type_box_size =
			u32::from_be_bytes(buffer[*offset..*offset + 4].try_into().unwrap());
		let box_type = std::str::from_utf8(buffer[*offset + 4..*offset + 8].try_into().unwrap());

		*offset += 8;

		Cr3Box {
			size: file_type_box_size as usize,
			box_type: String::from(box_type.unwrap()),
		}
	}

	fn prase_hex(&self, offset: &mut usize, len: usize) -> String {
		let buffer = &self.buffer;

		let mut uuid = String::new();
		for i in 0..len {
			let hex = format!(
				"{:x}",
				u8::from_be_bytes(buffer[*offset + i..*offset + i + 1].try_into().unwrap())
			);
			uuid += &String::from(hex);
		}

		*offset += len;

		return uuid;
	}
}

impl ThumbnailImage for Cr3 {
	fn new(file: File) -> Cr3 {
		let mut reader = BufReader::new(file);
		let mut buffer = Vec::new();

		reader.read_to_end(&mut buffer).unwrap();

		Cr3 { buffer }
	}

	fn get_thumbnail(&self) -> Result<Option<DynamicImage>, ImageError> {
		let mut offset: usize = 0;

		for _ in 0..6 {
			if offset >= self.buffer.len() && offset > 0 {
				break;
			}

			let mut box_offset = offset.clone();
			let cr3_box = self.parse_box(&mut box_offset);

			if cr3_box.box_type == "uuid" {
				let uuid = self.prase_hex(&mut box_offset, 16);

				if uuid == UUID_PRVW {
					box_offset += 28;
					let byte_size = u32::from_be_bytes(
						self.buffer[box_offset..box_offset + 4].try_into().unwrap(),
					);
					box_offset += 4;

					let image_data = &self.buffer[box_offset..box_offset + byte_size as usize];

					return Ok(Some(
						ImageReader::new(Cursor::new(image_data))
							.with_guessed_format()?
							.decode()?,
					));
				}
			}

			offset += cr3_box.size;
		}

		Ok(None)
	}
}
