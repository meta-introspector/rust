// Zombie compiler that extracts real data
use std::collections::HashMap;

struct ZombieCallbacks {
    extracted_data: HashMap<String, Vec<String>>,
}

impl ZombieCallbacks {
    fn new() -> Self {
        ZombieCallbacks {
            extracted_data: HashMap::new(),
        }
    }
    
    fn extract_args(&mut self, args: &[String]) {
        // Extract actual rustc arguments
        let mut file_names = Vec::new();
        let mut crate_types = Vec::new();
        let mut features = Vec::new();
        
        for arg in args {
            if arg.ends_with(".rs") {
                file_names.push(arg.clone());
            } else if arg.starts_with("--crate-type") {
                crate_types.push(arg.clone());
            } else if arg.starts_with("--cfg") {
                features.push(arg.clone());
            }
        }
        
        self.extracted_data.insert("files".to_string(), file_names);
        self.extracted_data.insert("crate_types".to_string(), crate_types);
        self.extracted_data.insert("features".to_string(), features);
    }
    
    fn print_extracted_data(&self) {
        println!("EXTRACTED DATA:");
        for (key, values) in &self.extracted_data {
            println!("  {}: {} items", key, values.len());
            for value in values {
                println!("    - {}", value);
            }
        }
    }
}

fn main() {
    let mut callbacks = ZombieCallbacks::new();
    let args: Vec<String> = std::env::args().collect();
    
    // Extract real data from arguments
    callbacks.extract_args(&args);
    callbacks.print_extracted_data();
    
    // Pass to rustc
    if args.len() > 1 {
        let mut cmd = std::process::Command::new("rustc");
        for arg in &args[1..] {
            cmd.arg(arg);
        }
        std::process::exit(cmd.status().unwrap().code().unwrap_or(1));
    }
}
