use image::{DynamicImage, GenericImageView, Rgba};

/// Removes white borders from an image, if they exist.
///
/// This function crops the image to remove white (or near-white) borders
/// around the content. If the image has no borders, the original image is returned.
pub fn remove_white_borders(img: &DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();

    // Define a threshold for "white" - allowing for some noise/variation
    // RGB values above this threshold (out of 255) are considered "white"
    let threshold: u8 = 240;

    // Find the bounding box of non-white content
    let mut min_x: u32 = width;
    let mut min_y: u32 = height;
    let mut max_x: u32 = 0;
    let mut max_y: u32 = 0;

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let Rgba([r, g, b, _]) = pixel;

            // If this pixel is not white (any RGB channel is below threshold)
            if r < threshold || g < threshold || b < threshold {
                min_x = min_x.min(x);
                min_y = min_y.min(y);
                max_x = max_x.max(x);
                max_y = max_y.max(y);
            }
        }
    }

    // If no non-white pixels were found, or the bounding box is the whole image,
    // return the original image
    if min_x >= max_x
        || min_y >= max_y
        || (min_x == 0 && min_y == 0 && max_x == width - 1 && max_y == height - 1)
    {
        return img.clone();
    }

    // Add a small border (1 pixel) around the content if possible
    let border = 1;
    min_x = min_x.saturating_sub(border);
    min_y = min_y.saturating_sub(border);
    max_x = (max_x + border).min(width - 1);
    max_y = (max_y + border).min(height - 1);

    // Crop the image to the bounding box
    img.crop_imm(min_x, min_y, max_x - min_x + 1, max_y - min_y + 1)
}
