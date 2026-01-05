use std::collections::HashMap;

/// Meaningful bit address analyzer - find prime number regions
fn main() {
    println!("=== MEANINGFUL BIT ADDRESS ANALYSIS ===\n");
    
    // Prime constants and their signatures from our matrix
    let prime_constants = [
        (2, 0x1D94C9),   // PRIME_2
        (3, 0xBFB4C0),   // PRIME_3  
        (5, 0xC674C8),   // PRIME_5
        (7, 0x0A04D0),   // PRIME_7
        (11, 0x6C0008),  // PRIME_11
        (13, 0x46FC50),  // PRIME_13
        (17, 0x40D8C8),  // PRIME_17
        (19, 0x664710),  // PRIME_19
        (23, 0x0DA090),  // PRIME_23
        (29, 0x2CF810),  // PRIME_29
        (31, 0xF8B488),  // PRIME_31
        (37, 0xD40C08),  // PRIME_37
        (41, 0xC5E6C8),  // PRIME_41
        (43, 0xA0D510),  // PRIME_43
        (47, 0x9AB048),  // PRIME_47
        (53, 0x678610),  // PRIME_53
        (59, 0x8050D0),  // PRIME_59
        (61, 0x0E9A08),  // PRIME_61
        (67, 0x2E94C8),  // PRIME_67
    ];
    
    println!("Prime Number Signature Analysis:");
    for (prime, sig) in &prime_constants {
        let region = sig / 4096;
        let offset = sig % 4096;
        println!("  Prime {}: 0x{:06X} -> Region {} + 0x{:03X}", 
                 prime, sig, region, offset);
    }
    
    // Group by regions
    let mut regions: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();
    for (prime, sig) in &prime_constants {
        let region = sig / 4096;
        regions.entry(region).or_insert_with(Vec::new).push((*prime, *sig));
    }
    
    println!("\nPrime Regions:");
    let mut region_list: Vec<_> = regions.iter().collect();
    region_list.sort_by_key(|(region, _)| *region);
    
    for (region, primes) in region_list {
        println!("  Region {}: {} primes", region, primes.len());
        for (prime, sig) in primes {
            println!("    Prime {} -> 0x{:06X}", prime, sig);
        }
    }
    
    // Analyze bit patterns in signatures
    println!("\nBit Pattern Analysis:");
    
    // Check if prime value appears in signature bits
    for (prime, sig) in &prime_constants {
        let sig_bits = format!("{:024b}", sig);
        let prime_bits = format!("{:08b}", prime);
        
        if sig_bits.contains(&prime_bits) {
            println!("  Prime {} bits found in signature 0x{:06X}", prime, sig);
        }
        
        // Check mathematical relationships
        let modulo_check = sig % prime;
        if modulo_check == 0 {
            println!("  Prime {} divides signature 0x{:06X}", prime, sig);
        }
        
        // Check if signature contains prime as factor
        if sig % prime == prime - 1 {
            println!("  Prime {} signature shows (p-1) modulo: 0x{:06X}", prime, sig);
        }
    }
    
    // Look for prime basis influence
    println!("\nPrime Basis Influence Analysis:");
    let prime_basis = [2, 3, 5, 7, 11, 13, 17, 19];
    
    for (prime, sig) in &prime_constants {
        println!("  Prime {}: 0x{:06X}", prime, sig);
        
        // Decompose signature by prime basis
        let mut remaining = *sig;
        for (i, &basis_prime) in prime_basis.iter().enumerate() {
            let shift = i * 3;
            let mask = 0x7 << shift;  // 3 bits
            let component = (remaining & mask) >> shift;
            
            if component > 0 {
                println!("    Basis prime {} contributes: {} (shift {})", 
                         basis_prime, component, shift);
            }
        }
    }
    
    // Check for meaningful address patterns
    println!("\nMeaningful Address Patterns:");
    
    // Sort primes by signature to see if there's ordering
    let mut sorted_by_sig = prime_constants.clone();
    sorted_by_sig.sort_by_key(|(_, sig)| *sig);
    
    println!("  Primes sorted by signature:");
    for (prime, sig) in &sorted_by_sig {
        println!("    {} -> 0x{:06X}", prime, sig);
    }
    
    // Check if smaller primes get smaller signatures
    let mut order_preserved = 0;
    for i in 1..sorted_by_sig.len() {
        if sorted_by_sig[i-1].0 < sorted_by_sig[i].0 {
            order_preserved += 1;
        }
    }
    
    let order_percentage = (order_preserved as f64 / (sorted_by_sig.len() - 1) as f64) * 100.0;
    println!("  Prime order preservation: {:.1}% ({}/{})", 
             order_percentage, order_preserved, sorted_by_sig.len() - 1);
    
    // Look for prime clustering in address space
    println!("\nPrime Clustering Analysis:");
    let mut distances = Vec::new();
    for i in 1..sorted_by_sig.len() {
        let distance = sorted_by_sig[i].1 - sorted_by_sig[i-1].1;
        distances.push(distance);
    }
    
    distances.sort();
    let median_distance = distances[distances.len() / 2];
    let min_distance = distances[0];
    let max_distance = distances[distances.len() - 1];
    
    println!("  Signature distances:");
    println!("    Min: 0x{:06X}", min_distance);
    println!("    Median: 0x{:06X}", median_distance);  
    println!("    Max: 0x{:06X}", max_distance);
    
    // Check if primes cluster in specific bit ranges
    let mut bit_analysis = HashMap::new();
    for (prime, sig) in &prime_constants {
        let high_bits = (sig >> 16) & 0xFF;  // Top 8 bits
        let mid_bits = (sig >> 8) & 0xFF;    // Middle 8 bits  
        let low_bits = sig & 0xFF;           // Bottom 8 bits
        
        *bit_analysis.entry(high_bits).or_insert(0) += 1;
    }
    
    println!("\nHigh Bit Distribution (top 8 bits):");
    let mut bit_dist: Vec<_> = bit_analysis.iter().collect();
    bit_dist.sort_by_key(|(bits, _)| *bits);
    
    for (bits, count) in bit_dist {
        if *count > 1 {
            println!("  0x{:02X}: {} primes", bits, count);
        }
    }
    
    println!("\n✓ Meaningful bit address analysis complete");
}
