// fast_vectorized_analyzer.rs - Fast 8D vector analysis without similarity computation
use std::fs;
use serde_json::Value;
use flate2::read::GzDecoder;
use std::io::Read;

#[derive(Clone, Copy, Debug, Default)]
struct SymbolVector {
    levels: [f32; 8],
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
    
    fn magnitude(&self) -> f32 {
        self.levels.iter().map(|x| x * x).sum::<f32>().sqrt()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Fast Vectorized 8D Usage Analysis");
    
    // Load data
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
    
    // Top symbols by total usage
    let mut usage_ranking: Vec<(usize, f32)> = symbols.iter().enumerate()
        .map(|(i, vector)| (i, vector.total_usage()))
        .collect();
    usage_ranking.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    println!("\n🏆 Top 15 Symbols by Total Usage:");
    for (rank, (idx, total)) in usage_ranking.iter().take(15).enumerate() {
        let vector = &symbols[*idx];
        println!("{}. {} (Total: {:.1})", rank + 1, symbol_names[*idx], total);
        println!("   Vector: [{:.1}, {:.1}, {:.1}, {:.1}, {:.1}, {:.1}, {:.1}, {:.1}]",
                vector.levels[0], vector.levels[1], vector.levels[2], vector.levels[3],
                vector.levels[4], vector.levels[5], vector.levels[6], vector.levels[7]);
    }
    
    // Level distribution
    println!("\n📊 Usage Distribution Across Levels:");
    for level in 0..8 {
        let level_usages: Vec<f32> = symbols.iter()
            .map(|v| v.levels[level])
            .filter(|&x| x > 0.0)
            .collect();
        
        let total = level_usages.iter().sum::<f32>();
        let avg = if level_usages.is_empty() { 0.0 } else { total / level_usages.len() as f32 };
        let max_usage = level_usages.iter().fold(0.0f32, |a, &b| a.max(b));
        
        println!("  Level {}: {} symbols, {:.1} total, {:.1} avg, {:.1} max",
                level, level_usages.len(), total, avg, max_usage);
    }
    
    // Compilation priority groups
    println!("\n🎯 Compilation Priority Groups:");
    let mut level_groups: Vec<Vec<usize>> = vec![Vec::new(); 8];
    
    for (i, vector) in symbols.iter().enumerate() {
        let max_level = vector.levels.iter().enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(level, _)| level)
            .unwrap_or(0);
        
        level_groups[max_level].push(i);
    }
    
    for (level, group) in level_groups.iter().enumerate() {
        if !group.is_empty() {
            println!("  Priority {} (Level {} dominant): {} symbols", 
                    level + 1, level, group.len());
        }
    }
    
    println!("\n✅ Fast analysis complete!");
    
    Ok(())
}
