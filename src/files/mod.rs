pub trait ThumbnailImage {
	fn new(file: File) -> Self;

	fn get_thumbnail(&self) -> RawResult<image::DynamicImage>;
}

mod arw;
mod cr2;
mod cr3;
mod tiff;

use std::{fs, fs::File, path::Path};

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
		let ext = path.extension().ok_or(RawError::OK)?;
		let p = path.to_str().ok_or(RawError::InvalidPath)?;
		let file = File::open(p)?;

		match ext.to_ascii_lowercase().to_str() {
			Some("cr3") => cr3::Cr3::new(file).get_thumbnail().unwrap(),
			Some("cr2") => cr2::Cr2::new(file).get_thumbnail().unwrap(),
			Some("arw") => arw::Arw::new(file).get_thumbnail().unwrap(),
			// Some("nef") => any::Any::new(file).get_thumbnail().unwrap(),
			// Some("dng") => any::Any::new(file).get_thumbnail().unwrap(),
			_ => return Err(RawError::Unsupported),
		}
	};

	Ok(img)
}
