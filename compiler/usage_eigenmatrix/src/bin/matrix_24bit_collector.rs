#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_hir;

use rustc_driver::{Callbacks, Compilation};
use rustc_interface::interface;
use rustc_middle::ty::TyCtxt;
use rustc_hir::def_id::LOCAL_CRATE;
use std::collections::HashMap;
use std::sync::OnceLock;

/// 24-bit Matrix Usage Collector
/// Maps usage patterns directly into 24-bit space during compilation

static MATRIX_24BIT: OnceLock<Matrix24Bit> = OnceLock::new();

struct Matrix24Bit {
    pages: Vec<Option<Box<[u8; 4096]>>>,
    signature_count: u64,
    prime_basis: [u64; 8],
}

impl Matrix24Bit {
    fn new() -> Self {
        Self {
            pages: vec![None; 4096], // 16MB / 4KB = 4096 pages
            signature_count: 0,
            prime_basis: [2, 3, 5, 7, 11, 13, 17, 19],
        }
    }

    /// Map usage pattern to 24-bit signature
    fn map_usage_to_signature(&self, usage: &str, def_id: &str) -> u32 {
        let mut signature = 1u64;
        let combined = format!("{}:{}", usage, def_id);
        
        for (i, byte) in combined.bytes().enumerate() {
            let prime_idx = i % self.prime_basis.len();
            let prime = self.prime_basis[prime_idx];
            signature = signature.wrapping_mul(prime).wrapping_add(byte as u64);
        }
        
        (signature & 0xFFFFFF) as u32 // 24-bit space
    }

    /// Set usage in 24-bit matrix
    fn set_usage(&mut self, signature: u32) {
        let page_idx = (signature as usize) / 4096;
        let offset = (signature as usize) % 4096;
        
        if page_idx < 4096 {
            if self.pages[page_idx].is_none() {
                self.pages[page_idx] = Some(Box::new([0u8; 4096]));
            }
            
            if let Some(ref mut page) = self.pages[page_idx] {
                page[offset] = 1;
                self.signature_count += 1;
            }
        }
    }
}

struct Matrix24BitCollector;

impl Callbacks for Matrix24BitCollector {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &interface::Compiler,
        queries: &'tcx rustc_interface::Queries<'tcx>,
    ) -> Compilation {
        queries.global_ctxt().unwrap().enter(|tcx| {
            self.collect_usage_signatures(tcx);
        });
        Compilation::Continue
    }
}

impl Matrix24BitCollector {
    fn collect_usage_signatures(&self, tcx: TyCtxt<'_>) {
        let matrix = MATRIX_24BIT.get_or_init(|| Matrix24Bit::new());
        
        // Get all DefIds in the crate
        let hir = tcx.hir();
        let crate_items = hir.items();
        
        println!("Collecting usage signatures for 24-bit matrix...");
        
        for item_id in crate_items {
            let def_id = item_id.owner_id.def_id.to_def_id();
            let def_path = tcx.def_path_str(def_id);
            
            // Create signature from DefId and path
            let signature = matrix.map_usage_to_signature(&def_path, &format!("{:?}", def_id));
            
            // This is unsafe but needed for demo - in real implementation would use proper synchronization
            let matrix_mut = unsafe { 
                &mut *(matrix as *const Matrix24Bit as *mut Matrix24Bit) 
            };
            matrix_mut.set_usage(signature);
            
            println!("  {} -> 0x{:06X}", def_path, signature);
        }
        
        let allocated_pages = matrix.pages.iter().filter(|p| p.is_some()).count();
        println!("Matrix usage: {} pages, {} signatures", allocated_pages, matrix.signature_count);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <rust_file>", args[0]);
        std::process::exit(1);
    }
    
    let mut rustc_args = vec![
        "rustc".to_string(),
        args[1].clone(),
        "--crate-type".to_string(),
        "lib".to_string(),
    ];
    
    let mut callbacks = Matrix24BitCollector;
    
    rustc_driver::RunCompiler::new(&rustc_args, &mut callbacks)
        .run()
        .unwrap();
    
    // Print final matrix statistics
    if let Some(matrix) = MATRIX_24BIT.get() {
        let allocated_pages = matrix.pages.iter().filter(|p| p.is_some()).count();
        let usage_percent = (allocated_pages as f64 / 4096.0) * 100.0;
        
        println!("\n24-bit Matrix Results:");
        println!("Signatures collected: {}", matrix.signature_count);
        println!("Pages allocated: {} / 4096 ({:.2}%)", allocated_pages, usage_percent);
        println!("Memory usage: {:.1} MB / 16.0 MB", (allocated_pages as f64 * 4.0) / 1024.0);
    }
}
