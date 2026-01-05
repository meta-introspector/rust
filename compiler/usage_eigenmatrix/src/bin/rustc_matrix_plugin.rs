#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_hir;

use rustc_driver::{Callbacks, Compilation};
use rustc_interface::interface;
use rustc_middle::ty::TyCtxt;
use rustc_hir::def_id::DefId;
use std::sync::Mutex;

static MATRIX_24BIT: Mutex<Vec<Option<Box<[u8; 4096]>>>> = Mutex::new(Vec::new());
static SIGNATURE_COUNT: Mutex<u64> = Mutex::new(0);

struct Matrix24BitCollector;

impl Callbacks for Matrix24BitCollector {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &interface::Compiler,
        queries: &'tcx interface::queries::Queries<'tcx>,
    ) -> Compilation {
        queries.global_ctxt().unwrap().enter(|tcx| {
            collect_defids_to_matrix(tcx);
        });
        Compilation::Continue
    }
}

fn collect_defids_to_matrix(tcx: TyCtxt<'_>) {
    let mut matrix = MATRIX_24BIT.lock().unwrap();
    if matrix.is_empty() {
        *matrix = vec![None; 4096];
    }
    
    let mut count = SIGNATURE_COUNT.lock().unwrap();
    
    // Get all local DefIds
    let all_def_ids = tcx.hir_body_owners();
    
    for def_id in all_def_ids {
        let def_id = def_id.to_def_id();
        let signature = map_defid_to_24bit(def_id);
        
        set_in_matrix(&mut matrix, signature);
        *count += 1;
        
        if *count <= 10 {
            println!("DefId {:?} -> 0x{:06X}", def_id, signature);
        }
    }
    
    let allocated_pages = matrix.iter().filter(|p| p.is_some()).count();
    println!("Matrix: {} DefIds, {} pages", *count, allocated_pages);
}

fn map_defid_to_24bit(def_id: DefId) -> u32 {
    let crate_num = def_id.krate.as_u32();
    let def_index = def_id.index.as_u32();
    
    let signature = ((crate_num as u64) << 16) | (def_index as u64);
    (signature & 0xFFFFFF) as u32
}

fn set_in_matrix(matrix: &mut Vec<Option<Box<[u8; 4096]>>>, signature: u32) {
    let page_idx = (signature as usize) / 4096;
    let offset = (signature as usize) % 4096;
    
    if page_idx < 4096 {
        if matrix[page_idx].is_none() {
            matrix[page_idx] = Some(Box::new([0u8; 4096]));
        }
        
        if let Some(ref mut page) = matrix[page_idx] {
            page[offset] = 1;
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    let mut rustc_args = vec![
        "rustc".to_string(),
        "--crate-type".to_string(),
        "lib".to_string(),
    ];
    
    if args.len() > 1 {
        rustc_args.push(args[1].clone());
    } else {
        rustc_args.push("simple_prime_sieve.rs".to_string());
    }
    
    let mut callbacks = Matrix24BitCollector;
    
    rustc_driver::run_compiler(&rustc_args, &mut callbacks);
    
    let count = *SIGNATURE_COUNT.lock().unwrap();
    let matrix = MATRIX_24BIT.lock().unwrap();
    let allocated_pages = matrix.iter().filter(|p| p.is_some()).count();
    
    println!("\n24-bit Matrix Results:");
    println!("DefIds processed: {}", count);
    println!("Pages allocated: {}/{} ({:.2}%)", 
        allocated_pages, 4096, (allocated_pages as f64 / 4096.0) * 100.0);
}
