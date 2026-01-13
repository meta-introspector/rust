use serde_json::Value;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "🔍 ENUM-TO-STRING FUNCTION SCANNER - Finding Functions with Enum Args & String Returns"
    );
    println!("===================================================================================");

    let mut enum_string_functions = Vec::new();
    let mut total_scanned = 0;

    println!("📊 SCANNING ALL FUNCTIONS FOR ENUM→STRING PATTERNS...");

    let analysis_dir = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/complete_analysis/";

    for chunk_dir in fs::read_dir(analysis_dir)? {
        let chunk_dir = chunk_dir?;
        if !chunk_dir.file_type()?.is_dir() {
            continue;
        }

        for file in fs::read_dir(chunk_dir.path())? {
            let file = file?;
            let filename = file.file_name();
            let filename_str = filename.to_string_lossy();

            if filename_str.ends_with(".json") {
                total_scanned += 1;

                if let Ok(content) = fs::read_to_string(file.path()) {
                    if let Ok(data) = serde_json::from_str::<Value>(&content) {
                        if is_enum_to_string_function(&data) {
                            let addr =
                                data["memory_address"].as_str().unwrap_or("unknown").to_string();
                            let symbol = data["symbol_name"].as_str().unwrap_or("").to_string();
                            let demangled =
                                data["demangled_name"].as_str().unwrap_or("").to_string();

                            println!("   ✅ Found: {}", addr);
                            enum_string_functions.push((addr, symbol, demangled, data));
                        }
                    }
                }

                if total_scanned % 1000 == 0 {
                    println!("   Scanned {} functions...", total_scanned);
                }
            }
        }
    }

    println!("✅ Scan complete: {} functions analyzed", total_scanned);
    println!("🎯 Found {} enum-to-string functions", enum_string_functions.len());

    // Analyze the found functions
    println!("\n🔍 ANALYZING ENUM-TO-STRING FUNCTIONS:");
    for (i, (addr, symbol, demangled, data)) in enum_string_functions.iter().enumerate() {
        analyze_enum_function(i + 1, addr, symbol, demangled, data);
    }

    // Look for demangle functions specifically
    println!("\n🔧 SEARCHING FOR DEMANGLE FUNCTIONS:");
    find_demangle_functions(&enum_string_functions);

    Ok(())
}

fn is_enum_to_string_function(data: &Value) -> bool {
    let symbol = data["symbol_name"].as_str().unwrap_or("");
    let demangled = data["demangled_name"].as_str().unwrap_or("");

    // Look for Debug trait implementations (enum → string)
    if symbol.contains("fmt..Debug") && symbol.contains("fmt17h") {
        return true;
    }

    // Look for Display trait implementations
    if symbol.contains("fmt..Display") && symbol.contains("fmt17h") {
        return true;
    }

    // Look for to_string functions
    if symbol.contains("to_string") || demangled.contains("to_string") {
        return true;
    }

    // Look for functions that mention known enum types
    let enum_types =
        ["TokenKind", "ExprKind", "ItemKind", "TyKind", "DefKind", "PickKind", "NodeKind"];
    for enum_type in &enum_types {
        if (symbol.contains(enum_type) || demangled.contains(enum_type))
            && (symbol.contains("fmt") || symbol.contains("string") || symbol.contains("str"))
        {
            return true;
        }
    }

    // Look for functions with string-related output
    if let Some(strings) = data["related_strings"].as_array() {
        if !strings.is_empty() && (symbol.contains("Kind") || demangled.contains("Kind")) {
            return true;
        }
    }

    false
}

fn analyze_enum_function(index: usize, addr: &str, symbol: &str, demangled: &str, data: &Value) {
    println!("   {}. ADDRESS: {}", index, addr);

    // Extract enum type
    let enum_type = extract_enum_type_from_symbol(symbol, demangled);
    println!("      ENUM TYPE: {}", enum_type);

    // Extract function purpose
    let purpose = extract_function_purpose(symbol, demangled);
    println!("      PURPOSE: {}", purpose);

    // Show related strings
    if let Some(strings) = data["related_strings"].as_array() {
        let string_list: Vec<String> =
            strings.iter().filter_map(|s| s.as_str()).map(|s| s.to_string()).collect();
        if !string_list.is_empty() {
            println!("      STRINGS: {:?}", string_list);
        }
    }

    // Show function size and complexity
    if let Some(size) = data["size"].as_u64() {
        println!("      SIZE: {} bytes", size);
    }

    // Show LMFDB signature for authenticity
    if let Some(lmfdb) = data["mathematical_analysis"]["lmfdb_analysis"]["lmfdb_key"].as_str() {
        println!("      LMFDB: {}", lmfdb);
    }

    println!();
}

fn extract_enum_type_from_symbol(symbol: &str, demangled: &str) -> String {
    let combined = format!("{} {}", symbol, demangled);

    if combined.contains("TokenKind") {
        return "TokenKind".to_string();
    }
    if combined.contains("ExprKind") {
        return "ExprKind".to_string();
    }
    if combined.contains("ItemKind") {
        return "ItemKind".to_string();
    }
    if combined.contains("TyKind") {
        return "TyKind".to_string();
    }
    if combined.contains("DefKind") {
        return "DefKind".to_string();
    }
    if combined.contains("PickKind") {
        return "PickKind".to_string();
    }
    if combined.contains("NodeKind") {
        return "NodeKind".to_string();
    }
    if combined.contains("BinOpKind") {
        return "BinOpKind".to_string();
    }
    if combined.contains("UnOpKind") {
        return "UnOpKind".to_string();
    }

    "Unknown".to_string()
}

fn extract_function_purpose(symbol: &str, demangled: &str) -> String {
    let combined = format!("{} {}", symbol, demangled);

    if combined.contains("fmt..Debug") {
        return "Debug formatter".to_string();
    }
    if combined.contains("fmt..Display") {
        return "Display formatter".to_string();
    }
    if combined.contains("to_string") {
        return "String conversion".to_string();
    }
    if combined.contains("fmt17h") {
        return "Format function".to_string();
    }

    "String-related function".to_string()
}

fn find_demangle_functions(functions: &[(String, String, String, Value)]) {
    let mut demangle_count = 0;

    for (addr, symbol, demangled, data) in functions {
        if symbol.contains("demangle")
            || demangled.contains("demangle")
            || symbol.contains("mangle")
            || demangled.contains("mangle")
        {
            demangle_count += 1;
            println!("   🔧 DEMANGLE FUNCTION: {}", addr);
            println!("      Symbol: {}", symbol);
            println!("      Demangled: {}", demangled);

            // This would be a perfect candidate for decompilation
            if let Some(size) = data["size"].as_u64() {
                println!("      Size: {} bytes (good for decompilation)", size);
            }
            println!();
        }
    }

    if demangle_count == 0 {
        println!("   No demangle functions found in enum-to-string set");
        println!("   (Demangle functions might be in separate analysis)");
    } else {
        println!("   Found {} demangle functions", demangle_count);
    }
}
