
// Add to enhanced_usage_collector.rs

#[derive(Debug, Clone)]
struct UsageChain {
    from_method: String,
    to_method: String,
    count: u32,
}

impl EnhancedUsageCollector {
    fn add_usage_chain(&mut self, from: &str, to: &str) {
        let chain_key = format!("{}_to_{}", from, to);
        let entry = UsageEntry {
            usage: format!("CHAIN: {} -> {}", from, to),
            def_id: None,
            tyctxt_method: Some(chain_key),
        };
        self.module_data.entry("chains".to_string()).or_insert_with(Vec::new).push(entry);
    }
    
    fn track_usage_chains<'tcx>(&mut self, tcx: TyCtxt<'tcx>) {
        let hir = tcx.hir();
        
        // Most common chain: hir -> predicates_of
        for item_id in hir.items() {
            let def_id = item_id.owner_id.def_id;
            self.add_tyctxt_usage("hir_items", None);
            
            let _predicates = tcx.predicates_of(def_id);
            self.add_tyctxt_usage("predicates_of", Some(format!("{:?}", def_id)));
            self.add_usage_chain("hir", "predicates_of");
        }
    }
}
