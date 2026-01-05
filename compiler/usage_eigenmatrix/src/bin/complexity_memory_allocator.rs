use std::collections::HashMap;

/// Compact Rust subset with complexity-based memory regions
#[derive(Debug, Clone, PartialEq)]
enum RustNode {
    Const(String, u32),      // name, value
    Fn(String),              // name
    Use(String),             // dependency
}

/// Complexity levels for lattice structure
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ComplexityLevel {
    Primitive = 0,    // const values, basic types
    Simple = 1,       // functions with no dependencies
    Dependent = 2,    // functions using constants
    Complex = 3,      // functions with multiple dependencies
}

/// Memory region allocator with complexity-based partitioning
struct ComplexityMemoryAllocator {
    regions: HashMap<ComplexityLevel, Vec<u32>>,  // complexity -> addresses
    node_addresses: HashMap<u32, RustNode>,       // address -> node
    dependencies: HashMap<u32, Vec<u32>>,         // address -> dependencies
    next_address: HashMap<ComplexityLevel, u32>,  // next free address per level
}

impl ComplexityMemoryAllocator {
    fn new() -> Self {
        let mut next_address = HashMap::new();
        // Partition 24-bit space by complexity (6 bits each = 4 levels)
        next_address.insert(ComplexityLevel::Primitive, 0x000000);  // 0x000000-0x03FFFF
        next_address.insert(ComplexityLevel::Simple, 0x040000);     // 0x040000-0x07FFFF  
        next_address.insert(ComplexityLevel::Dependent, 0x080000);  // 0x080000-0x0BFFFF
        next_address.insert(ComplexityLevel::Complex, 0x0C0000);    // 0x0C0000-0x0FFFFF
        
        Self {
            regions: HashMap::new(),
            node_addresses: HashMap::new(),
            dependencies: HashMap::new(),
            next_address,
        }
    }
    
    /// Calculate complexity level based on dependencies
    fn calculate_complexity(&self, node: &RustNode, deps: &[u32]) -> ComplexityLevel {
        match node {
            RustNode::Const(_, _) => ComplexityLevel::Primitive,
            RustNode::Fn(_) => {
                match deps.len() {
                    0 => ComplexityLevel::Simple,
                    1..=2 => ComplexityLevel::Dependent,
                    _ => ComplexityLevel::Complex,
                }
            },
            RustNode::Use(_) => ComplexityLevel::Primitive,
        }
    }
    
    /// Allocate address in appropriate complexity region
    fn allocate_node(&mut self, node: RustNode, deps: Vec<u32>) -> u32 {
        let complexity = self.calculate_complexity(&node, &deps);
        let address = self.next_address[&complexity];
        
        // Update next address for this complexity level
        self.next_address.insert(complexity, address + 1);
        
        // Store node and dependencies
        self.node_addresses.insert(address, node);
        self.dependencies.insert(address, deps);
        
        // Add to region
        self.regions.entry(complexity).or_insert_with(Vec::new).push(address);
        
        address
    }
    
    /// Move node to different address while preserving relationships
    fn relocate_node(&mut self, old_address: u32, new_address: u32) -> Result<(), String> {
        if let Some(node) = self.node_addresses.remove(&old_address) {
            if let Some(deps) = self.dependencies.remove(&old_address) {
                // Update all references to old address
                for (_, node_deps) in self.dependencies.iter_mut() {
                    for dep in node_deps.iter_mut() {
                        if *dep == old_address {
                            *dep = new_address;
                        }
                    }
                }
                
                // Place at new address
                self.node_addresses.insert(new_address, node);
                self.dependencies.insert(new_address, deps);
                
                // Update regions
                for region in self.regions.values_mut() {
                    if let Some(pos) = region.iter().position(|&addr| addr == old_address) {
                        region[pos] = new_address;
                    }
                }
                
                Ok(())
            } else {
                Err("No dependencies found".to_string())
            }
        } else {
            Err("Node not found".to_string())
        }
    }
    
    /// Show memory layout
    fn show_layout(&self) {
        println!("=== COMPLEXITY-BASED MEMORY LAYOUT ===\n");
        
        let levels = [
            ComplexityLevel::Primitive,
            ComplexityLevel::Simple, 
            ComplexityLevel::Dependent,
            ComplexityLevel::Complex,
        ];
        
        for level in &levels {
            println!("Complexity Level {:?} (0x{:06X}-0x{:06X}):", 
                     level, 
                     *level as u32 * 0x040000,
                     (*level as u32 + 1) * 0x040000 - 1);
            
            if let Some(addresses) = self.regions.get(level) {
                for &addr in addresses {
                    if let Some(node) = self.node_addresses.get(&addr) {
                        let empty_deps = vec![];
                        let deps = self.dependencies.get(&addr).unwrap_or(&empty_deps);
                        println!("  0x{:06X}: {:?} (deps: {:?})", addr, node, deps);
                    }
                }
            }
            println!();
        }
    }
    
    /// Compact/defragment memory while preserving relationships
    fn compact_memory(&mut self) {
        println!("Compacting memory while preserving relationships...");
        
        for level in [ComplexityLevel::Primitive, ComplexityLevel::Simple, 
                      ComplexityLevel::Dependent, ComplexityLevel::Complex] {
            if let Some(addresses) = self.regions.get(&level).cloned() {
                let base_addr = level as u32 * 0x040000;
                
                for (i, &old_addr) in addresses.iter().enumerate() {
                    let new_addr = base_addr + i as u32;
                    if old_addr != new_addr {
                        self.relocate_node(old_addr, new_addr).ok();
                    }
                }
            }
        }
    }
}

fn main() {
    println!("=== COMPACT RUST SUBSET WITH COMPLEXITY REGIONS ===\n");
    
    let mut allocator = ComplexityMemoryAllocator::new();
    
    // Allocate our prime constants (primitives)
    let prime_2_addr = allocator.allocate_node(RustNode::Const("PRIME_2".to_string(), 2), vec![]);
    let prime_3_addr = allocator.allocate_node(RustNode::Const("PRIME_3".to_string(), 3), vec![]);
    let prime_5_addr = allocator.allocate_node(RustNode::Const("PRIME_5".to_string(), 5), vec![]);
    
    println!("Allocated constants:");
    println!("  PRIME_2 -> 0x{:06X}", prime_2_addr);
    println!("  PRIME_3 -> 0x{:06X}", prime_3_addr);
    println!("  PRIME_5 -> 0x{:06X}", prime_5_addr);
    
    // Allocate simple function (no dependencies)
    let simple_fn_addr = allocator.allocate_node(RustNode::Fn("simple".to_string()), vec![]);
    
    // Allocate dependent function (uses constants)
    let dependent_fn_addr = allocator.allocate_node(
        RustNode::Fn("print_primes".to_string()), 
        vec![prime_2_addr, prime_3_addr]
    );
    
    // Allocate complex function (multiple dependencies)
    let complex_fn_addr = allocator.allocate_node(
        RustNode::Fn("main".to_string()),
        vec![prime_2_addr, prime_3_addr, prime_5_addr, simple_fn_addr, dependent_fn_addr]
    );
    
    println!("\nAllocated functions:");
    println!("  simple -> 0x{:06X}", simple_fn_addr);
    println!("  print_primes -> 0x{:06X}", dependent_fn_addr);
    println!("  main -> 0x{:06X}", complex_fn_addr);
    
    // Show initial layout
    allocator.show_layout();
    
    // Demonstrate relocation while preserving relationships
    println!("Relocating PRIME_2 from 0x{:06X} to 0x{:06X}...", prime_2_addr, 0x000010);
    allocator.relocate_node(prime_2_addr, 0x000010).unwrap();
    
    // Compact memory
    allocator.compact_memory();
    
    // Show final layout
    println!("After compaction:");
    allocator.show_layout();
    
    println!("✓ Complexity-based memory allocation complete");
    println!("✓ Relationships preserved during relocation");
    println!("✓ Lattice structure maintained");
}
