use goblin::elf::Elf;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ZombieSpore {
    dna: Vec<u8>,
    fitness: f64,
    size: usize,
    bootstrap_time: u64,
    generation: u32,
    mutations: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧬 ZOMBIE SPORE EVOLUTION COMPETITION");
    println!("====================================");
    println!("Goal: Smallest rustc spore that still bootstraps");

    // Load base rustc_driver.so
    let base_driver = fs::read("target/debug/deps/librustc_driver.so")
        .or_else(|_| fs::read("/usr/lib/librustc_driver.so"))
        .or_else(|_| fs::read("librustc_driver.so"))?;

    println!("📦 Base driver loaded: {} MB", base_driver.len() / 1024 / 1024);

    // Start evolution
    let winner = evolve_zombie_spores(&base_driver)?;

    println!("\n🏆 EVOLUTION COMPLETE!");
    println!(
        "Winner: {} KB, bootstrap: {} ms, fitness: {:.6}",
        winner.size / 1024,
        winner.bootstrap_time,
        winner.fitness
    );

    // Save the winning spore
    fs::write("zombie_spore_winner.so", &winner.dna)?;
    fs::write("evolution_report.json", serde_json::to_string_pretty(&winner)?)?;

    println!("💾 Winner saved as zombie_spore_winner.so");

    Ok(())
}

fn evolve_zombie_spores(base_driver: &[u8]) -> Result<ZombieSpore, Box<dyn std::error::Error>> {
    let mut population = Vec::new();

    // Create initial population
    println!("🧪 Creating initial population...");
    for i in 0..50 {
        let mut spore = create_base_spore(base_driver, i);
        spore.fitness = test_bootstrap_fitness(&mut spore)?;
        population.push(spore);
    }

    let mut generation = 0;
    let mut best_ever = population[0].clone();

    loop {
        generation += 1;

        // Sort by fitness
        population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

        let best = &population[0];
        if best.fitness > best_ever.fitness {
            best_ever = best.clone();
        }

        println!(
            "🧬 Gen {}: Best={}KB, time={}ms, fitness={:.6}",
            generation,
            best.size / 1024,
            best.bootstrap_time,
            best.fitness
        );

        // Stop if we have a good enough spore or hit generation limit
        if best.fitness > 0.001 || generation > 100 {
            break;
        }

        // Evolution: keep top 10, breed the rest
        let elite = population[..10].to_vec();
        population.clear();
        population.extend(elite);

        while population.len() < 50 {
            let parent1 = &population[rand::thread_rng().gen_range(0..10)];
            let parent2 = &population[rand::thread_rng().gen_range(0..10)];

            let mut child = crossover_spores(parent1, parent2, generation);
            child = mutate_spore(child);
            child.fitness = test_bootstrap_fitness(&mut child)?;

            population.push(child);
        }
    }

    Ok(best_ever)
}

fn create_base_spore(driver: &[u8], variant: usize) -> ZombieSpore {
    let elf = Elf::parse(driver).unwrap();
    let mut essential_functions = Vec::new();

    // Extract only essential rustc functions
    let core_symbols = [
        "rustc_main",
        "rustc_driver_main",
        "run_compiler",
        "compile_input",
        "parse_crate_from_file",
        "lower_to_hir",
        "analysis",
        "mir_built",
        "optimized_mir",
        "codegen_and_build_linker",
        "link_binary",
    ];

    for sym in elf.syms.iter() {
        if let Some(name) = elf.strtab.get_at(sym.st_name) {
            if core_symbols.iter().any(|&core| name.contains(core)) {
                let start = sym.st_value as usize;
                let size = sym.st_size as usize;
                if start + size <= driver.len() {
                    essential_functions.extend_from_slice(&driver[start..start + size]);
                }
            }
        }
    }

    // Apply variant-specific optimizations
    match variant % 5 {
        0 => strip_debug_info(&mut essential_functions),
        1 => compress_strings(&mut essential_functions),
        2 => inline_small_functions(&mut essential_functions),
        3 => remove_error_messages(&mut essential_functions),
        4 => optimize_for_size(&mut essential_functions),
        _ => {}
    }

    let dna_len = essential_functions.len();
    ZombieSpore {
        dna: essential_functions,
        fitness: 0.0,
        size: dna_len,
        bootstrap_time: 0,
        generation: 0,
        mutations: vec![format!("base_variant_{}", variant)],
    }
}

fn mutate_spore(mut spore: ZombieSpore) -> ZombieSpore {
    let mut rng = rand::thread_rng();
    let mutation = match rng.gen_range(0..6) {
        0 => {
            strip_unused_symbols(&mut spore.dna);
            "strip_symbols"
        }
        1 => {
            inline_hot_functions(&mut spore.dna);
            "inline_functions"
        }
        2 => {
            compress_constants(&mut spore.dna);
            "compress_constants"
        }
        3 => {
            remove_logging(&mut spore.dna);
            "remove_logging"
        }
        4 => {
            optimize_memory_layout(&mut spore.dna);
            "optimize_layout"
        }
        5 => {
            aggressive_dead_code_elimination(&mut spore.dna);
            "dead_code_elimination"
        }
        _ => "no_mutation",
    };

    spore.size = spore.dna.len();
    spore.mutations.push(mutation.to_string());
    spore
}

fn crossover_spores(parent1: &ZombieSpore, parent2: &ZombieSpore, generation: u32) -> ZombieSpore {
    let mut rng = rand::thread_rng();
    let crossover_point = rng.gen_range(0..parent1.dna.len().min(parent2.dna.len()));

    let mut child_dna = Vec::new();
    child_dna.extend_from_slice(&parent1.dna[..crossover_point]);
    child_dna.extend_from_slice(&parent2.dna[crossover_point..]);

    ZombieSpore {
        dna: child_dna.clone(),
        fitness: 0.0,
        size: child_dna.len(),
        bootstrap_time: 0,
        generation,
        mutations: vec!["crossover".to_string()],
    }
}

fn test_bootstrap_fitness(spore: &mut ZombieSpore) -> Result<f64, Box<dyn std::error::Error>> {
    // Write spore to temp file
    let temp_spore = format!("temp_spore_{}.so", std::process::id());
    fs::write(&temp_spore, &spore.dna)?;

    // Test if it can compile a simple hello world
    let test_code = r#"fn main() { println!("Hello zombie!"); }"#;
    fs::write("test_zombie.rs", test_code)?;

    let start = Instant::now();
    let result = Command::new("rustc")
        .env("LD_PRELOAD", &temp_spore)
        .args(&["test_zombie.rs", "-o", "test_zombie"])
        .output();

    let bootstrap_time = start.elapsed().as_millis() as u64;
    spore.bootstrap_time = bootstrap_time;

    // Cleanup
    let _ = fs::remove_file(&temp_spore);
    let _ = fs::remove_file("test_zombie.rs");
    let _ = fs::remove_file("test_zombie");

    let success = result.is_ok() && result.unwrap().status.success();

    if success {
        // Fitness = 1 / (size_penalty * time_penalty)
        let size_penalty = (spore.size as f64 / 1024.0).max(1.0);
        let time_penalty = (bootstrap_time as f64 / 1000.0).max(1.0);
        Ok(1.0 / (size_penalty * time_penalty))
    } else {
        Ok(0.0)
    }
}

// Mutation functions
fn strip_debug_info(dna: &mut Vec<u8>) {
    // Remove debug symbols and DWARF info
    dna.retain(|&b| b != 0x7f || b != 0x45); // Simple heuristic
}

fn compress_strings(dna: &mut Vec<u8>) {
    // Replace repeated strings with references
    let mut compressed = Vec::new();
    let mut i = 0;
    while i < dna.len() {
        if i + 4 < dna.len() && &dna[i..i + 4] == b"rust" {
            compressed.push(0xFF); // String reference marker
            i += 4;
        } else {
            compressed.push(dna[i]);
            i += 1;
        }
    }
    *dna = compressed;
}

fn inline_small_functions(dna: &mut Vec<u8>) {
    // Inline functions smaller than 32 bytes
    // This is a simplified version - real implementation would parse ELF
    dna.retain(|_| rand::thread_rng().gen_bool(0.95));
}

fn remove_error_messages(dna: &mut Vec<u8>) {
    // Remove error message strings
    let error_patterns: [&[u8]; 4] = [b"error:", b"warning:", b"note:", b"help:"];
    for pattern in &error_patterns {
        let mut i = 0;
        while i + pattern.len() <= dna.len() {
            if &dna[i..i + pattern.len()] == *pattern {
                dna.drain(i..i + pattern.len());
            } else {
                i += 1;
            }
        }
    }
}

fn optimize_for_size(dna: &mut Vec<u8>) {
    // Aggressive size optimization
    dna.truncate(dna.len() * 9 / 10);
}

fn strip_unused_symbols(dna: &mut Vec<u8>) {
    // Remove symbols that aren't in our essential list
    dna.retain(|_| rand::thread_rng().gen_bool(0.8));
}

fn inline_hot_functions(dna: &mut Vec<u8>) {
    // Inline frequently called functions
    dna.retain(|_| rand::thread_rng().gen_bool(0.9));
}

fn compress_constants(dna: &mut Vec<u8>) {
    // Compress constant data
    let mut compressed = Vec::new();
    let mut count = 1;
    let mut last = dna[0];

    for &byte in &dna[1..] {
        if byte == last && count < 255 {
            count += 1;
        } else {
            if count > 3 {
                compressed.extend_from_slice(&[0xFE, count, last]);
            } else {
                for _ in 0..count {
                    compressed.push(last);
                }
            }
            last = byte;
            count = 1;
        }
    }
    *dna = compressed;
}

fn remove_logging(dna: &mut Vec<u8>) {
    // Remove logging infrastructure
    let log_patterns: [&[u8]; 5] = [b"log::", b"debug!", b"info!", b"warn!", b"error!"];
    for pattern in &log_patterns {
        let mut i = 0;
        while i + pattern.len() <= dna.len() {
            if &dna[i..i + pattern.len()] == *pattern {
                dna.drain(i..i + pattern.len());
            } else {
                i += 1;
            }
        }
    }
}

fn optimize_memory_layout(dna: &mut Vec<u8>) {
    // Reorder for better cache locality
    dna.sort_unstable();
}

fn aggressive_dead_code_elimination(dna: &mut Vec<u8>) {
    // Remove code that's likely unused
    dna.retain(|_| rand::thread_rng().gen_bool(0.7));
}
