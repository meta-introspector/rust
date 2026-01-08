use std::collections::HashMap;

#[cfg(feature = "rustc_private")]
use rustc_driver::{Callbacks, Compilation};
#[cfg(feature = "rustc_private")]
use rustc_interface::interface;
#[cfg(feature = "rustc_private")]
use rustc_middle::ty::TyCtxt;

use crate::types::*;

pub struct TrollArmy {
    pub span_fixmes: HashMap<String, Vec<String>>,
    pub tcx_captures: Vec<TcxCapture>,
}

impl TrollArmy {
    pub fn new() -> Self {
        TrollArmy {
            span_fixmes: HashMap::new(),
            tcx_captures: Vec::new(),
        }
    }

    #[cfg(feature = "rustc_private")]
    pub fn collect_all_spans(&mut self, tcx: TyCtxt<'_>) {
        // Step 1: Visit all TyCtxt via function calls and macros
        let tcx_data = self.visit_tyctxt(tcx);
        
        // Step 2: Serialize TyCtxt to JSON
        let json_data = serde_json::to_value(&tcx_data).unwrap();
        
        // Step 3: Flatten JSON to array
        let flat_array = self.flatten_json_to_array(json_data);
        
        // Step 4: Split array in harmonics
        let harmonic_splits = self.split_harmonics(&flat_array);
        
        // Step 5: Capture topology
        let topology = self.capture_topology(harmonic_splits);
        
        let capture = TcxCapture::new_with_topology(tcx, topology);
        self.tcx_captures.push(capture);
    }
    
    #[cfg(feature = "rustc_private")]
    fn visit_tyctxt(&self, tcx: TyCtxt<'_>) -> serde_json::Value {
        use serde_json::json;
        
        let local_def_ids: Vec<_> = tcx.hir().body_owners().collect();
        let crate_name = tcx.crate_name(rustc_hir::def_id::LOCAL_CRATE).to_string();
        
        json!({
            "crate_name": crate_name,
            "def_count": local_def_ids.len(),
            "definitions": local_def_ids.iter().map(|id| {
                json!({
                    "def_id": format!("{:?}", id),
                    "path": tcx.def_path_str(*id),
                    "kind": format!("{:?}", tcx.def_kind(*id))
                })
            }).collect::<Vec<_>>()
        })
    }
    
    fn flatten_json_to_array(&self, json: serde_json::Value) -> Vec<String> {
        let mut flat = Vec::new();
        self.flatten_recursive(json, &mut flat);
        flat
    }
    
    fn flatten_recursive(&self, value: serde_json::Value, flat: &mut Vec<String>) {
        match value {
            serde_json::Value::String(s) => flat.push(s),
            serde_json::Value::Number(n) => flat.push(n.to_string()),
            serde_json::Value::Bool(b) => flat.push(b.to_string()),
            serde_json::Value::Array(arr) => {
                for item in arr {
                    self.flatten_recursive(item, flat);
                }
            }
            serde_json::Value::Object(obj) => {
                for (_, v) in obj {
                    self.flatten_recursive(v, flat);
                }
            }
            _ => {}
        }
    }
    
    fn split_harmonics(&self, array: &[String]) -> Vec<Vec<String>> {
        let primes = [2, 3, 5, 7, 11, 13, 17, 19];
        let mut harmonics = vec![Vec::new(); 8];
        
        for (i, item) in array.iter().enumerate() {
            let harmonic_idx = i % 8;
            harmonics[harmonic_idx].push(item.clone());
        }
        
        harmonics
    }
    
    fn capture_topology(&self, harmonics: Vec<Vec<String>>) -> String {
        let topology = harmonics.iter().enumerate()
            .map(|(i, h)| format!("Prime {}: {} items", [2,3,5,7,11,13,17,19][i], h.len()))
            .collect::<Vec<_>>()
            .join(", ");
        
        println!("🎵 Harmonic Topology: {}", topology);
        topology
    }
}

#[cfg(feature = "rustc_private")]
impl Callbacks for TrollArmy {
    fn after_parsing<'tcx>(
        &mut self,
        _compiler: &interface::Compiler,
        queries: &'tcx interface::Queries<'tcx>,
    ) -> Compilation {
        queries.global_ctxt().unwrap().enter(|tcx| {
            self.collect_all_spans(tcx);
        });
        Compilation::Continue
    }
}

pub struct TcxCapture {
    pub crate_name: String,
    pub def_ids: Vec<String>,
    pub topology: String,
}

impl TcxCapture {
    #[cfg(feature = "rustc_private")]
    pub fn new(tcx: TyCtxt<'_>) -> Self {
        let crate_name = tcx.crate_name(rustc_hir::def_id::LOCAL_CRATE).to_string();
        TcxCapture {
            crate_name,
            def_ids: Vec::new(),
            topology: String::new(),
        }
    }
    
    #[cfg(feature = "rustc_private")]
    pub fn new_with_topology(tcx: TyCtxt<'_>, topology: String) -> Self {
        let crate_name = tcx.crate_name(rustc_hir::def_id::LOCAL_CRATE).to_string();
        TcxCapture {
            crate_name,
            def_ids: Vec::new(),
            topology,
        }
    }

    #[cfg(not(feature = "rustc_private"))]
    pub fn new(_: ()) -> Self {
        TcxCapture {
            crate_name: "unknown".to_string(),
            def_ids: Vec::new(),
            topology: String::new(),
        }
    }
}
