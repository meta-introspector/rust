use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct ZombieConfig {
    pub paths: PathConfig,
    pub analysis: AnalysisConfig,
    pub output: OutputConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PathConfig {
    pub rustc_driver_so: String,
    pub target_debug: String,
    pub signatures_output: String,
    pub analysis_output: String,
    pub rust_files_index: String,
    pub lattice_csv: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnalysisConfig {
    pub max_instruction_bytes: usize,
    pub complexity_threshold: f64,
    pub novelty_threshold: f64,
    pub detect_unsafe_patterns: bool,
    pub analyze_memory_safety: bool,
    pub check_stack_protection: bool,
    pub extract_generics: bool,
    pub analyze_trait_bounds: bool,
    pub track_lifetimes: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OutputConfig {
    pub json_pretty: bool,
    pub include_timestamps: bool,
    pub compress_large_files: bool,
    pub max_duplicates_shown: usize,
    pub show_unique_functions: bool,
    pub calculate_novelty_score: bool,
}

impl ZombieConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = "zombie_config.toml";
        
        if !Path::new(config_path).exists() {
            return Self::create_default();
        }
        
        let config_str = fs::read_to_string(config_path)?;
        let config: ZombieConfig = toml::from_str(&config_str)?;
        Ok(config)
    }
    
    pub fn create_default() -> Result<Self, Box<dyn std::error::Error>> {
        let config = ZombieConfig {
            paths: PathConfig {
                rustc_driver_so: "./target/debug/librustc_driver.so".to_string(),
                target_debug: "./target/debug".to_string(),
                signatures_output: "./semantic_signatures".to_string(),
                analysis_output: "./analysis_results".to_string(),
                rust_files_index: "~/nix/vendor/rust/cargo2nix/files.txt".to_string(),
                lattice_csv: "/tmp/full_rustc_lattice.csv".to_string(),
            },
            analysis: AnalysisConfig {
                max_instruction_bytes: 32,
                complexity_threshold: 10.0,
                novelty_threshold: 75.0,
                detect_unsafe_patterns: true,
                analyze_memory_safety: true,
                check_stack_protection: true,
                extract_generics: true,
                analyze_trait_bounds: true,
                track_lifetimes: true,
            },
            output: OutputConfig {
                json_pretty: true,
                include_timestamps: true,
                compress_large_files: false,
                max_duplicates_shown: 10,
                show_unique_functions: true,
                calculate_novelty_score: true,
            },
        };
        
        let config_str = toml::to_string_pretty(&config)?;
        fs::write("zombie_config.toml", config_str)?;
        println!("✅ Created default config: zombie_config.toml");
        
        Ok(config)
    }
    
    pub fn expand_path(&self, path: &str) -> String {
        if path.starts_with("~/") {
            if let Some(home) = std::env::var("HOME").ok() {
                return path.replace("~", &home);
            }
        }
        path.to_string()
    }
}
