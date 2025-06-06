use std::{fs::File, path::Path};

use csv::{Reader, StringRecord};
use url::Url;

/// Validates a CSV file and returns a reader, hash index, and link index.
///
/// ## Arguments
///
/// * `path` - Path to the CSV file
///
/// ## Returns
fn validate_csv_file(path: &String) -> Result<(Reader<File>, usize, usize), String> {
    // Check if the file exists and is a CSV
    let path_ref: &Path = path.as_ref();
    if !path_ref.exists() {
        return Err(format!("File does not exist: {}", path_ref.display()));
    }

    if path_ref.extension().and_then(|ext| ext.to_str()) != Some("csv") {
        return Err(format!("File is not a CSV: {}", path_ref.display()));
    }

    // Read the CSV file
    let file: File = match std::fs::File::open(path_ref) {
        Ok(file) => file,
        Err(e) => return Err(format!("Failed to open file: {}", e)),
    };

    let mut reader: Reader<File> = csv::ReaderBuilder::new()
        .flexible(false)
        .trim(csv::Trim::All)
        .from_reader(file);

    // Get headers
    let headers: StringRecord = match reader.headers() {
        Ok(headers) => headers.clone(),
        Err(e) => return Err(format!("Failed to read CSV headers: {}", e)),
    };

    // Check column count and find hash and link columns
    let mut hash_index: Option<usize> = None;
    let mut link_index: Option<usize> = None;

    if headers.len() == 2 {
        // If there are exactly two columns, use them directly
        hash_index = Some(0);
        link_index = Some(1);
    } else {
        // Otherwise, look for "hash" and "link" columns
        for (i, header) in headers.iter().enumerate() {
            let lower_header: String = header.to_lowercase();
            if lower_header == "hash" {
                hash_index = Some(i);
            } else if lower_header == "link" {
                link_index = Some(i);
            }
        }
    }

    // Check if both hash and link columns were found
    if hash_index.is_none() || link_index.is_none() {
        return Err(format!(
            "CSV must have either exactly two columns or columns named 'hash' and 'link'"
        ));
    }

    Ok((reader, hash_index.unwrap(), link_index.unwrap()))
}

/// Loads data from a CSV file and returns a vector of hash-link pairs.
///
/// ## Arguments
///
/// * `path` - Path to the CSV file
///
/// ## Returns
pub fn load_data_from_csv(path: String) -> Result<Vec<(String, String)>, String> {
    // Validate the CSV file and get column indices and reader
    let (mut reader, hash_idx, link_idx) = validate_csv_file(&path)?;

    // Read the records
    let mut links: Vec<(String, String)> = Vec::new();
    for (row_idx, result) in reader.records().enumerate() {
        let record: StringRecord = match result {
            Ok(record) => record,
            Err(e) => return Err(format!("Failed to read row {}: {}", row_idx + 1, e)),
        };

        if record.len() <= hash_idx || record.len() <= link_idx {
            return Err(format!(
                "Row {} has fewer columns than expected",
                row_idx + 1
            ));
        }

        let hash: String = record[hash_idx].trim().to_string();
        let link: String = record[link_idx].trim().to_string();

        if !hash.is_empty() && !link.is_empty() {
            // Validate that the link is a proper URL
            match Url::parse(&link) {
                Ok(_) => links.push((hash, link)),
                Err(_) => {}
            }
        }
    }

    if links.is_empty() {
        return Err("CSV file contains no valid links".to_string());
    }

    Ok(links)
}
