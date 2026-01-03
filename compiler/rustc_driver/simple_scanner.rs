#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_ast;

use rustc_driver::Callbacks;
use rustc_interface::interface;
use rustc_middle::ty::TyCtxt;
use rustc_ast::Crate;
use std::collections::HashMap;

include!("witness_macros.rs");

struct SimpleConstantCallbacks {
    item_counts: HashMap<String, u32>,
}

impl SimpleConstantCallbacks {
    fn new() -> Self {
        Self { item_counts: HashMap::new() }
    }
    
    fn count_item(&mut self, item_type: &str) {
        *self.item_counts.entry(item_type.to_string()).or_insert(0) += 1;
    }
}

impl Callbacks for SimpleConstantCallbacks {
    fn after_crate_root_parsing(&mut self, _: &interface::Compiler, krate: &mut Crate) -> rustc_driver::Compilation {
        let crate_name = std::env::var("CARGO_PKG_NAME").unwrap_or_else(|_| "unknown".to_string());
        
        // Create scan_results directory in project root
        let project_root = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/rustc_driver";
        let scan_dir = format!("{}/scan_results", project_root);
        std::fs::create_dir_all(&scan_dir).ok();
        println!("SCAN_DIR: {}", scan_dir);
        
        let output_file = format!("{}/scan_results/{}.json", project_root, crate_name);
        
        let mut file = match std::fs::File::create(&output_file) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("ERROR: Failed to create {}: {}", output_file, e);
                return rustc_driver::Compilation::Continue;
            }
        };
        
        use std::io::Write;
        writeln!(file, "{{\"crate\":\"{}\",\"items\":[", crate_name).ok();
        
        for (i, item) in krate.items.iter().enumerate() {
            let item_type = format!("{:?}", item.kind);
            let clean_type = item_type.split('(').next().unwrap_or(&item_type);
            
            if i > 0 { writeln!(file, ",").ok(); }
            
            // Extract function names and details
            let mut item_detail = format!("\"type\":\"{}\",\"span\":\"{:?}\"", clean_type, item.span);
            if let rustc_ast::ItemKind::Fn(_) = item.kind {
                let debug_str = format!("{:?}", item.kind);
                let fn_name = debug_str.split('(').nth(1).unwrap_or("unknown").split(',').next().unwrap_or("unknown");
                item_detail = format!("\"type\":\"Fn\",\"name\":\"{}\",\"span\":\"{:?}\"", 
                    fn_name, item.span);
            }
            
            writeln!(file, "  {{{}}}", item_detail).ok();
            self.count_item(&item_type);
        }
        
        writeln!(file, "]}}").ok();
        println!("SAVED: {} items to {}", krate.items.len(), output_file);
        
        rustc_driver::Compilation::Continue
    }

    fn after_analysis<'tcx>(&mut self, _: &interface::Compiler, _tcx: TyCtxt<'tcx>) -> rustc_driver::Compilation {
        let crate_name = std::env::var("CARGO_PKG_NAME").unwrap_or_else(|_| "unknown".to_string());
        
        // Create scan_results directory in project root
        let project_root = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/rustc_driver";
        let scan_dir = format!("{}/scan_results", project_root);
        std::fs::create_dir_all(&scan_dir).ok();
        println!("SUMMARY_DIR: {}", scan_dir);
        
        // Save summary
        let summary_file = format!("{}/scan_results/{}_summary.json", project_root, crate_name);
        if let Ok(mut file) = std::fs::File::create(&summary_file) {
            use std::io::Write;
            writeln!(file, "{{\"crate\":\"{}\",\"summary\":{{", crate_name).ok();
            
            let mut first = true;
            for (item_type, count) in &self.item_counts {
                if !first { writeln!(file, ",").ok(); }
                writeln!(file, "  \"{}\":{}", item_type.replace('"', "\\\""), count).ok();
                first = false;
            }
            
            writeln!(file, "}}}}").ok();
            println!("SUMMARY: {} item types saved to {}", self.item_counts.len(), summary_file);
        }
        
        rustc_driver::Compilation::Continue
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    // Handle cargo's --print and --version queries by delegating to real rustc
    if args.iter().any(|arg| arg.starts_with("--print") || arg == "--version" || arg == "-V") {
        let mut cmd = std::process::Command::new("rustc");
        cmd.args(&args[1..]);
        let status = cmd.status().expect("Failed to run rustc");
        std::process::exit(status.code().unwrap_or(1));
    }
    
    // Handle cargo's --print and --version queries by delegating to real rustc
    if args.iter().any(|arg| arg.starts_with("--print") || arg == "--version" || arg == "-V") {
        let mut cmd = std::process::Command::new("rustc");
        cmd.args(&args[1..]);
        let status = cmd.status().expect("Failed to run rustc");
        std::process::exit(status.code().unwrap_or(1));
    }
    
    // Set required environment variables for rustc compilation
    std::env::set_var("CFG_RELEASE_CHANNEL", "dev");
    std::env::set_var("CFG_RELEASE", "1.91.1");
    std::env::set_var("CFG_VERSION", "1.91.1");
    
    let mut callbacks = SimpleConstantCallbacks::new();
    
    witness!(symbol: "simple_scanner_start", from: "main");
    
    let result = rustc_driver::catch_fatal_errors(|| {
        rustc_driver::run_compiler(&args, &mut callbacks)
    });
    
    witness!(symbol: "simple_scanner_end", from: "main");
    
    std::process::exit(match result { Ok(_) => 0, Err(_) => 1 });
}
