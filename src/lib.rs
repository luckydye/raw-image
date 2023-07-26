mod files;
use std::path::Path;

pub fn raw_to_dynamic_image(path: &Path) -> files::RawResult<image::DynamicImage> {
	files::raw_to_dynamic_image(path)
}
