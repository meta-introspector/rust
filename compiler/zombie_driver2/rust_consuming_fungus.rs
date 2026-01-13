use goblin::elf::Elf;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

macro_rules! mkfun {
    ("rand", float) => {
        |arg: usize| {
            let spore_hash = (arg as u64).wrapping_mul(0x9e3779b9).wrapping_add(0x85ebca6b);
            ((spore_hash >> 16) & 0xFFFF) as f64 / 65536.0
        }
    };
}

macro_rules! grow_hyphae {
    (rust_consumer => $target:expr) => {{
        println!("🍄 GROWING RUST-CONSUMING HYPHAE: {}", $target);
        RustFungus::new($target.to_string())
    }};

    (specialized => $nutrient:expr, $enzyme:expr) => {{
        println!("🧬 SPECIALIZED HYPHAE: {} enzyme for {} nutrients", $enzyme, $nutrient);
        Hyphae {
            nutrient_type: $nutrient.to_string(),
            enzyme_type: $enzyme.to_string(),
            digestive_strength: mkfun!("rand", float)($nutrient.len() + $enzyme.len()),
            spore_count: 0,
        }
    }};
}

macro_rules! digest_rust {
    ($fungus:expr, $binary:expr) => {{
        println!("🍽️ DIGESTING RUST BINARY: {}", $binary);
        $fungus.digest_binary($binary)
    }};
}

macro_rules! sporulate {
    ($fungus:expr, conditions: $conditions:expr) => {{
        if $conditions {
            println!("🌱 SPORULATION TRIGGERED - RELEASING SPORES");
            $fungus.release_spores()
        } else {
            println!("🍄 CONDITIONS NOT MET - CONTINUING GROWTH");
            0
        }
    }};
}

#[derive(Debug, Clone)]
struct Hyphae {
    nutrient_type: String,   // What type of Rust code it digests
    enzyme_type: String,     // How it breaks down the code
    digestive_strength: f64, // Efficiency of digestion
    spore_count: u32,        // Reproductive potential
}

struct RustFungus {
    species: String,
    hyphae_network: Vec<Hyphae>,
    consumed_binaries: Vec<String>,
    nutrient_reserves: f64,
    generation: u32,
}

impl RustFungus {
    fn new(species: String) -> Self {
        Self {
            species,
            hyphae_network: Vec::new(),
            consumed_binaries: Vec::new(),
            nutrient_reserves: 1.0,
            generation: 1,
        }
    }

    fn grow_specialized_hyphae(&mut self) {
        println!("\n🍄 GROWING SPECIALIZED HYPHAE - Generation {}", self.generation);

        // Grow different types of hyphae for different Rust nutrients
        let rust_nutrients = [
            ("opcodes", "disassembler"),
            ("symbols", "demangler"),
            ("strings", "parser"),
            ("elf_headers", "loader"),
            ("debug_info", "analyzer"),
            ("vtables", "polymorphic_resolver"),
            ("closures", "lambda_extractor"),
            ("async_state", "future_unwrapper"),
        ];

        for (nutrient, enzyme) in &rust_nutrients {
            let hyphae = grow_hyphae!(specialized => nutrient, enzyme);
            self.hyphae_network.push(hyphae);
        }

        println!("🧬 Grown {} specialized hyphae", self.hyphae_network.len());
    }

    fn digest_binary(&mut self, binary_path: &str) -> f64 {
        if !Path::new(binary_path).exists() {
            return 0.0;
        }

        let binary = match fs::read(binary_path) {
            Ok(data) => data,
            Err(_) => return 0.0,
        };

        let elf = match Elf::parse(&binary) {
            Ok(elf) => elf,
            Err(_) => return 0.0,
        };

        let mut total_nutrients = 0.0;

        // Each hyphae type digests different parts
        let mut updates = Vec::new();
        for (i, hyphae) in self.hyphae_network.iter().enumerate() {
            let nutrients = match hyphae.nutrient_type.as_str() {
                "opcodes" => self.digest_opcodes(&binary, &elf, hyphae),
                "symbols" => self.digest_symbols(&elf, hyphae),
                "strings" => self.digest_strings(&binary, hyphae),
                "elf_headers" => self.digest_headers(&elf, hyphae),
                "debug_info" => self.digest_debug_info(&elf, hyphae),
                "vtables" => self.digest_vtables(&binary, &elf, hyphae),
                "closures" => self.digest_closures(&elf, hyphae),
                "async_state" => self.digest_async_state(&elf, hyphae),
                _ => 0.0,
            };
            updates.push((i, nutrients));
        }
        
        for (i, nutrients) in updates {
            total_nutrients += nutrients;
            self.hyphae_network[i].spore_count += (nutrients * 10.0) as u32;
        }

        self.nutrient_reserves += total_nutrients;
        self.consumed_binaries.push(binary_path.to_string());

        println!(
            "🍽️ Digested {} - Extracted {:.2} nutrients",
            Path::new(binary_path).file_name().unwrap().to_str().unwrap(),
            total_nutrients
        );

        total_nutrients
    }

    fn digest_opcodes(&self, binary: &[u8], elf: &Elf, hyphae: &Hyphae) -> f64 {
        if let Some(text_section) = elf
            .section_headers
            .iter()
            .find(|sh| elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("") == ".text")
        {
            let text_start = text_section.sh_offset as usize;
            let text_size = text_section.sh_size as usize;

            if text_start + text_size <= binary.len() {
                let text_bytes = &binary[text_start..text_start + text_size];
                let opcode_diversity = text_bytes
                    .chunks_exact(4)
                    .map(|chunk| chunk[0])
                    .collect::<std::collections::HashSet<_>>()
                    .len();

                return (opcode_diversity as f64) * hyphae.digestive_strength * 0.01;
            }
        }
        0.0
    }

    fn digest_symbols(&self, elf: &Elf, hyphae: &Hyphae) -> f64 {
        let symbol_count = elf.syms.len();
        let rust_symbols = elf
            .syms
            .iter()
            .filter_map(|sym| elf.strtab.get_at(sym.st_name))
            .filter(|name| name.starts_with("_ZN"))
            .count();

        (rust_symbols as f64) * hyphae.digestive_strength * 0.001
    }

    fn digest_strings(&self, binary: &[u8], hyphae: &Hyphae) -> f64 {
        let rust_strings = binary
            .windows(4)
            .filter(|window| {
                window == b"rust" || window == b"Rust" || window == b"core" || window == b"std::"
            })
            .count();

        (rust_strings as f64) * hyphae.digestive_strength * 0.1
    }

    fn digest_headers(&self, elf: &Elf, hyphae: &Hyphae) -> f64 {
        (elf.section_headers.len() as f64) * hyphae.digestive_strength * 0.05
    }

    fn digest_debug_info(&self, elf: &Elf, hyphae: &Hyphae) -> f64 {
        let debug_sections = elf
            .section_headers
            .iter()
            .filter(|sh| {
                if let Some(name) = elf.shdr_strtab.get_at(sh.sh_name) {
                    name.starts_with(".debug")
                } else {
                    false
                }
            })
            .count();

        (debug_sections as f64) * hyphae.digestive_strength * 0.2
    }

    fn digest_vtables(&self, _binary: &[u8], elf: &Elf, hyphae: &Hyphae) -> f64 {
        let vtable_symbols = elf
            .syms
            .iter()
            .filter_map(|sym| elf.strtab.get_at(sym.st_name))
            .filter(|name| name.contains("vtable"))
            .count();

        (vtable_symbols as f64) * hyphae.digestive_strength * 0.3
    }

    fn digest_closures(&self, elf: &Elf, hyphae: &Hyphae) -> f64 {
        let closure_symbols = elf
            .syms
            .iter()
            .filter_map(|sym| elf.strtab.get_at(sym.st_name))
            .filter(|name| name.contains("closure"))
            .count();

        (closure_symbols as f64) * hyphae.digestive_strength * 0.25
    }

    fn digest_async_state(&self, elf: &Elf, hyphae: &Hyphae) -> f64 {
        let async_symbols = elf
            .syms
            .iter()
            .filter_map(|sym| elf.strtab.get_at(sym.st_name))
            .filter(|name| {
                name.contains("async") || name.contains("future") || name.contains("poll")
            })
            .count();

        (async_symbols as f64) * hyphae.digestive_strength * 0.4
    }

    fn release_spores(&mut self) -> u32 {
        let total_spores: u32 = self.hyphae_network.iter().map(|h| h.spore_count).sum();

        // Reset spore counts after release
        for hyphae in &mut self.hyphae_network {
            hyphae.spore_count = 0;
        }

        // Next generation is more specialized
        self.generation += 1;
        self.nutrient_reserves *= 0.5; // Sporulation costs energy

        println!("🌱 Released {} spores - Generation {} begins", total_spores, self.generation);
        total_spores
    }

    fn hunt_rust_binaries(&self, search_dirs: &[&str]) -> Vec<String> {
        let mut found_binaries = Vec::new();

        for dir in search_dirs {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(ext) = path.extension() {
                        if ext == "so" || ext == "rlib" || ext == "exe" {
                            if let Some(path_str) = path.to_str() {
                                found_binaries.push(path_str.to_string());
                            }
                        }
                    }
                }
            }
        }

        found_binaries
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🍄 RUST-CONSUMING FUNGUS SIMULATION");
    println!("===================================");

    let mut fungus = grow_hyphae!(rust_consumer => "RustDigester");

    // Initial growth phase
    fungus.grow_specialized_hyphae();

    // Hunt for Rust binaries to consume
    let search_dirs = ["target/debug", "target/release", "."];

    let available_binaries = fungus.hunt_rust_binaries(&search_dirs);
    println!("🔍 Found {} potential Rust binaries", available_binaries.len());

    // Consumption phase
    for binary in available_binaries.iter().take(5) {
        let nutrients = digest_rust!(fungus, binary);

        // Check sporulation conditions
        let spores = sporulate!(fungus, conditions: nutrients > 5.0);

        if spores > 100 {
            println!("🍄 FUNGAL BLOOM - Growing new specialized hyphae");
            fungus.grow_specialized_hyphae();
        }
    }

    // Final report
    println!("\n🍄 FUNGAL GROWTH REPORT:");
    println!("========================");
    println!("   Species: {}", fungus.species);
    println!("   Generation: {}", fungus.generation);
    println!("   Hyphae network size: {}", fungus.hyphae_network.len());
    println!("   Binaries consumed: {}", fungus.consumed_binaries.len());
    println!("   Nutrient reserves: {:.2}", fungus.nutrient_reserves);

    println!("\n🧬 SPECIALIZED HYPHAE:");
    for (i, hyphae) in fungus.hyphae_network.iter().enumerate() {
        println!(
            "   {}: {} -> {} (strength: {:.3}, spores: {})",
            i + 1,
            hyphae.nutrient_type,
            hyphae.enzyme_type,
            hyphae.digestive_strength,
            hyphae.spore_count
        );
    }

    println!("\n🌱 FUNGUS HAS LEARNED TO CONSUME RUST");
    println!("   Each hyphae specializes in different Rust components");
    println!("   System evolves through sporulation and specialization");

    Ok(())
}
