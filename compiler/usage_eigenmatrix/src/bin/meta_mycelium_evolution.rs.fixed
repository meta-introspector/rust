use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde_json::Value;

/// Meta-Mycelium Evolution System
/// Each usage_data file becomes a spore in our growing substrate database
/// The system evolves by applying DWIM fixes to its own codebase

#[derive(Debug, Clone)]
struct Spore {
    file_path: String,
    defid_count: u32,
    usage_patterns: Vec<String>,
    complexity_score: f64,
    connections: Vec<String>, // Connected spores
    evolution_stage: EvolutionStage,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum EvolutionStage {
    Dormant,     // Raw usage data
    Germinating, // Being processed
    Growing,     // Generating patterns
    Mature,      // Ready for cross-pollination
    Reproducing, // Creating new spores
}

#[derive(Debug)]
struct MetaMycelium {
    spores: HashMap<String, Spore>,
    mycelium_network: HashMap<String, Vec<String>>, // Spore connections
    substrate_health: f64,
    evolution_cycles: u32,
    dwim_fixes_generated: Vec<String>,
}

impl MetaMycelium {
    fn new() -> Self {
        Self {
            spores: HashMap::new(),
            mycelium_network: HashMap::new(),
            substrate_health: 0.0,
            evolution_cycles: 0,
            dwim_fixes_generated: Vec::new(),
        }
    }
    
    fn inoculate_substrate(&mut self) {
        println!("🍄 Inoculating substrate with usage_data spores...");
        
        let usage_dir = "../../usage_data";
        let mut spores_created = 0;
        
        if let Ok(entries) = fs::read_dir(usage_dir) {
            for entry in entries.flatten().take(100) { // Sample for demo
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".json") {
                        let spore = self.create_spore_from_file(&entry.path().display().to_string());
                        self.spores.insert(name.to_string(), spore);
                        spores_created += 1;
                    }
                }
            }
        }
        
        println!("  Created {} spores in the mycelium network", spores_created);
        self.calculate_substrate_health();
    }
    
    fn create_spore_from_file(&self, filepath: &str) -> Spore {
        let mut spore = Spore {
            file_path: filepath.to_string(),
            defid_count: 0,
            usage_patterns: Vec::new(),
            complexity_score: 0.0,
            connections: Vec::new(),
            evolution_stage: EvolutionStage::Dormant,
        };
        
        // Extract spore data from usage file
        if let Ok(content) = fs::read_to_string(filepath) {
            if let Ok(json) = serde_json::from_str::<Value>(&content) {
                if let Some(usages) = json["usages"].as_array() {
                    spore.defid_count = usages.len() as u32;
                    
                    for usage in usages.iter().take(5) {
                        if let Some(usage_str) = usage["usage"].as_str() {
                            spore.usage_patterns.push(usage_str.to_string());
                        }
                    }
                    
                    // Calculate complexity based on usage diversity
                    spore.complexity_score = (spore.defid_count as f64).ln() * 
                                           (spore.usage_patterns.len() as f64);
                }
            }
        }
        
        spore
    }
    
    fn evolve_cycle(&mut self) {
        println!("🌱 Evolution cycle {} beginning...", self.evolution_cycles + 1);
        
        // Stage 1: Germinate dormant spores
        self.germinate_spores();
        
        // Stage 2: Grow connections between spores
        self.grow_mycelium_network();
        
        // Stage 3: Cross-pollinate mature spores
        self.cross_pollinate();
        
        // Stage 4: Generate DWIM fixes from evolved patterns
        self.generate_dwim_fixes();
        
        // Stage 5: Apply fixes to our own codebase
        self.apply_fixes_to_substrate();
        
        self.evolution_cycles += 1;
        self.calculate_substrate_health();
        
        println!("  Evolution cycle complete. Substrate health: {:.2}", self.substrate_health);
    }
    
    fn germinate_spores(&mut self) {
        let mut germinated = 0;
        
        for spore in self.spores.values_mut() {
            if spore.evolution_stage == EvolutionStage::Dormant && spore.complexity_score > 5.0 {
                spore.evolution_stage = EvolutionStage::Germinating;
                germinated += 1;
            }
        }
        
        println!("  🌱 {} spores germinated", germinated);
    }
    
    fn grow_mycelium_network(&mut self) {
        let mut connections_formed = 0;
        
        let spore_names: Vec<String> = self.spores.keys().cloned().collect();
        
        for i in 0..spore_names.len() {
            for j in i+1..spore_names.len() {
                let spore1_name = &spore_names[i];
                let spore2_name = &spore_names[j];
                
                if let (Some(spore1), Some(spore2)) = (
                    self.spores.get(spore1_name),
                    self.spores.get(spore2_name)
                ) {
                    if self.spores_can_connect(spore1, spore2) {
                        // Form bidirectional connection
                        self.mycelium_network.entry(spore1_name.clone())
                            .or_insert_with(Vec::new)
                            .push(spore2_name.clone());
                        
                        self.mycelium_network.entry(spore2_name.clone())
                            .or_insert_with(Vec::new)
                            .push(spore1_name.clone());
                        
                        connections_formed += 1;
                    }
                }
            }
        }
        
        println!("  🕸️  {} mycelium connections formed", connections_formed);
    }
    
    fn spores_can_connect(&self, spore1: &Spore, spore2: &Spore) -> bool {
        // Spores connect if they share similar usage patterns
        let shared_patterns = spore1.usage_patterns.iter()
            .filter(|p1| spore2.usage_patterns.iter().any(|p2| self.patterns_similar(p1, p2)))
            .count();
        
        shared_patterns > 0 && 
        (spore1.complexity_score - spore2.complexity_score).abs() < 10.0
    }
    
    fn patterns_similar(&self, pattern1: &str, pattern2: &str) -> bool {
        // Simple similarity check - could be enhanced
        pattern1.len() > 10 && pattern2.len() > 10 && 
        (pattern1.contains("::") && pattern2.contains("::"))
    }
    
    fn cross_pollinate(&mut self) {
        let mut pollinated = 0;
        
        for (spore_name, connections) in &self.mycelium_network {
            if let Some(spore) = self.spores.get_mut(spore_name) {
                if spore.evolution_stage == EvolutionStage::Germinating && connections.len() > 2 {
                    spore.evolution_stage = EvolutionStage::Growing;
                    spore.connections = connections.clone();
                    pollinated += 1;
                }
            }
        }
        
        println!("  🌸 {} spores cross-pollinated", pollinated);
    }
    
    fn generate_dwim_fixes(&mut self) {
        let mut fixes_generated = 0;
        
        // Collect spores that need processing
        let mut spores_to_process = Vec::new();
        for (name, spore) in &self.spores {
            if spore.evolution_stage == EvolutionStage::Growing {
                spores_to_process.push((name.clone(), spore.clone()));
            }
        }
        
        // Process collected spores
        for (name, spore) in spores_to_process {
            let fixes = self.extract_fixes_from_spore(&spore);
            self.dwim_fixes_generated.extend(fixes);
            fixes_generated += 1;
            
            // Update evolution stage
            if let Some(spore_mut) = self.spores.get_mut(&name) {
                spore_mut.evolution_stage = EvolutionStage::Mature;
            }
        }
        
        println!("  🔧 {} DWIM fixes generated from mature spores", fixes_generated);
    }
    
    fn extract_fixes_from_spore(&self, spore: &Spore) -> Vec<String> {
        let mut fixes = Vec::new();
        
        for pattern in &spore.usage_patterns {
            // Extract common usage patterns that could become fixes
            if pattern.contains("::len()") {
                fixes.push("Replace `.len` with `.len()`".to_string());
            }
            
            if pattern.contains("println!") {
                fixes.push("Use `println!` macro instead of `println`".to_string());
            }
            
            if pattern.contains("Debug") {
                fixes.push("Add `#[derive(Debug)]` for debug formatting".to_string());
            }
            
            if pattern.contains("Clone") {
                fixes.push("Add `#[derive(Clone)]` for cloning support".to_string());
            }
        }
        
        fixes
    }
    
    fn apply_fixes_to_substrate(&mut self) {
        println!("  🩹 Applying evolved fixes to our own codebase...");
        
        // Apply DWIM fixes to our own source files
        let our_files = [
            "./src/bin/syn_prime_analyzer.rs",
            "./src/bin/usage_convergence_analyzer.rs",
            "./src/bin/monster_group_homotopy.rs",
        ];
        
        let mut fixes_applied = 0;
        
        for file_path in &our_files {
            if Path::new(file_path).exists() {
                if let Ok(content) = fs::read_to_string(file_path) {
                    let mut fixed_content = content.clone();
                    let mut file_modified = false;
                    
                    // Apply evolved fixes
                    for fix in &self.dwim_fixes_generated {
                        if fix.contains("Replace `.len` with `.len()`") {
                            if fixed_content.contains(".len") && !fixed_content.contains(".len()") {
                                fixed_content = fixed_content.replace(".len", ".len()");
                                file_modified = true;
                            }
                        }
                        
                        if fix.contains("Add `#[derive(Debug)]`") {
                            if fixed_content.contains("struct ") && !fixed_content.contains("#[derive(Debug)]") {
                                // Simple heuristic - could be more sophisticated
                                if let Some(struct_pos) = fixed_content.find("struct ") {
                                    let before = &fixed_content[..struct_pos];
                                    let after = &fixed_content[struct_pos..];
                                    if !before.ends_with("#[derive(Debug)]\n") {
                                        fixed_content = format!("{}#[derive(Debug)]\n{}", before, after);
                                        file_modified = true;
                                    }
                                }
                            }
                        }
                    }
                    
                    if file_modified {
                        // Save evolved version
                        let evolved_path = format!("{}.evolved", file_path);
                        fs::write(&evolved_path, fixed_content).unwrap();
                        fixes_applied += 1;
                        println!("    ✅ Evolved: {} -> {}", file_path, evolved_path);
                    }
                }
            }
        }
        
        println!("  🧬 {} files evolved through mycelium substrate", fixes_applied);
    }
    
    fn calculate_substrate_health(&mut self) {
        let total_spores = self.spores.len() as f64;
        let mature_spores = self.spores.values()
            .filter(|s| matches!(s.evolution_stage, EvolutionStage::Mature | EvolutionStage::Reproducing))
            .count() as f64;
        
        let connection_density = self.mycelium_network.values()
            .map(|connections| connections.len())
            .sum::<usize>() as f64 / (total_spores * 2.0); // Normalize
        
        self.substrate_health = (mature_spores / total_spores) * 0.7 + connection_density * 0.3;
    }
    
    fn generate_evolution_report(&self) -> String {
        let mut report = String::new();
        report.push_str("# Meta-Mycelium Evolution Report\n\n");
        
        report.push_str("## Substrate Status\n");
        report.push_str(&format!("- Evolution cycles: {}\n", self.evolution_cycles));
        report.push_str(&format!("- Substrate health: {:.2}\n", self.substrate_health));
        report.push_str(&format!("- Total spores: {}\n", self.spores.len()));
        report.push_str(&format!("- Mycelium connections: {}\n", 
                                self.mycelium_network.values().map(|v| v.len()).sum::<usize>()));
        
        report.push_str("\n## Evolution Stages\n");
        let mut stage_counts = HashMap::new();
        for spore in self.spores.values() {
            *stage_counts.entry(&spore.evolution_stage).or_insert(0) += 1;
        }
        
        for (stage, count) in stage_counts {
            report.push_str(&format!("- {:?}: {} spores\n", stage, count));
        }
        
        report.push_str("\n## Generated DWIM Fixes\n");
        for (i, fix) in self.dwim_fixes_generated.iter().enumerate() {
            report.push_str(&format!("{}. {}\n", i + 1, fix));
        }
        
        report.push_str("\n## Mycelium Network Health\n");
        let avg_connections = if !self.mycelium_network.is_empty() {
            self.mycelium_network.values().map(|v| v.len()).sum::<usize>() as f64 / 
            self.mycelium_network.len() as f64
        } else {
            0.0
        };
        report.push_str(&format!("- Average connections per spore: {:.1}\n", avg_connections));
        
        report
    }
}

fn main() {
    println!("🍄 Meta-Mycelium Evolution System");
    println!("=================================");
    println!("Growing intelligent substrate from usage_data spores...");
    
    let mut mycelium = MetaMycelium::new();
    
    // Inoculate substrate with spores
    mycelium.inoculate_substrate();
    
    // Run evolution cycles
    for cycle in 1..=3 {
        println!("\n🌱 EVOLUTION CYCLE {}:", cycle);
        println!("===================");
        mycelium.evolve_cycle();
    }
    
    println!("\n🧬 FINAL SUBSTRATE STATE:");
    println!("========================");
    println!("Substrate health: {:.2}", mycelium.substrate_health);
    println!("Evolution cycles: {}", mycelium.evolution_cycles);
    println!("DWIM fixes generated: {}", mycelium.dwim_fixes_generated.len());
    
    // Generate evolution report
    let report = mycelium.generate_evolution_report();
    fs::write("meta_mycelium_evolution_report.md", report).unwrap();
    
    println!("\n📁 Evolution report saved to: meta_mycelium_evolution_report.md");
    println!("🎉 Meta-mycelium substrate successfully evolved!");
    println!("🍄 The system has grown from its own usage patterns!");
    println!("🧬 Each spore contributes to the collective intelligence!");
}
