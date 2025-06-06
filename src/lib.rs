use image::DynamicImage;

use crate::algorithm::{ahash, remove_borders};
use std::path::Path;

pub mod algorithm;

/// Computes a perceptual hash of a `DynamicImage` using the ahash (average hash) algorithm.
///
/// ## Arguments
///
/// * `image` - DynamicImage (already loaded from file)
/// * `remove_white_border` - Whether to remove white borders from the image before computing the hash
///
/// ## Returns
///
/// * `Ok(String)` - The computed perceptual hash as a hex string
/// * `Err(String)` - Error message if the hash couldn't be computed
pub fn compute_hash(image: DynamicImage, remove_white_border: bool) -> Result<String, String> {
    let processed_img: DynamicImage = if remove_white_border {
        remove_borders::remove_white_borders(&image)
    } else {
        image
    };
    Ok(ahash::compute_image_hash(&processed_img))
}

/// Computes a perceptual hash of an image file using the ahash (average hash) algorithm.
///
/// This function removes the image white borders before hash computation.
///
/// ## Arguments
///
/// * `path` - Path to the image file
///
/// ## Returns
///
/// * `Ok(String)` - The computed perceptual hash as a hex string
/// * `Err(String)` - Error message if the image couldn't be opened
pub fn compute_hash_from_file<P: AsRef<Path>>(path: P) -> Result<String, String> {
    match image::open(path) {
        Ok(img) => compute_hash(img, false),
        Err(e) => Err(format!("Failed to open image: {}", e)),
    }
}
