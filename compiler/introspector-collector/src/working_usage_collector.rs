#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_hir;
extern crate rustc_ast;


use rustc_driver::{Callbacks, Compilation};
use rustc_interface::interface;
use rustc_middle::ty::TyCtxt;
use rustc_hir::def_id::LOCAL_CRATE;
use rustc_hir::intravisit::{self, Visitor};
use std::io::Write;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CleanGraphData {
    crate_name: String,
    call_graph: HashMap<String, Vec<String>>,
    symbol_table: HashMap<String, String>,
    def_paths: HashMap<String, String>,
    total_nodes: usize,
}

#[derive(Serialize, Deserialize, Clone)]
struct UsageEntry {
    symbol: String,
    kind: String,
    usage_count: usize,
    usage_type: String,
    node_type: String,
    user_def_id: String,
    used_def_id: String,
    user_crate: Option<String>,
    used_crate: Option<String>,
    used_def_kind: Option<String>, // DefKind of the thing being used
    user_def_kind: Option<String>, // DefKind of the thing doing the using
}


#[derive(Serialize, Deserialize, Clone, Default)]
struct UsageClassification {
    string_conversion: usize,    // to_string, Display, fmt usage
    pattern_matching: usize,     // match arms, if let usage  
    construction: usize,         // Enum::Variant construction
    comparison: usize,           // == != usage
    debug_format: usize,         // Debug, {:?} usage
    serialization: usize,        // serde, json usage
    general: usize,              // other usage
}

#[derive(Serialize, Deserialize, Clone)]
struct EnumVariantUsage {
    enum_name: String,
    variant_name: String,
    usage_classes: UsageClassification,
    top_converters: HashMap<String, Vec<(String, usize)>>, // usage_type -> [(function, count)]
}

#[derive(Serialize, Deserialize, Clone)]
struct ItemComplexity {
    name: String,
    item_type: String, // "struct", "enum", "function", "macro"
    complexity: String, // "simple", "medium", "complex"
    field_count: Option<usize>,
    variant_count: Option<usize>,
    param_count: Option<usize>,
    fields: Option<Vec<FieldInfo>>,
    variants: Option<Vec<VariantInfo>>,
    parameters: Option<Vec<ParamInfo>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct FieldInfo {
    name: String,
    field_type: String,
    is_public: bool,
}

#[derive(Serialize, Deserialize, Clone)]
struct VariantInfo {
    name: String,
    has_fields: bool,
    field_count: usize,
}

#[derive(Serialize, Deserialize, Clone)]
struct ParamInfo {
    name: String,
    param_type: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct EnumInfo {
    name: String,
    variants: Vec<EnumVariantUsage>,
    total_usage_classes: UsageClassification,
}

#[derive(Serialize, Deserialize)]
struct ModuleData {
    #[serde(rename = "crate")]
    crate_name: String,
    module: String,
    usages: Vec<UsageEntry>,
}

struct UsageCollector {
    module_data: HashMap<String, Vec<UsageEntry>>,
    enum_data: HashMap<String, EnumInfo>,
    item_complexity: HashMap<String, ItemComplexity>,
}

impl UsageCollector {
    fn new() -> Self {
        Self {
            module_data: HashMap::new(),
            enum_data: HashMap::new(),
            item_complexity: HashMap::new(),
        }
    }
    
    fn get_defkind_from_item<'tcx>(&self, tcx: TyCtxt<'tcx>, item: &rustc_hir::Item) -> String {
        match &item.kind {
            rustc_hir::ItemKind::Const(_, _, _, body_id) => {
                let body = tcx.hir_body(*body_id);
                match &body.value.kind {
                    rustc_hir::ExprKind::Lit(lit) => match lit.node {
                        rustc_ast::LitKind::Str(..) => "StringLiteral",
                        rustc_ast::LitKind::Int(..) => "IntLiteral", 
                        rustc_ast::LitKind::Float(..) => "FloatLiteral",
                        rustc_ast::LitKind::Bool(..) => "BoolLiteral",
                        _ => "Const"
                    },
                    _ => "Const"
                }
            },
            rustc_hir::ItemKind::Static(..) => "Static",
            rustc_hir::ItemKind::Struct(..) => "Struct",
            rustc_hir::ItemKind::Enum(..) => "Enum",
            rustc_hir::ItemKind::Fn(..) => "Fn",
            _ => "Other"
        }.to_string()
    }
    

    }
    
    fn classify_usage(&self, usage: &str, usage_type: &str, _used_def_id: &str) -> UsageClassification {
        let mut classification = UsageClassification::default();
        
        // String conversion patterns
        if usage.contains("to_string") || usage.contains("Display") || usage.contains("fmt") || 
           usage.contains("format!") || usage.contains("write!") {
            classification.string_conversion = 1;
        }
        // Pattern matching
        else if usage.contains("match") || usage.contains("if let") || usage_type == "PatternMatch" {
            classification.pattern_matching = 1;
        }
        // Construction
        else if usage.contains("::") && !usage.contains("(") {
            classification.construction = 1;
        }
        // Comparison
        else if usage.contains("==") || usage.contains("!=") || usage.contains("cmp") {
            classification.comparison = 1;
        }
        // Debug formatting
        else if usage.contains("Debug") || usage.contains("{:?}") || usage.contains("dbg!") {
            classification.debug_format = 1;
        }
        // Serialization
        else if usage.contains("serde") || usage.contains("serialize") || usage.contains("json") {
            classification.serialization = 1;
        }
        // General usage
        else {
            classification.general = 1;
        }
        
        classification
    }
    
    fn extract_enum_variant(&self, def_id: &str) -> Option<(String, String)> {
        // Parse DefId format to extract enum and variant names
        if let Some(path_part) = def_id.strip_prefix("DefId(").and_then(|s| s.split(" ~ ").nth(1)) {
            if let Some(clean_path) = path_part.strip_suffix(")") {
                let parts: Vec<&str> = clean_path.split("::").collect();
                if parts.len() >= 2 {
                    let variant = parts[parts.len() - 1].to_string();
                    let enum_name = parts[parts.len() - 2].to_string();
                    
                    // Check if this looks like an enum variant (not a function)
                    if !variant.contains("(") && !variant.contains("<") && 
                       (variant.chars().next().unwrap_or('a').is_uppercase() || 
                        variant == "true" || variant == "false") {
                        return Some((enum_name, variant));
                    }
                }
            }
        }
        None
    }
    
    fn update_enum_usage(&mut self, enum_name: String, variant_name: String, 
                        classification: UsageClassification, usage_type: &str, converter_fn: &str) {
        let enum_info = self.enum_data.entry(enum_name.clone()).or_insert_with(|| EnumInfo {
            name: enum_name.clone(),
            variants: Vec::new(),
            total_usage_classes: UsageClassification::default(),
        });
        
        // Find or create variant
        if let Some(variant) = enum_info.variants.iter_mut().find(|v| v.variant_name == variant_name) {
            // Update existing variant
            variant.usage_classes.string_conversion += classification.string_conversion;
            variant.usage_classes.pattern_matching += classification.pattern_matching;
            variant.usage_classes.construction += classification.construction;
            variant.usage_classes.comparison += classification.comparison;
            variant.usage_classes.debug_format += classification.debug_format;
            variant.usage_classes.serialization += classification.serialization;
            variant.usage_classes.general += classification.general;
            
            // Track top converter functions (keep top 3)
            let converters = variant.top_converters.entry(usage_type.to_string()).or_insert_with(Vec::new);
            if let Some(existing) = converters.iter_mut().find(|(fn_name, _)| fn_name == converter_fn) {
                existing.1 += 1;
            } else {
                converters.push((converter_fn.to_string(), 1));
            }
            converters.sort_by(|a, b| b.1.cmp(&a.1));
            converters.truncate(3); // Keep top 3
        } else {
            // Create new variant
            let mut top_converters = HashMap::new();
            top_converters.insert(usage_type.to_string(), vec![(converter_fn.to_string(), 1)]);
            
            enum_info.variants.push(EnumVariantUsage {
                enum_name: enum_name.clone(),
                variant_name: variant_name.clone(),
                usage_classes: classification.clone(),
                top_converters,
            });
        }
        
        // Update total usage for enum
        enum_info.total_usage_classes.string_conversion += classification.string_conversion;
        enum_info.total_usage_classes.pattern_matching += classification.pattern_matching;
        enum_info.total_usage_classes.construction += classification.construction;
        enum_info.total_usage_classes.comparison += classification.comparison;
        enum_info.total_usage_classes.debug_format += classification.debug_format;
        enum_info.total_usage_classes.serialization += classification.serialization;
        enum_info.total_usage_classes.general += classification.general;
    }
    
    fn save_to_files(&self, crate_name: &str) {
        // Use proper Cargo environment variables for target directory
        let output_dir = std::env::var("USAGE_OUTPUT_DIR")
            .or_else(|_| {
                // Try CARGO_TARGET_DIR first (user override)
                std::env::var("CARGO_TARGET_DIR")
                    .map(|target_dir| format!("{}/harmonic/usage", target_dir))
            })
            .or_else(|_| {
                // Fall back to calculated target path using CARGO_MANIFEST_DIR + PROFILE
                std::env::var("CARGO_MANIFEST_DIR")
                    .and_then(|manifest_dir| {
                        std::env::var("PROFILE")
                            .map(|profile| format!("{}/target/{}/harmonic/usage", manifest_dir, profile))
                    })
            })
            .unwrap_or_else(|_| "usage_data".to_string());
        
        std::fs::create_dir_all(&output_dir).unwrap();
        
        let mut total_usages = 0;
        let mut generated_files = Vec::new();
        
        for (module, usages) in &self.module_data {
            let module_data = ModuleData {
                crate_name: crate_name.to_string(),
                module: module.clone(),
                usages: usages.clone(),
            };
            
            // Clean filename by removing invalid characters
            let module_clean = module
                .replace("::", "_")
                .replace("<", "_")
                .replace(">", "_")
                .replace(" ", "_")
                .replace("/", "_")
                .replace("\\", "_")
                .replace("*", "_")
                .replace("?", "_")
                .replace("\"", "_")
                .replace("|", "_")
                .replace("'", "_")
                .replace("#", "_")
                .replace("{", "_")
                .replace("}", "_")
                .replace("(", "_")
                .replace(")", "_")
                .replace("[", "_")
                .replace("]", "_")
                .replace("&", "_")
                .replace("$", "_")
                .replace("@", "_")
                .replace("!", "_")
                .replace("%", "_")
                .replace("^", "_")
                .replace("+", "_")
                .replace("=", "_")
                .replace("~", "_")
                .replace("`", "_")
                .replace(";", "_")
                .replace(",", "_")
                .replace(".", "_");
            
            let filename = if module_clean.len() > 100 {
                let hash = std::collections::hash_map::DefaultHasher::new();
                use std::hash::{Hash, Hasher};
                let mut hasher = hash;
                module.hash(&mut hasher);
                format!("{}/{}_{:x}.json", output_dir, crate_name, hasher.finish())
            } else {
                format!("{}/{}_{}.json", output_dir, crate_name, module_clean)
            };
            
            let json = serde_json::to_string_pretty(&module_data).unwrap();
            std::fs::write(&filename, json).unwrap();
            generated_files.push(filename.clone());
            total_usages += usages.len();
        }
        
        // Save complexity data
        if !self.item_complexity.is_empty() {
            let complexity_filename = format!("{}/{}_complexity.json", output_dir, crate_name);
            let complexity_data = serde_json::json!({
                "crate": crate_name,
                "items": self.item_complexity.values().collect::<Vec<_>>()
            });
            let complexity_json = serde_json::to_string_pretty(&complexity_data).unwrap();
            std::fs::write(&complexity_filename, complexity_json).unwrap();
            generated_files.push(complexity_filename);
            eprintln!("=== SAVED {} COMPLEXITY ITEMS FOR CRATE: {} ===", self.item_complexity.len(), crate_name);
        }
        
        // Generate manifest for this crate
        self.generate_manifest(crate_name, &output_dir, &generated_files, total_usages);
        
        eprintln!("=== COLLECTING USAGE DATA FOR CRATE: {} === ({} total usages)", crate_name, total_usages);
    }
    
    fn generate_manifest(&self, crate_name: &str, output_dir: &str, generated_files: &[String], total_usages: usize) {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        // Find other manifest files (dependencies)
        let mut dependency_manifests = Vec::new();
        if let Ok(entries) = std::fs::read_dir(output_dir) {
            for entry in entries.flatten() {
                if let Some(filename) = entry.file_name().to_str() {
                    if filename.ends_with("_manifest.json") && !filename.starts_with(&format!("{}_", crate_name)) {
                        dependency_manifests.push(entry.path().to_string_lossy().to_string());
                    }
                }
            }
        }
        
        let manifest = serde_json::json!({
            "crate_name": crate_name,
            "timestamp": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            "collector_version": "1.0.0",
            "rustc_version": std::env::var("RUSTC_VERSION").unwrap_or_else(|_| "unknown".to_string()),
            "output_directory": output_dir,
            "total_usages": total_usages,
            "total_modules": self.module_data.len(),
            "total_enums": self.enum_data.len(),
            "generated_files": generated_files.iter().map(|f| {
                serde_json::json!({
                    "path": f,
                    "size_bytes": std::fs::metadata(f).map(|m| m.len()).unwrap_or(0),
                    "type": if f.contains("_enums_classified") { "enum_classification" } else { "usage_data" }
                })
            }).collect::<Vec<_>>(),
            "dependency_manifests": dependency_manifests,
            "environment": {
                "pwd": std::env::current_dir().ok().map(|p| p.to_string_lossy().to_string()),
                "cargo_manifest_dir": std::env::var("CARGO_MANIFEST_DIR").ok(),
                "cargo_target_dir": std::env::var("CARGO_TARGET_DIR").ok(),
                "profile": std::env::var("PROFILE").ok()
            }
        });
        
        let manifest_path = format!("{}/{}_manifest.json", output_dir, crate_name);
        std::fs::write(&manifest_path, serde_json::to_string_pretty(&manifest).unwrap()).unwrap();
        eprintln!("=== MANIFEST SAVED: {} ===", manifest_path);
    }
}

impl Callbacks for UsageCollector {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &interface::Compiler,
        tcx: TyCtxt<'tcx>,
    ) -> Compilation {
        let local_crate = tcx.crate_name(LOCAL_CRATE);
        let all_items = tcx.hir_crate_items(());
        
        // Optional HIR dump for debugging
        if std::env::var("DUMP_HIR").is_ok() {
            eprintln!("🔍 Creating HIR dump...");
            let hir_dump_path = "hir_dump.json";
            if let Ok(mut hir_dump) = std::fs::File::create(hir_dump_path) {
                let crate_items = tcx.hir_crate_items(());
                let _ = writeln!(hir_dump, "{{\"crate_name\": \"{}\", \"items\": [", local_crate);
                
                let items: Vec<_> = crate_items.free_items().collect();
                for (i, item_id) in items.iter().enumerate() {
                    let item = tcx.hir_expect_item(item_id.owner_id.def_id);
                    let comma = if i == items.len() - 1 { "" } else { "," };
                    let _ = writeln!(hir_dump, "  {{\"id\": {}, \"kind\": \"{:?}\"}}{}", 
                        item_id.owner_id.def_id.local_def_index.index(), 
                        std::mem::discriminant(&item.kind), 
                        comma);
                }
                let _ = writeln!(hir_dump, "]}}");
                eprintln!("🔍 HIR dump saved to {}", hir_dump_path);
            }
        }
        
        // Continue with original usage collection logic
        for item_id in all_items.free_items() {
            let def_id = item_id.owner_id.to_def_id();
            let item_path = tcx.def_path_str(def_id);
            let def_kind = tcx.def_kind(def_id);
                
                let (module, decl) = if item_path.contains("::") {
                    let parts: Vec<&str> = item_path.split("::").collect();
                    (parts[..parts.len()-1].join("::"), parts[parts.len()-1])
                } else {
                    ("root".to_string(), item_path.as_str())
                };
                
                // Only process items that have bodies and can be type-checked
                if !matches!(def_kind, 
                    rustc_hir::def::DefKind::Fn | 
                    rustc_hir::def::DefKind::Const | 
                    rustc_hir::def::DefKind::Static { .. } |
                    rustc_hir::def::DefKind::AssocFn |
                    rustc_hir::def::DefKind::AssocConst
                ) {
                    continue;
                }
                
                let user_location = format!("crate::{}::{}::{}", local_crate, module, decl);
                let typeck = tcx.typeck(item_id.owner_id.def_id);
                
                for (_local_id, result) in typeck.type_dependent_defs().items_in_stable_order() {
                    if let Ok((def_kind, used_def_id)) = result {
                        let used_crate = tcx.crate_name(used_def_id.krate);
                        let used_path = tcx.def_path_str(*used_def_id);
                        let used_location = format!("crate::{}::{}", used_crate, used_path);
                        let usage = format!("{} USES {} ({:?}) [DefId: {:?}]", user_location, used_location, def_kind, used_def_id);
                        
                        // Get the actual HIR node type where this usage occurs
                        let node_type = match tcx.hir_node_by_def_id(item_id.owner_id.def_id) {
                            rustc_hir::Node::Item(_) => "Item",
                            rustc_hir::Node::Expr(_) => "Expr", 
                            rustc_hir::Node::Stmt(_) => "Stmt",
                            rustc_hir::Node::TraitItem(_) => "TraitItem",
                            rustc_hir::Node::ImplItem(_) => "ImplItem",
                            rustc_hir::Node::Pat(_) => "Pat",
                            rustc_hir::Node::Ty(_) => "Ty",
                            _ => "Other",
                        }.to_string();
                        
                        let usage_type = match def_kind {
                            rustc_hir::def::DefKind::AssocFn => "MethodCall",
                            rustc_hir::def::DefKind::AssocConst => "AssocConstAccess",
                            rustc_hir::def::DefKind::Variant => "EnumVariant", 
                            rustc_hir::def::DefKind::Struct => "StructUsage",
                            rustc_hir::def::DefKind::Enum => "EnumUsage",
                            rustc_hir::def::DefKind::Fn => "FunctionCall",
                            rustc_hir::def::DefKind::Const => "ConstantAccess",
                            rustc_hir::def::DefKind::Static { .. } => "StaticAccess",
                            rustc_hir::def::DefKind::Macro(_) => "MacroCall",
                            rustc_hir::def::DefKind::TyAlias => "TypeAlias",
                            rustc_hir::def::DefKind::Union => "UnionUsage",
                            rustc_hir::def::DefKind::Trait => "TraitUsage",
                            _ => "Other",
                        }.to_string();
                        
                        // Extract symbol name from path
                        let symbol = used_path.split("::").last().unwrap_or(&used_path).to_string();
                        let kind = match def_kind {
                            rustc_hir::def::DefKind::Variant => "enum_variant_usage",
                            rustc_hir::def::DefKind::Struct => "struct_usage", 
                            rustc_hir::def::DefKind::Enum => "enum_usage",
                            rustc_hir::def::DefKind::Macro(_) => "macro_usage",
                            rustc_hir::def::DefKind::AssocFn => "method_call",
                            rustc_hir::def::DefKind::Fn => "function_call",
                            rustc_hir::def::DefKind::Const => "const_usage",
                            rustc_hir::def::DefKind::Static { .. } => "static_usage",
                            rustc_hir::def::DefKind::TyAlias => "type_alias_usage",
                            rustc_hir::def::DefKind::Union => "union_usage",
                            rustc_hir::def::DefKind::Trait => "trait_usage",
                            rustc_hir::def::DefKind::AssocTy => "assoc_type_usage",
                            rustc_hir::def::DefKind::AssocConst => "assoc_const_usage",
                            rustc_hir::def::DefKind::Mod => "module_usage",
                            rustc_hir::def::DefKind::Field => "field_usage",
                            rustc_hir::def::DefKind::Ctor(_, _) => "constructor_usage",
                            // Only use other_usage for truly unknown cases
                            _ => {
                                eprintln!("Unknown DefKind: {:?} for symbol: {}", def_kind, used_path);
                                "unknown_usage"
                            },
                        }.to_string();
                        
                        let user_def_id_str = format!("{:?}", item_id.owner_id.to_def_id());
                        let used_def_id_str = format!("{:?}", used_def_id);
                        let used_def_kind_str = format!("{:?}", def_kind);
                        let user_def_kind_str = format!("{:?}", tcx.def_kind(item_id.owner_id.to_def_id()));
                        
                        self.add_usage(&module, symbol, kind, usage_type, node_type, user_def_id_str, used_def_id_str, Some(local_crate.to_string()), Some(used_crate.to_string()), Some(used_def_kind_str), Some(user_def_kind_str));
                    }
                }
            }
            
            // NEW: Collect constants, structs, enums, and usage patterns
            self.collect_constants(tcx);
            // Skip type usage collection for now due to API changes
            
            self.save_to_files(&local_crate.to_string());
        
        Compilation::Continue
    }
}

impl UsageCollector {
    fn collect_constants<'tcx>(&mut self, tcx: TyCtxt<'tcx>) {
        let crate_name = tcx.crate_name(LOCAL_CRATE).to_string();
        
        for item_id in tcx.hir_crate_items(()).free_items() {
            let item = tcx.hir_expect_item(item_id.owner_id.def_id);
            
            // Dump the entire item for debugging
            if std::env::var("DUMP_HIR").is_ok() {
                eprintln!("=== ITEM DUMP ===");
                eprintln!("Item ID: {:?}", item_id);
                eprintln!("Item Kind: {:?}", std::mem::discriminant(&item.kind));
                eprintln!("Item: {:#?}", item);
                eprintln!("=== END ITEM DUMP ===");
            }
            
            // Skip items that don't have proper names (like use statements)
            let item_name = match &item.kind {
                rustc_hir::ItemKind::Use(..) => continue,
                rustc_hir::ItemKind::ExternCrate(..) => continue,
                rustc_hir::ItemKind::Impl(..) => continue,
                rustc_hir::ItemKind::ForeignMod { .. } => continue,
                _ => tcx.item_name(item.owner_id.to_def_id()).to_string(),
            };
            
            match &item.kind {
                // Const items
                rustc_hir::ItemKind::Const(_, _, _, body_id) => {
                    let const_defkind = self.get_defkind_from_item(tcx, item);
                    self.add_usage(
                        "constants",
                        item_name.clone(),
                        "const_decl".to_string(),
                        "ConstDecl".to_string(),
                        "Item".to_string(),
                        crate_name.clone(),
                        item_name.clone(),
                        Some(crate_name.clone()),
                        None,
                        Some(const_defkind),
                        Some("Const".to_string())
                    );
                    
                    // Collect literals from const body
                    let body = tcx.hir_body(*body_id);
                    self.collect_literals_from_expr(&body.value, &format!("const {}", item_name));
                }
                
                // Static items  
                rustc_hir::ItemKind::Static(_, _, _, body_id) => {
                    self.add_usage(
                        "constants",
                        item_name.clone(),
                        "static_decl".to_string(),
                        "StaticDecl".to_string(),
                        "Item".to_string(),
                        crate_name.clone(),
                        item_name.clone(),
                        Some(crate_name.clone()),
                        None,
                        Some("Static".to_string()),
                        Some("Static".to_string())
                    );
                    
                    // Collect literals from static body
                    let body = tcx.hir_body(*body_id);
                    self.collect_literals_from_expr(&body.value, &format!("static {}", item_name));
                }
                
                // Function items - collect literals from function bodies
                // rustc_hir::ItemKind::Fn(_, _, body_id) => {
                //     let body = tcx.hir_body(*body_id);
                //     self.collect_literals_from_expr(&body.value, &format!("fn {}", item_name));
                // }
                
                // Struct definitions
                rustc_hir::ItemKind::Struct(variant_data, generics, _) => {
                    self.add_usage(
                        "structs",
                        item_name.clone(),
                        "struct_decl".to_string(),
                        "StructDecl".to_string(),
                        "Item".to_string(),
                        crate_name.clone(),
                        item_name.clone(),
                        Some(crate_name.clone()),
                        None,
                        Some("Struct".to_string()),
                        Some("Struct".to_string())
                    );
                    
                    // Track generic parameters (HIGH IMPACT - 16 occurrences found)
                    // self.track_generics(generics, &item_name, &crate_name, "struct"); // TODO: Implement track_generics
                    
                    // Collect struct complexity
                    // let fields = self.extract_struct_fields(variant_data);
                    let fields = Vec::new(); // TODO: Fix when we understand the new ItemKind::Struct structure
                    let complexity = match fields.len() {
                        0..=3 => "simple",
                        4..=8 => "medium", 
                        _ => "complex"
                    };
                    
                    self.item_complexity.insert(item_name.clone(), ItemComplexity {
                        name: item_name.clone(),
                        item_type: "struct".to_string(),
                        complexity: complexity.to_string(),
                        field_count: Some(fields.len()),
                        variant_count: None,
                        param_count: None,
                        fields: Some(fields),
                        variants: None,
                        parameters: None,
                    });
                }
                
                // Enum definitions
                rustc_hir::ItemKind::Enum(enum_def, generics, _) => {
                    self.add_usage(
                        "enums",
                        item_name.clone(),
                        "enum_decl".to_string(),
                        "EnumDecl".to_string(),
                        "Item".to_string(),
                        crate_name.clone(),
                        item_name.clone(),
                        Some(crate_name.clone()),
                        None
                    );
                    
                    // Track generic parameters (HIGH IMPACT - 16 occurrences found)
                    // self.track_generics(generics, &item_name, &crate_name, "enum"); // TODO: Implement track_generics
                    
                    // Collect enum complexity
                    // let variants = self.extract_enum_variants(enum_def);
                    let variants = Vec::new(); // TODO: Fix when we understand the new ItemKind::Enum structure
                    let complexity = match variants.len() {
                        0..=3 => "simple",
                        4..=8 => "medium",
                        _ => "complex"
                    };
                    
                    self.item_complexity.insert(item_name.clone(), ItemComplexity {
                        name: item_name.clone(),
                        item_type: "enum".to_string(),
                        complexity: complexity.to_string(),
                        field_count: None,
                        variant_count: Some(variants.len()),
                        param_count: None,
                        fields: None,
                        variants: Some(variants),
                        parameters: None,
                    });
                }
                
                // Function definitions
                rustc_hir::ItemKind::Fn { sig, .. } => {
                    let params = self.extract_function_params(sig);
                    let complexity = match params.len() {
                        0..=2 => "simple",
                        3..=5 => "medium",
                        _ => "complex"
                    };
                    
                    self.item_complexity.insert(item_name.clone(), ItemComplexity {
                        name: item_name.clone(),
                        item_type: "function".to_string(),
                        complexity: complexity.to_string(),
                        field_count: None,
                        variant_count: None,
                        param_count: Some(params.len()),
                        fields: None,
                        variants: None,
                        parameters: Some(params),
                    });
                }
                
                _ => {}
            }
        }
    }
    
    fn collect_literals_from_expr<'tcx>(&mut self, expr: &'tcx rustc_hir::Expr<'tcx>, context: &str) {
        use rustc_hir::intravisit::{self, Visitor};
        
        struct LiteralVisitor<'a> {
            collector: &'a mut UsageCollector,
            context: String,
        }
        
        impl<'tcx> intravisit::Visitor<'tcx> for LiteralVisitor<'_> {
            fn visit_expr(&mut self, expr: &'tcx rustc_hir::Expr<'tcx>) {
                match &expr.kind {
                    rustc_hir::ExprKind::Lit(lit) => {
                        let (literal_value, type_prefix) = match &lit.node {
                            rustc_ast::LitKind::Bool(b) => (format!("{}", b), "b"),
                            rustc_ast::LitKind::Char(c) => (format!("'{}'", c), "c"),
                            rustc_ast::LitKind::Int(i, ty) => {
                                let prefix = match ty {
                                    rustc_ast::LitIntType::Signed(rustc_ast::IntTy::I8) => "i8_",
                                    rustc_ast::LitIntType::Signed(rustc_ast::IntTy::I16) => "i16_",
                                    rustc_ast::LitIntType::Signed(rustc_ast::IntTy::I32) => "i32_",
                                    rustc_ast::LitIntType::Signed(rustc_ast::IntTy::I64) => "i64_",
                                    rustc_ast::LitIntType::Signed(rustc_ast::IntTy::I128) => "i128_",
                                    rustc_ast::LitIntType::Signed(rustc_ast::IntTy::Isize) => "isize_",
                                    rustc_ast::LitIntType::Unsigned(rustc_ast::UintTy::U8) => "u8_",
                                    rustc_ast::LitIntType::Unsigned(rustc_ast::UintTy::U16) => "u16_",
                                    rustc_ast::LitIntType::Unsigned(rustc_ast::UintTy::U32) => "u32_",
                                    rustc_ast::LitIntType::Unsigned(rustc_ast::UintTy::U64) => "u64_",
                                    rustc_ast::LitIntType::Unsigned(rustc_ast::UintTy::U128) => "u128_",
                                    rustc_ast::LitIntType::Unsigned(rustc_ast::UintTy::Usize) => "usize_",
                                    rustc_ast::LitIntType::Unsuffixed => "int_",
                                };
                                (format!("{}", i), prefix)
                            },
                            rustc_ast::LitKind::Float(f, ty) => {
                                let prefix = match ty {
                                    rustc_ast::LitFloatType::Suffixed(rustc_ast::FloatTy::F32) => "f32_",
                                    rustc_ast::LitFloatType::Suffixed(rustc_ast::FloatTy::F64) => "f64_",
                                    rustc_ast::LitFloatType::Suffixed(rustc_ast::FloatTy::F16) => "f16_",
                                    rustc_ast::LitFloatType::Suffixed(rustc_ast::FloatTy::F128) => "f128_",
                                    rustc_ast::LitFloatType::Unsuffixed => "float_",
                                };
                                (format!("{}", f), prefix)
                            },
                            rustc_ast::LitKind::Str(s, _) => (format!("\"{}\"", s), "str_"),
                            rustc_ast::LitKind::Byte(b) => (format!("{}", b), "byte_"),
                            rustc_ast::LitKind::ByteStr(bytes, _) => (format!("b\"{}\"", String::from_utf8_lossy(bytes.as_byte_str())), "bstr_"),
                            _ => return,
                        };
                        
                        self.collector.add_usage(
                            "literals", 
                            format!("{}{}", type_prefix, literal_value),
                            "literal".to_string(),
                            "NumericLiteral".to_string(), 
                            "Expression".to_string(), 
                            self.context.clone(),
                            literal_value,
                            None,
                            None
                        );
                    }
                    
                    // Synthesized implementations from usage pattern analysis
                    rustc_hir::ExprKind::Field(expr, field) => {
                        let field_name = field.name.to_string();
                        self.collector.add_usage(
                            "field_access",
                            format!("field_{}", field_name),
                            "field_access".to_string(),
                            "FieldAccess".to_string(),
                            "Expression".to_string(),
                            self.context.clone(),
                            field_name,
                            None,
                            None
                        );
                        self.visit_expr(expr);
                    }
                    
                    rustc_hir::ExprKind::Call(func, args) => {
                        let arg_count = args.len();
                        self.collector.add_usage(
                            "function_calls",
                            format!("call_{}_args", arg_count),
                            "function_call".to_string(),
                            "FunctionCall".to_string(),
                            "Expression".to_string(),
                            self.context.clone(),
                            format!("{}_args", arg_count),
                            None,
                            None
                        );
                        self.visit_expr(func);
                        for arg in args.iter() { self.visit_expr(arg); }
                    }
                    
                    rustc_hir::ExprKind::AddrOf(_, mutability, expr) => {
                        let ref_type = match mutability {
                            rustc_hir::Mutability::Mut => "mut_ref",
                            rustc_hir::Mutability::Not => "ref",
                        };
                        self.collector.add_usage(
                            "memory_safety",
                            format!("addr_{}", ref_type),
                            "memory_ref".to_string(),
                            "MemoryRef".to_string(),
                            "Expression".to_string(),
                            self.context.clone(),
                            ref_type.to_string(),
                            None,
                            None
                        );
                        self.visit_expr(expr);
                    }
                    
                    rustc_hir::ExprKind::Index(base, index, _) => {
                        self.collector.add_usage(
                            "indexing",
                            "index_access".to_string(),
                            "index_access".to_string(),
                            "IndexAccess".to_string(),
                            "Expression".to_string(),
                            self.context.clone(),
                            "array_index".to_string(),
                            None,
                            None
                        );
                        self.visit_expr(base);
                        self.visit_expr(index);
                    }
                    
                    rustc_hir::ExprKind::Match(expr, arms, _) => {
                        let arm_count = arms.len();
                        self.collector.add_usage(
                            "patterns",
                            format!("match_{}_arms", arm_count),
                            "pattern_match".to_string(),
                            "PatternMatch".to_string(),
                            "Expression".to_string(),
                            self.context.clone(),
                            format!("{}_arms", arm_count),
                            None,
                            None
                        );
                        self.visit_expr(expr);
                        for arm in arms.iter() {
                            if let Some(guard) = &arm.guard { 
                                // self.visit_expr(&guard); // TODO: Fix guard access
                            }
                            self.visit_expr(&arm.body);
                        }
                    }
                    
                    _ => {}
                }
                
                intravisit::walk_expr(self, expr);
            }
        }
        
        let mut visitor = LiteralVisitor { 
            collector: self, 
            context: context.to_string()
        };
        
        visitor.visit_expr(expr);
    }
    
    fn collect_type_usage<'tcx>(&mut self, tcx: TyCtxt<'tcx>) {
        let crate_name = tcx.crate_name(LOCAL_CRATE).to_string();
        
        // Collect type usage from function signatures, struct fields, etc.
        use rustc_hir::intravisit::{self, Visitor};
        
        // Create visitor outside the loop to avoid borrow conflicts
        struct TypeUsageVisitor<'a, 'tcx> {
            collector: &'a mut UsageCollector,
            crate_name: String,
            tcx: TyCtxt<'tcx>,
        }
        
        impl<'tcx> intravisit::Visitor<'tcx> for TypeUsageVisitor<'_, 'tcx> {
            fn visit_ty(&mut self, ty: &'tcx rustc_hir::Ty<'tcx, rustc_hir::AmbigArg>) {
                match &ty.kind {
                    rustc_hir::TyKind::Path(rustc_hir::QPath::Resolved(_, path)) => {
                        if let Some(def_id) = path.res.opt_def_id() {
                            let def_kind = self.tcx.def_kind(def_id);
                            match def_kind {
                                rustc_hir::def::DefKind::Struct | 
                                rustc_hir::def::DefKind::Enum |
                                rustc_hir::def::DefKind::Union => {
                                    let symbol = self.tcx.item_name(def_id).to_string();
                                    // Record type usage
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
                intravisit::walk_ty(self, ty);
            }
        }

        for item_id in tcx.hir_crate_items(()).free_items() {
            let item = tcx.hir_expect_item(item_id.owner_id.def_id);
            
            // Create visitor outside the loop to avoid borrow conflicts
            struct TypeUsageVisitor<'a, 'tcx> {
                collector: &'a mut UsageCollector,
                crate_name: String,
                tcx: TyCtxt<'tcx>,
            }
            
            impl<'tcx> intravisit::Visitor<'tcx> for TypeUsageVisitor<'_, 'tcx> {
                fn visit_ty(&mut self, ty: &'tcx rustc_hir::Ty<'tcx, rustc_hir::AmbigArg>) {
                    match &ty.kind {
                        rustc_hir::TyKind::Path(rustc_hir::QPath::Resolved(_, path)) => {
                            if let Some(def_id) = path.res.opt_def_id() {
                                let def_kind = self.tcx.def_kind(def_id);
                                match def_kind {
                                    rustc_hir::def::DefKind::Struct | 
                                    rustc_hir::def::DefKind::Enum |
                                    rustc_hir::def::DefKind::Union => {
                                        let symbol = self.tcx.item_name(def_id).to_string();
                                        // Record type usage
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                    intravisit::walk_ty(self, ty);
                }
            }
            
            let mut visitor = TypeUsageVisitor { 
                collector: self, 
                crate_name: crate_name.clone(),
                tcx
            };
            
            intravisit::walk_item(&mut visitor, item);
        }
    }
    
    fn extract_struct_fields(&self, variant_data: &rustc_hir::VariantData) -> Vec<FieldInfo> {
        variant_data.fields().iter().map(|field| {
            FieldInfo {
                name: field.ident.name.to_string(),
                field_type: "unknown".to_string(),
                is_public: field.vis_span.is_dummy(),
            }
        }).collect()
    }
    
    fn extract_enum_variants(&self, enum_def: &rustc_hir::EnumDef) -> Vec<VariantInfo> {
        enum_def.variants.iter().map(|variant| {
            let field_count = variant.data.fields().len();
            VariantInfo {
                name: variant.ident.name.to_string(),
                has_fields: field_count > 0,
                field_count,
            }
        }).collect()
    }
    
    fn extract_function_params(&self, sig: &rustc_hir::FnSig) -> Vec<ParamInfo> {
        sig.decl.inputs.iter().enumerate().map(|(i, _param)| {
            ParamInfo {
                name: format!("param_{}", i),
                param_type: "unknown".to_string(),
            }
        }).collect()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.iter().any(|arg| arg.starts_with("--print") || arg == "--version" || arg == "-V") {
        let mut cmd = std::process::Command::new("rustc");
        cmd.args(&args[1..]);
        std::process::exit(cmd.status().unwrap().code().unwrap_or(1));
    }
    
    let mut callbacks = UsageCollector::new();
    let result = rustc_driver::catch_fatal_errors(|| {
        rustc_driver::run_compiler(&args, &mut callbacks)
    });
    
    std::process::exit(match result { Ok(_) => 0, Err(_) => 1 });
}
