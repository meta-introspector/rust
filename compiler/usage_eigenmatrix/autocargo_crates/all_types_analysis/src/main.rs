use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use syn::{parse_file, visit::Visit, Item, ItemStruct, ItemEnum, ItemFn, ItemType, ItemConst, ItemStatic, ItemTrait, Type, TypePath, Path as SynPath, Expr, ExprPath};

fn main() {
    println!("🔬 Complete rustc type analysis - all types, all crates");
    
    // Step 1: Collect ALL types from ALL crates
    println!("📋 Pass 1: Collecting ALL types from ALL crates...");
    let all_types = collect_all_types();
    println!("✅ Found {} total types", all_types.len());
    
    // Step 2: Count ALL usages of ALL types in ALL programs
    println!("🔍 Pass 2: Counting ALL usages...");
    let usage_counts = count_all_usages(&all_types);
    println!("✅ Counted usages for {} types", usage_counts.len());
    
    // Step 3: Scan again and relate each type with each other
    println!("🔄 Pass 3: Relating types with each other...");
    let type_relations = build_type_relations(&all_types, &usage_counts);
    println!("✅ Built {} type relationships", type_relations.len());
    
    // Step 4: Generate comprehensive output
    output_complete_analysis(&all_types, &usage_counts, &type_relations);
    
    println!("✅ Complete analysis finished");
}

fn collect_all_types() -> HashSet<String> {
    let mut all_types = HashSet::new();
    scan_directory_for_types("submodules/rust", &mut all_types);
    all_types
}

fn scan_directory_for_types(dir_path: &str, types: &mut HashSet<String>) {
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                scan_directory_for_types(&path.to_string_lossy(), types);
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                extract_types_from_file(&path, types);
            }
        }
    }
}

fn extract_types_from_file(file_path: &Path, types: &mut HashSet<String>) {
    if let Ok(content) = fs::read_to_string(file_path) {
        if let Ok(ast) = parse_file(&content) {
            let mut visitor = TypeCollector::new();
            visitor.visit_file(&ast);
            types.extend(visitor.types);
        }
    }
}

fn count_all_usages(all_types: &HashSet<String>) -> HashMap<String, usize> {
    let mut usage_counts = HashMap::new();
    scan_directory_for_usages("submodules/rust", all_types, &mut usage_counts);
    usage_counts
}

fn scan_directory_for_usages(dir_path: &str, all_types: &HashSet<String>, counts: &mut HashMap<String, usize>) {
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                scan_directory_for_usages(&path.to_string_lossy(), all_types, counts);
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                count_usages_in_file(&path, all_types, counts);
            }
        }
    }
}

fn count_usages_in_file(file_path: &Path, all_types: &HashSet<String>, counts: &mut HashMap<String, usize>) {
    if let Ok(content) = fs::read_to_string(file_path) {
        if let Ok(ast) = parse_file(&content) {
            let mut visitor = UsageCounter::new(all_types);
            visitor.visit_file(&ast);
            for (type_name, count) in visitor.usage_counts {
                *counts.entry(type_name).or_insert(0) += count;
            }
        }
    }
}

fn build_type_relations(all_types: &HashSet<String>, usage_counts: &HashMap<String, usize>) -> HashMap<String, Vec<String>> {
    let mut relations = HashMap::new();
    scan_directory_for_relations("submodules/rust", all_types, &mut relations);
    relations
}

fn scan_directory_for_relations(dir_path: &str, all_types: &HashSet<String>, relations: &mut HashMap<String, Vec<String>>) {
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                scan_directory_for_relations(&path.to_string_lossy(), all_types, relations);
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                find_relations_in_file(&path, all_types, relations);
            }
        }
    }
}

fn find_relations_in_file(file_path: &Path, all_types: &HashSet<String>, relations: &mut HashMap<String, Vec<String>>) {
    if let Ok(content) = fs::read_to_string(file_path) {
        if let Ok(ast) = parse_file(&content) {
            let mut visitor = RelationFinder::new(all_types);
            visitor.visit_file(&ast);
            for (type_name, related_types) in visitor.relations {
                relations.entry(type_name).or_default().extend(related_types);
            }
        }
    }
}

fn output_complete_analysis(all_types: &HashSet<String>, usage_counts: &HashMap<String, usize>, relations: &HashMap<String, Vec<String>>) {
    let mut output = String::new();
    output.push_str("# Complete Rustc Type Analysis\n\n");
    
    // Sort by usage frequency
    let mut sorted_types: Vec<_> = all_types.iter().collect();
    sorted_types.sort_by(|a, b| {
        let count_a = usage_counts.get(*a).unwrap_or(&0);
        let count_b = usage_counts.get(*b).unwrap_or(&0);
        count_b.cmp(count_a)
    });
    
    output.push_str(&format!("## Summary\n- Total types: {}\n- Types with usages: {}\n- Total relationships: {}\n\n", 
        all_types.len(),
        usage_counts.len(),
        relations.len()
    ));
    
    output.push_str("## Top 100 Most Used Types\n\n");
    for type_name in sorted_types.iter().take(100) {
        let count = usage_counts.get(*type_name).unwrap_or(&0);
        let related_count = relations.get(*type_name).map_or(0, |v| v.len());
        if *count > 0 {
            output.push_str(&format!("- **{}** - {} usages, {} relationships\n", type_name, count, related_count));
        }
    }
    
    fs::write("complete_rustc_analysis.md", output).expect("Failed to write analysis");
    
    // Generate macros for top types
    generate_macros_for_top_types(&sorted_types, usage_counts, relations);
    
    println!("✅ Analysis saved to complete_rustc_analysis.md");
}

fn generate_macros_for_top_types(sorted_types: &[&String], usage_counts: &HashMap<String, usize>, relations: &HashMap<String, Vec<String>>) {
    let mut macro_code = String::new();
    macro_code.push_str("// Auto-generated macros for all rustc types\n\n");
    
    for type_name in sorted_types.iter().take(200) {
        let count = usage_counts.get(*type_name).unwrap_or(&0);
        if *count > 5 {
            let macro_name = sanitize_name(type_name);
            macro_code.push_str(&format!(
                "macro_rules! rustc_{} {{\n    () => {{ {} }}; // {} uses\n}}\n\n",
                macro_name, type_name, count
            ));
        }
    }
    
    fs::write("src/all_rustc_macros.rs", macro_code).expect("Failed to write macros");
    println!("✅ Generated macros in src/all_rustc_macros.rs");
}

fn sanitize_name(name: &str) -> String {
    name.replace("::", "_").replace("<", "_").replace(">", "_").chars().filter(|c| c.is_alphanumeric() || *c == '_').collect()
}

struct TypeCollector {
    types: HashSet<String>,
}

impl TypeCollector {
    fn new() -> Self {
        Self { types: HashSet::new() }
    }
}

impl<'ast> Visit<'ast> for TypeCollector {
    fn visit_item_struct(&mut self, item: &'ast ItemStruct) {
        self.types.insert(item.ident.to_string());
        syn::visit::visit_item_struct(self, item);
    }
    
    fn visit_item_enum(&mut self, item: &'ast ItemEnum) {
        self.types.insert(item.ident.to_string());
        for variant in &item.variants {
            self.types.insert(format!("{}::{}", item.ident, variant.ident));
        }
        syn::visit::visit_item_enum(self, item);
    }
    
    fn visit_item_fn(&mut self, item: &'ast ItemFn) {
        self.types.insert(item.sig.ident.to_string());
        syn::visit::visit_item_fn(self, item);
    }
    
    fn visit_item_type(&mut self, item: &'ast ItemType) {
        self.types.insert(item.ident.to_string());
        syn::visit::visit_item_type(self, item);
    }
    
    fn visit_item_trait(&mut self, item: &'ast ItemTrait) {
        self.types.insert(item.ident.to_string());
        syn::visit::visit_item_trait(self, item);
    }
}

struct UsageCounter<'a> {
    all_types: &'a HashSet<String>,
    usage_counts: HashMap<String, usize>,
}

impl<'a> UsageCounter<'a> {
    fn new(all_types: &'a HashSet<String>) -> Self {
        Self { all_types, usage_counts: HashMap::new() }
    }
}

impl<'ast> Visit<'ast> for UsageCounter<'_> {
    fn visit_type(&mut self, ty: &'ast Type) {
        if let Type::Path(type_path) = ty {
            let path_str = path_to_string(&type_path.path);
            for type_name in self.all_types {
                if path_str.contains(type_name) {
                    *self.usage_counts.entry(type_name.clone()).or_insert(0) += 1;
                }
            }
        }
        syn::visit::visit_type(self, ty);
    }
    
    fn visit_expr(&mut self, expr: &'ast Expr) {
        if let Expr::Path(expr_path) = expr {
            let path_str = path_to_string(&expr_path.path);
            for type_name in self.all_types {
                if path_str.contains(type_name) {
                    *self.usage_counts.entry(type_name.clone()).or_insert(0) += 1;
                }
            }
        }
        syn::visit::visit_expr(self, expr);
    }
}

struct RelationFinder<'a> {
    all_types: &'a HashSet<String>,
    relations: HashMap<String, Vec<String>>,
}

impl<'a> RelationFinder<'a> {
    fn new(all_types: &'a HashSet<String>) -> Self {
        Self { all_types, relations: HashMap::new() }
    }
}

impl<'ast> Visit<'ast> for RelationFinder<'_> {
    fn visit_type(&mut self, ty: &'ast Type) {
        if let Type::Path(type_path) = ty {
            let path_str = path_to_string(&type_path.path);
            let mut found_types = Vec::new();
            for type_name in self.all_types {
                if path_str.contains(type_name) {
                    found_types.push(type_name.clone());
                }
            }
            // Relate all found types with each other
            for i in 0..found_types.len() {
                for j in 0..found_types.len() {
                    if i != j {
                        self.relations.entry(found_types[i].clone()).or_default().push(found_types[j].clone());
                    }
                }
            }
        }
        syn::visit::visit_type(self, ty);
    }
}

fn path_to_string(path: &SynPath) -> String {
    path.segments.iter().map(|seg| seg.ident.to_string()).collect::<Vec<_>>().join("::")
}
