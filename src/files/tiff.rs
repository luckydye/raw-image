#[derive(Debug)]
pub struct IFD {
	pub tags: Vec<Tag>,
	pub next: usize,
}

#[derive(Debug)]
pub struct Strip {
	pub offset: usize,
	pub length: usize,
	pub compression: u16,
}

pub struct Thumbnail {
	pub offset: usize,
	pub length: usize,
}

impl IFD {
	pub fn get_tag(&self, tag: u16) -> Option<&Tag> {
		self.tags.iter().find(|t| t.tag == tag)
	}

	pub fn get_thumbnail(&self) -> Option<Thumbnail> {
		let offset_tag = self.get_tag(Tags::ThumbnailOffset as u16);
		let length_tag = self.get_tag(Tags::ThumbnailLength as u16);

		if offset_tag.is_some() && length_tag.is_some() {
			let offset = u32::from_le_bytes(offset_tag.unwrap().value[0..4].try_into().unwrap());
			let length = u32::from_le_bytes(length_tag.unwrap().value[0..4].try_into().unwrap());

			return Some(Thumbnail {
				offset: offset as usize,
				length: length as usize,
			});
		}

		return None;
	}

	pub fn get_strips(&self) -> Vec<Strip> {
		let mut strips = Vec::new();

		let compression_tag = self.get_tag(Tags::Compression as u16);
		let mut compression = 0;
		if compression_tag.is_some() {
			compression =
				u16::from_le_bytes(compression_tag.unwrap().value[0..2].try_into().unwrap());
		}

		let offsets = self
			.tags
			.iter()
			.find(|tag| tag.tag == Tags::StripOffsets as u16);

		let lengths = self
			.tags
			.iter()
			.find(|tag| tag.tag == Tags::StripByteCounts as u16);

		if lengths.is_some() && offsets.is_some() {
			for i in 0..lengths.unwrap().count {
				let offset = u32::from_le_bytes(
					offsets.unwrap().value[i as usize * 4..i as usize * 4 + 4]
						.try_into()
						.unwrap(),
				);
				let length = u32::from_le_bytes(
					lengths.unwrap().value[i as usize * 4..i as usize * 4 + 4]
						.try_into()
						.unwrap(),
				);

				strips.push(Strip {
					offset: offset as usize,
					length: length as usize,
					compression,
				});
			}
		}

		return strips;
	}
}

// pub enum Subfile {
// 	FullResolutionImage = 1,
// 	ReducedResolutionImage = 2,
// 	SinglePageOfMultiPageImage = 3,
// }

// pub enum Compression {
// 	Uncompressed = 1,
// 	CCITT1D = 2,
// 	CCITTGroup3 = 3,
// 	CCITTGroup4 = 4,
// 	LZW = 5,
// 	JPEGOldStyle = 6,
// 	JPEG = 7,
// 	Deflate = 8,
// 	JBIGBAndW = 9,
// 	JBIGColor = 10,
// 	JPEG2000 = 11,
// }

pub enum Tags {
	// NewSubfileType = 254,
	// SubfileType = 255,
	// ImageWidth = 256,
	// ImageLength = 257,
	// BitsPerSample = 258,
	Compression = 259,

	// PhotometricInterpretation = 262,
	StripOffsets = 273,
	// Orientation = 274,

	// SamplesPerPixel = 277,
	// RowsPerStrip = 278,
	StripByteCounts = 279,

	// XResolution = 282,
	// YResolution = 283,

	// TileWidth = 322,
	// TileLength = 323,
	// TileOffsets = 324,
	// TileByteCounts = 325,
	ThumbnailOffset = 513,
	ThumbnailLength = 514,

	SubIDF = 330,
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

pub struct Tiff {}

impl Tiff {
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
			// its possible for the value to be stored inside the offset field
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

		let mut tags = Vec::new();

		for i in 0..count {
			tags.push(Tiff::parse_tag(&buffer, offset + 2 + (i as usize * 12)));
		}

		let size = usize::from(count) * 12;
		let next_offset_int = offset + 2 + size;
		let next_offset = u32::from_le_bytes(
			buffer[next_offset_int..next_offset_int + 4]
				.try_into()
				.unwrap(),
		);

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
