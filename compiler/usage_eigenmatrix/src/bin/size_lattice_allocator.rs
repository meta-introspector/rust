use std::collections::HashMap;

/// Size-based lattice allocator with chunk fitting
struct SizeLatticeAllocator {
    size_buckets: HashMap<usize, Vec<u32>>,  // size -> addresses of objects that size
    allocated_chunks: HashMap<usize, (u32, u32)>, // size -> (start_addr, end_addr) 
    objects: HashMap<u32, (String, usize)>,  // address -> (content, size)
    next_chunk_start: u32,
}

impl SizeLatticeAllocator {
    fn new() -> Self {
        Self {
            size_buckets: HashMap::new(),
            allocated_chunks: HashMap::new(),
            objects: HashMap::new(),
            next_chunk_start: 0x000000,
        }
    }
    
    /// Calculate object size (for demo, use content length)
    fn calculate_size(&self, content: &str) -> usize {
        content.len()
    }
    
    /// Allocate chunk for specific size if not exists
    fn allocate_chunk_for_size(&mut self, size: usize) -> (u32, u32) {
        if let Some(&chunk) = self.allocated_chunks.get(&size) {
            return chunk;
        }
        
        // Allocate new chunk - size determines chunk size
        let chunk_size = match size {
            0..=16 => 0x1000,      // 4KB for small objects
            17..=64 => 0x4000,     // 16KB for medium objects  
            65..=256 => 0x10000,   // 64KB for large objects
            _ => 0x40000,          // 256KB for huge objects
        };
        
        let start_addr = self.next_chunk_start;
        let end_addr = start_addr + chunk_size - 1;
        
        self.allocated_chunks.insert(size, (start_addr, end_addr));
        self.next_chunk_start = end_addr + 1;
        
        println!("Allocated chunk for size {}: 0x{:06X}-0x{:06X} ({} bytes)", 
                 size, start_addr, end_addr, chunk_size);
        
        (start_addr, end_addr)
    }
    
    /// Allocate object in appropriate size bucket
    fn allocate_object(&mut self, content: String) -> u32 {
        let size = self.calculate_size(&content);
        
        // Ensure chunk exists for this size
        let (chunk_start, _) = self.allocate_chunk_for_size(size);
        
        // Find next free address in this size's chunk
        let objects_in_bucket = self.size_buckets.entry(size).or_insert_with(Vec::new);
        let address = chunk_start + objects_in_bucket.len() as u32;
        
        // Store object
        objects_in_bucket.push(address);
        self.objects.insert(address, (content, size));
        
        address
    }
    
    /// Show lattice structure
    fn show_lattice(&self) {
        println!("\n=== SIZE LATTICE STRUCTURE ===\n");
        
        // Show size buckets
        let mut sizes: Vec<_> = self.size_buckets.keys().collect();
        sizes.sort();
        
        for &size in &sizes {
            let objects = &self.size_buckets[&size];
            let chunk = self.allocated_chunks.get(&size);
            
            println!("Size {} bucket ({} objects):", size, objects.len());
            if let Some(&(start, end)) = chunk {
                println!("  Chunk: 0x{:06X}-0x{:06X}", start, end);
            }
            
            for &addr in objects.iter().take(3) {
                if let Some((content, _)) = self.objects.get(&addr) {
                    let display = if content.len() > 30 { 
                        format!("{}...", &content[..27]) 
                    } else { 
                        content.clone() 
                    };
                    println!("    0x{:06X}: \"{}\"", addr, display);
                }
            }
            if objects.len() > 3 {
                println!("    ... ({} more objects)", objects.len() - 3);
            }
            println!();
        }
    }
    
    /// Show memory utilization
    fn show_utilization(&self) {
        println!("=== MEMORY UTILIZATION ===\n");
        
        let mut total_allocated = 0u32;
        let mut total_used = 0u32;
        
        for (&size, &(start, end)) in &self.allocated_chunks {
            let chunk_size = end - start + 1;
            let objects_count = self.size_buckets.get(&size).map_or(0, |v| v.len());
            let used_space = objects_count * size;
            let utilization = (used_space as f64 / chunk_size as f64) * 100.0;
            
            println!("Size {} chunk: {}/{} bytes used ({:.1}%)", 
                     size, used_space, chunk_size, utilization);
            
            total_allocated += chunk_size;
            total_used += used_space as u32;
        }
        
        let overall_utilization = (total_used as f64 / total_allocated as f64) * 100.0;
        println!("\nOverall: {}/{} bytes used ({:.1}%)", 
                 total_used, total_allocated, overall_utilization);
    }
    
    /// Compact objects within their size buckets
    fn compact_buckets(&mut self) {
        println!("\nCompacting size buckets...");
        
        for (&size, objects) in &mut self.size_buckets {
            if let Some(&(chunk_start, _)) = self.allocated_chunks.get(&size) {
                // Collect old addresses first
                let old_addresses: Vec<u32> = objects.clone();
                
                // Reassign addresses sequentially within chunk
                for (i, &old_addr) in old_addresses.iter().enumerate() {
                    let new_addr = chunk_start + i as u32;
                    if old_addr != new_addr {
                        if let Some(obj_data) = self.objects.remove(&old_addr) {
                            self.objects.insert(new_addr, obj_data);
                            objects[i] = new_addr;
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    println!("=== SIZE LATTICE ALLOCATOR ===");
    
    let mut allocator = SizeLatticeAllocator::new();
    
    // Allocate objects of different sizes
    let objects = [
        "const PRIME_2: u32 = 2;",  // size 21
        "const PRIME_3: u32 = 3;",  // size 21  
        "fn main() {}",              // size 12
        "fn simple() { println!(\"hello\"); }", // size 33
        "struct Point { x: i32, y: i32 }", // size 31
        "const PRIME_5: u32 = 5;",  // size 21
        "fn complex_function_with_long_name() { /* lots of code */ }", // size 62
        "use std::collections::HashMap;", // size 30
        "impl Display for Point { fn fmt(&self, f: &mut Formatter) -> Result<(), Error> { write!(f, \"({}, {})\", self.x, self.y) } }", // size 120
    ];
    
    println!("Allocating objects:");
    for obj in &objects {
        let addr = allocator.allocate_object(obj.to_string());
        let size = allocator.calculate_size(obj);
        println!("  Size {}: 0x{:06X} <- \"{}\"", size, addr, 
                 if obj.len() > 40 { format!("{}...", &obj[..37]) } else { obj.to_string() });
    }
    
    // Show lattice structure
    allocator.show_lattice();
    
    // Show utilization
    allocator.show_utilization();
    
    // Compact buckets
    allocator.compact_buckets();
    
    println!("\n✓ Size lattice allocation complete");
    println!("✓ Objects grouped by size in dedicated chunks");
    println!("✓ Good fit allocation achieved");
}
