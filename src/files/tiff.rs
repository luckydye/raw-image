#[derive(Debug)]
pub struct IFD {
	pub tags: Vec<Tag>,
	pub next: usize,
	// implement ifd methods here
}

#[derive(Debug)]
pub struct Tag {
	pub tag: u16,
	pub tag_type: u16,
	pub count: u32,
	pub value: Vec<u8>,
}

#[derive(Debug)]
pub struct Fileheader {
	pub byte_order: String,
	pub number: u16,
	pub ifd_offset: u16,
}

pub struct Tiff {
	buffer: Vec<u8>,
}

impl Tiff {
	pub fn parse_tag_name(tag: u16) -> &'static str {
		match tag {
			254 => "NewSubfileType",
			255 => "SubfileType",
			256 => "ImageWidth",
			257 => "ImageLength",
			258 => "BitsPerSample",
			259 => "Compression",

			262 => "PhotometricInterpretation",

			273 => "StripOffsets",
			274 => "Orientation",

			277 => "SamplesPerPixel",
			278 => "RowsPerStrip",
			279 => "StripByteCounts",

			282 => "XResolution",
			283 => "YResolution",

			322 => "TileWidth",
			323 => "TileLength",
			324 => "TileOffsets",
			325 => "TileByteCounts",

			_ => "Unknown",
		}
	}

	pub fn parse_tag(buffer: &Vec<u8>, offset: usize) -> Tag {
		let tag = u16::from_le_bytes(buffer[offset..offset + 2].try_into().unwrap());
		let entry_type = u16::from_le_bytes(buffer[offset + 2..offset + 4].try_into().unwrap());
		let count = u32::from_le_bytes(buffer[offset + 4..offset + 8].try_into().unwrap());
		let mut value_offset =
			u32::from_le_bytes(buffer[offset + 8..offset + 12].try_into().unwrap());

		let tag_type_size = match entry_type {
			1 => 1,
			2 => 1,
			3 => 2,
			4 => 4,
			5 => 8,
			7 => 1,
			9 => 4,
			10 => 8,
			12 => 8,
			13 => 4,
			_ => 0,
		};

		if count * tag_type_size <= 4 {
			value_offset = (offset + 8) as u32;
		}

		Tag {
			tag: tag,
			tag_type: entry_type,
			count: count,
			value: buffer[value_offset as usize
				..value_offset as usize + (count as usize * tag_type_size as usize)]
				.to_vec(),
		}
	}

	pub fn parse_ifd(buffer: &Vec<u8>, offset: usize) -> IFD {
		let count = u16::from_le_bytes(buffer[offset..offset + 2].try_into().unwrap());
		let size = usize::from(count) * 12;
		let next_offset_int = offset + 2 + size;
		let next_offset = u32::from_le_bytes(
			buffer[next_offset_int..next_offset_int + 4]
				.try_into()
				.unwrap(),
		);

		let mut tags = Vec::new();

		for i in 0..count {
			tags.push(Tiff::parse_tag(&buffer, offset + 2 + (i as usize * 12)));
		}

		IFD {
			tags: tags,
			next: usize::try_from(next_offset).unwrap(),
		}
	}

	pub fn parse_header(buffer: &Vec<u8>) -> Fileheader {
		Fileheader {
			byte_order: String::from(
				std::str::from_utf8(buffer[0..2].try_into().unwrap()).unwrap(),
			),
			number: u16::from_le_bytes(buffer[2..4].try_into().unwrap()),
			ifd_offset: u16::from_le_bytes(buffer[4..6].try_into().unwrap()),
		}
	}
}

// impl ThumbnailImage for Tiff {
// 	fn new(file: File) -> Tiff {
// 		let mut reader = BufReader::new(file);
// 		let mut buffer = Vec::new();

// 		reader.read_to_end(&mut buffer).unwrap();

// 		Tiff { buffer: buffer }
// 	}

// 	fn get_thumbnail(&self) -> RawResult<DynamicImage> {
// 		let header = self.parse_header();
// 		let mut offset: usize = usize::from(header.ifd_offset).try_into().unwrap();

// 		for _ in 0..3 {
// 			let ifd = self.parse_ifd(offset);

// 			println!("ifd: {:?} tags: {:?}", offset, ifd.tags.len());

// 			// for tag in &ifd.tags {
// 			// 	if tag.name == "Orientation" {
// 			// 		println!("tag: {:?}", tag);
// 			// 	}
// 			// 	if tag.name == "ImageWidth" {
// 			// 		println!("tag: {:?}", tag);
// 			// 	}
// 			// 	if tag.name == "StripOffsets" {
// 			// 		println!("tag: {:?}", tag);
// 			// 	}
// 			// 	if tag.name == "TileOffsets" {
// 			// 		println!("tag: {:?}", tag);
// 			// 	}
// 			// 	if tag.name == "NewSubfileType" {
// 			// 		println!("tag: {:?}", tag);
// 			// 	}
// 			// 	if tag.name == "Compression" {
// 			// 		println!("tag: {:?}", tag);
// 			// 	}
// 			// }

// 			offset = ifd.next;
// 		}

// 		Err(RawError::ExtractThumbnail)
// 	}
// }
