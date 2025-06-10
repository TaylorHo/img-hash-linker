# img_hash_linker

A Rust library and CLI tool for linking images to URLs via perceptual hashing.

## Overview

`img_hash_linker` computes perceptual image hashes (using the aHash algorithm) and associates them with URLs, allowing you to:

1. Compute perceptual hashes of images
2. Link images to URLs via their perceptual hash
3. Open the appropriate URL when given an image

## Installation

```bash
cargo install img_hash_linker
```

## CLI Usage

The binary interface provides two main functions:

```bash
# Compute and display an image hash
img_hash_linker <image_path>

# Open a URL associated with an image
img_hash_linker <image_path> <csv_dict_path>
```

Where:

- `<image_path>` is the path to the image file
- `<csv_dict_path>` is the path to a CSV file containing hash-URL pairs

## Library Usage

The crate provides several functions for integration into your Rust projects:

```rust
use img_hash_linker::{
    compute_hash,
    compute_hash_from_file,
    open_link_from_hash,
    open_link_from_image,
    open_link_from_image_file
};
```

### Computing Image Hashes

```rust
// From an image file path
let hash = compute_hash_from_file("path/to/image.jpg").unwrap();

// From a DynamicImage
let image = image::open("path/to/image.jpg").unwrap();
let hash = compute_hash(image, true).unwrap();  // true = remove white borders
```

### Opening Links

```rust
// From an image file
open_link_from_image_file("path/to/image.jpg", "path/to/links.csv");

// From a DynamicImage
let image = image::open("path/to/image.jpg").unwrap();
open_link_from_image(image, "path/to/links.csv");

// From a pre-computed hash and in-memory link database
let links = vec![
    ("a1b2c3d4e5f6...".to_string(), "https://example.com".to_string()),
    // More hash-link pairs...
];
open_link_from_hash(links, "a1b2c3d4e5f6...".to_string());
```

## CSV Format

The CSV file should contain hash-URL pairs, one per line:

```csv
hash,link
hash1,https://example.com/page1
hash2,https://example.com/page2
```

Note: the URL part can also be an application URL handler, like `spotify://` or `vscode://`.

There's an example in the root of the project: [example.csv](./example.csv)
It's very important to have the `hash` and `link` headers (it can also have other fields, but these two will be used)

## How aHash Works

The Average Hash (aHash) algorithm creates a perceptual hash of an image through these steps:

1. **Resize** the image to 8×8 pixels (64 pixels total)
2. **Convert** to grayscale
3. **Calculate** the average pixel value across all 64 pixels
4. **Compare** each pixel to the average:
   - If a pixel's value is greater than or equal to the average, set the corresponding bit to 1
   - Otherwise, set it to 0
5. **Output** the resulting 64 bits as a 16-character hexadecimal string

This creates a "fingerprint" of the image that:

- Is resilient to minor modifications (resizing, compression, etc.)
- Can identify visually similar images
- Is compact (only 64 bits / 16 hex characters)

## Features

- Fast, lightweight perceptual image hashing
- Automatic white border removal for more consistent hashing
- Support for both CLI and library usage
- Simple CSV-based hash-to-URL mapping

## License

[MIT](./LICENSE)
