use image::DynamicImage;

use crate::algorithm::{ahash, remove_borders};

pub mod algorithm;
pub mod data_handle;

/// Computes a perceptual hash for the given image.
///
/// This function processes an image by optionally removing white borders and then
/// computing an average hash (ahash) for the processed image. The hash can be used
/// for image similarity detection and comparison.
///
/// # Arguments
///
/// * `image` - A `DynamicImage` containing the image to hash
/// * `remove_white_border` - A boolean flag indicating whether to remove white borders
///   from the image before computing the hash
/// * `hash_size` - An optional hash size. If `Some(size)` is provided, that size will be used.
///   If `None` is provided, the default size of 8 will be used.
///
/// # Returns
///
/// Returns a `Result<String, String>` where:
/// * `Ok(String)` contains the computed hash as a string
/// * `Err(String)` contains an error message if hash computation fails
///
/// # Examples
///
/// ```
/// use image::DynamicImage;
///
/// // Compute hash with border removal and custom size
/// let hash = compute_hash(image, true, Some(10))?;
///
/// // Compute hash with default settings
/// let hash = compute_hash(image, false, None)?;
/// ```
pub fn compute_hash(
    image: DynamicImage,
    remove_white_border: bool,
    hash_size: Option<u32>,
) -> Result<String, String> {
    let processed_img: DynamicImage = if remove_white_border {
        remove_borders::remove_white_borders(&image)
    } else {
        image
    };

    if let Some(hash_size) = hash_size {
        Ok(ahash::compute_image_hash(&processed_img, hash_size))
    } else {
        Ok(ahash::compute_image_hash(&processed_img, None))
    }
}

/// Opens a link associated with the given hash.
///
/// This function searches through a collection of hash-link pairs to find a matching hash,
/// and if found, opens the corresponding link using the system's default application.
///
/// # Arguments
///
/// * `links` - A vector of tuples where each tuple contains `(hash, link)` as `(String, String)`
/// * `hash` - The hash string to search for in the links collection
///
/// # Returns
///
/// Returns a `Result<String, String>` where:
/// * `Ok(String)` contains a success message with the opened link
/// * `Err(String)` contains an error message if the hash is not found
///
/// # Examples
///
/// ```
/// let links = vec![
///     ("abc123".to_string(), "https://example.com".to_string()),
///     ("def456".to_string(), "https://another.com".to_string()),
/// ];
///
/// match open_link_from_hash(links, "abc123".to_string()) {
///     Ok(message) => println!("{}", message), // "Link opened: https://example.com"
///     Err(error) => println!("Error: {}", error),
/// }
/// ```
///
/// Getting the links vector from a csv file:
/// ```
/// let links: Vec<(String, String)> = load_data_from_csv("path/to/example.csv").unwrap();
/// ```
pub fn open_link_from_hash(links: Vec<(String, String)>, hash: String) -> Result<String, String> {
    for (h, link) in &links {
        if *h == hash {
            open::that(&link).unwrap();
            return Ok(format!("Link opened: {}", link));
        }
    }

    Err(format!("Hash not found: {}", hash))
}
