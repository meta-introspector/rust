use crate::data_structures::*;
use std::collections::HashMap;
use std::io::Write;

pub struct FileManager;

impl FileManager {
    pub fn save_to_files(
        module_data: &HashMap<String, Vec<UsageEntry>>,
        enum_data: &HashMap<String, EnumInfo>,
        item_complexity: &HashMap<String, ItemComplexity>,
        crate_name: &str,
    ) {
        let output_dir = Self::get_output_dir();
        std::fs::create_dir_all(&output_dir).unwrap();
        
        let mut total_usages = 0;
        let mut generated_files = Vec::new();
        
        for (module, usages) in module_data {
            let module_data = ModuleData {
                crate_name: crate_name.to_string(),
                module: module.clone(),
                usages: usages.clone(),
            };
            
            let module_clean = Self::clean_filename(module);
            let filename = Self::generate_filename(&output_dir, crate_name, &module_clean, module);
            
            let json = serde_json::to_string_pretty(&module_data).unwrap();
            std::fs::write(&filename, json).unwrap();
            generated_files.push(filename.clone());
            total_usages += usages.len();
        }
        
        if !item_complexity.is_empty() {
            let complexity_filename = format!("{}/{}_complexity.json", output_dir, crate_name);
            let complexity_data = serde_json::json!({
                "crate": crate_name,
                "items": item_complexity.values().collect::<Vec<_>>()
            });
            let complexity_json = serde_json::to_string_pretty(&complexity_data).unwrap();
            std::fs::write(&complexity_filename, complexity_json).unwrap();
            generated_files.push(complexity_filename);
            eprintln!("=== SAVED {} COMPLEXITY ITEMS FOR CRATE: {} ===", item_complexity.len(), crate_name);
        }
        
        // Save enum data with variants
        if !enum_data.is_empty() {
            let enum_filename = format!("{}/{}_enum_variants.json", output_dir, crate_name);
            let enum_output = serde_json::json!({
                "crate": crate_name,
                "enums": enum_data.values().collect::<Vec<_>>()
            });
            let enum_json = serde_json::to_string_pretty(&enum_output).unwrap();
            std::fs::write(&enum_filename, enum_json).unwrap();
            generated_files.push(enum_filename);
            eprintln!("=== SAVED {} ENUMS WITH VARIANTS FOR CRATE: {} ===", enum_data.len(), crate_name);
        }
        
        Self::generate_manifest(crate_name, &output_dir, &generated_files, total_usages);
        eprintln!("=== COLLECTING USAGE DATA FOR CRATE: {} === ({} total usages)", crate_name, total_usages);
    }
    
    fn get_output_dir() -> String {
        std::env::var("USAGE_OUTPUT_DIR")
            .or_else(|_| {
                std::env::var("CARGO_TARGET_DIR")
                    .map(|target_dir| format!("{}/harmonic/usage", target_dir))
            })
            .or_else(|_| {
                std::env::var("CARGO_MANIFEST_DIR")
                    .and_then(|manifest_dir| {
                        std::env::var("PROFILE")
                            .map(|profile| format!("{}/target/{}/harmonic/usage", manifest_dir, profile))
                    })
            })
            .unwrap_or_else(|_| "usage_data".to_string())
    }
    
    fn clean_filename(module: &str) -> String {
        module
            .replace("::", "_")
            .replace("<", "_")
            .replace(">", "_")
            .replace(" ", "_")
            .replace("/", "_")
            .replace("\\", "_")
            .replace("*", "_")
            .replace("?", "_")
            .replace("\"", "_")
            .replace("|", "_")
            .replace("'", "_")
            .replace("#", "_")
            .replace("{", "_")
            .replace("}", "_")
            .replace("(", "_")
            .replace(")", "_")
            .replace("[", "_")
            .replace("]", "_")
            .replace("&", "_")
            .replace("$", "_")
            .replace("@", "_")
            .replace("!", "_")
            .replace("%", "_")
            .replace("^", "_")
            .replace("+", "_")
            .replace("=", "_")
            .replace("~", "_")
            .replace("`", "_")
            .replace(";", "_")
            .replace(",", "_")
            .replace(".", "_")
    }
    
    fn generate_filename(output_dir: &str, crate_name: &str, module_clean: &str, module: &str) -> String {
        if module_clean.len() > 100 {
            let hash = std::collections::hash_map::DefaultHasher::new();
            use std::hash::{Hash, Hasher};
            let mut hasher = hash;
            module.hash(&mut hasher);
            format!("{}/{}_{:x}.json", output_dir, crate_name, hasher.finish())
        } else {
            format!("{}/{}_{}.json", output_dir, crate_name, module_clean)
        }
    }
    
    fn generate_manifest(crate_name: &str, output_dir: &str, generated_files: &[String], total_usages: usize) {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let mut dependency_manifests = Vec::new();
        if let Ok(entries) = std::fs::read_dir(output_dir) {
            for entry in entries.flatten() {
                if let Some(filename) = entry.file_name().to_str() {
                    if filename.ends_with("_manifest.json") && !filename.starts_with(&format!("{}_", crate_name)) {
                        dependency_manifests.push(entry.path().to_string_lossy().to_string());
                    }
                }
            }
        }
        
        let manifest = serde_json::json!({
            "crate_name": crate_name,
            "timestamp": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            "collector_version": "1.0.0",
            "rustc_version": std::env::var("RUSTC_VERSION").unwrap_or_else(|_| "unknown".to_string()),
            "output_directory": output_dir,
            "total_usages": total_usages,
            "generated_files": generated_files.iter().map(|f| {
                serde_json::json!({
                    "path": f,
                    "size_bytes": std::fs::metadata(f).map(|m| m.len()).unwrap_or(0),
                    "type": if f.contains("_enums_classified") { "enum_classification" } else { "usage_data" }
                })
            }).collect::<Vec<_>>(),
            "dependency_manifests": dependency_manifests,
            "environment": {
                "pwd": std::env::current_dir().ok().map(|p| p.to_string_lossy().to_string()),
                "cargo_manifest_dir": std::env::var("CARGO_MANIFEST_DIR").ok(),
                "cargo_target_dir": std::env::var("CARGO_TARGET_DIR").ok(),
                "profile": std::env::var("PROFILE").ok()
            }
        });
        
        let manifest_path = format!("{}/{}_manifest.json", output_dir, crate_name);
        std::fs::write(&manifest_path, serde_json::to_string_pretty(&manifest).unwrap()).unwrap();
        eprintln!("=== MANIFEST SAVED: {} ===", manifest_path);
    }
}
