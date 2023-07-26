pub trait RawImage {
    fn new(buffer: Vec<u8>) -> Self;

    fn get_thumbnail(&self) -> RawResult<image::DynamicImage>;
}

mod cr3;

use std::{
	fs,
	fs::File,
	io::{Read, BufReader},
	path::Path,
};

use image::DynamicImage;
use thiserror::Error;

/// The maximum file size that an image can be in order to have a thumbnail generated.
///
/// This value is in MiB.
const RAW_MAXIMUM_FILE_SIZE: u64 = 1048576 * 100;

#[derive(Error, Debug)]
pub enum RawError {
	#[error("error while loading the image (via the `image` crate): {0}")]
	Image(#[from] image::ImageError),
	#[error("io error: {0}")]
	Io(#[from] std::io::Error),
	#[error("the image provided is unsupported")]
	Unsupported,
	#[error("the image provided is too large (over 100MiB)")]
	TooLarge,
	#[error("invalid path provided (non UTF-8)")]
	InvalidPath,
	#[error("failed to extract thumbnail from file")]
	ExtractThumbnail,
	#[error("idk")]
	OK,
}

pub type RawResult<T> = Result<T, RawError>;

pub fn raw_to_dynamic_image(path: &Path) -> RawResult<DynamicImage> {
	if fs::metadata(path)?.len() > RAW_MAXIMUM_FILE_SIZE {
		return Err(RawError::TooLarge);
	}

	let img: DynamicImage = {
		let p = path.to_str().ok_or(RawError::InvalidPath)?;
		let f = File::open(p)?;
		let ext = path.extension().ok_or(RawError::OK)?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

		reader.read_to_end(&mut buffer)?;

		match ext.to_ascii_lowercase().to_str() {
			Some("cr3") => cr3::Cr3::new(buffer).get_thumbnail().unwrap(),
			// Some("tiff") => tiff::Tiff::new(buffer).get_thumbnail(),
			// Some("cr2") => cr2::Cr2::new(buffer).get_thumbnail(),
			// Some("dng") => dng::Dng::new(buffer).get_thumbnail(),
			// Some("arw") => arw::Arw::new(buffer).get_thumbnail(),
			// Some("nef") => nef::Nef::new(buffer).get_thumbnail(),
			_ => {
				return Err(RawError::Unsupported)
			},
		}
	};

	Ok(img)
}
