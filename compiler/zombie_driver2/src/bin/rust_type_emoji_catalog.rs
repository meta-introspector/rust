use goblin::elf::Elf;
use serde_json;
use std::collections::{BTreeMap, HashMap};
use std::fs;

#[derive(Debug, Clone)]
struct RustTypeEntry {
    name: String,
    category: String,
    emoji: String,
    curve_class: String,
    frequency: usize,
    complexity_score: f64,
    mathematical_properties: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 RUST ENUM & TYPE EMOJI CATALOG GENERATOR");
    println!("===========================================");

    let rustc_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/deps/librustc_driver.so";

    println!("📂 Loading rustc driver: {}", rustc_path);
    let buffer = fs::read(rustc_path)?;

    println!("🧮 Parsing ELF...");
    let elf = Elf::parse(&buffer)?;

    // Extract .rodata for string analysis
    let mut rodata_section = None;
    for section in &elf.section_headers {
        if let Some(name) = elf.shdr_strtab.get_at(section.sh_name) {
            if name == ".rodata" {
                let start = section.sh_offset as usize;
                let size = section.sh_size as usize;
                if start + size <= buffer.len() {
                    rodata_section = Some(&buffer[start..start + size]);
                }
                break;
            }
        }
    }

    let rodata = rodata_section.ok_or("No .rodata section found")?;

    // Extract Rust type names from symbols and rodata
    let mut rust_types = HashMap::new();

    println!("🔍 Extracting Rust types from symbols...");

    // Analyze symbols for type information
    for sym in elf.syms.iter() {
        if let Some(name) = elf.strtab.get_at(sym.st_name) {
            if sym.st_type() == 2 && sym.st_size > 0 {
                // FUNC type
                extract_types_from_symbol(name, &mut rust_types);
            }
        }
    }

    println!("🔤 Extracting types from rodata strings...");

    // Extract type names from rodata strings
    extract_types_from_rodata(rodata, &mut rust_types);

    println!("📊 Found {} unique Rust types", rust_types.len());

    // Create emoji catalog
    let mut catalog = Vec::new();

    for (type_name, frequency) in rust_types {
        let entry = create_type_entry(&type_name, frequency);
        catalog.push(entry);
    }

    // Sort by frequency and complexity
    catalog.sort_by(|a, b| {
        b.frequency.cmp(&a.frequency).then(
            b.complexity_score
                .partial_cmp(&a.complexity_score)
                .unwrap_or(std::cmp::Ordering::Equal),
        )
    });

    println!("\n🎨 RUST TYPE EMOJI CATALOG");
    println!("==========================");

    // Display top entries
    for (i, entry) in catalog.iter().take(50).enumerate() {
        println!(
            "{:2}: {} {} | freq:{} | complexity:{:.2} | curve:{}",
            i + 1,
            entry.emoji,
            entry.name,
            entry.frequency,
            entry.complexity_score,
            entry.curve_class
        );
    }

    // Group by categories
    let mut categories = BTreeMap::new();
    for entry in &catalog {
        categories.entry(&entry.category).or_insert(Vec::new()).push(entry);
    }

    println!("\n📚 CATALOG BY CATEGORY:");
    println!("======================");

    for (category, entries) in &categories {
        println!("\n🏷️ {}: {} types", category, entries.len());
        for entry in entries.iter().take(10) {
            println!("   {} {} ({}x)", entry.emoji, entry.name, entry.frequency);
        }
    }

    // Create comprehensive emoji mapping
    create_emoji_mapping_file(&catalog)?;

    // Save complete catalog
    let catalog_json = serde_json::json!({
        "total_types": catalog.len(),
        "categories": categories.keys().collect::<Vec<_>>(),
        "catalog": catalog.iter().take(200).map(|e| {
            serde_json::json!({
                "name": e.name,
                "category": e.category,
                "emoji": e.emoji,
                "curve_class": e.curve_class,
                "frequency": e.frequency,
                "complexity_score": e.complexity_score,
                "mathematical_properties": e.mathematical_properties
            })
        }).collect::<Vec<_>>()
    });

    fs::write("rust_type_emoji_catalog.json", serde_json::to_string_pretty(&catalog_json)?)?;
    println!("\n💾 Saved catalog to rust_type_emoji_catalog.json");

    Ok(())
}

fn extract_types_from_symbol(symbol_name: &str, types: &mut HashMap<String, usize>) {
    // Extract Rust type names from mangled symbols
    if symbol_name.contains("_ZN") {
        // Demangle common Rust patterns
        if let Some(type_part) = extract_rust_type_from_mangled(symbol_name) {
            *types.entry(type_part).or_insert(0) += 1;
        }
    }

    // Look for common type patterns
    let type_patterns = [
        "Result",
        "Option",
        "Vec",
        "HashMap",
        "BTreeMap",
        "String",
        "str",
        "ExprKind",
        "TyKind",
        "ItemKind",
        "PatKind",
        "StmtKind",
        "BinOpKind",
        "UnOpKind",
        "LitKind",
        "FieldDef",
        "Variant",
        "TraitItem",
        "ImplItem",
        "FnDecl",
        "Generics",
        "WhereClause",
        "Lifetime",
        "Path",
        "QPath",
        "Ty",
        "Expr",
        "Pat",
        "Stmt",
        "Block",
        "Arm",
        "Local",
        "Item",
        "Crate",
        "Mod",
        "ForeignMod",
        "UseTree",
        "VisibilityKind",
        "Mutability",
        "Constness",
        "Defaultness",
        "Unsafety",
        "Asyncness",
        "IsAuto",
        "ImplPolarity",
    ];

    for pattern in &type_patterns {
        if symbol_name.contains(pattern) {
            *types.entry(pattern.to_string()).or_insert(0) += 1;
        }
    }
}

fn extract_rust_type_from_mangled(mangled: &str) -> Option<String> {
    // Simple extraction of type names from Rust mangled symbols
    if let Some(start) = mangled.find("_$LT$") {
        if let Some(end) = mangled.find("$GT$") {
            let inner = &mangled[start + 5..end];
            let cleaned = inner.replace("$u20$", " ").replace("..", "::");
            if let Some(type_name) = cleaned.split("::").last() {
                if type_name.len() > 2 && type_name.len() < 30 {
                    return Some(type_name.to_string());
                }
            }
        }
    }
    None
}

fn extract_types_from_rodata(rodata: &[u8], types: &mut HashMap<String, usize>) {
    // Extract Rust type names from string literals in rodata
    let mut i = 0;
    while i < rodata.len() {
        if rodata[i] >= 32 && rodata[i] <= 126 {
            // Printable ASCII
            let start = i;
            while i < rodata.len() && rodata[i] != 0 && rodata[i] >= 32 && rodata[i] <= 126 {
                i += 1;
            }

            if i > start && (i - start) >= 3 && (i - start) <= 50 {
                if let Ok(s) = std::str::from_utf8(&rodata[start..i]) {
                    // Look for Rust type patterns
                    if is_rust_type_name(s) {
                        *types.entry(s.to_string()).or_insert(0) += 1;
                    }
                }
            }
        } else {
            i += 1;
        }
    }
}

fn is_rust_type_name(s: &str) -> bool {
    // Heuristics to identify Rust type names
    if s.len() < 3 || s.len() > 30 {
        return false;
    }

    // Must start with uppercase or be a known lowercase type
    let first_char = s.chars().next().unwrap();
    if !first_char.is_uppercase() && !["str", "bool", "char", "isize", "usize"].contains(&s) {
        return false;
    }

    // Common Rust type suffixes/patterns
    let rust_patterns = [
        "Kind", "Type", "Def", "Item", "Expr", "Pat", "Stmt", "Decl", "Result", "Option", "Vec",
        "Map", "Set", "Tree", "List", "Error", "Info", "Data", "Node", "Token", "Span", "Symbol",
    ];

    for pattern in &rust_patterns {
        if s.contains(pattern) {
            return true;
        }
    }

    // Check if it looks like a Rust identifier
    s.chars().all(|c| c.is_alphanumeric() || c == '_') && s.chars().any(|c| c.is_uppercase())
}

fn create_type_entry(type_name: &str, frequency: usize) -> RustTypeEntry {
    let category = categorize_rust_type(type_name);
    let emoji = assign_emoji(type_name, &category);
    let curve_class = calculate_curve_class(type_name, frequency);
    let complexity_score = calculate_complexity_score(type_name, frequency);
    let math_props = derive_mathematical_properties(type_name, frequency);

    RustTypeEntry {
        name: type_name.to_string(),
        category,
        emoji,
        curve_class,
        frequency,
        complexity_score,
        mathematical_properties: math_props,
    }
}

fn categorize_rust_type(type_name: &str) -> String {
    if type_name.contains("Kind") {
        "Enum Variants".to_string()
    } else if type_name.contains("Expr") {
        "Expressions".to_string()
    } else if type_name.contains("Ty") || type_name.contains("Type") {
        "Type System".to_string()
    } else if type_name.contains("Pat") {
        "Patterns".to_string()
    } else if type_name.contains("Stmt") {
        "Statements".to_string()
    } else if type_name.contains("Item") {
        "Items".to_string()
    } else if type_name.contains("Decl") {
        "Declarations".to_string()
    } else if type_name.contains("Error") {
        "Error Types".to_string()
    } else if ["Result", "Option", "Vec", "HashMap"].contains(&type_name) {
        "Standard Collections".to_string()
    } else if ["str", "String", "bool", "char"].contains(&type_name) {
        "Primitive Types".to_string()
    } else {
        "Other Types".to_string()
    }
}

fn assign_emoji(type_name: &str, category: &str) -> String {
    // Assign emojis based on type characteristics and mathematical curves
    match category {
        "Enum Variants" => match type_name {
            s if s.contains("Expr") => "🌳", // Expression tree
            s if s.contains("Ty") => "🔢",   // Type numbers
            s if s.contains("Pat") => "🎯",  // Pattern matching
            s if s.contains("Stmt") => "📝", // Statements
            s if s.contains("Item") => "📦", // Items/modules
            s if s.contains("Bin") => "⚖️",  // Binary operations
            s if s.contains("Un") => "🔄",   // Unary operations
            s if s.contains("Lit") => "💎",  // Literals
            _ => "🎭",                       // Generic enum
        },
        "Expressions" => match type_name {
            s if s.contains("Call") => "📞",  // Function calls
            s if s.contains("Path") => "🛤️",  // Paths
            s if s.contains("Block") => "🧱", // Blocks
            s if s.contains("If") => "🔀",    // Conditionals
            s if s.contains("Loop") => "🔄",  // Loops
            s if s.contains("Match") => "🎯", // Pattern matching
            _ => "🌳",                        // Generic expression tree
        },
        "Type System" => match type_name {
            s if s.contains("Ref") => "👉",   // References
            s if s.contains("Ptr") => "🎯",   // Pointers
            s if s.contains("Slice") => "🍰", // Slices
            s if s.contains("Array") => "📊", // Arrays
            s if s.contains("Tuple") => "📦", // Tuples
            s if s.contains("Fn") => "⚡",    // Functions
            _ => "🔢",                        // Generic types
        },
        "Patterns" => "🎯",
        "Statements" => "📝",
        "Items" => "📦",
        "Declarations" => "📋",
        "Error Types" => "🚨",
        "Standard Collections" => match type_name {
            "Result" => "✅",
            "Option" => "❓",
            "Vec" => "📊",
            "HashMap" => "🗺️",
            "BTreeMap" => "🌳",
            _ => "📚",
        },
        "Primitive Types" => match type_name {
            "str" | "String" => "📝",
            "bool" => "🔘",
            "char" => "🔤",
            _ => "🔹",
        },
        _ => "⬜",
    }
    .to_string()
}

fn calculate_curve_class(type_name: &str, frequency: usize) -> String {
    // Map to our mathematical curve classes based on complexity
    let complexity = type_name.len() + (frequency as f64).log2() as usize;

    match complexity {
        0..=5 => "Linear".to_string(),
        6..=10 => "Quadratic".to_string(),
        11..=15 => "Cubic".to_string(),
        16..=20 => "Quartic".to_string(),
        21..=25 => "Quintic".to_string(),
        _ => "Elliptic".to_string(),
    }
}

fn calculate_complexity_score(type_name: &str, frequency: usize) -> f64 {
    let base_complexity = type_name.len() as f64;
    let frequency_factor = (frequency as f64 + 1.0).log2();
    let pattern_complexity = if type_name.contains("Kind") { 2.0 } else { 1.0 };

    base_complexity * frequency_factor * pattern_complexity
}

fn derive_mathematical_properties(type_name: &str, frequency: usize) -> String {
    let genus = (type_name.len() / 5).min(3);
    let rank = (frequency as f64).log2() as usize % 3;
    let torsion = if type_name.contains("Kind") { "Z/2Z" } else { "trivial" };

    format!("genus:{}, rank:{}, torsion:{}", genus, rank, torsion)
}

fn create_emoji_mapping_file(catalog: &[RustTypeEntry]) -> Result<(), Box<dyn std::error::Error>> {
    let mut mapping = String::new();
    mapping.push_str("# 🦀 RUST TYPE EMOJI MAPPING\n");
    mapping.push_str("# ===========================\n\n");

    // Group by category for the mapping file
    let mut categories = BTreeMap::new();
    for entry in catalog {
        categories.entry(&entry.category).or_insert(Vec::new()).push(entry);
    }

    for (category, entries) in categories {
        mapping.push_str(&format!("## {} {}\n\n", get_category_emoji(category), category));

        for entry in entries.iter().take(20) {
            mapping.push_str(&format!(
                "- {} `{}` ({}x) - {}\n",
                entry.emoji, entry.name, entry.frequency, entry.curve_class
            ));
        }
        mapping.push_str("\n");
    }

    mapping.push_str("## 🎨 Usage in Tapestries\n\n");
    mapping.push_str("These emojis are used in our tapestry generation system to create\n");
    mapping.push_str("visual representations of Rust code character. Each type gets mapped\n");
    mapping.push_str("to its corresponding emoji based on:\n\n");
    mapping.push_str("- **Frequency**: How often it appears in the codebase\n");
    mapping.push_str("- **Complexity**: Mathematical complexity score\n");
    mapping.push_str("- **Curve Class**: Mathematical curve classification\n");
    mapping.push_str("- **Category**: Semantic category in Rust\n\n");

    fs::write("RUST_TYPE_EMOJI_MAPPING.md", mapping)?;
    println!("📝 Created emoji mapping file: RUST_TYPE_EMOJI_MAPPING.md");

    Ok(())
}

fn get_category_emoji(category: &str) -> &str {
    match category {
        "Enum Variants" => "🎭",
        "Expressions" => "🌳",
        "Type System" => "🔢",
        "Patterns" => "🎯",
        "Statements" => "📝",
        "Items" => "📦",
        "Declarations" => "📋",
        "Error Types" => "🚨",
        "Standard Collections" => "📚",
        "Primitive Types" => "🔹",
        _ => "⬜",
    }
}
