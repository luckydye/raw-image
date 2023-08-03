use image::DynamicImage;
use image::ImageError;
use std::fs::File;

pub trait ThumbnailImage {
	fn new(file: File) -> Self;

	fn get_thumbnail(&self) -> Result<Option<DynamicImage>, ImageError>;
}

pub mod arw;
pub mod cr2;
pub mod cr3;
pub mod nef;
pub mod tiff;
