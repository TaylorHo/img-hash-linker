use csv::{Reader, StringRecord, Writer};
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

/// Adds entries to a CSV file with hash and link columns
///
/// # Example
///
/// ```
/// fn main() {
///     let data = vec![(
///         "xxxxxxxxxxxxxxxx".to_string(),
///         "https://example.com".to_string(),
///     )];
///
///     add_entry_to_data_file(data, "example.csv".to_string())?;
/// }
/// ```
pub fn add_entry_to_data_file(
    data: Vec<(String, String)>,
    dict_path: String,
) -> Result<(), String> {
    let path: &Path = Path::new(&dict_path);

    // If file doesn't exist, create it already with the headers
    if !path.exists() {
        create_csv_file(&dict_path)?;
    }

    // Get header positions
    let (hash_index, link_index, num_columns) = get_header_positions(&dict_path)?;

    // Append data to file
    append_data_to_csv(&dict_path, data, hash_index, link_index, num_columns)?;

    Ok(())
}

fn create_csv_file(file_path: &str) -> Result<(), String> {
    let file: File =
        File::create(file_path).map_err(|e| format!("Failed to create CSV file: {}", e))?;
    let mut writer: Writer<File> = Writer::from_writer(file);

    // Write headers
    writer
        .write_record(&["hash", "link"])
        .map_err(|e| format!("Failed to write headers: {}", e))?;

    // Flush writer
    writer
        .flush()
        .map_err(|e| format!("Failed to flush CSV writer: {}", e))?;
    Ok(())
}

fn get_header_positions(file_path: &str) -> Result<(usize, usize, usize), String> {
    let file: File =
        File::open(file_path).map_err(|e| format!("Failed to open CSV file: {}", e))?;
    let mut reader: Reader<File> = Reader::from_reader(file);

    let headers: &StringRecord = reader
        .headers()
        .map_err(|e| format!("Failed to read CSV headers: {}", e))?;

    let hash_index: usize = headers
        .iter()
        .position(|h| h == "hash")
        .ok_or("CSV file must contain a 'hash' header")?;
    let link_index: usize = headers
        .iter()
        .position(|h| h == "link")
        .ok_or("CSV file must contain a 'link' header")?;

    Ok((hash_index, link_index, headers.len()))
}

fn append_data_to_csv(
    file_path: &str,
    data: Vec<(String, String)>,
    hash_index: usize,
    link_index: usize,
    num_columns: usize,
) -> Result<(), String> {
    if data.is_empty() {
        return Ok(());
    }

    // Add newline if needed
    ensure_file_ends_with_newline(file_path)?;

    // Open file for CSV writing
    let file: File = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)
        .map_err(|e| format!("Failed to open CSV file for append: {}", e))?;

    let mut writer: Writer<File> = Writer::from_writer(file);

    // Write data
    for (hash, link) in data {
        let mut row: Vec<String> = vec![String::new(); num_columns];
        row[hash_index] = hash;
        row[link_index] = link;
        writer
            .write_record(&row)
            .map_err(|e| format!("Failed to write new record: {}", e))?;
    }

    writer
        .flush()
        .map_err(|e| format!("Failed to flush CSV writer: {}", e))?;
    Ok(())
}

/// Ensures that the file ends with a newline character
fn ensure_file_ends_with_newline(file_path: &str) -> Result<(), String> {
    // First open the file for reading to check if we need a newline
    let mut file: File = OpenOptions::new()
        .read(true)
        .write(true) // Open for both reading and writing
        .append(true)
        .open(file_path)
        .map_err(|e| format!("Failed to open CSV file: {}", e))?;

    // Get file size
    let file_size = file
        .metadata()
        .map_err(|e| format!("Failed to get file metadata: {}", e))?
        .len();

    // If file is empty, no need to add newline
    if file_size == 0 {
        return Ok(());
    }

    // Read the last byte
    let mut last_byte: [u8; 1] = [0u8; 1];
    file.seek(SeekFrom::End(-1))
        .map_err(|e| format!("Failed to seek to end of file: {}", e))?;
    file.read_exact(&mut last_byte)
        .map_err(|e| format!("Failed to read from file: {}", e))?;

    // If the last byte is not a newline, add one
    if last_byte[0] != b'\n' {
        // Seek to the end and write the newline
        file.seek(SeekFrom::End(0))
            .map_err(|e| format!("Failed to seek to end of file: {}", e))?;
        file.write_all(b"\n")
            .map_err(|e| format!("Failed to write newline: {}", e))?;
    }

    Ok(())
}
