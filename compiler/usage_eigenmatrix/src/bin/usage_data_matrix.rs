use std::fs::{self, OpenOptions};
use std::io::Write;
use std::collections::HashMap;
use serde_json::Value;

/// Map usage data DefIds into 24-bit matrix with file-based collision tracking
struct UsageDataMatrix {
    pages: Vec<Option<Box<[u8; 4096]>>>,
    defid_count: u64,
    collision_count: u64,
    collision_file: std::fs::File,
    first_defids: HashMap<u32, String>, // signature -> first DefId seen
    prime_basis: [u64; 8],
}

impl UsageDataMatrix {
    fn new() -> Self {
        let collision_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("collisions.txt")
            .expect("Failed to create collision file");
            
        Self {
            pages: vec![None; 4096],
            defid_count: 0,
            collision_count: 0,
            collision_file,
            first_defids: HashMap::new(),
            prime_basis: [2, 3, 5, 7, 11, 13, 17, 19],
        }
    }

    fn map_defid_to_24bit(&self, defid: &str) -> u32 {
        let mut signature = 1u64;
        
        for (i, byte) in defid.bytes().enumerate() {
            let prime_idx = i % self.prime_basis.len();
            let prime = self.prime_basis[prime_idx];
            signature = signature.wrapping_mul(prime).wrapping_add(byte as u64);
        }
        
        (signature & 0xFFFFFF) as u32
    }

    fn set_defid(&mut self, signature: u32, defid: &str) {
        let page_idx = (signature as usize) / 4096;
        let offset = (signature as usize) % 4096;
        
        if page_idx < 4096 {
            if self.pages[page_idx].is_none() {
                self.pages[page_idx] = Some(Box::new([0u8; 4096]));
            }
            
            if let Some(ref mut page) = self.pages[page_idx] {
                if page[offset] == 0 {
                    page[offset] = 1;
                    self.defid_count += 1;
                    self.first_defids.insert(signature, defid.to_string());
                } else {
                    // Check if this is a different DefId (actual collision)
                    if let Some(first_defid) = self.first_defids.get(&signature) {
                        if first_defid != defid {
                            writeln!(self.collision_file, "{:06X} {} | {}", signature, first_defid, defid).ok();
                            self.collision_count += 1;
                        }
                    }
                }
            }
        }
    }

    fn load_usage_file(&mut self, file_path: &str) -> u32 {
        let mut defids_loaded = 0;
        
        if let Ok(content) = fs::read_to_string(file_path) {
            if let Ok(json) = serde_json::from_str::<Value>(&content) {
                if let Some(usages) = json["usages"].as_array() {
                    for usage in usages {
                        if let Some(used_def_id) = usage["used_def_id"].as_str() {
                            let signature = self.map_defid_to_24bit(used_def_id);
                            self.set_defid(signature, used_def_id);
                            defids_loaded += 1;
                        }
                    }
                }
            }
        }
        
        defids_loaded
    }

    fn load_all_usage_data(&mut self) -> u32 {
        let mut total_defids = 0;
        let mut files_processed = 0;
        
        if let Ok(entries) = fs::read_dir("../../usage_data") {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".json") {
                        let defids = self.load_usage_file(&entry.path().display().to_string());
                        total_defids += defids;
                        files_processed += 1;
                        
                        if files_processed % 1000 == 0 {
                            println!("Processed {} files, {} DefIds so far...", files_processed, total_defids);
                        }
                    }
                }
            }
        }
        
        total_defids
    }

    fn get_stats(&self) -> (usize, f64) {
        let allocated_pages = self.pages.iter().filter(|p| p.is_some()).count();
        let usage_percent = (allocated_pages as f64 / 4096.0) * 100.0;
        (allocated_pages, usage_percent)
    }
}

fn main() {
    let mut matrix = UsageDataMatrix::new();
    
    println!("Loading usage data into 24-bit matrix...");
    let total_defids = matrix.load_all_usage_data();
    
    let (allocated_pages, usage_percent) = matrix.get_stats();
    
    println!("\n24-bit Matrix Results:");
    println!("DefIds processed: {}", total_defids);
    println!("Unique signatures: {}", matrix.defid_count);
    println!("Collisions: {}", matrix.collision_count);
    println!("Collision rate: {:.2}%", (matrix.collision_count as f64 / total_defids as f64) * 100.0);
    println!("Pages allocated: {}/{} ({:.2}%)", allocated_pages, 4096, usage_percent);
    println!("Memory usage: {:.1} MB / 16.0 MB", (allocated_pages as f64 * 4.0) / 1024.0);
    
    // Check if matrix is full
    let matrix_full = allocated_pages == 4096;
    println!("Matrix full: {}", matrix_full);
}
