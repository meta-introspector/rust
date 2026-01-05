use std::fs;
use std::collections::HashMap;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== ARROW SIGNATURE RANGE ANALYSIS ===\n");
    
    // Load matrix
    let json = fs::read_to_string("prime_constants_program.rs.matrix.json")?;
    let matrix: Value = serde_json::from_str(&json)?;
    
    let mut arrows = Vec::new();
    let mut all_signatures = Vec::new();
    
    // Extract arrows and signatures
    if let Some(arrows_obj) = matrix["arrows"].as_object() {
        for (from_sig, to_sigs) in arrows_obj {
            let from: u32 = from_sig.parse()?;
            all_signatures.push(from);
            
            if let Some(to_array) = to_sigs.as_array() {
                for to_sig in to_array {
                    if let Some(to) = to_sig.as_u64() {
                        let to = to as u32;
                        all_signatures.push(to);
                        arrows.push((from, to));
                    }
                }
            }
        }
    }
    
    // Add node signatures
    if let Some(nodes_obj) = matrix["nodes"].as_object() {
        for sig in nodes_obj.keys() {
            if let Ok(s) = sig.parse::<u32>() {
                all_signatures.push(s);
            }
        }
    }
    
    all_signatures.sort();
    all_signatures.dedup();
    
    println!("Total signatures: {}", all_signatures.len());
    println!("Total arrows: {}", arrows.len());
    
    // Analyze signature ranges
    let min_sig = *all_signatures.first().unwrap();
    let max_sig = *all_signatures.last().unwrap();
    let range = max_sig - min_sig;
    
    println!("\nSignature Range Analysis:");
    println!("  Min: {} (0x{:06X})", min_sig, min_sig);
    println!("  Max: {} (0x{:06X})", max_sig, max_sig);
    println!("  Range: {} (0x{:06X})", range, range);
    
    // Create partitions (4 ranges)
    let partition_size = range / 4;
    let mut partitions = vec![Vec::new(); 4];
    
    for &sig in &all_signatures {
        let partition = ((sig - min_sig) / partition_size).min(3) as usize;
        partitions[partition].push(sig);
    }
    
    println!("\nPartition Analysis (4 ranges):");
    for (i, partition) in partitions.iter().enumerate() {
        let start = min_sig + (i as u32 * partition_size);
        let end = if i == 3 { max_sig } else { start + partition_size - 1 };
        println!("  Partition {}: 0x{:06X}-0x{:06X} ({} signatures)", 
                 i, start, end, partition.len());
    }
    
    // Analyze arrow patterns by partition
    println!("\nArrow Partition Analysis:");
    let mut intra_partition = 0;  // arrows within same partition
    let mut inter_partition = 0;  // arrows across partitions
    let mut partition_matrix = vec![vec![0; 4]; 4];
    
    for (from, to) in &arrows {
        let from_partition = ((*from - min_sig) / partition_size).min(3) as usize;
        let to_partition = ((*to - min_sig) / partition_size).min(3) as usize;
        
        partition_matrix[from_partition][to_partition] += 1;
        
        if from_partition == to_partition {
            intra_partition += 1;
        } else {
            inter_partition += 1;
        }
    }
    
    println!("  Intra-partition arrows: {} ({:.1}%)", 
             intra_partition, (intra_partition as f64 / arrows.len() as f64) * 100.0);
    println!("  Inter-partition arrows: {} ({:.1}%)", 
             inter_partition, (inter_partition as f64 / arrows.len() as f64) * 100.0);
    
    // Show partition matrix
    println!("\nPartition Connectivity Matrix:");
    println!("     0   1   2   3");
    for (i, row) in partition_matrix.iter().enumerate() {
        print!("  {}: ", i);
        for &count in row {
            print!("{:3} ", count);
        }
        println!();
    }
    
    // Region-based analysis (using 4KB regions like our sparse allocator)
    println!("\nRegion Analysis (4KB regions):");
    let mut regions = HashMap::new();
    
    for &sig in &all_signatures {
        let region = sig / 4096;
        *regions.entry(region).or_insert(0) += 1;
    }
    
    let mut region_list: Vec<_> = regions.iter().collect();
    region_list.sort_by_key(|(region, _)| *region);
    
    println!("  Total regions used: {}", regions.len());
    for (region, count) in region_list.iter().take(10) {
        println!("    Region {}: {} signatures", region, count);
    }
    
    // Check for clustering
    let mut region_arrows = HashMap::new();
    for (from, to) in &arrows {
        let from_region = from / 4096;
        let to_region = to / 4096;
        
        if from_region == to_region {
            *region_arrows.entry(from_region).or_insert(0) += 1;
        }
    }
    
    println!("\nRegion Clustering:");
    if region_arrows.is_empty() {
        println!("  No intra-region arrows (highly distributed)");
    } else {
        for (region, count) in region_arrows {
            println!("  Region {}: {} internal arrows", region, count);
        }
    }
    
    println!("\n✓ Signature range analysis complete");
    
    Ok(())
}
