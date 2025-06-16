# img_hash_linker

A Rust library and CLI tool for linking images to URLs via perceptual hashing.

## Overview

`img_hash_linker` computes perceptual image hashes (using the aHash algorithm) and associates them with URLs, allowing you to:

1. Compute perceptual hashes of images with configurable hash sizes
2. Link images to URLs via their perceptual hash
3. Find exact matches or similar images within a proximity threshold
4. Open the appropriate URL when given an image

## Installation

### CLI Tool

```bash
cargo install img_hash_linker
```

### Rust Crate

Add this to your `Cargo.toml`:

```toml
[dependencies]
img_hash_linker = "1.1.0"
```

## Usage

### CLI Usage

The binary interface provides image hash computation and URL opening:

```bash
# Compute and display an image hash
img_hash_linker <image_path>

# Open a URL associated with an image
img_hash_linker <image_path> <csv_dict_path>
```

Where:

- `<image_path>` is the path to the image file
- `<csv_dict_path>` is the path to a CSV file containing hash-URL pairs (example in [example.csv](https://github.com/TaylorHo/img-hash-linker/blob/main/example.csv))

### Library Usage

```rust
use img_hash_linker::{
    compute_hash,
    load_data_from_csv,
    open_link_from_hash,
    try_finding_similar_hash
};

let image_path = "path/to/image.jpg";
let dict_path = "path/to/links.csv";

// Configure hash size (optional, defaults to 8)
let hash_size: Option<u32> = Some(8); // Can also be None

// Compute hash from image
let hash: String = compute_hash(
    image::open(image_path).unwrap(),
    true,       // remove white borders
    hash_size   // hash size configuration
).unwrap();

// Load hash-URL pairs from CSV
let links: Vec<(String, String)> = load_data_from_csv(dict_path).unwrap();

// Try to find exact match first
match open_link_from_hash(links.clone(), hash.clone()) {
    Ok(message) => println!("{}", message), // Found exact match
    Err(e) => {
        // If no exact match, try finding similar hash
        match try_finding_similar_hash(hash.clone(), links.clone(), None) {
            Ok((similar_hash, _link, proximity)) => {
                println!(
                    "{} (Proximity: {:.2}%)",
                    open_link_from_hash(links.clone(), similar_hash).unwrap(),
                    proximity * 100.0
                );
            }
            Err(_) => {
                println!("{}", e);
            }
        }
    }
}
```

### CSV Format

The CSV file should contain hash-URL pairs with headers:

```csv
hash,link
hash1,https://example.com/page1
hash2,https://example.com/page2
```

**Important:** The CSV must have `hash` and `link` headers. Additional fields are allowed but will be ignored.

Note: URLs can also be application URL handlers like `spotify://` or `vscode://`.

## Understanding the aHash Algorithm

The Average Hash (aHash) algorithm creates a perceptual hash of an image through these steps:

1. **Resize** the image to N×N pixels (configurable through "hash_size" parameter, default 8×8 = 64 pixels total)
2. **Convert** to grayscale
3. **Calculate** the average pixel value across all pixels
4. **Compare** each pixel to the average:
   - If a pixel's value is greater than or equal to the average, set the corresponding bit to 1
   - Otherwise, set it to 0
5. **Output** the resulting bits as a hexadecimal string

This creates a "fingerprint" of the image that:

- Is resilient to minor modifications (resizing, compression, etc.)
- Can identify visually similar images
- Is compact (configurable size, default 64 bits)
- Supports similarity matching within proximity thresholds

### Hash Size Configuration

- **Default**: 8×8 (64 bits, 16 hex characters)
- **Configurable**: Any size N×N where N is specified
- **Trade-off**: Larger sizes provide more specific detail but less resilience to modifications (something like 8 or even shorter is perfect)

## Features

- Fast, lightweight perceptual image hashing
- Configurable hash sizes for different use cases
- Automatic white border removal for consistent hashing
- Exact match and similarity-based hash matching
- Proximity scoring for similar images
- Support for both CLI and library usage
- Simple CSV-based hash-to-URL mapping

## License

[MIT](https://github.com/TaylorHo/img-hash-linker/blob/main/LICENSE)
