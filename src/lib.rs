use image::DynamicImage;

use crate::{
    algorithm::{ahash, remove_borders},
    data_handle::load_csv::load_data_from_csv,
};

pub mod algorithm;
pub mod data_handle;

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
pub fn compute_hash_from_file(path: String) -> Result<String, String> {
    match image::open(path) {
        Ok(img) => compute_hash(img, true),
        Err(e) => Err(format!("Failed to open image: {}", e)),
    }
}

/// Opens a link associated with a given hash from a dictionary file.
///
/// ## Arguments
///
/// * `links` - Structure containing hash-link pairs
/// * `hash` - The hash to look up in the dictionary
///
/// ## Behavior
///
/// If the hash is found in the dictionary, opens the associated link.
/// If the hash is not found, prints a message to the console.
pub fn open_link_from_hash(links: Vec<(String, String)>, hash: String) {
    for (h, link) in links {
        if h == hash {
            open::that(&link).unwrap();
            return;
        }
    }
    println!("Hash not found: {}", hash);
}

/// Opens a link associated with a given image using its perceptual hash.
///
/// This function computes the hash of the provided image, removing white borders,
/// and then opens the associated link if found in the dictionary.
///
/// ## Arguments
///
/// * `image` - DynamicImage to compute the hash from
/// * `dict_path` - Path to the CSV file containing hash-link pairs
pub fn open_link_from_image(image: DynamicImage, dict_path: String) {
    let hash: String = compute_hash(image, true).unwrap();
    let links: Vec<(String, String)> = load_data_from_csv(dict_path).unwrap();
    open_link_from_hash(links, hash);
}

/// Opens a link associated with an image file using its perceptual hash.
///
/// This function loads an image from a file, computes its hash after removing
/// white borders, and then opens the associated link if found in the dictionary
///
/// ## Arguments
///
/// * `image_path` - Path to the image file
/// * `dict_path` - Path to the CSV file containing hash-link pairs
pub fn open_link_from_image_file(image_path: String, dict_path: String) {
    let hash: String = compute_hash_from_file(image_path).unwrap();
    let links: Vec<(String, String)> = load_data_from_csv(dict_path).unwrap();
    open_link_from_hash(links, hash);
}
