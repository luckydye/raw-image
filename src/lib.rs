mod files;

use files::arw::Arw;
use files::cr2::Cr2;
use files::cr3::Cr3;
use files::nef::Nef;
use files::ThumbnailImage;

use std::{fs, fs::File, path::Path};

use image::DynamicImage;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImagesError {
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
	#[error("invalid extension")]
	InvalidExtension,
	#[error("failed to extract thumbnail from file")]
	ExtractThumbnail,
}

/// The maximum file size that an image can be in order to have a thumbnail generated.
///
/// This value is in MiB.
const RAW_MAXIMUM_FILE_SIZE: u64 = 1048576 * 100;

pub const EXTENSIONS: [&str; 4] = ["cr2", "cr3", "nef", "arw"];

pub fn raw_to_dynamic_image(path: &Path) -> Result<DynamicImage, ImagesError> {
	if fs::metadata(path)?.len() > RAW_MAXIMUM_FILE_SIZE {
		return Err(ImagesError::TooLarge);
	}

	let img = {
		let ext = path.extension().ok_or(ImagesError::InvalidExtension)?;
		let p = path.to_str().ok_or(ImagesError::InvalidPath)?;
		let file = File::open(p)?;

		match ext.to_ascii_lowercase().to_str() {
			Some("cr3") => Cr3::new(file).get_thumbnail(),
			Some("cr2") => Cr2::new(file).get_thumbnail(),
			Some("arw") => Arw::new(file).get_thumbnail(),
			Some("nef") => Nef::new(file).get_thumbnail(),
			_ => return Err(ImagesError::Unsupported),
		}
	};

	match img? {
		Some(img) => Ok(img),
		None => Err(ImagesError::ExtractThumbnail),
	}
}
