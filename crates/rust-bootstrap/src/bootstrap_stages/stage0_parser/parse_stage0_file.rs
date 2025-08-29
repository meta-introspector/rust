use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn parse_stage0_file(path: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut result = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() || line.starts_with("#") {
            continue;
        }

        if let Some((key, value)) = line.split_once("=") {
            result.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    Ok(result)
}
