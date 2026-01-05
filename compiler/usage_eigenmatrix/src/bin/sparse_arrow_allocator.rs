use std::collections::HashMap;

/// Sparse arrow allocation system with signature regions
struct SparseArrowAllocator {
    signature_regions: HashMap<u16, Vec<u32>>, // region_id -> signatures in region
    node_attributes: HashMap<u32, Vec<String>>, // signature -> internal arrows (attributes)
    cross_region_arrows: HashMap<(u16, u16), Vec<(u32, u32)>>, // (from_region, to_region) -> arrows
    region_size: u32,
}

impl SparseArrowAllocator {
    fn new(region_size: u32) -> Self {
        Self {
            signature_regions: HashMap::new(),
            node_attributes: HashMap::new(),
            cross_region_arrows: HashMap::new(),
            region_size,
        }
    }
    
    /// Truncate 24-bit signature to region ID
    fn signature_to_region(&self, signature: u32) -> u16 {
        ((signature & 0xFFFFFF) / self.region_size) as u16
    }
    
    /// Add signature to appropriate region
    fn allocate_signature(&mut self, signature: u32) {
        let region_id = self.signature_to_region(signature);
        self.signature_regions
            .entry(region_id)
            .or_insert_with(Vec::new)
            .push(signature);
    }
    
    /// Add arrow - either attribute (same signature) or cross-region pointer
    fn add_arrow(&mut self, from_sig: u32, to_sig: u32, arrow_type: String) {
        let from_region = self.signature_to_region(from_sig);
        let to_region = self.signature_to_region(to_sig);
        
        if from_sig == to_sig {
            // Point/attribute - arrow within same signature
            self.node_attributes
                .entry(from_sig)
                .or_insert_with(Vec::new)
                .push(format!("self: {}", arrow_type));
        } else if from_region == to_region {
            // Same region - store as attribute of source
            self.node_attributes
                .entry(from_sig)
                .or_insert_with(Vec::new)
                .push(format!("local: {} -> 0x{:06X}", arrow_type, to_sig));
        } else {
            // Cross-region pointer - gets region allocation
            self.cross_region_arrows
                .entry((from_region, to_region))
                .or_insert_with(Vec::new)
                .push((from_sig, to_sig));
        }
    }
    
    /// Display allocation summary
    fn show_allocation(&self) {
        println!("=== SPARSE ARROW ALLOCATION ===\n");
        
        println!("Region Summary:");
        let mut regions: Vec<_> = self.signature_regions.keys().collect();
        regions.sort();
        
        for &region_id in &regions {
            let sigs = &self.signature_regions[&region_id];
            println!("  Region {}: {} signatures", region_id, sigs.len());
        }
        
        println!("\nNode Attributes (internal arrows):");
        let mut attr_count = 0;
        for (sig, attrs) in &self.node_attributes {
            if attr_count < 5 {
                println!("  0x{:06X}: {} attributes", sig, attrs.len());
                for attr in attrs.iter().take(2) {
                    println!("    - {}", attr);
                }
            }
            attr_count += 1;
        }
        if attr_count > 5 {
            println!("  ... ({} more signatures with attributes)", attr_count - 5);
        }
        
        println!("\nCross-Region Arrows:");
        for ((from_region, to_region), arrows) in &self.cross_region_arrows {
            println!("  Region {} -> Region {}: {} arrows", 
                     from_region, to_region, arrows.len());
            
            for (from_sig, to_sig) in arrows.iter().take(2) {
                println!("    0x{:06X} -> 0x{:06X}", from_sig, to_sig);
            }
        }
        
        println!("\nAllocation Stats:");
        println!("  Total regions: {}", self.signature_regions.len());
        println!("  Signatures with attributes: {}", self.node_attributes.len());
        println!("  Cross-region connections: {}", self.cross_region_arrows.len());
        
        let total_arrows: usize = self.node_attributes.values().map(|v| v.len()).sum::<usize>()
            + self.cross_region_arrows.values().map(|v| v.len()).sum::<usize>();
        println!("  Total arrows allocated: {}", total_arrows);
    }
}

fn main() {
    println!("=== SPARSE ARROW ALLOCATION SYSTEM ===\n");
    
    let mut allocator = SparseArrowAllocator::new(4096); // 4KB regions
    
    // Simulate signature allocation from our rustc data
    let sample_signatures = [
        0x8BCB54, // rustc_errors::emit_err
        0x97FE28, // tracing_core::metadata
        0x99C077, // core::fmt::new_display
        0xC9B1F3, // core::fmt::new_v1
        0xE9ADF9, // core::iter::next
        0xA55D75, // core::iter::into_iter
        0x000032, // numeric collision
        0x000033, // numeric collision
        0x1D94C9, // const PRIME_2
        0xBFB4C0, // const PRIME_3
    ];
    
    println!("Allocating signatures to regions:");
    for &sig in &sample_signatures {
        allocator.allocate_signature(sig);
        let region = allocator.signature_to_region(sig);
        println!("  0x{:06X} -> Region {}", sig, region);
    }
    
    println!("\nAdding arrows:");
    
    // Self-arrows (attributes/points)
    allocator.add_arrow(0x8BCB54, 0x8BCB54, "error_level".to_string());
    allocator.add_arrow(0x97FE28, 0x97FE28, "metadata_type".to_string());
    
    // Same region arrows
    allocator.add_arrow(0x000032, 0x000033, "numeric_sequence".to_string());
    allocator.add_arrow(0x1D94C9, 0xBFB4C0, "prime_sequence".to_string());
    
    // Cross-region arrows
    allocator.add_arrow(0x8BCB54, 0x97FE28, "error_to_trace".to_string());
    allocator.add_arrow(0x99C077, 0xE9ADF9, "format_to_iter".to_string());
    allocator.add_arrow(0x000032, 0x8BCB54, "numeric_to_error".to_string());
    
    allocator.show_allocation();
    
    println!("\n=== REGION-BASED ARROW SYSTEM COMPLETE ===");
    println!("✓ Signatures grouped into regions by truncation");
    println!("✓ Internal arrows stored as node attributes");
    println!("✓ Cross-region arrows get separate allocation");
    println!("✓ Sparse allocation minimizes memory usage");
}
