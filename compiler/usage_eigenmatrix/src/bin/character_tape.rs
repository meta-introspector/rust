use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

struct RustCharacter {
    emoji_codebook: HashMap<String, String>,
    execution_history: Vec<String>,
    current_generation: usize,
}

impl RustCharacter {
    fn new() -> Self {
        Self {
            emoji_codebook: HashMap::new(),
            execution_history: Vec::new(),
            current_generation: 0,
        }
    }

    fn load_emoji_codebook(&mut self) -> Result<()> {
        let nodes = self.load_and_rank_nodes()?;
        self.emoji_codebook = self.generate_emoji_codes(&nodes);
        Ok(())
    }

    fn apply_matrix_to_itself(&mut self) -> String {
        self.current_generation += 1;
        let mut trace_vector = String::new();
        
        // Start with the most powerful eigenvalue
        trace_vector.push_str("👑"); // false - the ultimate truth
        
        // Apply matrix operations in order of power
        let operations = vec![
            ("⚡", "🔍"),     // display -> v1 (formatting chain)
            ("🔘", "🔢"),     // wildcard -> metadata (pattern to trace)
            ("📝", "🔄"),     // META -> CALLSITE (tracing flow)
            ("💾", "⚖️"),     // comparison -> iteration (logic flow)
            ("🎨", "⚠️"),     // next -> None (iterator end)
            ("🔧", "🪄"),     // Some -> eq (value comparison)
            ("📦", "🎮"),     // fields -> TRACE (data to observation)
        ];
        
        for (input, output) in operations {
            trace_vector.push_str(input);
            trace_vector.push_str("→");
            trace_vector.push_str(output);
            trace_vector.push_str(" ");
        }
        
        // Add generation marker
        trace_vector.push_str(&format!("#{}", self.current_generation));
        
        self.execution_history.push(trace_vector.clone());
        trace_vector
    }

    fn evolve_character(&mut self, iterations: usize) -> Vec<String> {
        let mut traces = Vec::new();
        
        for i in 0..iterations {
            let trace = self.apply_matrix_to_itself();
            traces.push(trace);
            
            // Character evolution: add complexity over time
            if i % 3 == 0 {
                self.add_complexity_mutation();
            }
        }
        
        traces
    }

    fn add_complexity_mutation(&mut self) {
        // Simulate system evolution by adding new patterns
        let mutations = vec!["🌟", "🔮", "⭐", "💫", "✨"];
        let mutation = mutations[self.current_generation % mutations.len()];
        
        if let Some(last_trace) = self.execution_history.last_mut() {
            last_trace.push_str(mutation);
        }
    }

    fn analyze_character(&self) -> String {
        let mut analysis = String::from("🎭 Rust Character Analysis\n\n");
        
        // Trace frequency analysis
        let mut emoji_freq = HashMap::new();
        for trace in &self.execution_history {
            for emoji in trace.chars() {
                if emoji.is_ascii_graphic() || emoji as u32 > 127 {
                    *emoji_freq.entry(emoji).or_insert(0) += 1;
                }
            }
        }
        
        let mut sorted_freq: Vec<_> = emoji_freq.into_iter().collect();
        sorted_freq.sort_by(|a, b| b.1.cmp(&a.1));
        
        analysis.push_str("🔥 Most Frequent Character Traits:\n");
        for (emoji, count) in sorted_freq.iter().take(10) {
            analysis.push_str(&format!("  {} appears {} times\n", emoji, count));
        }
        
        // Pattern analysis
        analysis.push_str("\n🔄 Execution Patterns:\n");
        for (i, trace) in self.execution_history.iter().enumerate() {
            let display_trace = if trace.chars().count() > 30 { 
                trace.chars().take(30).collect::<String>() + "..." 
            } else { 
                trace.clone() 
            };
            analysis.push_str(&format!("  Gen {}: {}\n", i+1, display_trace));
        }
        
        // Character summary
        analysis.push_str(&format!("\n📊 Character Summary:\n"));
        analysis.push_str(&format!("  Total generations: {}\n", self.current_generation));
        analysis.push_str(&format!("  Average trace length: {:.1}\n", 
            self.execution_history.iter().map(|t| t.len()).sum::<usize>() as f64 / self.execution_history.len() as f64));
        
        analysis
    }

    fn load_and_rank_nodes(&self) -> Result<Vec<(String, usize)>> {
        let mut node_weights = HashMap::new();
        let usage_dir = "../../usage_data";
        
        for entry in fs::read_dir(usage_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content = fs::read_to_string(&path)?;
                let data: Value = serde_json::from_str(&content)?;
                
                if let Some(usages) = data["usages"].as_array() {
                    for usage_obj in usages {
                        let used_id = usage_obj["used_def_id"].as_str().unwrap_or("unknown");
                        let count = usage_obj["usage_count"].as_u64().unwrap_or(1) as usize;
                        
                        *node_weights.entry(used_id.to_string()).or_insert(0) += count;
                    }
                }
            }
        }
        
        let mut sorted: Vec<_> = node_weights.into_iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));
        Ok(sorted.into_iter().take(16).collect()) // Top 16 for single emojis
    }

    fn generate_emoji_codes(&self, nodes: &[(String, usize)]) -> HashMap<String, String> {
        let base_emojis = vec!["👑", "⚡", "🔍", "🔘", "🔢", "📝", "🔄", "💾", "⚖️", "🎨", "⚠️", "🔧", "🪄", "📦", "🎮", "❓"];
        
        nodes.iter().enumerate().map(|(i, (node, _))| {
            (node.clone(), base_emojis[i].to_string())
        }).collect()
    }
}

fn main() -> Result<()> {
    println!("🎭 Rust Character Tape Generator");
    
    let mut rust_character = RustCharacter::new();
    rust_character.load_emoji_codebook()?;
    
    println!("\n🔄 Evolving Rust Character over 10 generations...");
    let traces = rust_character.evolve_character(10);
    
    println!("\n📼 Character Tape (Execution Traces):");
    for (i, trace) in traces.iter().enumerate() {
        println!("  Gen {}: {}", i+1, trace);
    }
    
    let analysis = rust_character.analyze_character();
    println!("\n{}", analysis);
    
    // Save character history
    let mut history = String::from("# 🎭 Rust Character History\n\n");
    history.push_str("## Execution Traces Over Time\n\n");
    
    for (i, trace) in traces.iter().enumerate() {
        history.push_str(&format!("**Generation {}**: `{}`\n\n", i+1, trace));
    }
    
    history.push_str("## Character Analysis\n\n");
    history.push_str(&analysis.replace('\n', "\n\n"));
    
    fs::write("rust_character_tape.md", history)?;
    println!("\n💾 Character tape saved to rust_character_tape.md");
    
    println!("\n🎯 Rust's Character Revealed:");
    println!("  The eigenmatrix applied to itself creates a living emoji tape");
    println!("  Each generation shows Rust's computational DNA in action");
    println!("  The character emerges from the pattern of all executions over time");
    
    Ok(())
}
