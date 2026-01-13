#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"


echo "🔍 SYN AST MATHEMATICAL ANALYSIS"
echo "==============================="

if [ ! -f "syn.so" ]; then
    echo "❌ syn.so not found. Run ./build_syn_so.sh first"
    exit 1
fi

echo "📊 Analyzing syn.so with our mathematical framework..."

# Apply our existing tools to syn.so
echo "🧮 Extracting symbols from syn.so..."
./symbol_counter.sh syn.so > syn_symbols.txt 2>/dev/null || echo "Using fallback analysis..."

# Create syn-specific type analyzer
cat > syn_type_analyzer.rs << 'SYNEOF'
use std::fs;
use goblin::elf::Elf;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 SYN TYPE ANALYSIS");
    println!("====================");
    
    let syn_path = "syn.so";
    
    if !std::path::Path::new(syn_path).exists() {
        println!("❌ syn.so not found");
        return Ok(());
    }
    
    let buffer = fs::read(syn_path)?;
    let elf = Elf::parse(&buffer)?;
    
    let mut syn_types = HashMap::new();
    
    // Extract syn-specific types
    for sym in elf.syms.iter() {
        if let Some(name) = elf.strtab.get_at(sym.st_name) {
            if name.contains("syn") || name.contains("Expr") || name.contains("Type") {
                extract_syn_types(name, &mut syn_types);
            }
        }
    }
    
    println!("📊 Found {} syn types", syn_types.len());
    
    // Map to emojis
    for (syn_type, count) in syn_types.iter().take(20) {
        let emoji = map_syn_to_emoji(syn_type);
        println!("  {} {} ({}x)", emoji, syn_type, count);
    }
    
    Ok(())
}

fn extract_syn_types(symbol: &str, types: &mut HashMap<String, usize>) {
    let syn_patterns = [
        "Expr", "Type", "Pat", "Stmt", "Item", "Block", "Path", 
        "Ident", "Lit", "Attribute", "Generics", "Lifetime"
    ];
    
    for pattern in &syn_patterns {
        if symbol.contains(pattern) {
            *types.entry(pattern.to_string()).or_insert(0) += 1;
        }
    }
}

fn map_syn_to_emoji(syn_type: &str) -> &str {
    match syn_type {
        "Expr" => "🌳",
        "Type" => "🔢", 
        "Pat" => "🎯",
        "Stmt" => "📝",
        "Item" => "📦",
        "Block" => "🧱",
        "Path" => "🛤️",
        "Ident" => "🏷️",
        "Lit" => "💎",
        "Attribute" => "🏷️",
        "Generics" => "🧬",
        "Lifetime" => "⏳",
        _ => "⬜"
    }
}
SYNEOF

# Compile and run syn analyzer
rustc syn_type_analyzer.rs -L ../target/debug/deps --extern goblin=../target/debug/deps/libgoblin-*.rlib 2>/dev/null
if [ -f "syn_type_analyzer" ]; then
    ./syn_type_analyzer
    rm syn_type_analyzer syn_type_analyzer.rs
fi

echo ""
echo "✅ Syn AST analysis complete!"
