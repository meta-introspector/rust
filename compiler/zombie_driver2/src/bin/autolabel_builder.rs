use goblin::elf::Elf;
use serde_json;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, serde::Serialize)]
struct TypeStringFunction {
    symbol_name: String,
    type_name: String,
    function_size: u64,
    address: u64,
    references: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏷️ RUSTC TYPE->STRING AUTOLABELING SET");
    println!("======================================");

    let rustc_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/deps/librustc_driver.so";

    println!("📂 Reading: {}", rustc_path);
    let buffer = fs::read(rustc_path)?;

    println!("🧮 Parsing ELF...");
    let elf = Elf::parse(&buffer)?;

    // Count references
    let mut reference_count = HashMap::new();
    for (_, reloc_section) in &elf.shdr_relocs {
        for reloc in reloc_section.iter() {
            let sym_idx = reloc.r_sym;
            if sym_idx < elf.syms.len() {
                *reference_count.entry(sym_idx).or_insert(0) += 1;
            }
        }
    }

    let mut type_string_functions = Vec::new();

    // Extract all type->string conversion functions
    for (i, sym) in elf.syms.iter().enumerate() {
        if let Some(name) = elf.strtab.get_at(sym.st_name) {
            if sym.st_type() == 2 && sym.st_size > 0 {
                // FUNC type with size
                // Look for Debug/Display/ToString implementations
                if (name.contains("fmt") && (name.contains("Debug") || name.contains("Display")))
                    || name.contains("to_string")
                    || name.contains("ToString")
                {
                    let type_name = extract_type_name(name);
                    let refs = reference_count.get(&i).unwrap_or(&0);

                    type_string_functions.push(TypeStringFunction {
                        symbol_name: name.to_string(),
                        type_name,
                        function_size: sym.st_size,
                        address: sym.st_value,
                        references: *refs,
                    });
                }
            }
        }
    }

    // Sort by importance (size + references)
    type_string_functions.sort_by(|a, b| {
        let weight_a = a.function_size + (a.references as u64 * 10);
        let weight_b = b.function_size + (b.references as u64 * 10);
        weight_b.cmp(&weight_a)
    });

    println!("🎯 Found {} type->string functions", type_string_functions.len());
    println!("\n📋 TOP AUTOLABELING FUNCTIONS:");

    let mut unique_types = HashMap::new();
    for func in &type_string_functions {
        *unique_types.entry(&func.type_name).or_insert(0) += 1;
    }

    println!("📊 Unique types with string conversion: {}", unique_types.len());

    // Display top functions
    for (i, func) in type_string_functions.iter().take(30).enumerate() {
        let weight = func.function_size + (func.references as u64 * 10);
        println!(
            "{:2}: {} | type: {} | size: {} | refs: {} | weight: {}",
            i + 1,
            truncate_symbol(&func.symbol_name, 60),
            func.type_name,
            func.function_size,
            func.references,
            weight
        );
    }

    // Group by type categories
    let mut categories = HashMap::new();
    for func in &type_string_functions {
        let category = categorize_type(&func.type_name);
        categories.entry(category).or_insert(Vec::new()).push(func);
    }

    println!("\n🗂️ TYPE CATEGORIES:");
    for (category, funcs) in &categories {
        println!("  {}: {} functions", category, funcs.len());
    }

    // Save autolabeling set
    let autolabel_set = serde_json::json!({
        "total_functions": type_string_functions.len(),
        "unique_types": unique_types.len(),
        "categories": categories.keys().collect::<Vec<_>>(),
        "top_functions": type_string_functions.iter().take(50).collect::<Vec<_>>()
    });

    fs::write("rustc_autolabel_set.json", serde_json::to_string_pretty(&autolabel_set)?)?;
    println!("\n💾 Saved autolabeling set to rustc_autolabel_set.json");

    Ok(())
}

fn extract_type_name(mangled: &str) -> String {
    // Extract type name from Rust mangled symbol
    if mangled.contains("_$LT$") && mangled.contains("$GT$") {
        if let Some(start) = mangled.find("_$LT$") {
            if let Some(end) = mangled.find("$GT$") {
                let inner = &mangled[start + 5..end];
                let cleaned = inner
                    .replace("$u20$", " ")
                    .replace("..", "::")
                    .split("$u20$as$u20$")
                    .next()
                    .unwrap_or(inner)
                    .to_string();
                return cleaned;
            }
        }
    }

    // Fallback: extract from function name patterns
    if let Some(pos) = mangled.rfind("::") {
        let after_colon = &mangled[pos + 2..];
        if after_colon.len() < 50 {
            return after_colon.to_string();
        }
    }

    "Unknown".to_string()
}

fn categorize_type(type_name: &str) -> String {
    if type_name.contains("rustc") {
        if type_name.contains("ty") {
            "Compiler-Types".to_string()
        } else if type_name.contains("mir") {
            "Compiler-MIR".to_string()
        } else if type_name.contains("hir") {
            "Compiler-HIR".to_string()
        } else {
            "Compiler-Other".to_string()
        }
    } else if type_name.contains("core") || type_name.contains("std") {
        "Standard-Library".to_string()
    } else if type_name.contains("Error") || type_name.contains("errno") {
        "Error-Types".to_string()
    } else if type_name.contains("bool") || type_name.contains("i32") || type_name.contains("u64") {
        "Primitive-Types".to_string()
    } else {
        "External-Crates".to_string()
    }
}

fn truncate_symbol(s: &str, max_len: usize) -> String {
    if s.len() <= max_len { s.to_string() } else { format!("{}...", &s[..max_len - 3]) }
}
