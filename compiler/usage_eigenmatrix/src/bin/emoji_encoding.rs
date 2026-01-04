use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<()> {
    println!("🔤 Rustc Emoji Encoding System");
    
    let nodes = load_and_rank_nodes()?;
    let emoji_codes = generate_emoji_codes(&nodes);
    
    println!("\n🏆 Top 20 Emoji Codes (Shortest = Most Powerful):");
    for (i, (node, weight, code)) in emoji_codes.iter().take(20).enumerate() {
        println!("{}. {} = {} ({} uses)", i+1, code, simplify_name(node), weight);
    }
    
    demonstrate_encoding(&emoji_codes);
    save_emoji_codebook(&emoji_codes)?;
    
    Ok(())
}

fn load_and_rank_nodes() -> Result<Vec<(String, usize)>> {
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
    Ok(sorted)
}

fn generate_emoji_codes(nodes: &[(String, usize)]) -> Vec<(String, usize, String)> {
    let base_emojis = vec!["👑", "⚡", "🔍", "🔘", "🔢", "📝", "🔄", "💾", "⚖️", "🎨", "⚠️", "🔧", "🪄", "📦", "🎮", "❓"];
    
    nodes.iter().enumerate().map(|(i, (node, weight))| {
        let code = generate_code_for_rank(i, &base_emojis);
        (node.clone(), *weight, code)
    }).collect()
}

fn generate_code_for_rank(rank: usize, base_emojis: &[&str]) -> String {
    match rank {
        // Top 16: Single emoji (most powerful)
        0..=15 => base_emojis[rank].to_string(),
        
        // Next 256: Two emojis
        16..=271 => {
            let idx = rank - 16;
            let first = idx / 16;
            let second = idx % 16;
            format!("{}{}", base_emojis[first], base_emojis[second])
        },
        
        // Next 4096: Three emojis
        272..=4367 => {
            let idx = rank - 272;
            let first = idx / 256;
            let second = (idx % 256) / 16;
            let third = idx % 16;
            format!("{}{}{}", base_emojis[first], base_emojis[second], base_emojis[third])
        },
        
        // Everything else: Four+ emojis (least powerful)
        _ => {
            let idx = rank - 4368;
            let first = idx / 4096;
            let second = (idx % 4096) / 256;
            let third = (idx % 256) / 16;
            let fourth = idx % 16;
            format!("{}{}{}{}", 
                base_emojis[first.min(15)], 
                base_emojis[second.min(15)], 
                base_emojis[third.min(15)], 
                base_emojis[fourth.min(15)])
        }
    }
}

fn demonstrate_encoding(emoji_codes: &[(String, usize, String)]) {
    println!("\n🎯 Encoding Power Levels:");
    
    let single_emoji = emoji_codes.iter().filter(|(_, _, code)| code.chars().count() == 1).count();
    let double_emoji = emoji_codes.iter().filter(|(_, _, code)| code.chars().count() == 2).count();
    let triple_emoji = emoji_codes.iter().filter(|(_, _, code)| code.chars().count() == 3).count();
    let quad_plus_emoji = emoji_codes.iter().filter(|(_, _, code)| code.chars().count() >= 4).count();
    
    println!("  👑 Single emoji (Ultra Powerful): {} terms", single_emoji);
    println!("  ⚡⚡ Double emoji (Very Powerful): {} terms", double_emoji);
    println!("  🔍🔍🔍 Triple emoji (Powerful): {} terms", triple_emoji);
    println!("  🔧🔧🔧🔧 Quad+ emoji (Standard): {} terms", quad_plus_emoji);
    
    println!("\n🔤 Sample Encoding Examples:");
    println!("  Most Powerful: {} = {}", emoji_codes[0].2, simplify_name(&emoji_codes[0].0));
    if emoji_codes.len() > 16 {
        println!("  Very Powerful: {} = {}", emoji_codes[16].2, simplify_name(&emoji_codes[16].0));
    }
    if emoji_codes.len() > 272 {
        println!("  Powerful: {} = {}", emoji_codes[272].2, simplify_name(&emoji_codes[272].0));
    }
    if emoji_codes.len() > 4368 {
        println!("  Standard: {} = {}", emoji_codes[4368].2, simplify_name(&emoji_codes[4368].0));
    }
}

fn save_emoji_codebook(emoji_codes: &[(String, usize, String)]) -> Result<()> {
    let mut output = String::from("# 🔤 Rustc Emoji Codebook\n\n");
    output.push_str("## Encoding System\n");
    output.push_str("- **Single emoji** (👑): Ultra powerful terms (top 16)\n");
    output.push_str("- **Double emoji** (⚡⚡): Very powerful terms (next 256)\n");
    output.push_str("- **Triple emoji** (🔍🔍🔍): Powerful terms (next 4096)\n");
    output.push_str("- **Quad+ emoji** (🔧🔧🔧🔧): Standard terms (everything else)\n\n");
    
    output.push_str("## Top 100 Emoji Codes\n\n");
    for (i, (node, weight, code)) in emoji_codes.iter().take(100).enumerate() {
        output.push_str(&format!("{}. `{}` = **{}** ({} uses)\n", 
            i+1, code, simplify_name(node), weight));
    }
    
    output.push_str("\n## Power Level Distribution\n\n");
    let single = emoji_codes.iter().filter(|(_, _, code)| code.chars().count() == 1).count();
    let double = emoji_codes.iter().filter(|(_, _, code)| code.chars().count() == 2).count();
    let triple = emoji_codes.iter().filter(|(_, _, code)| code.chars().count() == 3).count();
    let quad_plus = emoji_codes.iter().filter(|(_, _, code)| code.chars().count() >= 4).count();
    
    output.push_str(&format!("- Ultra Powerful (1 emoji): {} terms\n", single));
    output.push_str(&format!("- Very Powerful (2 emojis): {} terms\n", double));
    output.push_str(&format!("- Powerful (3 emojis): {} terms\n", triple));
    output.push_str(&format!("- Standard (4+ emojis): {} terms\n", quad_plus));
    
    fs::write("rustc_emoji_codebook.md", output)?;
    println!("\n💾 Emoji codebook saved to rustc_emoji_codebook.md");
    
    Ok(())
}

fn simplify_name(name: &str) -> String {
    if name.contains("DefId") {
        if let Some(start) = name.find("~ ") {
            if let Some(end) = name[start+2..].find(")") {
                return name[start+2..start+2+end].to_string();
            }
        }
    }
    
    if name.len() > 50 {
        format!("{}...", &name[..47])
    } else {
        name.to_string()
    }
}
