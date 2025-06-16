use fast_image_resize::{self as fir, Resizer};
use image::DynamicImage;

/// Computes the average hash (aHash) of an image.
///
/// The algorithm:
/// 1. Resize the image to hash_size x hash_size (default: 8x8)
/// 2. Convert to grayscale
/// 3. Calculate the average pixel value
/// 4. Compare each pixel to the average and set bits accordingly
/// 5. Return a hash as a hexadecimal string
pub fn compute_image_hash(img: &DynamicImage, hash_size: impl Into<Option<u32>>) -> String {
    // Get hash_size with default value of 8
    let hash_size = hash_size.into().unwrap_or(8);

    // Convert to grayscale
    let gray_img: DynamicImage = img.grayscale();

    // Create resizer and resize to hash_size x hash_size
    let mut resizer: Resizer = fir::Resizer::new();
    let dst_width: u32 = hash_size;
    let dst_height: u32 = hash_size;

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
    let total_pixels = (hash_size * hash_size) as u32;
    let avg: u32 = sum / total_pixels;

    // Create hash (size depends on number of pixels)
    let mut hash: u64 = 0;
    for (i, &pixel) in pixels.iter().enumerate() {
        if pixel as u32 >= avg {
            hash |= 1 << i;
        }
    }

    // Convert to hex string (width determined by number of bits needed)
    let hex_width = ((hash_size * hash_size + 3) / 4) as usize; // Round up to nearest hex digit
    format!("{:0width$x}", hash, width = hex_width)
}
