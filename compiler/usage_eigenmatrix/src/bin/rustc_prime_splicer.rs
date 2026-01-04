use std::collections::HashMap;
use std::fs;

// Monster Group prime factorization: 2^46 × 3^20 × 5^9 × 7^6 × 11^2 × 13^3 × 17 × 19 × 23 × 29 × 31 × 41 × 47 × 59 × 71
const MONSTER_PRIMES: [u64; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];

fn main() {
    println!("🔪 RUSTC PRIME SPLICER: Splitting graph into 15 prime factors");
    
    // Load the rustc graph
    let graph_data = fs::read_to_string("usage_eigenmatrix.json")
        .expect("Failed to read eigenmatrix");
    
    // Parse and splice by prime factors
    let spliced_graphs = splice_by_primes(&graph_data);
    
    // Output each prime-factored subgraph
    for (prime, subgraph) in spliced_graphs {
        println!("Prime {}: {} nodes", prime, subgraph.len());
        
        // Save each prime slice
        let filename = format!("rustc_prime_{}.json", prime);
        fs::write(&filename, serde_json::to_string_pretty(&subgraph).unwrap())
            .expect("Failed to write prime slice");
    }
    
    println!("✅ Rustc graph spliced into 15 prime factors!");
}

fn splice_by_primes(graph_data: &str) -> HashMap<u64, Vec<String>> {
    let mut prime_slices: HashMap<u64, Vec<String>> = HashMap::new();
    
    // Initialize each prime slice
    for &prime in &MONSTER_PRIMES {
        prime_slices.insert(prime, Vec::new());
    }
    
    // Parse the graph and assign nodes to prime factors
    let lines: Vec<&str> = graph_data.lines().collect();
    
    for (index, line) in lines.iter().enumerate() {
        if line.contains("DefId") || line.contains("function") {
            // Hash the line to get a consistent prime assignment
            let hash = simple_hash(line);
            let prime_index = (hash % 15) as usize;
            let assigned_prime = MONSTER_PRIMES[prime_index];
            
            prime_slices.get_mut(&assigned_prime).unwrap().push(line.to_string());
        }
    }
    
    prime_slices
}

fn simple_hash(s: &str) -> u64 {
    s.bytes().fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64))
}
