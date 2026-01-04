use std::collections::HashMap;
use std::fs;

// Monster Group: 2^46 × 3^20 × 5^9 × 7^6 × 11^2 × 13^3 × 17 × 19 × 23 × 29 × 31 × 41 × 47 × 59 × 71
const MONSTER_EXPONENTS: [(u64, u64); 15] = [
    (2, 46),   // Most complex: 2^46 tuples
    (3, 20),   // 3^20 ternary choices
    (5, 9),    // 5^9 pentagon structures
    (7, 6),    // 7^6 weekly cycles
    (11, 2),   // 11^2 decimal pairs
    (13, 3),   // 13^3 baker's dozen
    (17, 1),   // 17^1 simple prime
    (19, 1),   // 19^1 simple prime
    (23, 1),   // 23^1 simple prime
    (29, 1),   // 29^1 simple prime
    (31, 1),   // 31^1 simple prime
    (41, 1),   // 41^1 simple prime
    (47, 1),   // 47^1 simple prime
    (59, 1),   // 59^1 simple prime
    (71, 1),   // 71^1 metaprogramming
];

fn main() {
    println!("🎯 MONSTER COMPLEXITY MATCHER: Assigning by complexity");
    
    let graph_data = fs::read_to_string("usage_eigenmatrix.json")
        .expect("Failed to read eigenmatrix");
    
    // Calculate complexity of each node
    let node_complexities = calculate_complexities(&graph_data);
    
    // Sort by complexity (most complex first)
    let mut sorted_nodes: Vec<_> = node_complexities.into_iter().collect();
    sorted_nodes.sort_by(|a, b| b.1.cmp(&a.1));
    
    // Assign to Monster exponents by complexity rank
    let assignments = assign_by_complexity(&sorted_nodes);
    
    // Generate the assignments
    for (prime, exponent) in MONSTER_EXPONENTS {
        let count = prime.pow(exponent as u32);
        println!("Prime {}^{} = {} objects assigned", prime, exponent, count);
        
        if let Some(nodes) = assignments.get(&prime) {
            let filename = format!("monster_{}_{}.json", prime, exponent);
            let assignment_data = serde_json::json!({
                "prime": prime,
                "exponent": exponent,
                "total_objects": count,
                "complexity_rank": get_complexity_rank(prime, exponent),
                "nodes": nodes
            });
            
            fs::write(&filename, serde_json::to_string_pretty(&assignment_data).unwrap())
                .expect("Failed to write assignment");
        }
    }
    
    println!("✅ Complexity-matched Monster assignments complete!");
}

fn calculate_complexities(graph_data: &str) -> HashMap<String, u64> {
    let mut complexities = HashMap::new();
    
    for line in graph_data.lines() {
        if line.contains("DefId") || line.contains("function") {
            let complexity = calculate_node_complexity(line);
            complexities.insert(line.to_string(), complexity);
        }
    }
    
    complexities
}

fn calculate_node_complexity(line: &str) -> u64 {
    let mut complexity = 0u64;
    
    // Count complexity indicators
    complexity += line.matches("::").len() as u64 * 10;      // Namespace depth
    complexity += line.matches("<").len() as u64 * 20;       // Generic parameters
    complexity += line.matches("impl").len() as u64 * 30;    // Implementation blocks
    complexity += line.matches("macro").len() as u64 * 50;   // Macro complexity
    complexity += line.matches("unsafe").len() as u64 * 100; // Unsafe complexity
    complexity += line.len() as u64;                         // Basic length
    
    complexity
}

fn assign_by_complexity(sorted_nodes: &[(String, u64)]) -> HashMap<u64, Vec<String>> {
    let mut assignments: HashMap<u64, Vec<String>> = HashMap::new();
    let mut node_index = 0;
    
    // Assign most complex objects to highest exponents
    for (prime, exponent) in MONSTER_EXPONENTS {
        let mut prime_nodes = Vec::new();
        let count = std::cmp::min(prime.pow(exponent as u32) as usize, 1000); // Cap for practicality
        
        for _ in 0..count {
            if node_index < sorted_nodes.len() {
                prime_nodes.push(sorted_nodes[node_index].0.clone());
                node_index += 1;
            }
        }
        
        assignments.insert(prime, prime_nodes);
    }
    
    assignments
}

fn get_complexity_rank(prime: u64, exponent: u64) -> u64 {
    // Higher exponents = higher complexity rank
    match exponent {
        46 => 1,  // Most complex: 2^46
        20 => 2,  // 3^20
        9 => 3,   // 5^9
        6 => 4,   // 7^6
        3 => 5,   // 13^3
        2 => 6,   // 11^2
        1 => 7,   // All single exponents (simplest)
        _ => 8,
    }
}
