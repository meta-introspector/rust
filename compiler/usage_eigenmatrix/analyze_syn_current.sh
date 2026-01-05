#!/bin/bash

# Analyze current syn crate with our prime-complexity analyzer
echo "=== Current Syn Crate Analysis ==="

SYN_REPO_DIR="/tmp/syn_current"

# Clone current syn repo
if [ ! -d "$SYN_REPO_DIR" ]; then
    echo "Cloning syn repository..."
    git clone https://github.com/dtolnay/syn.git "$SYN_REPO_DIR"
fi

cd "$SYN_REPO_DIR"

echo "Analyzing current syn crate structure..."

# Find key Rust files
echo "📁 Key syn source files:"
find src/ -name "*.rs" -type f | head -10

# Copy our analyzer and modify it for syn
echo "🔧 Adapting analyzer for syn crate..."

# Create syn-specific analyzer
cat > analyze_syn_current.rs << 'EOF'
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧬 Syn Crate Prime-Complexity Analysis");
    println!("═══════════════════════════════════════");
    
    let files_to_analyze = vec![
        "src/lib.rs",
        "src/parse.rs", 
        "src/expr.rs",
        "src/item.rs",
        "src/ty.rs",
        "src/pat.rs",
        "src/stmt.rs",
    ];
    
    for file_path in &files_to_analyze {
        if let Ok(content) = fs::read_to_string(file_path) {
            let lines = content.lines().count();
            let chars = content.len();
            println!("📄 {}: {} lines, {} chars", file_path, lines, chars);
            
            // Count key patterns
            let struct_count = content.matches("struct ").count();
            let enum_count = content.matches("enum ").count();
            let impl_count = content.matches("impl ").count();
            let fn_count = content.matches("fn ").count();
            
            println!("  Structs: {}, Enums: {}, Impls: {}, Functions: {}", 
                     struct_count, enum_count, impl_count, fn_count);
        }
    }
    
    Ok(())
}
EOF

rustc analyze_syn_current.rs -o analyze_syn_current
./analyze_syn_current

echo "✅ Syn analysis complete!"
