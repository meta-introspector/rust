use std::collections::BTreeMap;

// First 8 primes: 2, 3, 5, 7, 11, 13, 17, 19
const FIRST_8_PRIMES: [u32; 8] = [2, 3, 5, 7, 11, 13, 17, 19];

macro_rules! mklattice {
    ($name:ident, $($element:ident),*) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        enum $name {
            $($element),*
        }

        impl $name {
            fn join(&self, other: &Self) -> Self {
                match (self, other) {
                    (a, b) if a == b => a.clone(),
                    _ => self.clone()
                }
            }
            
            fn meet(&self, other: &Self) -> Self {
                match (self, other) {
                    (a, b) if a == b => a.clone(),
                    _ => self.clone()
                }
            }
        }
    };
}

mklattice!(PrimeComplexity, 
    Empty,
    P2, P3, P5, P7, P11, P13, P17, P19,
    P2P3, P2P5, P3P5, P5P7, P7P11, P11P13, P13P17, P17P19,
    P2P3P5, P3P5P7, P5P7P11, P7P11P13, P11P13P17, P13P17P19,
    Full
);

#[derive(Debug)]
struct PrimeComplexityTable {
    entries: BTreeMap<u16, ComplexityEntry>,
}

#[derive(Debug, Clone)]
struct ComplexityEntry {
    index: u16,
    prime_set: Vec<u32>,
    complexity_level: u8,
    canonical_form: String,
    monster_bits: u64,
}

impl PrimeComplexityTable {
    fn new() -> Self {
        let mut table = Self {
            entries: BTreeMap::new(),
        };
        table.generate_from_primes();
        table
    }
    
    fn generate_from_primes(&mut self) {
        let mut index = 0u16;
        
        // Level 0: Empty
        self.add_entry(index, vec![], 0, "∅".to_string());
        index += 1;
        
        // Level 1: Single primes
        for &prime in &FIRST_8_PRIMES {
            let form = format!("P{}", prime);
            self.add_entry(index, vec![prime], 1, form);
            index += 1;
        }
        
        // Level 2: Prime pairs
        let pairs = [(2,3), (2,5), (3,5), (5,7), (7,11), (11,13), (13,17), (17,19)];
        for &(p1, p2) in &pairs {
            let form = format!("P{}P{}", p1, p2);
            self.add_entry(index, vec![p1, p2], 2, form);
            index += 1;
        }
        
        // Level 3: Prime triples
        let triples = [(2,3,5), (3,5,7), (5,7,11), (7,11,13), (11,13,17), (13,17,19)];
        for &(p1, p2, p3) in &triples {
            let form = format!("P{}P{}P{}", p1, p2, p3);
            self.add_entry(index, vec![p1, p2, p3], 3, form);
            index += 1;
        }
        
        // Level 8: Full set
        self.add_entry(index, FIRST_8_PRIMES.to_vec(), 8, "Full".to_string());
    }
    
    fn add_entry(&mut self, index: u16, primes: Vec<u32>, level: u8, form: String) {
        let monster_bits = self.calculate_monster_bits(&primes);
        
        let entry = ComplexityEntry {
            index,
            prime_set: primes,
            complexity_level: level,
            canonical_form: form,
            monster_bits,
        };
        
        self.entries.insert(index, entry);
    }
    
    fn calculate_monster_bits(&self, primes: &[u32]) -> u64 {
        let mut bits = 0u64;
        for &prime in primes {
            let bit_pos = match prime {
                2 => 0, 3 => 1, 5 => 2, 7 => 3,
                11 => 4, 13 => 5, 17 => 6, 19 => 7,
                _ => 0,
            };
            bits |= 1u64 << bit_pos;
        }
        bits
    }
    
    fn print_complexity_table(&self) {
        println!("═══════════════════════════════════════════════════════════════");
        println!("🔢 PRIME LATTICE COMPLEXITY TABLE (First 8 Primes)");
        println!("═══════════════════════════════════════════════════════════════");
        println!("Primes: 2, 3, 5, 7, 11, 13, 17, 19");
        println!("Total Entries: {}", self.entries.len());
        println!();
        
        let mut by_level: BTreeMap<u8, Vec<&ComplexityEntry>> = BTreeMap::new();
        for entry in self.entries.values() {
            by_level.entry(entry.complexity_level).or_default().push(entry);
        }
        
        for (level, entries) in &by_level {
            println!("┌─────────────────────────────────────────────────────────────┐");
            println!("│ COMPLEXITY LEVEL {}: {} entries                           │", level, entries.len());
            println!("└─────────────────────────────────────────────────────────────┘");
            
            for entry in entries {
                println!("  [{:03x}] {} → Monster: 0b{:08b} (0x{:02x})", 
                         entry.index, 
                         entry.canonical_form,
                         entry.monster_bits,
                         entry.monster_bits);
            }
            println!();
        }
        
        println!("═══════════════════════════════════════════════════════════════");
        println!("📊 COMPLEXITY DISTRIBUTION");
        println!("═══════════════════════════════════════════════════════════════");
        
        for (level, entries) in &by_level {
            let percentage = (entries.len() as f64 / self.entries.len() as f64) * 100.0;
            println!("Level {}: {:>2} entries ({:>5.1}%)", level, entries.len(), percentage);
        }
        
        println!("═══════════════════════════════════════════════════════════════");
    }
}

macro_rules! mklattice {
    ($name:ident, $($element:ident),*) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        enum $name {
            $($element),*
        }

        impl $name {
            fn join(&self, other: &Self) -> Self {
                match (self, other) {
                    (a, b) if a == b => a.clone(),
                    _ => self.clone()
                }
            }
            
            fn meet(&self, other: &Self) -> Self {
                match (self, other) {
                    (a, b) if a == b => a.clone(),
                    _ => self.clone()
                }
            }
        }
    };
}

fn main() {
    println!("🔢 GENERATING PRIME LATTICE COMPLEXITY TABLE");
    
    let table = PrimeComplexityTable::new();
    table.print_complexity_table();
    
    println!("✅ Prime lattice complexity table generated from first 8 primes!");
}
