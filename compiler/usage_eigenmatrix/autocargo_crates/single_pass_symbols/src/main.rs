use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use syn::{parse_file, visit::Visit, Item};

fn main() {
    println!("🚀 Single-pass ALL rustc symbols analysis + save items");
    
    let mut all_symbols = HashSet::new();
    let mut usage_counts = HashMap::new();
    let mut relations = HashMap::new();
    let mut saved_items = HashMap::new();
    
    // Single pass: extract symbols AND count usages AND find relations AND save items
    scan_directory_once("submodules/rust", &mut all_symbols, &mut usage_counts, &mut relations, &mut saved_items);
    
    println!("✅ Found {} symbols, {} usages, {} relationships, {} saved items", 
        all_symbols.len(), usage_counts.len(), relations.len(), saved_items.len());
    
    output_results(&all_symbols, &usage_counts, &relations);
    save_items(&saved_items);
    generate_macros(&usage_counts);
}

fn scan_directory_once(
    dir: &str, 
    symbols: &mut HashSet<String>,
    usage_counts: &mut HashMap<String, usize>,
    relations: &mut HashMap<String, Vec<String>>,
    saved_items: &mut HashMap<String, String>
) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                scan_directory_once(&path.to_string_lossy(), symbols, usage_counts, relations, saved_items);
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                process_file_once(&path, symbols, usage_counts, relations, saved_items);
            }
        }
    }
}

fn process_file_once(
    path: &Path,
    symbols: &mut HashSet<String>,
    usage_counts: &mut HashMap<String, usize>,
    relations: &mut HashMap<String, Vec<String>>,
    saved_items: &mut HashMap<String, String>
) {
    if let Ok(content) = fs::read_to_string(path) {
        if let Ok(ast) = parse_file(&content) {
            // Extract symbols from AST and save items
            let mut extractor = SymbolExtractor::new(path.to_string_lossy().to_string());
            extractor.visit_file(&ast);
            
            symbols.extend(extractor.symbols.clone());
            saved_items.extend(extractor.items);
            
            // Count usages in content
            for symbol in &extractor.symbols {
                let count = content.matches(symbol).count();
                if count > 0 {
                    *usage_counts.entry(symbol.clone()).or_insert(0) += count;
                }
            }
            
            // Find relations
            let found_symbols: Vec<_> = extractor.symbols.iter()
                .filter(|s| content.contains(*s))
                .cloned()
                .collect();
            
            for i in 0..found_symbols.len() {
                for j in 0..found_symbols.len() {
                    if i != j {
                        relations.entry(found_symbols[i].clone())
                            .or_default()
                            .push(found_symbols[j].clone());
                    }
                }
            }
        }
    }
}

fn save_items(saved_items: &HashMap<String, String>) {
    let mut output = String::new();
    output.push_str("# All Rustc Items\n\n");
    
    for (symbol, item_code) in saved_items {
        output.push_str(&format!("## {}\n\n```rust\n{}\n```\n\n", symbol, item_code));
    }
    
    fs::write("all_rustc_items.md", output).unwrap();
    println!("✅ Saved {} items to all_rustc_items.md", saved_items.len());
}

fn output_results(
    symbols: &HashSet<String>, 
    usage_counts: &HashMap<String, usize>,
    relations: &HashMap<String, Vec<String>>
) {
    let mut sorted: Vec<_> = symbols.iter().collect();
    sorted.sort_by(|a, b| usage_counts.get(*b).unwrap_or(&0).cmp(usage_counts.get(*a).unwrap_or(&0)));
    
    let mut output = format!("# ALL Rustc Symbols (Single Pass)\n\nTotal: {}\nWith usages: {}\nRelationships: {}\n\n## Top 100\n\n", 
        symbols.len(), usage_counts.len(), relations.len());
    
    for symbol in sorted.iter().take(100) {
        let count = usage_counts.get(*symbol).unwrap_or(&0);
        let rel_count = relations.get(*symbol).map_or(0, |v| v.len());
        if *count > 0 {
            output.push_str(&format!("- **{}** - {} uses, {} relations\n", symbol, count, rel_count));
        }
    }
    
    fs::write("single_pass_rustc_symbols.md", output).unwrap();
    println!("✅ Results saved to single_pass_rustc_symbols.md");
}

fn generate_macros(usage_counts: &HashMap<String, usize>) {
    let mut sorted: Vec<_> = usage_counts.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    
    let mut macros = String::new();
    for (symbol, count) in sorted.iter().take(200) {
        if **count > 5 {
            let name = symbol.replace("::", "_").replace("!", "_")
                .chars().filter(|c| c.is_alphanumeric() || *c == '_').collect::<String>();
            macros.push_str(&format!("macro_rules! rustc_{} {{ () => {{ {} }}; }}\n", name, symbol));
        }
    }
    
    fs::write("src/single_pass_macros.rs", macros).unwrap();
    println!("✅ Generated {} macros", sorted.iter().filter(|(_, c)| **c > 5).count());
}

struct SymbolExtractor {
    symbols: HashSet<String>,
    items: HashMap<String, String>,
    file_path: String,
}

impl SymbolExtractor {
    fn new(file_path: String) -> Self { 
        Self { 
            symbols: HashSet::new(),
            items: HashMap::new(),
            file_path,
        } 
    }
}

impl<'ast> Visit<'ast> for SymbolExtractor {
    fn visit_item(&mut self, item: &'ast Item) {
        let item_code = quote::quote!(#item).to_string();
        
        match item {
            Item::Struct(s) => { 
                let name = s.ident.to_string();
                self.symbols.insert(name.clone());
                self.items.insert(format!("{}::{}", self.file_path, name), item_code);
            }
            Item::Enum(e) => { 
                let name = e.ident.to_string();
                self.symbols.insert(name.clone());
                self.items.insert(format!("{}::{}", self.file_path, name), item_code);
                for v in &e.variants { 
                    self.symbols.insert(format!("{}::{}", e.ident, v.ident)); 
                }
            }
            Item::Fn(f) => { 
                let name = f.sig.ident.to_string();
                self.symbols.insert(name.clone());
                self.items.insert(format!("{}::{}", self.file_path, name), item_code);
            }
            Item::Type(t) => { 
                let name = t.ident.to_string();
                self.symbols.insert(name.clone());
                self.items.insert(format!("{}::{}", self.file_path, name), item_code);
            }
            Item::Const(c) => { 
                let name = c.ident.to_string();
                self.symbols.insert(name.clone());
                self.items.insert(format!("{}::{}", self.file_path, name), item_code);
            }
            Item::Static(s) => { 
                let name = s.ident.to_string();
                self.symbols.insert(name.clone());
                self.items.insert(format!("{}::{}", self.file_path, name), item_code);
            }
            Item::Trait(t) => { 
                let name = t.ident.to_string();
                self.symbols.insert(name.clone());
                self.items.insert(format!("{}::{}", self.file_path, name), item_code);
            }
            Item::Macro(m) => { 
                if let Some(ident) = &m.ident { 
                    let name = format!("{}!", ident);
                    self.symbols.insert(name.clone());
                    self.items.insert(format!("{}::{}", self.file_path, name), item_code);
                }
            }
            _ => {}
        }
        syn::visit::visit_item(self, item);
    }
}
