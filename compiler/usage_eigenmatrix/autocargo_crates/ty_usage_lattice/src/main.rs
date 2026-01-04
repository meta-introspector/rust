use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use syn::{parse_file, visit::Visit, Item, ItemStruct, ItemEnum, ItemFn, ItemType, ItemConst, ItemStatic, ItemTrait, Type, TypePath, Path as SynPath, Expr, ExprPath};

fn main() {
    println!("🔬 Building rustc_ty usage lattice (depth 8) - single pass");
    
    // Step 1: Single pass over rustc_ty to collect all declarations
    println!("📋 Collecting ty declarations...");
    let ty_decls = collect_ty_declarations();
    println!("✅ Found {} declarations in rustc_ty", ty_decls.len());
    save_intermediate("ty_declarations.txt", &ty_decls.iter().collect::<Vec<_>>());
    
    // Step 2: Single pass over all other crates to find usages
    println!("🔍 Scanning all crates for ty usages...");
    let usage_map = scan_all_crates_for_ty_usage(&ty_decls);
    
    // Step 3: Build lattice from usage data
    let lattice = build_lattice_from_usage(&ty_decls, &usage_map, 8);
    
    // Step 4: Generate usage-based macros
    generate_usage_macros(&lattice);
    
    // Step 5: Output results
    output_lattice(&lattice);
    
    println!("✅ Built usage lattice with {} nodes", lattice.len());
}

fn collect_ty_declarations() -> HashSet<String> {
    let mut decls = HashSet::new();
    let ty_path = "submodules/rust/compiler/rustc_middle/src/ty";
    
    if let Ok(entries) = fs::read_dir(ty_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "rs") {
                extract_declarations_from_file(&path, &mut decls);
            }
        }
    }
    
    decls
}

fn extract_declarations_from_file(file_path: &Path, decls: &mut HashSet<String>) {
    if let Ok(content) = fs::read_to_string(file_path) {
        if let Ok(ast) = parse_file(&content) {
            let mut visitor = DeclVisitor::new();
            visitor.visit_file(&ast);
            decls.extend(visitor.declarations);
        }
    }
}

fn scan_all_crates_for_ty_usage(ty_decls: &HashSet<String>) -> HashMap<String, Vec<String>> {
    let mut usage_map = HashMap::new();
    let rustc_dir = "submodules/rust/compiler";
    
    if let Ok(entries) = fs::read_dir(rustc_dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let crate_name = entry.file_name().to_string_lossy().to_string();
                println!("  Scanning crate: {}", crate_name);
                scan_crate_for_all_ty_usage(&entry.path(), ty_decls, &mut usage_map, &crate_name);
            }
        }
    }
    
    usage_map
}

fn scan_crate_for_all_ty_usage(
    crate_path: &Path, 
    ty_decls: &HashSet<String>,
    usage_map: &mut HashMap<String, Vec<String>>,
    crate_name: &str
) {
    if let Ok(entries) = fs::read_dir(crate_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                scan_crate_for_all_ty_usage(&path, ty_decls, usage_map, crate_name);
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                find_all_ty_usage_in_file(&path, ty_decls, usage_map, crate_name);
            }
        }
    }
}

fn find_all_ty_usage_in_file(
    file_path: &Path,
    ty_decls: &HashSet<String>,
    usage_map: &mut HashMap<String, Vec<String>>,
    crate_name: &str
) {
    if let Ok(content) = fs::read_to_string(file_path) {
        if let Ok(ast) = parse_file(&content) {
            let mut visitor = AllUsageVisitor::new(ty_decls, crate_name);
            visitor.visit_file(&ast);
            
            // Merge found usages into usage_map
            for (ty_decl, usages) in visitor.found_usages {
                usage_map.entry(ty_decl).or_default().extend(usages);
            }
        }
    }
}

fn build_lattice_from_usage(
    ty_decls: &HashSet<String>,
    usage_map: &HashMap<String, Vec<String>>,
    max_depth: usize
) -> HashMap<String, UsageNode> {
    let mut lattice = HashMap::new();
    
    // Initialize with ty declarations at depth 0
    for decl in ty_decls {
        lattice.insert(decl.clone(), UsageNode {
            name: decl.clone(),
            depth: 0,
            used_by: usage_map.get(decl).cloned().unwrap_or_default().into_iter().collect(),
            uses: HashSet::new(),
        });
    }
    
    // Build deeper levels
    for depth in 1..max_depth {
        let current_level: Vec<_> = lattice.values()
            .filter(|node| node.depth == depth - 1)
            .flat_map(|node| &node.used_by)
            .cloned()
            .collect();
        
        for usage in current_level {
            if !lattice.contains_key(&usage) {
                lattice.insert(usage.clone(), UsageNode {
                    name: usage.clone(),
                    depth,
                    used_by: usage_map.get(&usage).cloned().unwrap_or_default().into_iter().collect(),
                    uses: HashSet::new(),
                });
            }
        }
    }
    
    lattice
}

struct DeclVisitor {
    declarations: HashSet<String>,
}

impl DeclVisitor {
    fn new() -> Self {
        Self {
            declarations: HashSet::new(),
        }
    }
}

impl<'ast> Visit<'ast> for DeclVisitor {
    fn visit_item_struct(&mut self, item: &'ast ItemStruct) {
        self.declarations.insert(format!("ty::{}", item.ident));
        syn::visit::visit_item_struct(self, item);
    }
    
    fn visit_item_enum(&mut self, item: &'ast ItemEnum) {
        self.declarations.insert(format!("ty::{}", item.ident));
        for variant in &item.variants {
            self.declarations.insert(format!("ty::{}::{}", item.ident, variant.ident));
        }
        syn::visit::visit_item_enum(self, item);
    }
    
    fn visit_item_fn(&mut self, item: &'ast ItemFn) {
        self.declarations.insert(format!("ty::{}", item.sig.ident));
        syn::visit::visit_item_fn(self, item);
    }
    
    fn visit_item_type(&mut self, item: &'ast ItemType) {
        self.declarations.insert(format!("ty::{}", item.ident));
        syn::visit::visit_item_type(self, item);
    }
    
    fn visit_item_const(&mut self, item: &'ast ItemConst) {
        self.declarations.insert(format!("ty::{}", item.ident));
        syn::visit::visit_item_const(self, item);
    }
    
    fn visit_item_static(&mut self, item: &'ast ItemStatic) {
        self.declarations.insert(format!("ty::{}", item.ident));
        syn::visit::visit_item_static(self, item);
    }
    
    fn visit_item_trait(&mut self, item: &'ast ItemTrait) {
        self.declarations.insert(format!("ty::{}", item.ident));
        syn::visit::visit_item_trait(self, item);
    }
}

struct AllUsageVisitor<'a> {
    ty_decls: &'a HashSet<String>,
    crate_name: String,
    found_usages: HashMap<String, Vec<String>>,
}

impl<'a> AllUsageVisitor<'a> {
    fn new(ty_decls: &'a HashSet<String>, crate_name: &str) -> Self {
        Self {
            ty_decls,
            crate_name: crate_name.to_string(),
            found_usages: HashMap::new(),
        }
    }
}

impl<'ast> Visit<'ast> for AllUsageVisitor<'_> {
    fn visit_type(&mut self, ty: &'ast Type) {
        if let Type::Path(type_path) = ty {
            let path_str = path_to_string(&type_path.path);
            for ty_decl in self.ty_decls {
                if path_str.contains(&ty_decl.replace("ty::", "")) {
                    self.found_usages.entry(ty_decl.clone())
                        .or_default()
                        .push(format!("{}::{}", self.crate_name, path_str));
                }
            }
        }
        syn::visit::visit_type(self, ty);
    }
    
    fn visit_expr(&mut self, expr: &'ast Expr) {
        if let Expr::Path(expr_path) = expr {
            let path_str = path_to_string(&expr_path.path);
            for ty_decl in self.ty_decls {
                if path_str.contains(&ty_decl.replace("ty::", "")) {
                    self.found_usages.entry(ty_decl.clone())
                        .or_default()
                        .push(format!("{}::{}", self.crate_name, path_str));
                }
            }
        }
        syn::visit::visit_expr(self, expr);
    }
}

fn path_to_string(path: &SynPath) -> String {
    path.segments.iter()
        .map(|seg| seg.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}

fn output_lattice(lattice: &HashMap<String, UsageNode>) {
    let mut output = String::new();
    
    // Group by depth
    let mut by_depth: HashMap<usize, Vec<&UsageNode>> = HashMap::new();
    for node in lattice.values() {
        by_depth.entry(node.depth).or_default().push(node);
    }
    
    output.push_str("# rustc_ty Usage Lattice (4-deep)\n\n");
    
    for depth in 0..4 {
        if let Some(nodes) = by_depth.get(&depth) {
            output.push_str(&format!("## Depth {} ({} nodes)\n\n", depth, nodes.len()));
            
            for node in nodes.iter().take(10) {
                output.push_str(&format!("- **{}** (used by {} nodes)\n", 
                    node.name, node.used_by.len()));
                
                if !node.used_by.is_empty() {
                    output.push_str("  - Used by: ");
                    let used_by: Vec<_> = node.used_by.iter().take(5).collect();
                    output.push_str(&used_by.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "));
                    if node.used_by.len() > 5 {
                        output.push_str(&format!(" (+{} more)", node.used_by.len() - 5));
                    }
                    output.push('\n');
                }
            }
            
            if nodes.len() > 10 {
                output.push_str(&format!("  ... and {} more nodes\n", nodes.len() - 10));
            }
            output.push('\n');
        }
    }
    
    fs::write("rustc_ty_usage_lattice.md", output)
        .expect("Failed to write lattice");
    
    println!("✅ Lattice saved to rustc_ty_usage_lattice.md");
}

fn save_intermediate(filename: &str, data: &[&String]) {
    let content = data.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n");
    fs::write(filename, content).expect("Failed to save intermediate");
    println!("💾 Saved intermediate: {}", filename);
}

#[derive(Debug)]
struct UsageNode {
    name: String,
    depth: usize,
    used_by: HashSet<String>,
    uses: HashSet<String>,
}
