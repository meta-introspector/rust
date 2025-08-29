use std::path::Path;
use std::fs;

pub fn program_out_of_date(stamp_path: &Path, key: &str, clean: bool) -> bool {
    if clean || !stamp_path.exists() {
        return true;
    }
    match fs::read_to_string(stamp_path) {
        Ok(content) => content.trim() != key,
        Err(_) => true, // Treat read errors as out of date
    }
}
