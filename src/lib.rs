use fast_image_resize::{self as fir, Resizer};
use image::DynamicImage;
use std::path::Path;

/// Computes the average hash (aHash) of an image.
///
/// The algorithm:
/// 1. Resize the image to 8x8
/// 2. Convert to grayscale
/// 3. Calculate the average pixel value
/// 4. Compare each pixel to the average and set bits accordingly
/// 5. Return a 64-bit hash as a hexadecimal string
pub fn compute_ahash(img: &DynamicImage) -> String {
    // Convert to grayscale
    let gray_img: DynamicImage = img.grayscale();

    // Create resizer and resize to 8x8
    let mut resizer: Resizer = fir::Resizer::new();
    let dst_width: u32 = 8;
    let dst_height: u32 = 8;

    // Create destination image
    let mut dst_image = fir::images::Image::new(dst_width, dst_height, fir::PixelType::U8);

    // Resize image
    resizer.resize(&gray_img, &mut dst_image, None).unwrap();

    // Get the resized pixels
    let pixels: &[u8] = dst_image.buffer();

    // Calculate average
    let mut sum: u32 = 0;
    for &p in pixels {
        sum += p as u32;
    }
    let avg: u32 = sum / 64;

    // Create 64-bit hash (as u64)
    let mut hash: u64 = 0;
    for (i, &pixel) in pixels.iter().enumerate() {
        if pixel as u32 >= avg {
            hash |= 1 << i;
        }
    }

    // Convert to hex string
    format!("{:016x}", hash)
}

/// Computes the average hash (aHash) of an image from a file.
pub fn compute_ahash_from_file<P: AsRef<Path>>(path: P) -> Result<String, String> {
    match image::open(path) {
        Ok(img) => Ok(compute_ahash(&img)),
        Err(e) => Err(format!("Failed to open image: {}", e)),
    }
}
