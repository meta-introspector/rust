// vectorized_usage_analyzer.rs - CPU-based 8D vector analysis with SIMD optimization
use std::fs;
use serde_json::Value;
use flate2::read::GzDecoder;
use std::io::Read;
use rayon::prelude::*;

// 8D vector representing symbol usage across 8 levels
#[derive(Clone, Copy, Debug, Default)]
struct SymbolVector {
    levels: [f32; 8],  // [L0, L1, L2, L3, L4, L5, L6, L7]
}

impl SymbolVector {
    fn new(direct_usage: f32) -> Self {
        let mut levels = [0.0; 8];
        levels[0] = direct_usage;
        Self { levels }
    }
    
    fn total_usage(&self) -> f32 {
        self.levels.iter().sum()
    }
    
    fn dot_product(&self, other: &SymbolVector) -> f32 {
        self.levels.iter().zip(other.levels.iter())
            .map(|(a, b)| a * b)
            .sum()
    }
    
    fn magnitude(&self) -> f32 {
        self.levels.iter().map(|x| x * x).sum::<f32>().sqrt()
    }
    
    fn cosine_similarity(&self, other: &SymbolVector) -> f32 {
        let dot = self.dot_product(other);
        let mag_product = self.magnitude() * other.magnitude();
        if mag_product > 0.0 {
            dot / mag_product
        } else {
            0.0
        }
    }
}

struct VectorizedAnalyzer {
    symbols: Vec<SymbolVector>,
    symbol_names: Vec<String>,
    dependency_matrix: Vec<Vec<f32>>,
}

impl VectorizedAnalyzer {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        println!("🔗 Loading usage data for vectorized analysis...");
        
        // Load usage levels data
        let file = fs::File::open("usage_levels_8.json.gz")?;
        let mut decoder = GzDecoder::new(file);
        let mut contents = String::new();
        decoder.read_to_string(&mut contents)?;
        
        let json: Value = serde_json::from_str(&contents)?;
        
        let mut symbols = Vec::new();
        let mut symbol_names = Vec::new();
        
        if let Value::Object(analysis) = &json {
            for (symbol_key, symbol_data) in analysis {
                symbol_names.push(symbol_key.clone());
                
                if let Value::Object(obj) = symbol_data {
                    let direct_usage = obj.get("direct_usage")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0) as f32;
                    
                    let mut vector = SymbolVector::new(direct_usage);
                    
                    // Extract level usages from existing analysis
                    if let Some(levels) = obj.get("levels").and_then(|v| v.as_array()) {
                        for (i, level_data) in levels.iter().enumerate() {
                            if i + 1 < 8 {
                                vector.levels[i + 1] = level_data.get("total_usage")
                                    .and_then(|v| v.as_u64())
                                    .unwrap_or(0) as f32;
                            }
                        }
                    }
                    
                    symbols.push(vector);
                }
            }
        }
        
        println!("📊 Loaded {} symbols as 8D vectors", symbols.len());
        
        // Create dependency matrix based on usage correlation
        let num_symbols = symbols.len();
        let mut dependency_matrix = vec![vec![0.0; num_symbols]; num_symbols];
        
        println!("🔄 Computing dependency correlations...");
        dependency_matrix.par_iter_mut().enumerate().for_each(|(i, row)| {
            for j in 0..num_symbols {
                if i != j {
                    // Use cosine similarity as dependency strength
                    row[j] = symbols[i].cosine_similarity(&symbols[j]);
                }
            }
        });
        
        println!("✅ Created {}x{} dependency correlation matrix", num_symbols, num_symbols);
        
        Ok(Self {
            symbols,
            symbol_names,
            dependency_matrix,
        })
    }
    
    fn analyze_clusters(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n🔍 Analyzing symbol clusters using 8D vectors...");
        
        // Find symbols with highest total usage
        let mut usage_ranking: Vec<(usize, f32)> = self.symbols.iter().enumerate()
            .map(|(i, vector)| (i, vector.total_usage()))
            .collect();
        usage_ranking.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        println!("\n🏆 Top 15 Symbols by Total Usage (All Levels):");
        for (rank, (idx, total)) in usage_ranking.iter().take(15).enumerate() {
            let vector = &self.symbols[*idx];
            println!("{}. {} (Total: {:.1})", rank + 1, self.symbol_names[*idx], total);
            println!("   Vector: [{:.1}, {:.1}, {:.1}, {:.1}, {:.1}, {:.1}, {:.1}, {:.1}]",
                    vector.levels[0], vector.levels[1], vector.levels[2], vector.levels[3],
                    vector.levels[4], vector.levels[5], vector.levels[6], vector.levels[7]);
        }
        
        // Find most similar symbol pairs
        println!("\n🔗 Most Similar Symbol Pairs (Cosine Similarity):");
        let mut similarities = Vec::new();
        
        for i in 0..self.symbols.len() {
            for j in (i+1)..self.symbols.len() {
                let similarity = self.symbols[i].cosine_similarity(&self.symbols[j]);
                if similarity > 0.1 { // Only significant similarities
                    similarities.push((i, j, similarity));
                }
            }
        }
        
        similarities.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        
        for (i, j, sim) in similarities.iter().take(10) {
            println!("  {:.3}: {} ↔ {}", sim, self.symbol_names[*i], self.symbol_names[*j]);
        }
        
        // Analyze level distribution
        println!("\n📊 Usage Distribution Across Levels:");
        let mut level_totals = [0.0; 8];
        let mut level_counts = [0; 8];
        
        for vector in &self.symbols {
            for (level, &usage) in vector.levels.iter().enumerate() {
                if usage > 0.0 {
                    level_totals[level] += usage;
                    level_counts[level] += 1;
                }
            }
        }
        
        for level in 0..8 {
            let avg = if level_counts[level] > 0 {
                level_totals[level] / level_counts[level] as f32
            } else {
                0.0
            };
            println!("  Level {}: {} symbols, {:.1} total usage, {:.1} avg usage",
                    level, level_counts[level], level_totals[level], avg);
        }
        
        // Find symbols with unique usage patterns
        println!("\n🎯 Symbols with Unique Usage Patterns:");
        let mut pattern_uniqueness: Vec<(usize, f32)> = self.symbols.iter().enumerate()
            .map(|(i, vector)| {
                // Calculate how different this symbol is from all others
                let avg_similarity: f32 = self.dependency_matrix[i].iter().sum::<f32>() / self.symbols.len() as f32;
                (i, 1.0 - avg_similarity) // Higher score = more unique
            })
            .collect();
        
        pattern_uniqueness.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        for (rank, (idx, uniqueness)) in pattern_uniqueness.iter().take(10).enumerate() {
            let vector = &self.symbols[*idx];
            println!("{}. {} (Uniqueness: {:.3})", rank + 1, self.symbol_names[*idx], uniqueness);
            println!("   Pattern: [{:.1}, {:.1}, {:.1}, {:.1}, {:.1}, {:.1}, {:.1}, {:.1}]",
                    vector.levels[0], vector.levels[1], vector.levels[2], vector.levels[3],
                    vector.levels[4], vector.levels[5], vector.levels[6], vector.levels[7]);
        }
        
        Ok(())
    }
    
    fn find_compilation_order(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n🎯 Optimal Compilation Order Analysis:");
        
        // Group symbols by their primary usage level
        let mut level_groups: Vec<Vec<usize>> = vec![Vec::new(); 8];
        
        for (i, vector) in self.symbols.iter().enumerate() {
            // Find the level with maximum usage for this symbol
            let max_level = vector.levels.iter().enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .map(|(level, _)| level)
                .unwrap_or(0);
            
            level_groups[max_level].push(i);
        }
        
        println!("\n📋 Compilation Priority Groups:");
        for (level, group) in level_groups.iter().enumerate() {
            if !group.is_empty() {
                println!("  Priority {} (Level {} dominant): {} symbols", 
                        level + 1, level, group.len());
                
                // Show top 5 symbols in this group
                let mut group_with_usage: Vec<(usize, f32)> = group.iter()
                    .map(|&idx| (idx, self.symbols[idx].total_usage()))
                    .collect();
                group_with_usage.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                
                for (rank, (idx, usage)) in group_with_usage.iter().take(5).enumerate() {
                    println!("    {}. {} (Total: {:.1})", rank + 1, self.symbol_names[*idx], usage);
                }
                
                if group.len() > 5 {
                    println!("    ... and {} more", group.len() - 5);
                }
            }
        }
        
        Ok(())
    }
    
    fn save_vectorized_results(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n💾 Saving vectorized analysis results...");
        
        // Build level statistics first
        let mut level_stats = Vec::new();
        for level in 0..8 {
            let level_usages: Vec<f32> = self.symbols.iter()
                .map(|v| v.levels[level])
                .filter(|&x| x > 0.0)
                .collect();
            
            level_stats.push(serde_json::json!({
                "level": level,
                "active_symbols": level_usages.len(),
                "total_usage": level_usages.iter().sum::<f32>(),
                "max_usage": level_usages.iter().fold(0.0f32, |a, &b| a.max(b)),
                "avg_usage": if level_usages.is_empty() { 0.0 } else { 
                    level_usages.iter().sum::<f32>() / level_usages.len() as f32 
                }
            }));
        }
        
        let results = serde_json::json!({
            "vectorized_analysis": true,
            "total_symbols": self.symbols.len(),
            "analysis_timestamp": chrono::Utc::now().to_rfc3339(),
            "symbols": self.symbols.iter().enumerate().map(|(i, vector)| {
                serde_json::json!({
                    "name": self.symbol_names[i],
                    "vector": vector.levels,
                    "total_usage": vector.total_usage(),
                    "magnitude": vector.magnitude(),
                    "primary_level": vector.levels.iter().enumerate()
                        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                        .map(|(level, _)| level)
                        .unwrap_or(0)
                })
            }).collect::<Vec<_>>(),
            "level_statistics": level_stats
        });
        
        // Save compressed results
        let compressed_data = {
            use flate2::write::GzEncoder;
            use flate2::Compression;
            use std::io::Write;
            
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(results.to_string().as_bytes())?;
            encoder.finish()?
        };
        
        fs::write("vectorized_usage_analysis.json.gz", &compressed_data)?;
        println!("✅ Results saved to: vectorized_usage_analysis.json.gz ({} KB)", 
                compressed_data.len() / 1024);
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Vectorized 8D Usage Analysis");
    
    let analyzer = VectorizedAnalyzer::new()?;
    
    analyzer.analyze_clusters()?;
    analyzer.find_compilation_order()?;
    analyzer.save_vectorized_results()?;
    
    println!("\n✅ Vectorized analysis complete!");
    
    Ok(())
}
