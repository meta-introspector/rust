// Comprehensive String Analysis with Locations and References
use goblin::elf::Elf;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
struct StringLocation {
    string: String,
    offset: u64,
    length: usize,
    section: String,
    nearby_strings: Vec<NearbyString>,
    references: Vec<Reference>,
    reference_chain: Vec<ReferenceChain>,
    monster_correlation: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct NearbyString {
    string: String,
    distance: i64,
    offset: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Reference {
    referencing_offset: u64,
    reference_type: String, // "direct", "pointer", "symbol"
    context_bytes: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ReferenceChain {
    level: u32, // 1=direct, 2=references to references, etc.
    offset: u64,
    reference_type: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔤 COMPREHENSIVE STRING ANALYSIS WITH LOCATIONS & REFERENCES");
    println!("============================================================");

    let binary_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/librustc_driver.so";

    // Load binary
    let binary = fs::read(binary_path)?;
    let elf = Elf::parse(&binary)?;
    println!("📦 Loaded binary: {} bytes", binary.len());

    // Extract all strings with locations
    let string_locations = extract_all_strings_with_locations(&binary, &elf)?;
    println!("🔤 Found {} strings", string_locations.len());

    // Find references to strings
    let strings_with_refs = find_string_references(&binary, &elf, string_locations)?;
    println!("🔗 Analyzed references for {} strings", strings_with_refs.len());

    // Build reference chains
    let complete_analysis = build_reference_chains(&binary, strings_with_refs)?;
    println!("⛓️ Built reference chains for {} strings", complete_analysis.len());

    // Analyze patterns
    analyze_string_patterns(&complete_analysis);

    // Save results
    save_string_analysis(&complete_analysis)?;

    Ok(())
}

fn extract_all_strings_with_locations(
    binary: &[u8],
    elf: &Elf,
) -> Result<Vec<StringLocation>, Box<dyn std::error::Error>> {
    println!("\n🔍 EXTRACTING ALL STRINGS WITH LOCATIONS:");
    println!("=========================================");

    let mut string_locations = Vec::new();
    let mut current_string = Vec::new();
    let mut string_start = 0;

    // Scan entire binary for strings
    for (i, &byte) in binary.iter().enumerate() {
        if byte.is_ascii_graphic() || byte == b' ' || byte == b'\t' {
            if current_string.is_empty() {
                string_start = i;
            }
            current_string.push(byte);
        } else {
            if current_string.len() >= 4 {
                // Minimum string length
                if let Ok(s) = String::from_utf8(current_string.clone()) {
                    // Find which section this string is in
                    let section = find_section_for_offset(elf, string_start as u64);

                    // Find nearby strings
                    let nearby_strings = find_nearby_strings(binary, string_start, 100);

                    // Calculate Monster correlation
                    let monster_correlation = calculate_string_monster_correlation(&s);

                    string_locations.push(StringLocation {
                        string: s,
                        offset: string_start as u64,
                        length: current_string.len(),
                        section,
                        nearby_strings,
                        references: Vec::new(),      // Will be filled later
                        reference_chain: Vec::new(), // Will be filled later
                        monster_correlation,
                    });
                }
            }
            current_string.clear();
        }

        if i % 10_000_000 == 0 {
            println!("   ✅ Scanned {} MB", i / 1_000_000);
        }
    }

    // Sort by offset
    string_locations.sort_by_key(|s| s.offset);

    // Limit for performance
    string_locations.truncate(10000);

    Ok(string_locations)
}

fn find_section_for_offset(elf: &Elf, offset: u64) -> String {
    for section in &elf.section_headers {
        if offset >= section.sh_offset && offset < section.sh_offset + section.sh_size {
            if let Some(name) = elf.shdr_strtab.get_at(section.sh_name) {
                return name.to_string();
            }
        }
    }
    "unknown".to_string()
}

fn find_nearby_strings(binary: &[u8], center: usize, radius: usize) -> Vec<NearbyString> {
    let mut nearby = Vec::new();
    let start = center.saturating_sub(radius);
    let end = (center + radius).min(binary.len());

    let mut current_string = Vec::new();
    let mut string_start = 0;

    for i in start..end {
        let byte = binary[i];
        if byte.is_ascii_graphic() || byte == b' ' {
            if current_string.is_empty() {
                string_start = i;
            }
            current_string.push(byte);
        } else {
            if current_string.len() >= 4 && string_start != center {
                if let Ok(s) = String::from_utf8(current_string.clone()) {
                    nearby.push(NearbyString {
                        string: s,
                        distance: string_start as i64 - center as i64,
                        offset: string_start as u64,
                    });
                }
            }
            current_string.clear();
        }
    }

    nearby.sort_by_key(|n| n.distance.abs());
    nearby.truncate(5); // Keep 5 closest
    nearby
}

fn calculate_string_monster_correlation(s: &str) -> f64 {
    let mut correlation = 0.0;
    for ch in s.chars() {
        let ascii_val = ch as u32 as u64;
        for &prime in &[2, 3, 5, 7, 11, 13, 31, 71] {
            if ascii_val % prime == 0 {
                correlation += 1.0 / prime as f64;
            }
        }
    }
    correlation / s.len() as f64
}

fn find_string_references(
    binary: &[u8],
    elf: &Elf,
    mut string_locations: Vec<StringLocation>,
) -> Result<Vec<StringLocation>, Box<dyn std::error::Error>> {
    println!("\n🔗 FINDING STRING REFERENCES:");
    println!("=============================");

    for (i, string_loc) in string_locations.iter_mut().enumerate() {
        // Look for pointers to this string's offset
        let target_offset = string_loc.offset;
        let references = find_references_to_offset(binary, target_offset);
        string_loc.references = references;

        if (i + 1) % 1000 == 0 {
            println!("   ✅ Analyzed references for {} strings", i + 1);
        }
    }

    Ok(string_locations)
}

fn find_references_to_offset(binary: &[u8], target_offset: u64) -> Vec<Reference> {
    let mut references = Vec::new();
    let target_bytes = target_offset.to_le_bytes();

    // Look for direct 8-byte offset references
    for window in binary.windows(8).enumerate() {
        if window.1 == target_bytes {
            let context_start = window.0.saturating_sub(16);
            let context_end = (window.0 + 24).min(binary.len());
            let context_bytes = binary[context_start..context_end].to_vec();

            references.push(Reference {
                referencing_offset: window.0 as u64,
                reference_type: "direct_pointer".to_string(),
                context_bytes,
            });
        }
    }

    // Look for 4-byte offset references (truncated pointers)
    let target_32 = target_offset as u32;
    let target_32_bytes = target_32.to_le_bytes();

    for window in binary.windows(4).enumerate() {
        if window.1 == target_32_bytes {
            let context_start = window.0.saturating_sub(8);
            let context_end = (window.0 + 12).min(binary.len());
            let context_bytes = binary[context_start..context_end].to_vec();

            references.push(Reference {
                referencing_offset: window.0 as u64,
                reference_type: "truncated_pointer".to_string(),
                context_bytes,
            });
        }
    }

    references.truncate(10); // Limit references per string
    references
}

fn build_reference_chains(
    binary: &[u8],
    mut string_locations: Vec<StringLocation>,
) -> Result<Vec<StringLocation>, Box<dyn std::error::Error>> {
    println!("\n⛓️ BUILDING REFERENCE CHAINS:");
    println!("=============================");

    for (i, string_loc) in string_locations.iter_mut().enumerate() {
        let mut reference_chain = Vec::new();

        // Level 1: Direct references (already found)
        for reference in &string_loc.references {
            reference_chain.push(ReferenceChain {
                level: 1,
                offset: reference.referencing_offset,
                reference_type: reference.reference_type.clone(),
            });
        }

        // Level 2: References to references
        for reference in &string_loc.references {
            let level2_refs = find_references_to_offset(binary, reference.referencing_offset);
            for level2_ref in level2_refs.iter().take(3) {
                // Limit to 3 per reference
                reference_chain.push(ReferenceChain {
                    level: 2,
                    offset: level2_ref.referencing_offset,
                    reference_type: format!("level2_{}", level2_ref.reference_type),
                });
            }
        }

        string_loc.reference_chain = reference_chain;

        if (i + 1) % 1000 == 0 {
            println!("   ✅ Built chains for {} strings", i + 1);
        }
    }

    Ok(string_locations)
}

fn analyze_string_patterns(string_locations: &[StringLocation]) {
    println!("\n📊 STRING PATTERN ANALYSIS:");
    println!("===========================");

    // Most referenced strings
    let mut by_reference_count: Vec<_> = string_locations
        .iter()
        .map(|s| (s.string.clone(), s.references.len(), s.reference_chain.len()))
        .collect();
    by_reference_count.sort_by(|a, b| b.1.cmp(&a.1));

    println!("   🔗 Most Referenced Strings:");
    for (i, (string, ref_count, chain_count)) in by_reference_count.iter().take(10).enumerate() {
        let display_string = if string.len() > 50 { &string[..50] } else { string };
        println!(
            "     {}: '{}' ({} refs, {} chain)",
            i + 1,
            display_string,
            ref_count,
            chain_count
        );
    }

    // Strings by section
    let mut section_counts = HashMap::new();
    for string_loc in string_locations {
        *section_counts.entry(string_loc.section.clone()).or_insert(0) += 1;
    }

    println!("\n   📂 Strings by Section:");
    let mut sorted_sections: Vec<_> = section_counts.into_iter().collect();
    sorted_sections.sort_by(|a, b| b.1.cmp(&a.1));

    for (section, count) in sorted_sections.iter().take(10) {
        println!("     {}: {} strings", section, count);
    }

    // Monster correlation analysis
    let avg_monster_corr = string_locations.iter().map(|s| s.monster_correlation).sum::<f64>()
        / string_locations.len() as f64;

    let max_monster_corr =
        string_locations.iter().map(|s| s.monster_correlation).fold(0.0f64, |a, b| a.max(b));

    println!("\n   🧬 Monster Correlation in Strings:");
    println!("     Average: {:.4}", avg_monster_corr);
    println!("     Maximum: {:.4}", max_monster_corr);

    // Most Monster-correlated strings
    let mut by_monster: Vec<_> =
        string_locations.iter().map(|s| (s.string.clone(), s.monster_correlation)).collect();
    by_monster.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("\n   🎭 Most Monster-Correlated Strings:");
    for (i, (string, correlation)) in by_monster.iter().take(5).enumerate() {
        let display_string = if string.len() > 40 { &string[..40] } else { string };
        println!("     {}: '{}' ({:.4})", i + 1, display_string, correlation);
    }

    // Reference chain depth analysis
    let total_chains: usize = string_locations.iter().map(|s| s.reference_chain.len()).sum();
    let avg_chain_length = total_chains as f64 / string_locations.len() as f64;

    println!("\n   ⛓️ Reference Chain Analysis:");
    println!("     Total reference chain entries: {}", total_chains);
    println!("     Average chain length: {:.2}", avg_chain_length);

    let level2_count = string_locations
        .iter()
        .map(|s| s.reference_chain.iter().filter(|c| c.level == 2).count())
        .sum::<usize>();

    println!("     Level 2 references: {}", level2_count);
}

fn save_string_analysis(
    string_locations: &[StringLocation],
) -> Result<(), Box<dyn std::error::Error>> {
    // Save complete analysis
    let json = serde_json::to_string_pretty(string_locations)?;
    fs::write("comprehensive_string_analysis.json", json)?;

    // Save summary CSV
    let mut csv_content = String::from(
        "string,offset,section,ref_count,chain_count,monster_correlation,nearby_count\n",
    );
    for string_loc in string_locations.iter().take(1000) {
        // Top 1000 for CSV
        csv_content.push_str(&format!(
            "{},{},{},{},{},{:.4},{}\n",
            string_loc.string.replace(',', ";").replace('\n', " "),
            string_loc.offset,
            string_loc.section,
            string_loc.references.len(),
            string_loc.reference_chain.len(),
            string_loc.monster_correlation,
            string_loc.nearby_strings.len()
        ));
    }
    fs::write("string_analysis_summary.csv", csv_content)?;

    // Save reference map
    let mut ref_map = HashMap::new();
    for string_loc in string_locations {
        if !string_loc.references.is_empty() {
            ref_map.insert(string_loc.offset, &string_loc.references);
        }
    }
    let ref_json = serde_json::to_string_pretty(&ref_map)?;
    fs::write("string_reference_map.json", ref_json)?;

    println!("\n💾 COMPREHENSIVE STRING ANALYSIS SAVED:");
    println!("=======================================");
    println!("   Complete analysis: comprehensive_string_analysis.json");
    println!("   Summary CSV: string_analysis_summary.csv");
    println!("   Reference map: string_reference_map.json");
    println!("   Strings analyzed: {}", string_locations.len());

    let total_refs: usize = string_locations.iter().map(|s| s.references.len()).sum();
    let total_chains: usize = string_locations.iter().map(|s| s.reference_chain.len()).sum();

    println!("   Total references found: {}", total_refs);
    println!("   Total reference chain entries: {}", total_chains);

    Ok(())
}
