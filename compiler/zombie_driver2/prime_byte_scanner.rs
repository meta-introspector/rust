// 🧟 DIRECT PRIME SCANNING: Find primes 0-71 as constants in rustc binary bytes
use std::fs;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct PrimeByteAnalysis {
    prime: u32,
    u8_count: u32,
    u16_count: u32,
    u32_count: u32,
    u64_count: u32,
    byte_positions: Vec<usize>,
    likely_constants: Vec<ConstantContext>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ConstantContext {
    position: usize,
    int_size: u8,
    surrounding_bytes: Vec<u8>,
    likely_usage: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧟 DIRECT PRIME BYTE SCANNING IN RUSTC BINARY");
    println!("=============================================");
    
    // Load rustc_driver.so binary
    let binary_data = load_rustc_binary()?;
    println!("📦 Loaded {} bytes from rustc binary", binary_data.len());
    
    // Scan for primes 0-71 as direct constants
    let prime_analysis = scan_primes_in_binary(&binary_data, 0..=71)?;
    
    // Display results
    display_prime_byte_analysis(&prime_analysis);
    
    // Save results
    save_prime_byte_analysis(&prime_analysis)?;
    
    Ok(())
}

fn load_rustc_binary() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Try multiple locations for rustc_driver.so
    let paths = [
        "target/debug/deps/librustc_driver.so",
        "/usr/lib/librustc_driver.so", 
        "librustc_driver.so",
        "zombie_spore_winner.so",
    ];
    
    for path in &paths {
        if let Ok(data) = fs::read(path) {
            println!("✅ Found rustc binary at: {}", path);
            return Ok(data);
        }
    }
    
    // Fallback: create mock binary data for demonstration
    println!("⚠️ Using mock binary data for demonstration");
    Ok(create_mock_binary_data())
}

fn create_mock_binary_data() -> Vec<u8> {
    let mut data = vec![0u8; 10000];
    
    // Inject some primes as constants at various positions
    inject_u8_constant(&mut data, 100, 2);
    inject_u8_constant(&mut data, 200, 3);
    inject_u8_constant(&mut data, 300, 5);
    inject_u8_constant(&mut data, 400, 7);
    inject_u8_constant(&mut data, 500, 31);
    
    inject_u16_constant(&mut data, 1000, 11);
    inject_u16_constant(&mut data, 1100, 13);
    inject_u16_constant(&mut data, 1200, 17);
    inject_u16_constant(&mut data, 1300, 19);
    inject_u16_constant(&mut data, 1400, 23);
    
    inject_u32_constant(&mut data, 2000, 29);
    inject_u32_constant(&mut data, 2100, 37);
    inject_u32_constant(&mut data, 2200, 41);
    inject_u32_constant(&mut data, 2300, 43);
    
    inject_u64_constant(&mut data, 3000, 47);
    inject_u64_constant(&mut data, 3100, 53);
    inject_u64_constant(&mut data, 3200, 59);
    inject_u64_constant(&mut data, 3300, 61);
    inject_u64_constant(&mut data, 3400, 67);
    inject_u64_constant(&mut data, 3500, 71);
    
    data
}

fn inject_u8_constant(data: &mut [u8], pos: usize, value: u8) {
    if pos < data.len() { data[pos] = value; }
}

fn inject_u16_constant(data: &mut [u8], pos: usize, value: u16) {
    let bytes = value.to_le_bytes();
    if pos + 1 < data.len() {
        data[pos] = bytes[0];
        data[pos + 1] = bytes[1];
    }
}

fn inject_u32_constant(data: &mut [u8], pos: usize, value: u32) {
    let bytes = value.to_le_bytes();
    if pos + 3 < data.len() {
        for (i, &byte) in bytes.iter().enumerate() {
            data[pos + i] = byte;
        }
    }
}

fn inject_u64_constant(data: &mut [u8], pos: usize, value: u64) {
    let bytes = value.to_le_bytes();
    if pos + 7 < data.len() {
        for (i, &byte) in bytes.iter().enumerate() {
            data[pos + i] = byte;
        }
    }
}

fn scan_primes_in_binary(binary: &[u8], prime_range: std::ops::RangeInclusive<u32>) -> Result<Vec<PrimeByteAnalysis>, Box<dyn std::error::Error>> {
    let mut results = Vec::new();
    
    for prime in prime_range {
        if prime > 255 && !is_prime(prime) {
            continue; // Skip non-primes > 255
        }
        
        let analysis = scan_prime_in_all_sizes(binary, prime);
        results.push(analysis);
    }
    
    Ok(results)
}

fn scan_prime_in_all_sizes(binary: &[u8], prime: u32) -> PrimeByteAnalysis {
    let mut analysis = PrimeByteAnalysis {
        prime,
        u8_count: 0,
        u16_count: 0,
        u32_count: 0,
        u64_count: 0,
        byte_positions: Vec::new(),
        likely_constants: Vec::new(),
    };
    
    // Scan as u8 (if prime fits)
    if prime <= 255 {
        let positions = scan_u8_constant(binary, prime as u8);
        analysis.u8_count = positions.len() as u32;
        analysis.byte_positions.extend(&positions);
        
        for &pos in &positions {
            analysis.likely_constants.push(ConstantContext {
                position: pos,
                int_size: 8,
                surrounding_bytes: get_surrounding_bytes(binary, pos, 4),
                likely_usage: classify_u8_usage(binary, pos, prime as u8),
            });
        }
    }
    
    // Scan as u16
    if prime <= 65535 {
        let positions = scan_u16_constant(binary, prime as u16);
        analysis.u16_count = positions.len() as u32;
        
        for &pos in &positions {
            analysis.byte_positions.push(pos);
            analysis.likely_constants.push(ConstantContext {
                position: pos,
                int_size: 16,
                surrounding_bytes: get_surrounding_bytes(binary, pos, 6),
                likely_usage: classify_u16_usage(binary, pos, prime as u16),
            });
        }
    }
    
    // Scan as u32
    let positions_u32 = scan_u32_constant(binary, prime);
    analysis.u32_count = positions_u32.len() as u32;
    
    for &pos in &positions_u32 {
        analysis.byte_positions.push(pos);
        analysis.likely_constants.push(ConstantContext {
            position: pos,
            int_size: 32,
            surrounding_bytes: get_surrounding_bytes(binary, pos, 8),
            likely_usage: classify_u32_usage(binary, pos, prime),
        });
    }
    
    // Scan as u64
    let positions_u64 = scan_u64_constant(binary, prime as u64);
    analysis.u64_count = positions_u64.len() as u32;
    
    for &pos in &positions_u64 {
        analysis.byte_positions.push(pos);
        analysis.likely_constants.push(ConstantContext {
            position: pos,
            int_size: 64,
            surrounding_bytes: get_surrounding_bytes(binary, pos, 12),
            likely_usage: classify_u64_usage(binary, pos, prime as u64),
        });
    }
    
    analysis
}

fn scan_u8_constant(binary: &[u8], value: u8) -> Vec<usize> {
    binary.iter().enumerate()
        .filter_map(|(i, &byte)| if byte == value { Some(i) } else { None })
        .collect()
}

fn scan_u16_constant(binary: &[u8], value: u16) -> Vec<usize> {
    let target_bytes = value.to_le_bytes();
    let mut positions = Vec::new();
    
    for i in 0..binary.len().saturating_sub(1) {
        if binary[i] == target_bytes[0] && binary[i + 1] == target_bytes[1] {
            positions.push(i);
        }
    }
    
    positions
}

fn scan_u32_constant(binary: &[u8], value: u32) -> Vec<usize> {
    let target_bytes = value.to_le_bytes();
    let mut positions = Vec::new();
    
    for i in 0..binary.len().saturating_sub(3) {
        if binary[i..i + 4] == target_bytes {
            positions.push(i);
        }
    }
    
    positions
}

fn scan_u64_constant(binary: &[u8], value: u64) -> Vec<usize> {
    let target_bytes = value.to_le_bytes();
    let mut positions = Vec::new();
    
    for i in 0..binary.len().saturating_sub(7) {
        if binary[i..i + 8] == target_bytes {
            positions.push(i);
        }
    }
    
    positions
}

fn get_surrounding_bytes(binary: &[u8], pos: usize, context_size: usize) -> Vec<u8> {
    let start = pos.saturating_sub(context_size / 2);
    let end = (pos + context_size / 2).min(binary.len());
    binary[start..end].to_vec()
}

fn classify_u8_usage(binary: &[u8], pos: usize, value: u8) -> String {
    match value {
        2 => "Binary flag/boolean".to_string(),
        3 => "Enum variant count".to_string(),
        5 => "Array size/loop bound".to_string(),
        7 => "Week days/bit flags".to_string(),
        31 => "Bit mask (2^5-1)".to_string(),
        _ => format!("u8 constant {}", value),
    }
}

fn classify_u16_usage(binary: &[u8], pos: usize, value: u16) -> String {
    format!("u16 constant {} (likely array size/offset)", value)
}

fn classify_u32_usage(binary: &[u8], pos: usize, value: u32) -> String {
    format!("u32 constant {} (likely hash/id/size)", value)
}

fn classify_u64_usage(binary: &[u8], pos: usize, value: u64) -> String {
    format!("u64 constant {} (likely pointer/large size)", value)
}

fn is_prime(n: u32) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    
    for i in (3..=(n as f64).sqrt() as u32).step_by(2) {
        if n % i == 0 { return false; }
    }
    true
}

fn display_prime_byte_analysis(analysis: &[PrimeByteAnalysis]) {
    println!("\n🔍 DIRECT PRIME CONSTANT ANALYSIS:");
    println!("=================================");
    
    // Sort by total occurrences
    let mut sorted = analysis.to_vec();
    sorted.sort_by(|a, b| {
        let total_a = a.u8_count + a.u16_count + a.u32_count + a.u64_count;
        let total_b = b.u8_count + b.u16_count + b.u32_count + b.u64_count;
        total_b.cmp(&total_a)
    });
    
    println!("\n📊 TOP PRIME CONSTANTS FOUND:");
    for item in sorted.iter().take(15) {
        let total = item.u8_count + item.u16_count + item.u32_count + item.u64_count;
        if total > 0 {
            println!("Prime {}: {} total (u8:{} u16:{} u32:{} u64:{})", 
                     item.prime, total, item.u8_count, item.u16_count, item.u32_count, item.u64_count);
            
            // Show first few contexts
            for context in item.likely_constants.iter().take(3) {
                println!("   @{:06x} ({}bit): {}", context.position, context.int_size, context.likely_usage);
            }
        }
    }
    
    println!("\n📈 PRIME DISTRIBUTION BY SIZE:");
    let mut u8_total = 0;
    let mut u16_total = 0;
    let mut u32_total = 0;
    let mut u64_total = 0;
    
    for item in analysis {
        u8_total += item.u8_count;
        u16_total += item.u16_count;
        u32_total += item.u32_count;
        u64_total += item.u64_count;
    }
    
    println!("   u8 constants:  {}", u8_total);
    println!("   u16 constants: {}", u16_total);
    println!("   u32 constants: {}", u32_total);
    println!("   u64 constants: {}", u64_total);
    println!("   Total: {}", u8_total + u16_total + u32_total + u64_total);
}

fn save_prime_byte_analysis(analysis: &[PrimeByteAnalysis]) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(analysis)?;
    std::fs::write("prime_byte_analysis.json", json)?;
    println!("\n💾 Byte analysis saved to prime_byte_analysis.json");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_prime_detection() {
        assert!(is_prime(2));
        assert!(is_prime(31));
        assert!(is_prime(71));
        assert!(!is_prime(4));
    }
    
    #[test]
    fn test_constant_injection() {
        let mut data = vec![0u8; 100];
        inject_u32_constant(&mut data, 10, 31);
        
        let positions = scan_u32_constant(&data, 31);
        assert_eq!(positions, vec![10]);
    }
    
    #[test]
    fn test_byte_scanning() {
        let data = vec![2, 3, 5, 7, 11, 13];
        let positions = scan_u8_constant(&data, 5);
        assert_eq!(positions, vec![2]);
    }
}
