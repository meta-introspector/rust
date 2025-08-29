use std::fs;

pub fn read_file_content(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(fs::read_to_string(path)?)
}