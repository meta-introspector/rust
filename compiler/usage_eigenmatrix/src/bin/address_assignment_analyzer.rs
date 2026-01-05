/// Exact address assignment analysis
fn main() {
    println!("=== EXACT ADDRESS ASSIGNMENT ANALYSIS ===\n");
    
    let prime_basis = [2, 3, 5, 7, 11, 13, 17, 19];
    
    // Test with simple inputs to see exact assignment
    let test_cases = [
        "const PRIME_2: u32 = 2;",
        "const PRIME_3: u32 = 3;", 
        "const PRIME_5: u32 = 5;",
        "const PRIME_7: u32 = 7;",
    ];
    
    for source in &test_cases {
        println!("Source: {}", source);
        
        // Step-by-step signature calculation
        let mut signature = 0u64;
        
        // Calculate character sum
        let char_sum: u64 = source.chars()
            .enumerate()
            .map(|(j, c)| {
                let contribution = (c as u64) * (j as u64 + 1);
                println!("  Char '{}' at pos {}: {} * {} = {}", 
                         c, j, c as u64, j + 1, contribution);
                contribution
            })
            .sum();
        
        println!("  Total char_sum: {}", char_sum);
        
        // Apply each prime basis
        for (i, &prime) in prime_basis.iter().enumerate() {
            let modulo = char_sum % prime;
            let shift = i * 3;
            let contribution = modulo << shift;
            signature += contribution;
            
            println!("  Prime {}: {} % {} = {} << {} = 0x{:X} (contribution: 0x{:X})", 
                     prime, char_sum, prime, modulo, shift, contribution, signature);
        }
        
        let final_sig = (signature & 0xFFFFFF) as u32;
        println!("  Final signature: 0x{:06X} ({})", final_sig, final_sig);
        
        // Show bit breakdown
        println!("  Bit breakdown:");
        for (i, &prime) in prime_basis.iter().enumerate() {
            let shift = i * 3;
            let mask = 0x7 << shift;
            let component = (final_sig & mask) >> shift;
            println!("    Bits {}-{} (prime {}): {}", shift, shift+2, prime, component);
        }
        
        // Show address assignment
        let region = final_sig / 4096;
        let offset = final_sig % 4096;
        println!("  Address: Region {} + Offset 0x{:03X}", region, offset);
        println!();
    }
    
    // Show how character position affects signature
    println!("Character Position Impact Analysis:");
    let base_char = 'A';
    for pos in 0..5 {
        let mut test_string = String::new();
        for _ in 0..pos {
            test_string.push('_');
        }
        test_string.push(base_char);
        
        let char_sum: u64 = test_string.chars()
            .enumerate()
            .map(|(j, c)| (c as u64) * (j as u64 + 1))
            .sum();
        
        let mut signature = 0u64;
        for (i, &prime) in prime_basis.iter().enumerate() {
            signature += ((char_sum % prime) << (i * 3));
        }
        let final_sig = (signature & 0xFFFFFF) as u32;
        
        println!("  '{}' at position {}: char_sum={}, signature=0x{:06X}", 
                 base_char, pos, char_sum, final_sig);
    }
    
    // Show how different characters affect signature
    println!("\nCharacter Value Impact Analysis:");
    for c in ['A', 'B', 'C', 'a', 'b', 'c', '0', '1', '2'] {
        let test_string = c.to_string();
        let char_sum = c as u64;
        
        let mut signature = 0u64;
        for (i, &prime) in prime_basis.iter().enumerate() {
            signature += ((char_sum % prime) << (i * 3));
        }
        let final_sig = (signature & 0xFFFFFF) as u32;
        
        println!("  '{}' (ASCII {}): signature=0x{:06X}", c, c as u64, final_sig);
    }
    
    println!("\n✓ Exact address assignment analysis complete");
}
