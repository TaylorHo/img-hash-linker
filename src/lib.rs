use image::DynamicImage;

use crate::algorithm::{ahash, remove_borders};
use std::path::Path;

pub mod algorithm;

fn compute_hash<P: AsRef<Path>>(path: P, remove_white_border: bool) -> Result<String, String> {
    match image::open(path) {
        Ok(img) => {
            let processed_img: DynamicImage = if remove_white_border {
                remove_borders::remove_white_borders(&img)
            } else {
                img
            };
            Ok(ahash::compute_image_hash(&processed_img))
        }
        Err(e) => Err(format!("Failed to open image: {}", e)),
    }
}

pub fn compute_hash_from_file<P: AsRef<Path>>(path: P) -> Result<String, String> {
    compute_hash(path, false)
}

pub fn compute_hash_from_file_without_removing_borders<P: AsRef<Path>>(
    path: P,
) -> Result<String, String> {
    compute_hash(path, true)
}
