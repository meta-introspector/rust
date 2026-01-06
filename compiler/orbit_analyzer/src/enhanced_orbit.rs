use std::collections::{HashMap, HashSet};
use serde_json::Value;
use std::fs;

#[derive(Debug, Clone)]
pub struct EnhancedOrbitAnalyzer {
    symbol_connections: HashMap<String, Vec<(String, u64)>>,
    orbital_frequencies: HashMap<String, u64>,
    harmonic_patterns: Vec<HarmonicPattern>,
    data_path: String,
}

#[derive(Debug, Clone)]
pub struct HarmonicPattern {
    symbols: Vec<String>,
    resonance_frequency: f64,
    pattern_strength: f64,
    cycle_type: String,
}

impl EnhancedOrbitAnalyzer {
    pub fn new() -> Self {
        Self {
            symbol_connections: HashMap::new(),
            orbital_frequencies: HashMap::new(),
            harmonic_patterns: Vec::new(),
            data_path: "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/compiler_tools_data".to_string(),
        }
    }
    
    pub fn analyze_enhanced_orbits(&mut self) {
        println!("🌌 === ENHANCED ORBITAL ANALYSIS ===");
        
        self.extract_symbol_relationships();
        self.identify_orbital_centers();
        self.find_harmonic_resonances();
        self.detect_orbital_cycles();
        self.generate_enhanced_report();
    }
    
    fn extract_symbol_relationships(&mut self) {
        println!("🔗 Extracting symbol relationships...");
        
        // Process all data files
        self.process_all_enhanced_data();
        
        println!("✅ Extracted relationships for {} symbols", self.symbol_connections.len());
    }
    
    fn process_all_enhanced_data(&mut self) {
        // Single files
        let single_files = vec![
            "enhanced_rustc_metadata_literals.json",
            "enhanced_rustc_target_constants.json",
            "enhanced_rustc_mir_transform_shim_async_destructor_ctor.json",
        ];
        
        for file in single_files {
            self.process_enhanced_file(file);
        }
        
        // Chunked files
        self.process_chunked_data("enhanced_rustc_query_impl_literals_chunk_");
        self.process_chunked_data("enhanced_rustc_lint_literals_chunk_");
    }
    
    fn process_enhanced_file(&mut self, filename: &str) {
        let file_path = format!("{}/{}", self.data_path, filename);
        
        if let Ok(content) = fs::read_to_string(&file_path) {
            if let Ok(json) = serde_json::from_str::<Value>(&content) {
                self.extract_relationships_from_json(&json);
            }
        }
    }
    
    fn process_chunked_data(&mut self, chunk_prefix: &str) {
        if let Ok(entries) = fs::read_dir(&self.data_path) {
            for entry in entries.flatten() {
                let filename = entry.file_name().to_string_lossy().to_string();
                if filename.starts_with(chunk_prefix) && filename.ends_with(".json") {
                    self.process_enhanced_file(&filename);
                }
            }
        }
    }
    
    fn extract_relationships_from_json(&mut self, json: &Value) {
        // Extract usage relationships
        if let Some(usages) = json["usages"].as_array() {
            for usage in usages {
                if let (Some(original), Some(hir_mapping)) = (
                    usage["original_symbol"].as_str(),
                    usage["hir_mapping"].as_str()
                ) {
                    if let Some(count) = usage["usage_count"].as_u64() {
                        // Track symbol connections
                        self.symbol_connections
                            .entry(original.to_string())
                            .or_default()
                            .push((hir_mapping.to_string(), count));
                        
                        // Track frequencies
                        *self.orbital_frequencies.entry(original.to_string()).or_insert(0) += count;
                    }
                }
            }
        }
        
        // Extract HIR node relationships
        if let Some(hir_nodes) = json["hir_nodes"].as_array() {
            for node in hir_nodes {
                if let (Some(symbol), Some(hir_type)) = (
                    node["symbol"].as_str(),
                    node["hir_type"].as_str()
                ) {
                    if let Some(count) = node["usage_count"].as_u64() {
                        self.symbol_connections
                            .entry(symbol.to_string())
                            .or_default()
                            .push((hir_type.to_string(), count));
                    }
                }
            }
        }
    }
    
    fn identify_orbital_centers(&mut self) {
        println!("🎯 Identifying orbital centers...");
        
        // Find symbols with highest connection counts (orbital centers)
        let mut connection_counts: Vec<_> = self.symbol_connections
            .iter()
            .map(|(symbol, connections)| (symbol.clone(), connections.len()))
            .collect();
        
        connection_counts.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
        
        println!("🌟 Top 10 Orbital Centers:");
        for (i, (symbol, count)) in connection_counts.iter().take(10).enumerate() {
            let frequency = self.orbital_frequencies.get(symbol).unwrap_or(&0);
            println!("  {}. {} ({} connections, {} frequency)", i + 1, symbol, count, frequency);
        }
    }
    
    fn find_harmonic_resonances(&mut self) {
        println!("🎵 Finding harmonic resonances...");
        
        // Look for patterns in symbol frequencies
        let mut frequency_groups: HashMap<u64, Vec<String>> = HashMap::new();
        
        for (symbol, &freq) in &self.orbital_frequencies {
            frequency_groups.entry(freq).or_default().push(symbol.clone());
        }
        
        // Find resonant frequencies (frequencies with multiple symbols)
        for (freq, symbols) in frequency_groups {
            if symbols.len() >= 3 && freq > 1 {
                let pattern = HarmonicPattern {
                    symbols: symbols.clone(),
                    resonance_frequency: freq as f64,
                    pattern_strength: (symbols.len() as f64) * (freq as f64).sqrt(),
                    cycle_type: "frequency_resonance".to_string(),
                };
                self.harmonic_patterns.push(pattern);
            }
        }
        
        // Sort by pattern strength
        self.harmonic_patterns.sort_by(|a, b| b.pattern_strength.partial_cmp(&a.pattern_strength).unwrap());
        
        println!("✅ Found {} harmonic patterns", self.harmonic_patterns.len());
    }
    
    fn detect_orbital_cycles(&mut self) {
        println!("🔄 Detecting orbital cycles...");
        
        // Find symbols that connect back to themselves through other symbols
        let mut cycle_patterns = Vec::new();
        
        for (symbol, connections) in &self.symbol_connections {
            // Look for 2-hop cycles
            for (connected_symbol, _) in connections {
                if let Some(second_connections) = self.symbol_connections.get(connected_symbol) {
                    for (second_symbol, _) in second_connections {
                        if second_symbol == symbol {
                            // Found a 2-cycle
                            let cycle_strength = self.orbital_frequencies.get(symbol).unwrap_or(&0) +
                                               self.orbital_frequencies.get(connected_symbol).unwrap_or(&0);
                            
                            let pattern = HarmonicPattern {
                                symbols: vec![symbol.clone(), connected_symbol.clone()],
                                resonance_frequency: cycle_strength as f64,
                                pattern_strength: cycle_strength as f64,
                                cycle_type: "orbital_cycle_2".to_string(),
                            };
                            cycle_patterns.push(pattern);
                        }
                    }
                }
            }
        }
        
        // Add cycle patterns to harmonic patterns
        cycle_patterns.sort_by(|a, b| b.pattern_strength.partial_cmp(&a.pattern_strength).unwrap());
        self.harmonic_patterns.extend(cycle_patterns.into_iter().take(20));
        
        println!("✅ Orbital cycle detection complete");
    }
    
    fn generate_enhanced_report(&self) {
        println!("\n🌌 === ENHANCED ORBITAL ANALYSIS REPORT ===");
        
        println!("📊 Enhanced Dataset Statistics:");
        println!("  • Total symbols analyzed: {}", self.symbol_connections.len());
        println!("  • Total orbital frequencies: {}", self.orbital_frequencies.len());
        println!("  • Harmonic patterns found: {}", self.harmonic_patterns.len());
        
        // Calculate total connections
        let total_connections: usize = self.symbol_connections.values()
            .map(|connections| connections.len())
            .sum();
        println!("  • Total symbol connections: {}", total_connections);
        
        println!("\n🎵 Top 10 Harmonic Patterns:");
        for (i, pattern) in self.harmonic_patterns.iter().take(10).enumerate() {
            println!("  {}. Type: {}, Strength: {:.2}, Frequency: {:.1}", 
                    i + 1, pattern.cycle_type, pattern.pattern_strength, pattern.resonance_frequency);
            
            if pattern.symbols.len() <= 5 {
                println!("     Symbols: {}", pattern.symbols.join(" ↔ "));
            } else {
                println!("     Symbols: {} (showing first 3: {})", 
                        pattern.symbols.len(), 
                        pattern.symbols.iter().take(3).cloned().collect::<Vec<_>>().join(", "));
            }
        }
        
        println!("\n🔥 Most Connected Orbital Centers:");
        let mut sorted_connections: Vec<_> = self.symbol_connections.iter().collect();
        sorted_connections.sort_by_key(|(_, connections)| std::cmp::Reverse(connections.len()));
        
        for (i, (symbol, connections)) in sorted_connections.iter().take(5).enumerate() {
            let frequency = self.orbital_frequencies.get(*symbol).unwrap_or(&0);
            println!("  {}. {} ({} connections, {} orbital frequency)", 
                    i + 1, symbol, connections.len(), frequency);
            
            // Show top connections
            let mut sorted_conns: Vec<(String, u64)> = connections.iter().cloned().collect();
            sorted_conns.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
            
            println!("     Top connections:");
            for (conn_symbol, count) in sorted_conns.iter().take(3) {
                println!("       → {} (strength: {})", conn_symbol, count);
            }
        }
        
        println!("\n🎯 Orbital Analysis Summary:");
        let resonance_patterns = self.harmonic_patterns.iter()
            .filter(|p| p.cycle_type == "frequency_resonance")
            .count();
        let cycle_patterns = self.harmonic_patterns.iter()
            .filter(|p| p.cycle_type.contains("cycle"))
            .count();
        
        println!("  • Frequency resonance patterns: {}", resonance_patterns);
        println!("  • Orbital cycle patterns: {}", cycle_patterns);
        
        let avg_connections = total_connections as f64 / self.symbol_connections.len() as f64;
        println!("  • Average connections per symbol: {:.2}", avg_connections);
        
        println!("\n✅ Enhanced orbital analysis complete!");
    }
}

fn main() {
    let mut analyzer = EnhancedOrbitAnalyzer::new();
    analyzer.analyze_enhanced_orbits();
}
