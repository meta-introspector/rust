use introspector_collector::hir_enum_collector::HirEnumCollector;
use rustc_driver::{Callbacks, Compilation, RunCompiler};
use rustc_interface::{interface, Queries};
use std::process;

struct EnumMacroGenerator;

impl Callbacks for EnumMacroGenerator {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &interface::Compiler,
        queries: &'tcx Queries<'tcx>,
    ) -> Compilation {
        queries.global_ctxt().unwrap().enter(|tcx| {
            println!("Collecting enums from HIR...");
            
            let mut collector = HirEnumCollector::new(tcx);
            collector.collect_all_enums();
            
            println!("Found {} enums", collector.enums.len());
            
            // Generate macros
            let macros = collector.generate_macros();
            std::fs::write("src/generated/hir_enum_macros.rs", &macros)
                .expect("Failed to write HIR macro file");
            
            // Generate orbit analysis
            let analysis = collector.generate_orbit_analysis();
            std::fs::write("src/generated/hir_orbit_analysis.rs", &analysis)
                .expect("Failed to write HIR analysis file");
            
            println!("Generated HIR-based macros written to src/generated/");
        });
        
        Compilation::Stop
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <rust_file>", args[0]);
        eprintln!("Example: {} src/lib.rs", args[0]);
        process::exit(1);
    }
    
    let mut rustc_args = vec![
        "rustc".to_string(),
        args[1].clone(),
        "--crate-type=lib".to_string(),
    ];
    
    let mut callbacks = EnumMacroGenerator;
    
    RunCompiler::new(&rustc_args, &mut callbacks).run().unwrap();
}
