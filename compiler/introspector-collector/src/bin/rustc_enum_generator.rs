use std::collections::HashMap;
use std::fs;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct EnumInfo {
    pub name: String,
    pub crate_name: String,
    pub variants: Vec<String>,
    pub usage_count: u32,
}

pub struct RustcEnumGenerator {
    pub enums: Vec<EnumInfo>,
}

impl RustcEnumGenerator {
    pub fn new() -> Self {
        Self {
            enums: Vec::new(),
        }
    }

    pub fn load_rustc_enums(&mut self) {
        let test_data_path = "../../test_usage_data";
        
        // Find all enum files
        let pattern = format!("{}/*enums.json", test_data_path);
        if let Ok(entries) = std::fs::read_dir(test_data_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().map_or(false, |ext| ext == "json") &&
                       path.file_name().map_or(false, |name| name.to_string_lossy().contains("enums")) {
                        self.load_enum_file(&path);
                    }
                }
            }
        }
        
        println!("Loaded {} enums from rustc", self.enums.len());
    }

    fn load_enum_file(&mut self, path: &std::path::Path) {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(json) = serde_json::from_str::<Value>(&content) {
                let crate_name = json["crate"].as_str().unwrap_or("unknown").to_string();
                
                if let Some(usages) = json["usages"].as_array() {
                    let mut enum_variants: HashMap<String, Vec<String>> = HashMap::new();
                    let mut enum_counts: HashMap<String, u32> = HashMap::new();
                    
                    for usage in usages {
                        let symbol = usage["symbol"].as_str().unwrap_or("").to_string();
                        let kind = usage["kind"].as_str().unwrap_or("");
                        let count = usage["usage_count"].as_u64().unwrap_or(0) as u32;
                        
                        if kind == "enum_decl" {
                            enum_variants.entry(symbol.clone()).or_insert_with(Vec::new);
                            *enum_counts.entry(symbol).or_insert(0) += count;
                        } else if kind == "enum_variant" {
                            if let Some(parent) = usage["parent_enum"].as_str() {
                                enum_variants.entry(parent.to_string())
                                    .or_insert_with(Vec::new)
                                    .push(symbol);
                            }
                        }
                    }
                    
                    for (enum_name, variants) in enum_variants {
                        let usage_count = enum_counts.get(&enum_name).copied().unwrap_or(0);
                        self.enums.push(EnumInfo {
                            name: enum_name,
                            crate_name: crate_name.clone(),
                            variants,
                            usage_count,
                        });
                    }
                }
            }
        }
    }

    pub fn generate_all_macros(&self) -> String {
        let mut output = String::new();
        
        output.push_str("//! Generated enum-to-string macros for all rustc enums\n");
        output.push_str("//! Auto-generated from usage data analysis\n\n");
        
        // Group by crate
        let mut by_crate: HashMap<String, Vec<&EnumInfo>> = HashMap::new();
        for enum_info in &self.enums {
            by_crate.entry(enum_info.crate_name.clone())
                .or_insert_with(Vec::new)
                .push(enum_info);
        }
        
        for (crate_name, enums) in by_crate {
            output.push_str(&format!("// === {} ===\n", crate_name));
            
            for enum_info in enums {
                if !enum_info.variants.is_empty() {
                    output.push_str(&self.generate_enum_macro(enum_info));
                    output.push('\n');
                }
            }
            output.push('\n');
        }
        
        // Generate summary documentation
        output.push_str(&self.generate_summary());
        
        output
    }

    fn generate_enum_macro(&self, enum_info: &EnumInfo) -> String {
        let macro_name = format!("mk_{}_to_string", 
            enum_info.name.to_lowercase().replace("::", "_"));
        
        let mut macro_body = format!(
            "/// Convert {} enum to string (usage: {})\n",
            enum_info.name, enum_info.usage_count
        );
        macro_body.push_str(&format!("/// Crate: {}\n", enum_info.crate_name));
        macro_body.push_str(&format!("/// Variants: {}\n", enum_info.variants.len()));
        macro_body.push_str(&format!("macro_rules! {} {{\n", macro_name));
        
        for variant in &enum_info.variants {
            macro_body.push_str(&format!(
                "    ({}::{}) => {{ \"{}\" }};\n",
                enum_info.name, variant, variant
            ));
        }
        
        macro_body.push_str("}\n");
        macro_body
    }

    fn generate_summary(&self) -> String {
        let mut summary = String::new();
        
        summary.push_str("// === RUSTC ENUM SUMMARY ===\n");
        summary.push_str(&format!("// Total enums: {}\n", self.enums.len()));
        
        let mut by_size: HashMap<usize, u32> = HashMap::new();
        let mut total_variants = 0;
        
        for enum_info in &self.enums {
            let size = enum_info.variants.len();
            *by_size.entry(size).or_insert(0) += 1;
            total_variants += size;
        }
        
        summary.push_str(&format!("// Total variants: {}\n", total_variants));
        summary.push_str("// Size distribution:\n");
        
        let mut sizes: Vec<_> = by_size.keys().collect();
        sizes.sort();
        
        for size in sizes {
            let count = by_size[size];
            summary.push_str(&format!("//   {} variants: {} enums\n", size, count));
        }
        
        summary.push_str("\n// Usage-based classification:\n");
        let high_usage: Vec<_> = self.enums.iter()
            .filter(|e| e.usage_count > 10)
            .collect();
        summary.push_str(&format!("// High usage (>10): {} enums\n", high_usage.len()));
        
        summary
    }

    pub fn generate_orbit_analysis(&self) -> String {
        let mut analysis = String::new();
        
        analysis.push_str("// === ORBIT ANALYSIS ===\n");
        analysis.push_str("// Enums grouped by variant count (orbit size)\n\n");
        
        let mut orbits: HashMap<usize, Vec<&EnumInfo>> = HashMap::new();
        for enum_info in &self.enums {
            orbits.entry(enum_info.variants.len())
                .or_insert_with(Vec::new)
                .push(enum_info);
        }
        
        let mut orbit_sizes: Vec<_> = orbits.keys().collect();
        orbit_sizes.sort();
        
        for &size in &orbit_sizes {
            let enums_in_orbit = &orbits[&size];
            analysis.push_str(&format!("// Orbit size {}: {} enums\n", size, enums_in_orbit.len()));
            
            for enum_info in enums_in_orbit.iter().take(5) {
                analysis.push_str(&format!("//   {}::{} (usage: {})\n", 
                    enum_info.crate_name, enum_info.name, enum_info.usage_count));
            }
            
            if enums_in_orbit.len() > 5 {
                analysis.push_str(&format!("//   ... and {} more\n", enums_in_orbit.len() - 5));
            }
            analysis.push('\n');
        }
        
        analysis
    }
}
