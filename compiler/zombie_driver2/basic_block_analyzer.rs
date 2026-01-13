use std::collections::HashMap;
use std::fs;
use goblin::elf::Elf;
use syn::{parse_file, visit::Visit};

#[derive(Debug, Clone)]
struct BasicBlock {
    start_address: u64,
    size: usize,
    instructions: Vec<u8>,
    hash: u64,
    function_name: String,
}

#[derive(Debug)]
struct NovelBlock {
    block: BasicBlock,
    binary_name: String,
    uniqueness_score: f64,
}

#[derive(Debug, Default)]
struct CharFrequency {
    chars: HashMap<char, usize>,
    total: usize,
}

#[derive(Debug, Default)]
struct SynAnalysis {
    functions: usize,
    structs: usize,
    enums: usize,
    traits: usize,
    impls: usize,
    macros: usize,
}

struct SynVisitor {
    analysis: SynAnalysis,
}

impl<'ast> Visit<'ast> for SynVisitor {
    fn visit_item_fn(&mut self, _: &'ast syn::ItemFn) {
        self.analysis.functions += 1;
    }
    
    fn visit_item_struct(&mut self, _: &'ast syn::ItemStruct) {
        self.analysis.structs += 1;
    }
    
    fn visit_item_enum(&mut self, _: &'ast syn::ItemEnum) {
        self.analysis.enums += 1;
    }
    
    fn visit_item_trait(&mut self, _: &'ast syn::ItemTrait) {
        self.analysis.traits += 1;
    }
    
    fn visit_item_impl(&mut self, _: &'ast syn::ItemImpl) {
        self.analysis.impls += 1;
    }
    
    fn visit_item_macro(&mut self, _: &'ast syn::ItemMacro) {
        self.analysis.macros += 1;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧬 COMPREHENSIVE CODE ANALYSIS");
    println!("==============================");
    
    let target_binary = "./target/debug/semantic_signature_generator";
    
    // 1. Basic Block Analysis
    let target_data = fs::read(target_binary)?;
    let target_elf = Elf::parse(&target_data)?;
    let target_blocks = extract_basic_blocks(&target_data, &target_elf, "semantic_signature_generator")?;
    
    println!("🎯 Target: {} ({} basic blocks)", target_binary, target_blocks.len());
    
    // 2. Character Frequency Analysis
    println!("\n📊 CHARACTER FREQUENCY ANALYSIS");
    println!("================================");
    let source_file = "./semantic_signature_generator.rs";
    let char_freq = analyze_character_frequency(source_file)?;
    
    println!("Total characters: {}", char_freq.total);
    let mut sorted_chars: Vec<_> = char_freq.chars.iter().collect();
    sorted_chars.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("Top 10 characters:");
    for (i, (ch, count)) in sorted_chars.iter().take(10).enumerate() {
        let freq = **count as f64 / char_freq.total as f64 * 100.0;
        let display_char = if ch.is_whitespace() {
            match *ch {
                ' ' => "SPACE".to_string(),
                '\n' => "NEWLINE".to_string(),
                '\t' => "TAB".to_string(),
                _ => format!("U+{:04X}", **ch as u32),
            }
        } else {
            ch.to_string()
        };
        println!("  {}. '{}': {} ({:.1}%)", i + 1, display_char, count, freq);
    }
    
    // 3. Syn AST Analysis
    println!("\n🌳 SYN AST ANALYSIS");
    println!("===================");
    let syn_analysis = analyze_syn_ast(source_file)?;
    
    println!("Functions: {}", syn_analysis.functions);
    println!("Structs: {}", syn_analysis.structs);
    println!("Enums: {}", syn_analysis.enums);
    println!("Traits: {}", syn_analysis.traits);
    println!("Impls: {}", syn_analysis.impls);
    println!("Macros: {}", syn_analysis.macros);
    
    let total_items = syn_analysis.functions + syn_analysis.structs + syn_analysis.enums + 
                     syn_analysis.traits + syn_analysis.impls + syn_analysis.macros;
    println!("Total AST items: {}", total_items);
    
    // 4. Basic Block Novelty Analysis
    let analysis_tools = find_analysis_tools()?;
    println!("\n🔍 Found {} analysis tools to compare against", analysis_tools.len());
    
    let mut all_blocks: HashMap<u64, Vec<String>> = HashMap::new();
    
    for tool_path in &analysis_tools {
        if let Ok(tool_data) = fs::read(tool_path) {
            if let Ok(tool_elf) = Elf::parse(&tool_data) {
                let tool_name = std::path::Path::new(tool_path)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                
                let tool_blocks = extract_basic_blocks(&tool_data, &tool_elf, &tool_name)?;
                
                for block in tool_blocks {
                    all_blocks.entry(block.hash)
                        .or_insert_with(Vec::new)
                        .push(tool_name.clone());
                }
            }
        }
    }
    
    let mut novel_blocks = Vec::new();
    
    for block in &target_blocks {
        let uniqueness_score = if let Some(occurrences) = all_blocks.get(&block.hash) {
            1.0 / (occurrences.len() as f64 + 1.0)
        } else {
            1.0
        };
        
        if uniqueness_score > 0.5 {
            novel_blocks.push(NovelBlock {
                block: block.clone(),
                binary_name: "semantic_signature_generator".to_string(),
                uniqueness_score,
            });
        }
    }
    
    novel_blocks.sort_by(|a, b| b.uniqueness_score.partial_cmp(&a.uniqueness_score).unwrap());
    
    println!("\n🆕 BASIC BLOCK NOVELTY:");
    println!("=======================");
    println!("Total blocks: {}", target_blocks.len());
    println!("Novel/rare blocks: {}", novel_blocks.len());
    println!("Novelty rate: {:.1}%", (novel_blocks.len() as f64 / target_blocks.len() as f64) * 100.0);
    
    // 5. Comprehensive Report
    let report = serde_json::json!({
        "analysis_type": "comprehensive_code_analysis",
        "target": "semantic_signature_generator",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "character_analysis": {
            "total_characters": char_freq.total,
            "unique_characters": char_freq.chars.len(),
            "top_characters": sorted_chars.iter().take(10).map(|(ch, count)| {
                let display_char = if ch.is_whitespace() {
                    match **ch {
                        ' ' => "SPACE".to_string(),
                        '\n' => "NEWLINE".to_string(),
                        '\t' => "TAB".to_string(),
                        _ => format!("U+{:04X}", **ch as u32),
                    }
                } else {
                    ch.to_string()
                };
                serde_json::json!({
                    "character": display_char,
                    "count": count,
                    "frequency": **count as f64 / char_freq.total as f64 * 100.0
                })
            }).collect::<Vec<_>>()
        },
        "syn_analysis": {
            "functions": syn_analysis.functions,
            "structs": syn_analysis.structs,
            "enums": syn_analysis.enums,
            "traits": syn_analysis.traits,
            "impls": syn_analysis.impls,
            "macros": syn_analysis.macros,
            "total_items": total_items
        },
        "basic_block_analysis": {
            "total_blocks": target_blocks.len(),
            "novel_blocks": novel_blocks.len(),
            "novelty_rate": (novel_blocks.len() as f64 / target_blocks.len() as f64) * 100.0,
            "top_novel_blocks": novel_blocks.iter().take(5).map(|n| serde_json::json!({
                "function": n.block.function_name,
                "size": n.block.size,
                "uniqueness": n.uniqueness_score * 100.0
            })).collect::<Vec<_>>()
        }
    });
    
    fs::write("comprehensive_analysis_report.json", serde_json::to_string_pretty(&report)?)?;
    println!("\n✅ Comprehensive analysis report saved to: comprehensive_analysis_report.json");
    
    Ok(())
}

fn analyze_character_frequency(file_path: &str) -> Result<CharFrequency, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let mut freq = CharFrequency::default();
    
    for ch in content.chars() {
        *freq.chars.entry(ch).or_insert(0) += 1;
        freq.total += 1;
    }
    
    Ok(freq)
}

fn analyze_syn_ast(file_path: &str) -> Result<SynAnalysis, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let syntax_tree = parse_file(&content)?;
    
    let mut visitor = SynVisitor {
        analysis: SynAnalysis::default(),
    };
    
    visitor.visit_file(&syntax_tree);
    Ok(visitor.analysis)
}

fn find_analysis_tools() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut tools = Vec::new();
    let target_dir = "./target/debug";
    
    // Analysis tool patterns
    let analysis_patterns = [
        "analyzer", "analysis", "extractor", "detector", "scanner", 
        "mapper", "profiler", "tracer", "monitor", "inspector"
    ];
    
    let entries = fs::read_dir(target_dir)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && !path.to_string_lossy().ends_with(".d") {
            let filename = path.file_name().unwrap().to_string_lossy().to_lowercase();
            
            for pattern in &analysis_patterns {
                if filename.contains(pattern) {
                    tools.push(path.to_string_lossy().to_string());
                    break;
                }
            }
        }
    }
    
    Ok(tools)
}

fn extract_basic_blocks(binary_data: &[u8], elf: &Elf, _binary_name: &str) -> Result<Vec<BasicBlock>, Box<dyn std::error::Error>> {
    let mut blocks = Vec::new();
    
    // Find .text section
    let text_section = elf.section_headers.iter()
        .find(|section| {
            if let Some(name) = elf.shdr_strtab.get_at(section.sh_name) {
                name == ".text"
            } else {
                false
            }
        });
    
    if let Some(text_section) = text_section {
        // Extract basic blocks from each function
        for sym in &elf.syms {
            if let Some(name) = elf.strtab.get_at(sym.st_name) {
                if sym.st_size > 0 && sym.st_value >= text_section.sh_addr {
                    let offset = sym.st_value - text_section.sh_addr;
                    let start = (text_section.sh_offset + offset) as usize;
                    let end = std::cmp::min(start + sym.st_size as usize, binary_data.len());
                    
                    if start < binary_data.len() && end > start {
                        let function_bytes = &binary_data[start..end];
                        let function_blocks = split_into_basic_blocks(function_bytes, sym.st_value, name);
                        blocks.extend(function_blocks);
                    }
                }
            }
        }
    }
    
    Ok(blocks)
}

fn split_into_basic_blocks(function_bytes: &[u8], base_address: u64, function_name: &str) -> Vec<BasicBlock> {
    let mut blocks = Vec::new();
    let mut current_block_start = 0;
    
    // Simple basic block splitting on control flow instructions
    for (i, &byte) in function_bytes.iter().enumerate() {
        let is_control_flow = matches!(byte, 
            0x70..=0x7F | // Conditional jumps
            0xE8 |        // Call
            0xE9 |        // Jump
            0xC3 |        // Return
            0xEB          // Short jump
        );
        
        if is_control_flow || i == function_bytes.len() - 1 {
            let block_end = if i == function_bytes.len() - 1 { i + 1 } else { i };
            
            if block_end > current_block_start {
                let block_bytes = &function_bytes[current_block_start..block_end];
                let block_hash = calculate_block_hash(block_bytes);
                
                blocks.push(BasicBlock {
                    start_address: base_address + current_block_start as u64,
                    size: block_bytes.len(),
                    instructions: block_bytes.to_vec(),
                    hash: block_hash,
                    function_name: rustc_demangle::demangle(function_name).to_string(),
                });
                
                current_block_start = i + 1;
            }
        }
    }
    
    blocks
}

fn calculate_block_hash(block_bytes: &[u8]) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    block_bytes.hash(&mut hasher);
    hasher.finish()
}
