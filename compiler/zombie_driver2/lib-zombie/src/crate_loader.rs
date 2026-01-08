use libloading::{Library, Symbol};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct CrateLoadOrder {
    crates: Vec<String>,
    dependencies: HashMap<String, Vec<String>>,
}

impl CrateLoadOrder {
    fn from_build_log() -> Self {
        let crates = vec![
            "rustc_middle".to_string(),
            "rustc_expand".to_string(),
            "rustc_builtin_macros".to_string(),
            "rustc_transmute".to_string(),
            "rustc_infer".to_string(),
            "rustc_mir_dataflow".to_string(),
            "rustc_incremental".to_string(),
            "rustc_pattern_analysis".to_string(),
            "rustc_symbol_mangling".to_string(),
            "rustc_ast_lowering".to_string(),
            "rustc_query_impl".to_string(),
            "rustc_public_bridge".to_string(),
            "rustc_monomorphize".to_string(),
            "rustc_metadata".to_string(),
            "rustc_trait_selection".to_string(),
            "rustc_resolve".to_string(),
            "rustc_lint".to_string(),
            "rustc_ty_utils".to_string(),
            "rustc_const_eval".to_string(),
            "rustc_traits".to_string(),
            "rustc_hir_analysis".to_string(),
            "rustc_codegen_ssa".to_string(),
            "rustc_borrowck".to_string(),
            "rustc_privacy".to_string(),
            "rustc_passes".to_string(),
            "rustc_mir_build".to_string(),
            "rustc_hir_typeck".to_string(),
            "rustc_mir_transform".to_string(),
            "rustc_interface".to_string(),
            "rustc_driver_impl".to_string(),
            "rustc_driver".to_string(),
        ];
        
        Self {
            crates,
            dependencies: HashMap::new(),
        }
    }
}

struct CrateWrapper {
    name: String,
    library: Library,
    entry_points: Vec<String>, // Just track entry point names
    types: HashMap<String, String>,    // type name -> wrapper type
}

impl CrateWrapper {
    unsafe fn load(crate_name: &str, so_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        println!("🔧 Wrapping crate: {}", crate_name);
        
        let library = Library::new(so_path)?;
        let mut entry_points = Vec::new();
        let mut types = HashMap::new();
        
        // Wrap common rustc functions with goo
        match crate_name {
            "rustc_driver" => {
                // Check for entry points
                if library.get::<Symbol<unsafe extern "C" fn()>>(b"run_compiler").is_ok() {
                    entry_points.push("run_compiler".to_string());
                }
                types.insert("RunCompiler".to_string(), "ZombieRunCompiler".to_string());
                types.insert("Compilation".to_string(), "ZombieCompilation".to_string());
            }
            "rustc_interface" => {
                types.insert("Queries".to_string(), "ZombieQueries".to_string());
                types.insert("Compiler".to_string(), "ZombieCompiler".to_string());
            }
            "rustc_middle" => {
                types.insert("TyCtxt".to_string(), "ZombieTyCtxt".to_string());
                types.insert("Ty".to_string(), "ZombieTy".to_string());
            }
            _ => {
                println!("   📦 Generic wrapper for {}", crate_name);
            }
        }
        
        Ok(Self {
            name: crate_name.to_string(),
            library,
            entry_points,
            types,
        })
    }
}

pub struct ZombieSOSystem {
    load_order: CrateLoadOrder,
    loaded_crates: HashMap<String, CrateWrapper>,
}

impl ZombieSOSystem {
    pub fn new() -> Self {
        Self {
            load_order: CrateLoadOrder::from_build_log(),
            loaded_crates: HashMap::new(),
        }
    }
    
    pub fn load_all_crates(&mut self, deps_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("🧟 Loading crates in dependency order...");
        
        for crate_name in &self.load_order.crates {
            if let Ok(so_path) = self.find_crate_so(deps_dir, crate_name) {
                unsafe {
                    let wrapper = CrateWrapper::load(crate_name, &so_path)?;
                    self.loaded_crates.insert(crate_name.clone(), wrapper);
                    println!("✅ Loaded: {}", crate_name);
                }
            } else {
                println!("⚠️  Skipping missing crate: {}", crate_name);
            }
        }
        
        Ok(())
    }
    
    fn find_crate_so(&self, deps_dir: &str, crate_name: &str) -> Result<String, Box<dyn std::error::Error>> {
        let dir = std::fs::read_dir(deps_dir)?;
        let prefix = format!("lib{}", crate_name.replace("_", "_"));
        
        for entry in dir {
            let entry = entry?;
            let path = entry.path();
            
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                if filename.starts_with(&prefix) && filename.ends_with(".so") {
                    return Ok(path.to_string_lossy().to_string());
                }
            }
        }
        
        Err(format!("No .so found for crate: {}", crate_name).into())
    }
    
    pub fn has_entry_point(&self, crate_name: &str, func_name: &str) -> bool {
        if let Some(wrapper) = self.loaded_crates.get(crate_name) {
            wrapper.entry_points.contains(&func_name.to_string())
        } else {
            false
        }
    }
    
    pub fn get_wrapped_type(&self, crate_name: &str, type_name: &str) -> Option<&String> {
        self.loaded_crates.get(crate_name)?.types.get(type_name)
    }
}
