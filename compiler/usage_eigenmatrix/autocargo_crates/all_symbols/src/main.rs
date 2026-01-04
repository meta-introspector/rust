use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use syn::{parse_file, visit::Visit, Item, Type, Expr, Macro, Pat};

fn main() {
    println!("🚀 ALL rustc symbols analysis - types, macros, consts, functions");
    
    // Pass 1: Extract ALL symbols
    let all_symbols = extract_all_symbols();
    println!("✅ Found {} symbols", all_symbols.len());
    
    // Pass 2: Count ALL usages
    let usage_counts = count_all_symbol_usages(&all_symbols);
    println!("✅ Counted {} usage patterns", usage_counts.len());
    
    // Pass 3: Build relationships
    let relations = build_symbol_relations(&all_symbols);
    println!("✅ Built {} relationships", relations.len());
    
    // Output everything
    output_all(&all_symbols, &usage_counts, &relations);
    generate_all_macros(&usage_counts);
}

fn extract_all_symbols() -> HashSet<String> {
    let mut symbols = HashSet::new();
    scan_all("submodules/rust", &mut symbols, extract_symbols_from_file);
    symbols
}

fn count_all_symbol_usages(symbols: &HashSet<String>) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    scan_all("submodules/rust", &mut (&symbols, &mut counts), |path, data| {
        let (symbols, counts) = data;
        count_usages_in_file(path, symbols, counts);
    });
    counts
}

fn build_symbol_relations(symbols: &HashSet<String>) -> HashMap<String, Vec<String>> {
    let mut relations = HashMap::new();
    scan_all("submodules/rust", &mut (&symbols, &mut relations), |path, data| {
        let (symbols, relations) = data;
        find_relations_in_file(path, symbols, relations);
    });
    relations
}

fn scan_all<T, F>(dir: &str, data: &mut T, mut process: F) 
where F: FnMut(&Path, &mut T) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                scan_all(&path.to_string_lossy(), data, &mut process);
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                process(&path, data);
            }
        }
    }
}

fn extract_symbols_from_file(path: &Path, symbols: &mut HashSet<String>) {
    if let Ok(content) = fs::read_to_string(path) {
        if let Ok(ast) = parse_file(&content) {
            let mut visitor = AllSymbolExtractor::new();
            visitor.visit_file(&ast);
            symbols.extend(visitor.symbols);
        }
    }
}

fn count_usages_in_file(path: &Path, symbols: &HashSet<String>, counts: &mut HashMap<String, usize>) {
    if let Ok(content) = fs::read_to_string(path) {
        for symbol in symbols {
            let count = content.matches(symbol).count();
            if count > 0 {
                *counts.entry(symbol.clone()).or_insert(0) += count;
            }
        }
    }
}

fn find_relations_in_file(path: &Path, symbols: &HashSet<String>, relations: &mut HashMap<String, Vec<String>>) {
    if let Ok(content) = fs::read_to_string(path) {
        let found: Vec<_> = symbols.iter().filter(|s| content.contains(*s)).cloned().collect();
        for i in 0..found.len() {
            for j in 0..found.len() {
                if i != j {
                    relations.entry(found[i].clone()).or_default().push(found[j].clone());
                }
            }
        }
    }
}

fn output_all(symbols: &HashSet<String>, counts: &HashMap<String, usize>, relations: &HashMap<String, Vec<String>>) {
    let mut sorted: Vec<_> = symbols.iter().collect();
    sorted.sort_by(|a, b| counts.get(*b).unwrap_or(&0).cmp(counts.get(*a).unwrap_or(&0)));
    
    let mut output = format!("# ALL Rustc Symbols\n\nTotal: {}\nWith usages: {}\n\n## Top 200\n\n", 
        symbols.len(), counts.len());
    
    for symbol in sorted.iter().take(200) {
        let count = counts.get(*symbol).unwrap_or(&0);
        let rel_count = relations.get(*symbol).map_or(0, |v| v.len());
        if *count > 0 {
            output.push_str(&format!("- **{}** - {} uses, {} relations\n", symbol, count, rel_count));
        }
    }
    
    fs::write("all_rustc_symbols.md", output).unwrap();
    println!("✅ Saved to all_rustc_symbols.md");
}

fn generate_all_macros(counts: &HashMap<String, usize>) {
    let mut sorted: Vec<_> = counts.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    
    let mut macros = String::new();
    for (symbol, count) in sorted.iter().take(500) {
        if **count > 10 {
            let name = symbol.replace("::", "_").replace("!", "_").chars().filter(|c| c.is_alphanumeric() || *c == '_').collect::<String>();
            macros.push_str(&format!("macro_rules! rustc_{} {{ () => {{ {} }}; }}\n", name, symbol));
        }
    }
    
    fs::write("src/all_rustc_symbol_macros.rs", macros).unwrap();
    println!("✅ Generated 500 macros in src/all_rustc_symbol_macros.rs");
}

struct AllSymbolExtractor {
    symbols: HashSet<String>,
}

impl AllSymbolExtractor {
    fn new() -> Self { Self { symbols: HashSet::new() } }
}

impl<'ast> Visit<'ast> for AllSymbolExtractor {
    fn visit_item(&mut self, item: &'ast Item) {
        match item {
            Item::Struct(s) => { self.symbols.insert(s.ident.to_string()); }
            Item::Enum(e) => { 
                self.symbols.insert(e.ident.to_string());
                for v in &e.variants { self.symbols.insert(format!("{}::{}", e.ident, v.ident)); }
            }
            Item::Fn(f) => { self.symbols.insert(f.sig.ident.to_string()); }
            Item::Type(t) => { self.symbols.insert(t.ident.to_string()); }
            Item::Const(c) => { self.symbols.insert(c.ident.to_string()); }
            Item::Static(s) => { self.symbols.insert(s.ident.to_string()); }
            Item::Trait(t) => { self.symbols.insert(t.ident.to_string()); }
            Item::Macro(m) => { 
                if let Some(ident) = &m.ident { self.symbols.insert(format!("{}!", ident)); }
            }
            _ => {}
        }
        syn::visit::visit_item(self, item);
    }
    
    fn visit_macro(&mut self, mac: &'ast Macro) {
        let path = mac.path.segments.iter().map(|s| s.ident.to_string()).collect::<Vec<_>>().join("::");
        self.symbols.insert(format!("{}!", path));
        syn::visit::visit_macro(self, mac);
    }
}
