use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use syn::{parse_file, visit::Visit, Item};

fn main() {
    println!("🚀 Single pass + merge + order analysis");
    
    // Pass 1: Collect ALL symbols and usages
    let (all_symbols, all_usages) = collect_symbols_and_usages();
    println!("✅ Collected {} symbols, {} usage patterns", all_symbols.len(), all_usages.len());
    
    // Pass 2: Merge and resolve (usage might be another decl)
    let resolved = merge_and_resolve(&all_symbols, &all_usages);
    println!("✅ Resolved {} symbol relationships", resolved.len());
    
    // Pass 3: Order by dependency chain
    let ordered = order_by_dependencies(&resolved);
    println!("✅ Ordered {} symbols by dependencies", ordered.len());
    
    output_ordered_results(&ordered);
}

fn collect_symbols_and_usages() -> (HashMap<String, String>, HashMap<String, Vec<String>>) {
    let mut symbols = HashMap::new(); // symbol -> source_code
    let mut usages = HashMap::new();  // symbol -> [files_using_it]
    
    scan_all_files("submodules/rust", &mut symbols, &mut usages);
    (symbols, usages)
}

fn scan_all_files(dir: &str, symbols: &mut HashMap<String, String>, usages: &mut HashMap<String, Vec<String>>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                scan_all_files(&path.to_string_lossy(), symbols, usages);
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                process_single_file(&path, symbols, usages);
            }
        }
    }
}

fn process_single_file(path: &Path, symbols: &mut HashMap<String, String>, usages: &mut HashMap<String, Vec<String>>) {
    if let Ok(content) = fs::read_to_string(path) {
        let file_name = path.to_string_lossy().to_string();
        
        // Extract symbols declared in this file
        if let Ok(ast) = parse_file(&content) {
            for item in ast.items {
                if let Some((name, code)) = extract_symbol(&item) {
                    symbols.insert(name, code);
                }
            }
        }
        
        // Find all symbol usages in this file
        for (symbol, _) in symbols.iter() {
            if content.contains(symbol) {
                usages.entry(symbol.clone()).or_default().push(file_name.clone());
            }
        }
    }
}

fn extract_symbol(item: &Item) -> Option<(String, String)> {
    let code = format!("{:#?}", item); // Simple debug format
    match item {
        Item::Struct(s) => Some((s.ident.to_string(), code)),
        Item::Enum(e) => Some((e.ident.to_string(), code)),
        Item::Fn(f) => Some((f.sig.ident.to_string(), code)),
        Item::Type(t) => Some((t.ident.to_string(), code)),
        Item::Const(c) => Some((c.ident.to_string(), code)),
        Item::Static(s) => Some((s.ident.to_string(), code)),
        Item::Trait(t) => Some((t.ident.to_string(), code)),
        _ => None,
    }
}

fn merge_and_resolve(symbols: &HashMap<String, String>, usages: &HashMap<String, Vec<String>>) -> HashMap<String, SymbolInfo> {
    let mut resolved = HashMap::new();
    
    // Merge symbols with their usage info
    for (symbol, code) in symbols {
        let usage_files = usages.get(symbol).cloned().unwrap_or_default();
        let depends_on = find_dependencies(code, symbols);
        
        resolved.insert(symbol.clone(), SymbolInfo {
            name: symbol.clone(),
            code: code.clone(),
            used_in: usage_files,
            depends_on,
        });
    }
    
    resolved
}

fn find_dependencies(code: &str, all_symbols: &HashMap<String, String>) -> Vec<String> {
    let mut deps = Vec::new();
    for symbol in all_symbols.keys() {
        if code.contains(symbol) {
            deps.push(symbol.clone());
        }
    }
    deps
}

fn order_by_dependencies(resolved: &HashMap<String, SymbolInfo>) -> Vec<String> {
    let mut ordered = Vec::new();
    let mut visited = HashSet::new();
    
    // Simple topological sort
    for symbol in resolved.keys() {
        if !visited.contains(symbol) {
            visit_symbol(symbol, resolved, &mut visited, &mut ordered);
        }
    }
    
    ordered
}

fn visit_symbol(symbol: &str, resolved: &HashMap<String, SymbolInfo>, visited: &mut HashSet<String>, ordered: &mut Vec<String>) {
    if visited.contains(symbol) {
        return;
    }
    
    visited.insert(symbol.to_string());
    
    if let Some(info) = resolved.get(symbol) {
        // Visit dependencies first
        for dep in &info.depends_on {
            if dep != symbol { // Avoid self-reference
                visit_symbol(dep, resolved, visited, ordered);
            }
        }
    }
    
    ordered.push(symbol.to_string());
}

fn output_ordered_results(ordered: &[String]) {
    let mut output = String::new();
    output.push_str("# Ordered Rustc Symbols by Dependencies\n\n");
    
    for (i, symbol) in ordered.iter().enumerate() {
        output.push_str(&format!("{}. {}\n", i + 1, symbol));
    }
    
    fs::write("ordered_rustc_symbols.md", output).unwrap();
    println!("✅ Saved ordered symbols to ordered_rustc_symbols.md");
}

#[derive(Debug)]
struct SymbolInfo {
    name: String,
    code: String,
    used_in: Vec<String>,
    depends_on: Vec<String>,
}
