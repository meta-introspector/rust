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
use rustc_hir::{Expr, ExprKind, Lit};
use std::collections::HashMap;
use std::sync::OnceLock;
use serde::{Serialize, Deserialize};

static USAGE_CACHE: OnceLock<HashMap<String, Vec<UsageEntry>>> = OnceLock::new();

#[derive(Serialize, Deserialize, Clone)]
struct UsageEntry {
    usage: String,
    usage_count: usize,
    usage_type: String,
    node_type: String,
    user_def_id: String,
    used_def_id: String,
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
}

impl UsageCollector {
    fn new() -> Self {
        Self {
            module_data: HashMap::new(),
        }
    }
    
    fn get_cached_usages(&self) -> &HashMap<String, Vec<UsageEntry>> {
        USAGE_CACHE.get_or_init(|| {
            self.load_previous_data().unwrap_or_default()
        })
    }
    
    fn load_previous_data(&self) -> Result<HashMap<String, Vec<UsageEntry>>, Box<dyn std::error::Error>> {
        // Load from previous eigenmatrix runs
        if let Ok(content) = std::fs::read_to_string("usage_eigenmatrix.json") {
            if let Ok(data) = serde_json::from_str::<serde_json::Value>(&content) {
                return Ok(HashMap::new()); // Simplified for now
            }
        }
        Ok(HashMap::new())
    }
    
    fn add_usage(&mut self, module: &str, usage: String, usage_type: String, node_type: String, user_def_id: String, used_def_id: String) {
        let entry = UsageEntry {
            usage,
            usage_count: 1,
            usage_type,
            node_type,
            user_def_id,
            used_def_id,
        };
        self.module_data.entry(module.to_string()).or_insert_with(Vec::new).push(entry);
    }
    
    fn save_to_files(&self, crate_name: &str) {
        std::fs::create_dir_all("usage_data").unwrap();
        
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
                format!("usage_data/{}_{:x}.json", crate_name, hasher.finish())
            } else {
                format!("usage_data/{}_{}.json", crate_name, module_clean)
            };
            
            let json = serde_json::to_string_pretty(&module_data).unwrap();
            std::fs::write(&filename, json).unwrap();
            
            eprintln!("Saved {} usages to {}", usages.len(), filename);
        }
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
        
        eprintln!("=== COLLECTING USAGE DATA FOR CRATE: {} ===", local_crate);
        
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
                    
                    // Classify usage type based on def_kind
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
                    
                    let user_def_id_str = format!("{:?}", item_id.owner_id.to_def_id());
                    let used_def_id_str = format!("{:?}", used_def_id);
                    
                    self.add_usage(&module, usage, usage_type, node_type, user_def_id_str, used_def_id_str);
                }
            }
        }
        
        // NEW: Collect constants and literals
        eprintln!("🔍 About to call collect_constants");
        self.collect_constants(tcx);
        eprintln!("✅ collect_constants returned");
        
        self.save_to_files(&local_crate.to_string());
        eprintln!("✅ COLLECTION COMPLETE");
        
        Compilation::Continue
    }
}

impl UsageCollector {
    fn collect_constants<'tcx>(&mut self, tcx: TyCtxt<'tcx>) {
        eprintln!("🔍 collect_constants called!");
        
        struct ConstantVisitor<'a> {
            collector: &'a mut UsageCollector,
        }
        
        impl<'tcx> Visitor<'tcx> for ConstantVisitor<'_> {
            fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) {
                eprintln!("🔍 Visiting expr: {:?}", expr.kind);
                match &expr.kind {
                    ExprKind::Lit(lit) => {
                        let constant_value = format!("{:?}", lit.node);
                        eprintln!("🎯 FOUND LITERAL: {}", constant_value);
                        self.collector.add_usage("constants", constant_value.clone(), "Literal".to_string(), 
                                               "Constant".to_string(), 
                                               format!("{:?}", expr.hir_id),
                                               constant_value);
                    }
                    _ => {
                        eprintln!("   Other expr: {:?}", expr.kind);
                    }
                }
                
                intravisit::walk_expr(self, expr);
            }
        }
        
        let mut visitor = ConstantVisitor { collector: self };
        let all_items = tcx.hir_crate_items(());
        eprintln!("🔍 Starting to visit items");
        
        for item_id in all_items.free_items() {
            let node = tcx.hir_node_by_def_id(item_id.owner_id.def_id);
            eprintln!("🔍 Visiting item: {:?}", item_id);
            if let rustc_hir::Node::Item(item) = node {
                eprintln!("🔍 Item is: {:?}", item.kind);
                visitor.visit_item(item);
            }
        }
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
