use std::collections::HashMap;

/// Per-module semantic address model
struct ModuleAddressModel {
    module_name: String,
    size_buckets: HashMap<usize, Vec<u32>>,
    objects: HashMap<u32, String>,
    base_address: u32,  // Module's base address in global space
}

/// Global address space manager for merging modules
struct GlobalAddressManager {
    modules: HashMap<String, ModuleAddressModel>,
    module_bases: HashMap<String, u32>,
    next_module_base: u32,
}

impl ModuleAddressModel {
    fn new(module_name: String, base_address: u32) -> Self {
        Self {
            module_name,
            size_buckets: HashMap::new(),
            objects: HashMap::new(),
            base_address,
        }
    }
    
    fn allocate_object(&mut self, content: String) -> u32 {
        let size = content.len();
        let objects_in_bucket = self.size_buckets.entry(size).or_insert_with(Vec::new);
        
        // Local address within module
        let local_addr = ((size as u32) << 16) | (objects_in_bucket.len() as u32);
        // Global address = module_base + local_addr
        let global_addr = self.base_address + local_addr;
        
        objects_in_bucket.push(global_addr);
        self.objects.insert(global_addr, content);
        
        global_addr
    }
}

impl GlobalAddressManager {
    fn new() -> Self {
        Self {
            modules: HashMap::new(),
            module_bases: HashMap::new(),
            next_module_base: 0x000000,
        }
    }
    
    /// Create new module with dedicated address space
    fn create_module(&mut self, module_name: String) -> &mut ModuleAddressModel {
        let base_address = self.next_module_base;
        self.module_bases.insert(module_name.clone(), base_address);
        
        // Each module gets 1MB address space (0x100000)
        self.next_module_base += 0x100000;
        
        let model = ModuleAddressModel::new(module_name.clone(), base_address);
        self.modules.insert(module_name.clone(), model);
        
        println!("Created module '{}' at base 0x{:06X}", module_name, base_address);
        
        self.modules.get_mut(&module_name).unwrap()
    }
    
    /// Merge all modules into unified address space
    fn merge_modules(&self) -> HashMap<u32, (String, String)> {  // addr -> (module, content)
        let mut merged = HashMap::new();
        
        for (module_name, model) in &self.modules {
            for (&addr, content) in &model.objects {
                merged.insert(addr, (module_name.clone(), content.clone()));
            }
        }
        
        merged
    }
    
    /// Decode semantic address
    fn decode_address(&self, addr: u32) -> Option<(String, usize, u32)> {  // (module, size, index)
        for (module_name, &base) in &self.module_bases {
            if addr >= base && addr < base + 0x100000 {
                let local_addr = addr - base;
                let size = (local_addr >> 16) as usize;
                let index = local_addr & 0xFFFF;
                return Some((module_name.clone(), size, index));
            }
        }
        None
    }
    
    /// Show global address map
    fn show_global_map(&self) {
        println!("\n=== GLOBAL SEMANTIC ADDRESS MAP ===\n");
        
        let merged = self.merge_modules();
        let mut addresses: Vec<_> = merged.keys().collect();
        addresses.sort();
        
        for &addr in addresses.iter().take(20) {  // Show first 20
            if let Some((module, content)) = merged.get(&addr) {
                if let Some((mod_name, size, index)) = self.decode_address(*addr) {
                    let display = if content.len() > 30 { 
                        format!("{}...", &content[..27]) 
                    } else { 
                        content.clone() 
                    };
                    println!("0x{:06X}: {}[size:{}][{}] \"{}\"", 
                             addr, mod_name, size, index, display);
                }
            }
        }
        
        if addresses.len() > 20 {
            println!("... ({} more addresses)", addresses.len() - 20);
        }
    }
    
    /// Show module statistics
    fn show_module_stats(&self) {
        println!("\n=== MODULE STATISTICS ===\n");
        
        for (module_name, model) in &self.modules {
            let object_count = model.objects.len();
            let size_buckets = model.size_buckets.len();
            let base = self.module_bases[module_name];
            
            println!("Module '{}': {} objects, {} size buckets, base 0x{:06X}", 
                     module_name, object_count, size_buckets, base);
            
            // Show size distribution
            let mut sizes: Vec<_> = model.size_buckets.keys().collect();
            sizes.sort();
            for &size in sizes.iter().take(5) {
                let count = model.size_buckets[size].len();
                println!("  Size {}: {} objects", size, count);
            }
            if sizes.len() > 5 {
                println!("  ... ({} more sizes)", sizes.len() - 5);
            }
            println!();
        }
    }
}

fn main() {
    println!("=== PER-MODULE SEMANTIC ADDRESSING ===");
    
    let mut global_mgr = GlobalAddressManager::new();
    
    // Create rustc modules
    let rustc_errors = global_mgr.create_module("rustc_errors".to_string());
    rustc_errors.allocate_object("fn emit_err(msg: &str)".to_string());
    rustc_errors.allocate_object("fn struct_span_err()".to_string());
    rustc_errors.allocate_object("struct DiagnosticBuilder".to_string());
    
    let rustc_middle = global_mgr.create_module("rustc_middle".to_string());
    rustc_middle.allocate_object("struct DefId(u32, u32)".to_string());
    rustc_middle.allocate_object("enum TyKind".to_string());
    rustc_middle.allocate_object("fn with_source_info()".to_string());
    
    let rustc_span = global_mgr.create_module("rustc_span".to_string());
    rustc_span.allocate_object("struct Span".to_string());
    rustc_span.allocate_object("fn dummy()".to_string());
    
    let core = global_mgr.create_module("core".to_string());
    core.allocate_object("const PRIME_2: u32 = 2;".to_string());
    core.allocate_object("const PRIME_3: u32 = 3;".to_string());
    core.allocate_object("fn cmp::PartialOrd::le()".to_string());
    
    // Show module stats
    global_mgr.show_module_stats();
    
    // Show global merged address map
    global_mgr.show_global_map();
    
    // Demonstrate address decoding
    println!("\n=== ADDRESS DECODING ===\n");
    let test_addresses = [0x000000, 0x100000, 0x200000, 0x300000];
    
    for &addr in &test_addresses {
        if let Some((module, size, index)) = global_mgr.decode_address(addr) {
            println!("0x{:06X} -> Module: {}, Size: {}, Index: {}", 
                     addr, module, size, index);
        }
    }
    
    println!("\n✓ Per-module semantic addressing complete");
    println!("✓ Modules merged into unified address space");
    println!("✓ Addresses encode module + size + position");
}
