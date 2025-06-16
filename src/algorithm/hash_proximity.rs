/// Calculate the proximity between two hex hash strings
/// Returns a value between 0.0 and 1.0, where 1.0 means identical hashes
/// and 0.0 means maximum difference
pub fn calculate_hex_hash_proximity(hash1: &str, hash2: &str) -> Result<f64, String> {
    // Find the minimum length and truncate both hashes to that length
    let min_len = hash1.len().min(hash2.len());

    // Ensure the minimum length is even (pairs of hex characters)
    let truncated_len = if min_len % 2 == 0 {
        min_len
    } else {
        min_len - 1
    };

    // Truncate both hashes to the same even length
    let hash1_truncated = &hash1[..truncated_len];
    let hash2_truncated = &hash2[..truncated_len];

    // Validate that all characters are valid hex
    if !hash1_truncated.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("Hash1 contains invalid hex characters".to_string());
    }
    if !hash2_truncated.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("Hash2 contains invalid hex characters".to_string());
    }

    let mut total_difference = 0u32;
    let byte_count = hash1_truncated.len() / 2;

    // Compare byte by byte (pairs of hex characters)
    for i in (0..hash1_truncated.len()).step_by(2) {
        let byte1_str = &hash1_truncated[i..i + 2];
        let byte2_str = &hash2_truncated[i..i + 2];

        // Parse hex bytes to u8 values
        let byte1 = u8::from_str_radix(byte1_str, 16)
            .map_err(|_| format!("Failed to parse hex byte: {}", byte1_str))?;
        let byte2 = u8::from_str_radix(byte2_str, 16)
            .map_err(|_| format!("Failed to parse hex byte: {}", byte2_str))?;

        // Calculate absolute difference between bytes
        let difference = (byte1 as i16 - byte2 as i16).abs() as u32;
        total_difference += difference;
    }

    // Maximum possible difference: 255 per byte * number of bytes
    let max_possible_difference = 255u32 * byte_count as u32;

    // Calculate proximity: 1.0 - (actual_difference / max_possible_difference)
    let proximity = if max_possible_difference == 0 {
        1.0 // Empty hashes are considered identical
    } else {
        1.0 - (total_difference as f64 / max_possible_difference as f64)
    };

    Ok(proximity)
}

/// Try to find the most similar hash in the list of links
/// Returns the hash-link pair with the highest proximity above the threshold
///
/// # Arguments
///
/// * `hash` - The hash to find a similar hash for
/// * `links` - The list of links to search through
/// * `proximity_threshold` - The minimum proximity to consider a hash similar (default: 0.95)
///
/// # Returns
///
/// * `Ok((hash, link, proximity))` - The hash-link pair with the highest proximity above threshold and the rounded proximity value
/// * `Err(String)` - Error message if no similar hash is found or calculation fails
pub fn try_finding_similar_hash(
    hash: String,
    links: Vec<(String, String)>,
    proximity_threshold: impl Into<Option<f64>>,
) -> Result<(String, String, f64), String> {
    let minimum_proximity: f64 = proximity_threshold.into().unwrap_or(0.95);

    let mut similar_hashes: Vec<(String, String, f64)> = Vec::new();

    // Collect all hashes that pass the proximity threshold
    for (h, link) in links {
        let hash_proximity: f64 = calculate_hex_hash_proximity(&hash, &h)
            .map_err(|e| format!("Failed to calculate proximity: {}", e))?;

        if hash_proximity >= minimum_proximity {
            similar_hashes.push((h, link, hash_proximity));
        }
    }

    // Check if we found any similar hashes
    if similar_hashes.is_empty() {
        return Err("No similar hash found".to_string());
    }

    // Find the hash with the highest proximity
    let most_similar: (String, String, f64) = similar_hashes
        .into_iter()
        .max_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap(); // Safe to unwrap since we checked for empty vector

    Ok(most_similar)
}
