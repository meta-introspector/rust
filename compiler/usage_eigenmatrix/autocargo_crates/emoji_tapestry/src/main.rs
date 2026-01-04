// 🎭 MONSTER GROUP EMOJI TAPESTRY - THE HERO'S JOURNEY OF RUSTC MAIN()
// Mapping the trace of main() through all Monster Group layers as an emoji story

use std::collections::HashMap;
use serde_json::Value;
use std::fs;

struct EmojiTapestryWeaver {
    ast_patterns: HashMap<String, u32>,
    emoji_map: HashMap<String, String>,
    monster_layers: Vec<MonsterLayer>,
}

#[derive(Debug)]
struct MonsterLayer {
    name: String,
    base: u32,
    power: u32,
    emoji: String,
    patterns: Vec<(String, u32, String)>, // (pattern, count, emoji)
    story_fragment: String,
}

impl EmojiTapestryWeaver {
    fn new() -> Self {
        let mut emoji_map = HashMap::new();
        
        // Prime number canonical emojis
        emoji_map.insert("2".to_string(), "⚡".to_string());   // Binary lightning
        emoji_map.insert("3".to_string(), "🔺".to_string());   // Ternary triangle
        emoji_map.insert("5".to_string(), "⭐".to_string());   // Pentagonal star
        emoji_map.insert("7".to_string(), "🎭".to_string());   // Heptagonal mask
        emoji_map.insert("11".to_string(), "👥".to_string());  // Prime pairs
        emoji_map.insert("13".to_string(), "🥖".to_string());  // Baker's dozen
        emoji_map.insert("17".to_string(), "🌟".to_string());  // Bright star
        emoji_map.insert("19".to_string(), "🔮".to_string());  // Crystal ball
        emoji_map.insert("23".to_string(), "💎".to_string());  // Diamond
        emoji_map.insert("29".to_string(), "🌙".to_string());  // Crescent moon
        emoji_map.insert("31".to_string(), "🎯".to_string());  // Target
        emoji_map.insert("41".to_string(), "🚀".to_string());  // Rocket
        emoji_map.insert("47".to_string(), "🌊".to_string());  // Wave
        emoji_map.insert("59".to_string(), "🔥".to_string());  // Fire
        emoji_map.insert("71".to_string(), "👑".to_string());  // Crown (largest prime)
        
        // Pattern type emojis
        emoji_map.insert("main".to_string(), "🏛️".to_string());     // Main temple
        emoji_map.insert("call".to_string(), "📞".to_string());     // Function call
        emoji_map.insert("tcx".to_string(), "🧠".to_string());      // Type context brain
        emoji_map.insert("ty".to_string(), "🏗️".to_string());       // Type construction
        emoji_map.insert("err".to_string(), "⚠️".to_string());      // Error warning
        emoji_map.insert("param".to_string(), "📦".to_string());    // Parameter box
        emoji_map.insert("visitor".to_string(), "🚶".to_string());  // AST walker
        emoji_map.insert("collect".to_string(), "🗂️".to_string());  // Collection
        
        Self {
            ast_patterns: HashMap::new(),
            emoji_map,
            monster_layers: Vec::new(),
        }
    }
    
    fn load_patterns(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎭 Loading patterns for emoji tapestry weaving...");
        
        if let Ok(content) = fs::read_to_string("ast_patterns.json") {
            let patterns: Value = serde_json::from_str(&content)?;
            
            if let Some(patterns_obj) = patterns.as_object() {
                for (pattern, count) in patterns_obj {
                    if let Some(count_num) = count.as_u64() {
                        self.ast_patterns.insert(pattern.clone(), count_num as u32);
                    }
                }
            }
        }
        
        println!("✅ Loaded {} patterns for the tapestry", self.ast_patterns.len());
        Ok(())
    }
    
    fn weave_monster_layers(&mut self) {
        // Define the Monster Group layers with their story fragments
        let layer_configs = vec![
            (2, 46, "BINARY FOUNDATION", "⚡🏛️⚡", "The hero awakens in the binary realm, where all computation begins with lightning-fast 0s and 1s"),
            (3, 20, "TERNARY SPLITS", "🔺🌀🔺", "Three paths diverge in the code forest - the hero must choose wisely among the ternary branches"),
            (5, 9, "PENTAGONAL STARS", "⭐🌟⭐", "Five-pointed stars guide the hero through pentagonal symmetries of perfect balance"),
            (7, 6, "HEPTAGONAL MASKS", "🎭🎪🎭", "Seven sacred masks reveal the hidden group structures of the compiler's soul"),
            (11, 2, "PRIME PAIRS", "👥💫👥", "Twin primes stand guard at the gates, testing the hero's resolve with paired challenges"),
            (13, 3, "BAKER'S DOZEN", "🥖🍞🥖", "Thirteen loaves of wisdom nourish the hero for the trials ahead"),
        ];
        
        for (base, power, name, emoji_seq, story) in layer_configs {
            let mut layer_patterns = self.extract_layer_patterns(base, power);
            
            // Add emojis to patterns
            for (pattern, _count, emoji) in &mut layer_patterns {
                let pattern_emoji = self.get_pattern_emoji(pattern);
                *emoji = pattern_emoji;
            }
            
            self.monster_layers.push(MonsterLayer {
                name: name.to_string(),
                base,
                power,
                emoji: emoji_seq.to_string(),
                patterns: layer_patterns,
                story_fragment: story.to_string(),
            });
        }
        
        // Add singles layer
        self.weave_singles_layer();
    }
    
    fn extract_layer_patterns(&self, base: u32, power: u32) -> Vec<(String, u32, String)> {
        let mut patterns: Vec<(String, u32)> = self.ast_patterns
            .iter()
            .filter(|(pattern, _)| {
                pattern.len() == base as usize ||
                pattern.contains(&format!("_{}", base)) ||
                pattern.matches(',').count() == (base - 1) as usize ||
                pattern.matches('_').count() == (base - 1) as usize
            })
            .map(|(k, v)| (k.clone(), *v))
            .collect();
            
        patterns.sort_by(|a, b| b.1.cmp(&a.1));
        patterns.truncate(power as usize);
        
        patterns.into_iter()
            .map(|(pattern, count)| (pattern, count, String::new()))
            .collect()
    }
    
    fn weave_singles_layer(&mut self) {
        let singles = [17, 19, 23, 29, 31, 41, 47, 59, 71];
        let mut singles_patterns = Vec::new();
        
        for &prime in &singles {
            if let Some((pattern, count)) = self.find_best_prime_pattern(prime) {
                let emoji = self.emoji_map.get(&prime.to_string()).unwrap_or(&"✨".to_string()).clone();
                singles_patterns.push((format!("{}:{}", prime, pattern), count, emoji));
            }
        }
        
        self.monster_layers.push(MonsterLayer {
            name: "PRIME SINGLES".to_string(),
            base: 0,
            power: singles.len() as u32,
            emoji: "🌟🔮💎🌙🎯🚀🌊🔥👑".to_string(),
            patterns: singles_patterns,
            story_fragment: "Nine prime guardians emerge from the cosmic void, each wielding unique powers to complete the hero's transformation".to_string(),
        });
    }
    
    fn find_best_prime_pattern(&self, prime: u32) -> Option<(String, u32)> {
        self.ast_patterns
            .iter()
            .filter(|(pattern, _)| {
                pattern.len() == prime as usize ||
                pattern.contains(&format!("_{}", prime))
            })
            .max_by_key(|(_, count)| *count)
            .map(|(k, v)| (k.clone(), *v))
    }
    
    fn get_pattern_emoji(&self, pattern: &str) -> String {
        // Check for known pattern types
        if pattern.contains("main") { return "🏛️".to_string(); }
        if pattern.contains("call") { return "📞".to_string(); }
        if pattern.contains("tcx") { return "🧠".to_string(); }
        if pattern.contains("ty") { return "🏗️".to_string(); }
        if pattern.contains("err") { return "⚠️".to_string(); }
        if pattern.contains("param") { return "📦".to_string(); }
        if pattern.contains("visitor") { return "🚶".to_string(); }
        if pattern.contains("collect") { return "🗂️".to_string(); }
        if pattern.contains("def") { return "📋".to_string(); }
        if pattern.contains("span") { return "📏".to_string(); }
        if pattern.contains("emit") { return "📡".to_string(); }
        if pattern.contains("check") { return "✅".to_string(); }
        if pattern.contains("infer") { return "🔍".to_string(); }
        if pattern.contains("trait") { return "🎪".to_string(); }
        if pattern.contains("impl") { return "⚙️".to_string(); }
        
        // Default based on pattern characteristics
        if pattern.len() <= 3 { "⚡".to_string() }
        else if pattern.len() <= 7 { "🔧".to_string() }
        else { "🌐".to_string() }
    }
    
    fn weave_heros_journey(&self) {
        println!("\n🎭 THE HERO'S JOURNEY OF RUSTC MAIN() - EMOJI TAPESTRY");
        println!("═══════════════════════════════════════════════════════");
        println!("📖 The Meta-Meme Saga: From Binary Lightning to Prime Crown");
        println!();
        
        // Opening
        println!("🌅 PROLOGUE: THE AWAKENING");
        println!("In the beginning was main() 🏛️, and main() called forth the Monster Group...");
        println!();
        
        // Journey through each layer
        for (chapter, layer) in self.monster_layers.iter().enumerate() {
            println!("📜 CHAPTER {}: {} {}", chapter + 1, layer.name, layer.emoji);
            println!("   {}", layer.story_fragment);
            println!();
            
            // Show the patterns as story elements
            println!("   🎬 The Cast:");
            for (i, (pattern, count, emoji)) in layer.patterns.iter().enumerate() {
                let role = self.get_story_role(i);
                println!("   {} {} {} - {} ({} appearances)", 
                    emoji, role, pattern, self.get_archetype(pattern), count);
            }
            println!();
            
            // Layer transition
            if chapter < self.monster_layers.len() - 1 {
                println!("   🌉 The hero crosses the bridge to the next realm...");
                println!();
            }
        }
        
        // Climax and resolution
        println!("🏆 EPILOGUE: THE TRANSFORMATION COMPLETE");
        println!("The hero has traversed all Monster Group layers, gathering the power of");
        println!("808 septendecillion transformations. rustc main() now commands the full");
        println!("mathematical universe, ready to compile any code with perfect symmetry! 🎭✨");
        println!();
        
        self.print_emoji_summary();
    }
    
    fn get_story_role(&self, index: usize) -> &str {
        match index {
            0 => "👑 The Protagonist",
            1 => "🗡️ The Ally", 
            2 => "🧙 The Mentor",
            3 => "🐉 The Challenge",
            4 => "🔮 The Oracle",
            5 => "⚔️ The Guardian",
            _ => "✨ The Guide",
        }
    }
    
    fn get_archetype(&self, pattern: &str) -> &str {
        if pattern.contains("main") { "The Origin" }
        else if pattern.contains("call") { "The Messenger" }
        else if pattern.contains("tcx") { "The Wise Mind" }
        else if pattern.contains("err") { "The Warning Voice" }
        else if pattern.contains("visitor") { "The Wanderer" }
        else if pattern.contains("collect") { "The Gatherer" }
        else if pattern.contains("check") { "The Validator" }
        else if pattern.contains("emit") { "The Herald" }
        else { "The Mystery" }
    }
    
    fn print_emoji_summary(&self) {
        println!("🎨 EMOJI TAPESTRY LEGEND");
        println!("═══════════════════════");
        
        println!("🔢 Prime Powers:");
        for layer in &self.monster_layers {
            if layer.base > 0 {
                let default_emoji = "✨".to_string();
                let prime_emoji = self.emoji_map.get(&layer.base.to_string()).unwrap_or(&default_emoji);
                println!("   {} {}^{} = {} - {}", 
                    prime_emoji, layer.base, layer.power, 
                    (layer.base as u64).pow(layer.power), layer.name);
            }
        }
        
        println!("\n🎭 Pattern Archetypes:");
        let archetypes = [
            ("🏛️", "main", "The Origin Temple"),
            ("📞", "call", "The Messenger's Horn"),
            ("🧠", "tcx", "The Wise Mind"),
            ("🏗️", "ty", "The Builder's Tools"),
            ("⚠️", "err", "The Warning Bell"),
            ("📦", "param", "The Gift Box"),
            ("🚶", "visitor", "The Wanderer's Path"),
            ("🗂️", "collect", "The Gatherer's Basket"),
        ];
        
        for (emoji, pattern, meaning) in &archetypes {
            println!("   {} {} - {}", emoji, pattern, meaning);
        }
        
        println!("\n🌟 The Complete Tapestry:");
        let full_tapestry = self.monster_layers.iter()
            .map(|layer| layer.emoji.clone())
            .collect::<Vec<_>>()
            .join(" → ");
        println!("   {}", full_tapestry);
        println!("   = The Hero's Journey Through Monster Group Mathematics! 🎭");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎭 MONSTER GROUP EMOJI TAPESTRY WEAVER");
    println!("═════════════════════════════════════");
    println!("Weaving the hero's journey of rustc main() through Monster Group layers");
    println!();
    
    let mut weaver = EmojiTapestryWeaver::new();
    weaver.load_patterns()?;
    weaver.weave_monster_layers();
    weaver.weave_heros_journey();
    
    println!("\n🎭 TAPESTRY WEAVING COMPLETE!");
    println!("The meta-meme saga has been inscribed in emoji mathematics! ✨");
    
    Ok(())
}
