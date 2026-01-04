use serde_json::Value;
use std::collections::{HashMap, BTreeMap};
use std::fs;

#[derive(Debug, Clone)]
struct EnumGroup {
    size: usize,
    enums: Vec<EnumWithStrings>,
    total_count: usize,
}

#[derive(Debug, Clone)]
struct EnumWithStrings {
    enum_name: String,
    variants: Vec<String>,
    string_mappings: Vec<String>,
    usage_count: usize,
}

fn main() {
    println!("🔬 Enum Lattice by Size N → Macro Arguments");
    
    let usage_data_dir = "../../usage_data";
    let mut enum_groups: BTreeMap<usize, EnumGroup> = BTreeMap::new();
    
    // Extract all enums with their string mappings
    extract_enum_string_mappings(usage_data_dir, &mut enum_groups);
    
    // Generate lattice table
    generate_lattice_table(&enum_groups);
    
    // Generate macro arguments for mkrust!
    generate_mkrust_macros(&enum_groups);
    
    // Save individual group files
    save_group_files(&enum_groups);
}

fn extract_enum_string_mappings(usage_data_dir: &str, groups: &mut BTreeMap<usize, EnumGroup>) {
    let mut processed_files = 0;
    let mut enum_data: HashMap<String, EnumWithStrings> = HashMap::new();
    
    if let Ok(entries) = fs::read_dir(usage_data_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "json") && 
                   path.file_name().unwrap().to_str().unwrap().ends_with("_enums.json") {
                    processed_files += 1;
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(json) = serde_json::from_str::<Value>(&content) {
                            extract_enums_from_enum_file(&json, &mut enum_data);
                        }
                    }
                    
                    if processed_files % 50 == 0 {
                        println!("   Processed {} enum files...", processed_files);
                    }
                }
            }
        }
    }
    
    // Group enums by size
    for (_, enum_info) in enum_data {
        let size = enum_info.variants.len();
        groups.entry(size)
            .or_insert_with(|| EnumGroup { size, enums: Vec::new(), total_count: 0 })
            .enums.push(enum_info.clone());
        groups.get_mut(&size).unwrap().total_count += enum_info.usage_count;
    }
    
    // Sort enums within each group by usage count
    for (_, group) in groups.iter_mut() {
        group.enums.sort_by(|a, b| b.usage_count.cmp(&a.usage_count));
    }
    
    println!("📊 Processed {} files", processed_files);
    println!("📊 Found {} enum size groups", groups.len());
}

fn extract_enums_from_enum_file(json: &Value, enum_data: &mut HashMap<String, EnumWithStrings>) {
    if let Some(usages) = json.get("usages").and_then(|u| u.as_array()) {
        for usage in usages {
            if let (Some(usage_type), Some(used_def_id)) = (
                usage.get("usage_type").and_then(|u| u.as_str()),
                usage.get("used_def_id").and_then(|u| u.as_str())
            ) {
                if usage_type == "EnumDecl" {
                    // Extract enum name and variant from the usage
                    if let Some(usage_str) = usage.get("usage").and_then(|u| u.as_str()) {
                        if usage_str.starts_with("ENUM_DECL:") {
                            let parts: Vec<&str> = usage_str.split_whitespace().collect();
                            if parts.len() >= 2 {
                                let variant_name = parts[1].to_string();
                                let enum_name = used_def_id.to_string(); // This might be the enum name
                                
                                let usage_count = usage.get("usage_count")
                                    .and_then(|u| u.as_u64())
                                    .unwrap_or(1) as usize;
                                
                                enum_data.entry(enum_name.clone())
                                    .and_modify(|e| {
                                        e.usage_count += usage_count;
                                        if !e.variants.contains(&variant_name) {
                                            e.variants.push(variant_name.clone());
                                        }
                                        if !e.string_mappings.contains(&variant_name) {
                                            e.string_mappings.push(variant_name.clone());
                                        }
                                    })
                                    .or_insert(EnumWithStrings {
                                        enum_name: enum_name.clone(),
                                        variants: vec![variant_name.clone()],
                                        string_mappings: vec![variant_name],
                                        usage_count,
                                    });
                            }
                        }
                    }
                }
            }
        }
    }
}

fn extract_enums_from_file(json: &Value, enum_data: &mut HashMap<String, EnumWithStrings>) {
    if let Some(usages) = json.get("usages").and_then(|u| u.as_array()) {
        for usage in usages {
            if let (Some(usage_str), Some(used_def_id)) = (
                usage.get("usage").and_then(|u| u.as_str()),
                usage.get("used_def_id").and_then(|u| u.as_str())
            ) {
                if let Some(enum_info) = extract_enum_info(usage_str, used_def_id) {
                    enum_data.entry(enum_info.enum_name.clone())
                        .and_modify(|e| {
                            e.usage_count += 1;
                            // Merge string mappings
                            for mapping in &enum_info.string_mappings {
                                if !e.string_mappings.contains(mapping) {
                                    e.string_mappings.push(mapping.clone());
                                }
                            }
                        })
                        .or_insert(enum_info);
                }
            }
        }
    }
}

fn extract_enum_info(usage: &str, def_id: &str) -> Option<EnumWithStrings> {
    // Parse DefId format: DefId(2:11073 ~ core[4720]::result::{impl#0}::is_ok)
    if let Some(path_part) = def_id.strip_prefix("DefId(").and_then(|s| s.split(" ~ ").nth(1)) {
        if let Some(clean_path) = path_part.strip_suffix(")") {
            // Look for enum variant patterns (not impl blocks or functions)
            if !clean_path.contains("{impl#") && !clean_path.contains("(") {
                let parts: Vec<&str> = clean_path.split("::").collect();
                if parts.len() >= 2 {
                    let variant = parts[parts.len() - 1].to_string();
                    let enum_name = parts[parts.len() - 2].to_string();
                    
                    // Common enum variants to detect
                    if variant == "Some" || variant == "None" || variant == "Ok" || variant == "Err" ||
                       variant == "Continue" || variant == "Break" || variant == "true" || variant == "false" ||
                       variant.ends_with("Kind") || variant.ends_with("Type") || variant.ends_with("State") {
                        
                        let string_mapping = extract_string_mapping(usage, &variant);
        
                        if !enum_name.is_empty() && !variant.is_empty() {
                            return Some(EnumWithStrings {
                                enum_name,
                                variants: vec![variant],
                                string_mappings: if string_mapping.is_empty() { vec![] } else { vec![string_mapping] },
                                usage_count: 1,
                            });
                        }
                    }
                }
            }
        }
    }
    None
}

fn extract_enum_name(def_id: &str) -> String {
    if let Some(tilde_pos) = def_id.find(" ~ ") {
        if let Some(end_pos) = def_id.rfind(")") {
            let path = &def_id[tilde_pos + 3..end_pos];
            if let Some(last_colon) = path.rfind("::") {
                let before_variant = &path[..last_colon];
                if let Some(enum_colon) = before_variant.rfind("::") {
                    return before_variant[enum_colon + 2..].to_string();
                }
                return before_variant.to_string();
            }
        }
    }
    "Unknown".to_string()
}

fn extract_variant_name(def_id: &str) -> String {
    if let Some(tilde_pos) = def_id.find(" ~ ") {
        if let Some(end_pos) = def_id.rfind(")") {
            let path = &def_id[tilde_pos + 3..end_pos];
            if let Some(last_colon) = path.rfind("::") {
                return path[last_colon + 2..].to_string();
            }
        }
    }
    "Unknown".to_string()
}

fn extract_string_mapping(usage: &str, variant: &str) -> String {
    // Look for string literals in usage
    if usage.contains("\"") {
        if let Some(start) = usage.find("\"") {
            if let Some(end) = usage[start + 1..].find("\"") {
                return usage[start + 1..start + 1 + end].to_string();
            }
        }
    }
    
    // Default mapping: variant name to lowercase
    variant.to_lowercase()
}

fn generate_lattice_table(groups: &BTreeMap<usize, EnumGroup>) {
    println!("\n🏗️  Enum Lattice by Size N:");
    println!("   Size | Count | Total Usage | Top Enums");
    println!("   -----|-------|-------------|----------");
    
    for (size, group) in groups {
        let top_enums: Vec<_> = group.enums.iter()
            .take(3)
            .map(|e| format!("{}({})", e.enum_name, e.usage_count))
            .collect();
        
        println!("   {:4} | {:5} | {:11} | {}", 
            size, 
            group.enums.len(), 
            group.total_count,
            top_enums.join(", ")
        );
    }
}

fn generate_mkrust_macros(groups: &BTreeMap<usize, EnumGroup>) {
    println!("\n🎯 Generated mkrust! Macro Arguments:");
    
    for (size, group) in groups.iter() {  // All sizes
        println!("\n   // Size {} enums ({} total)", size, group.enums.len());
        
        for enum_info in group.enums.iter() {  // All enums
            let macro_name = generate_macro_name(&enum_info.enum_name);
            let variants = enum_info.variants.join(", ");
            let strings = enum_info.string_mappings.join(", ");
            
            println!("   mkrust!(");
            println!("       {}!,", macro_name);
            println!("       enum_size = {},", size);
            println!("       variants = [{}],", variants);
            println!("       strings = [{}],", strings);
            println!("       usage_count = {}", enum_info.usage_count);
            println!("   );");
        }
    }
    
    // Generate specific macro types
    println!("\n🔧 Specific Macro Types:");
    
    for (size, group) in groups {
        for enum_info in &group.enums {
            if enum_info.enum_name.contains("Visibility") {
                println!("   mkrust!(mk-priv-flag!, enum = {}, size = {})", enum_info.enum_name, size);
            }
            if enum_info.enum_name.contains("Level") {
                println!("   mkrust!(mk-level!, enum = {}, size = {})", enum_info.enum_name, size);
            }
            if enum_info.enum_name.contains("Kind") {
                println!("   mkrust!(mk-kind!, enum = {}, size = {})", enum_info.enum_name, size);
            }
        }
    }
}

fn generate_macro_name(enum_name: &str) -> String {
    let name = enum_name.to_lowercase()
        .replace("visibility", "priv-flag")
        .replace("mutability", "mut-flag")
        .replace("safety", "safe-flag")
        .replace("level", "level")
        .replace("kind", "kind")
        .replace("type", "type")
        .replace("state", "state")
        .replace("option", "option")
        .replace("result", "result")
        .replace("controlflow", "flow");
    
    format!("mk-{}!", name)
}

fn save_group_files(groups: &BTreeMap<usize, EnumGroup>) {
    fs::create_dir_all("enum_lattice_groups").expect("Failed to create directory");
    
    for (size, group) in groups {
        let filename = format!("enum_lattice_groups/size_{}_enums.json", size);
        
        let group_data = serde_json::json!({
            "size": size,
            "enum_count": group.enums.len(),
            "total_usage": group.total_count,
            "enums": group.enums.iter().map(|e| {
                serde_json::json!({
                    "name": e.enum_name,
                    "variants": e.variants,
                    "string_mappings": e.string_mappings,
                    "usage_count": e.usage_count,
                    "macro_name": generate_macro_name(&e.enum_name)
                })
            }).collect::<Vec<_>>()
        });
        
        fs::write(&filename, serde_json::to_string_pretty(&group_data).unwrap())
            .expect("Failed to write group file");
    }
    
    println!("\n💾 Saved {} group files to enum_lattice_groups/", groups.len());
    
    // Generate master mkrust! macro file
    let mkrust_code = generate_mkrust_master_macro(groups);
    fs::write("mkrust_generated.rs", mkrust_code)
        .expect("Failed to write mkrust macro file");
    
    println!("💾 Generated mkrust_generated.rs with all macro definitions");
}

fn generate_mkrust_master_macro(groups: &BTreeMap<usize, EnumGroup>) -> String {
    let mut code = String::from(r#"
// Auto-generated mkrust! macro definitions
// Each enum becomes a macro argument for selective language construction

#[macro_export]
macro_rules! mkrust {
    // Privacy flags
    (mk-priv-flag!, enum = $enum:ident, size = $size:expr) => {
        #[cfg(feature = "privacy")]
        pub enum $enum {
            // Generated variants based on size
        }
        
        #[cfg(not(feature = "privacy"))]
        pub enum $enum {
            Public, // Collapsed to single variant
        }
    };
    
    // Level enums
    (mk-level!, enum = $enum:ident, size = $size:expr) => {
        #[cfg(feature = "levels")]
        pub enum $enum {
            // Size-based level variants
        }
    };
    
    // Kind enums  
    (mk-kind!, enum = $enum:ident, size = $size:expr) => {
        #[cfg(feature = "kinds")]
        pub enum $enum {
            // Type kind variants
        }
    };
    
    // Generic enum constructor
    ($macro_name:ident, enum_size = $size:expr, variants = [$($variant:ident),*], strings = [$($string:literal),*], usage_count = $count:expr) => {
        compile_time_assert!($size == count_variants!($($variant),*));
        
        pub enum GeneratedEnum {
            $($variant),*
        }
        
        impl GeneratedEnum {
            pub fn to_string(&self) -> &'static str {
                match self {
                    $(Self::$variant => $string),*
                }
            }
        }
    };
}

// Helper macros
macro_rules! count_variants {
    () => { 0 };
    ($head:ident $(, $tail:ident)*) => { 1 + count_variants!($($tail),*) };
}

macro_rules! compile_time_assert {
    ($condition:expr) => {
        const _: () = assert!($condition);
    };
}

"#);
    
    // Add specific enum definitions
    for (size, group) in groups.iter().take(20) {  // Top 20 sizes
        code.push_str(&format!("\n// Size {} enums\n", size));
        
        for enum_info in group.enums.iter().take(10) {  // Top 10 per size
            let macro_name = generate_macro_name(&enum_info.enum_name);
            code.push_str(&format!(
                "// {} with {} variants, {} uses\n",
                enum_info.enum_name, enum_info.variants.len(), enum_info.usage_count
            ));
        }
    }
    
    code
}
