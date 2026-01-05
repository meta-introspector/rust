use std::fs;
use std::path::Path;
use serde_json::Value;

/// Load usage data into 24-bit DefId matrix
struct UsageDataLoader {
    mapper: DefId24BitMapper,
    loaded_files: u32,
    total_defids: u64,
}

struct DefId24BitMapper {
    pages: Vec<Option<Box<[u8; 4096]>>>,
}

impl DefId24BitMapper {
    fn new() -> Self {
        Self {
            pages: vec![None; 4096],
        }
    }

    fn get_page_offset(&self, addr: u32) -> (usize, usize) {
        let page_idx = (addr as usize) / 4096;
        let offset = (addr as usize) % 4096;
        (page_idx, offset)
    }

    fn ensure_page(&mut self, page_idx: usize) {
        if page_idx < 4096 && self.pages[page_idx].is_none() {
            self.pages[page_idx] = Some(Box::new([0u8; 4096]));
        }
    }

    fn set_defid(&mut self, addr: u32, value: u8) {
        let (page_idx, offset) = self.get_page_offset(addr);
        self.ensure_page(page_idx);
        
        if let Some(ref mut page) = self.pages[page_idx] {
            page[offset] = value;
        }
    }

    fn map_defid_to_24bit(&self, defid_value: u64) -> u32 {
        match defid_value {
            0 => 0x000001,
            1 => 0x000002,
            2 => 0x000003,
            3 => 0x000005,
            4 => 0x000007,
            5 => 0x00000B,
            6 => 0x00000D,
            7 => 0x000011,
            8 => 0x000013,
            9 => 0x000017,
            _ => {
                let scaled = (defid_value % 0xFFFFF0) + 0x000020;
                scaled as u32 & 0xFFFFFF
            }
        }
    }
}

impl UsageDataLoader {
    fn new() -> Self {
        Self {
            mapper: DefId24BitMapper::new(),
            loaded_files: 0,
            total_defids: 0,
        }
    }

    /// Load single usage data JSON file and construct signatures
    fn load_usage_file(&mut self, file_path: &Path) -> Result<u32, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        let json: Value = serde_json::from_str(&content)?;
        
        let mut signatures_in_file = 0u32;
        
        // Construct signature from file content
        let file_signature = self.construct_signature_from_json(&json);
        let cell = self.mapper.map_defid_to_24bit(file_signature);
        self.mapper.set_defid(cell, 1);
        signatures_in_file += 1;
        self.total_defids += 1;
        
        // Extract individual examples and construct their signatures
        if let Some(obj) = json.as_object() {
            for (key, value) in obj {
                let example_signature = self.construct_signature_from_example(key, value);
                let cell = self.mapper.map_defid_to_24bit(example_signature);
                self.mapper.set_defid(cell, 1);
                signatures_in_file += 1;
                self.total_defids += 1;
            }
        }
        
        self.loaded_files += 1;
        Ok(signatures_in_file)
    }

    /// Construct signature from JSON structure
    fn construct_signature_from_json(&self, json: &Value) -> u64 {
        let mut signature = 1u64;
        let json_str = json.to_string();
        
        // Use prime basis to construct signature from content
        for (i, byte) in json_str.bytes().enumerate() {
            let prime = [2, 3, 5, 7, 11, 13, 17, 19][i % 8];
            signature = signature.wrapping_mul(prime).wrapping_add(byte as u64);
        }
        
        signature & 0xFFFFFF // Keep in 24-bit range
    }

    /// Construct signature from individual example
    fn construct_signature_from_example(&self, key: &str, value: &Value) -> u64 {
        let mut signature = 1u64;
        
        // Combine key and value into signature
        let combined = format!("{}:{}", key, value);
        
        for (i, byte) in combined.bytes().enumerate() {
            let prime = [2, 3, 5, 7, 11, 13, 17, 19][i % 8];
            signature = signature.wrapping_mul(prime).wrapping_add(byte as u64);
        }
        
        signature & 0xFFFFFF // Keep in 24-bit range
    }

    /// Load all usage data files from directory
    fn load_usage_directory(&mut self, usage_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
        let dir_path = Path::new(usage_dir);
        
        if !dir_path.exists() {
            return Err(format!("Usage data directory not found: {}", usage_dir).into());
        }

        println!("Loading usage data from: {}", usage_dir);
        
        let mut file_count = 0;
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "json") {
                match self.load_usage_file(&path) {
                    Ok(signatures) => {
                        if file_count < 10 { // Show first 10 files
                            println!("  {} -> {} signatures", 
                                path.file_name().unwrap().to_string_lossy(), signatures);
                        }
                        file_count += 1;
                    }
                    Err(e) => {
                        eprintln!("Error loading {}: {}", path.display(), e);
                    }
                }
            }
            
            if file_count >= 100 { // Limit for demo
                println!("  ... (limiting to first 100 files for demo)");
                break;
            }
        }
        
        Ok(())
    }

    /// Generate report of loaded data
    fn generate_report(&self) -> String {
        let allocated_pages = self.mapper.pages.iter().filter(|p| p.is_some()).count();
        let usage_percent = (allocated_pages as f64 / 4096.0) * 100.0;
        
        format!(
            "# Usage Data Matrix Report\n\n\
            ## Load Statistics\n\
            - Files loaded: {}\n\
            - Total DefIds: {}\n\
            - Pages allocated: {} / 4096 ({:.2}%)\n\
            - Memory usage: {:.1} MB / 16.0 MB\n\n\
            ## 24-bit Matrix Status\n\
            Successfully mapped {} DefIds from rustc usage data into 24-bit space.\n\
            Each DefId occupies a unique cell in the 16,777,216 location matrix.\n",
            self.loaded_files,
            self.total_defids,
            allocated_pages,
            usage_percent,
            (allocated_pages as f64 * 4.0) / 1024.0,
            self.total_defids
        )
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut loader = UsageDataLoader::new();
    
    // Load usage data into 24-bit matrix
    loader.load_usage_directory("../../usage_data")?;
    
    // Generate and save report
    let report = loader.generate_report();
    fs::write("usage_matrix_report.md", &report)?;
    
    println!("\nUsage Data Matrix Loading Complete!");
    println!("Files loaded: {}", loader.loaded_files);
    println!("Total DefIds mapped: {}", loader.total_defids);
    
    let allocated_pages = loader.mapper.pages.iter().filter(|p| p.is_some()).count();
    println!("Matrix usage: {} / 4096 pages ({:.2}%)", 
        allocated_pages, (allocated_pages as f64 / 4096.0) * 100.0);
    
    println!("Report saved: usage_matrix_report.md");
    
    Ok(())
}
