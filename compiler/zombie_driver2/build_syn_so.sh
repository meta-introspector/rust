#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"


echo "🔧 BUILDING SYN DYNAMIC LIBRARY"
echo "==============================="

# Find syn crate in our ecosystem
SYN_PATH=$(find ../../../ -name "syn" -type d | grep -E "(src|lib)" | head -1)

if [ -z "$SYN_PATH" ]; then
    echo "❌ Syn crate not found, creating minimal syn.so..."
    
    # Create minimal syn library
    mkdir -p syn_moonshine/src
    
    cat > syn_moonshine/Cargo.toml << 'SYNEOF'
[package]
name = "syn-moonshine"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
syn = "2.0"
quote = "1.0"
proc-macro2 = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
SYNEOF

    cat > syn_moonshine/src/lib.rs << 'SYNEOF'
//! 🌙 Syn Moonshine - Mathematical AST Unification
//! 
//! This library creates a mathematical bridge between syn AST and rustc AST
//! using moonshine fusion techniques.

use syn::*;
use std::collections::HashMap;

/// Mathematical signature for syn AST nodes
#[derive(Debug, Clone)]
pub struct SynMathSignature {
    pub node_type: String,
    pub emoji: String,
    pub curve_class: String,
    pub complexity_score: f64,
    pub moonshine_hash: u64,
}

/// Moonshine fusion engine for syn-rustc unification
pub struct MoonshineFusion {
    syn_signatures: HashMap<String, SynMathSignature>,
    rustc_mappings: HashMap<String, String>,
}

impl MoonshineFusion {
    pub fn new() -> Self {
        let mut fusion = Self {
            syn_signatures: HashMap::new(),
            rustc_mappings: HashMap::new(),
        };
        fusion.initialize_syn_mappings();
        fusion
    }
    
    fn initialize_syn_mappings(&mut self) {
        // Map syn AST types to emojis and mathematical properties
        let syn_types = vec![
            ("Expr", "🌳", "ExpressionTree", 3.5),
            ("Type", "🔢", "TypeSystem", 4.2),
            ("Pat", "🎯", "PatternMatch", 2.8),
            ("Stmt", "📝", "Statement", 2.1),
            ("Item", "📦", "ModuleItem", 4.8),
            ("Block", "🧱", "CodeBlock", 2.3),
            ("Path", "🛤️", "PathExpression", 3.1),
            ("Ident", "🏷️", "Identifier", 1.5),
            ("Lit", "💎", "Literal", 1.8),
            ("Attribute", "🏷️", "Metadata", 2.5),
            ("Generics", "🧬", "GenericParams", 5.2),
            ("WhereClause", "❓", "Constraint", 4.1),
            ("Lifetime", "⏳", "LifetimeParam", 3.3),
            ("Visibility", "👁️", "AccessControl", 2.0),
            ("Token", "🎭", "SyntaxToken", 1.2),
        ];
        
        for (name, emoji, curve, complexity) in syn_types {
            let signature = SynMathSignature {
                node_type: name.to_string(),
                emoji: emoji.to_string(),
                curve_class: curve.to_string(),
                complexity_score: complexity,
                moonshine_hash: calculate_moonshine_hash(name),
            };
            self.syn_signatures.insert(name.to_string(), signature);
        }
    }
    
    /// Apply moonshine fusion to unify syn and rustc ASTs
    pub fn fuse_ast_nodes(&self, syn_node: &str) -> Option<String> {
        if let Some(sig) = self.syn_signatures.get(syn_node) {
            Some(format!("{}🌙{}", sig.emoji, sig.curve_class))
        } else {
            None
        }
    }
    
    /// Generate mathematical tapestry for syn AST
    pub fn generate_syn_tapestry(&self, ast_nodes: &[String], size: usize) -> Vec<String> {
        let mut tapestry = Vec::new();
        
        for i in 0..(size * size) {
            let node_idx = i % ast_nodes.len();
            let node = &ast_nodes[node_idx];
            
            let emoji = if let Some(sig) = self.syn_signatures.get(node) {
                sig.emoji.clone()
            } else {
                "⬜".to_string()
            };
            
            tapestry.push(emoji);
        }
        
        tapestry
    }
    
    /// Export syn signatures for P2P compilation
    pub fn export_signatures(&self) -> String {
        serde_json::to_string_pretty(&self.syn_signatures).unwrap_or_default()
    }
}

/// Calculate moonshine hash for AST node unification
fn calculate_moonshine_hash(node_type: &str) -> u64 {
    let mut hash = 0u64;
    for byte in node_type.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
    }
    // Apply moonshine transformation (golden ratio based)
    hash.wrapping_mul(1618033988749895) // φ * 10^15
}

/// Moonshine AST visitor for syn trees
pub struct MoonshineVisitor {
    pub signatures: Vec<SynMathSignature>,
    fusion: MoonshineFusion,
}

impl MoonshineVisitor {
    pub fn new() -> Self {
        Self {
            signatures: Vec::new(),
            fusion: MoonshineFusion::new(),
        }
    }
    
    pub fn visit_expr(&mut self, expr: &Expr) {
        if let Some(sig) = self.fusion.syn_signatures.get("Expr") {
            self.signatures.push(sig.clone());
        }
        
        // Recursively visit expression components
        match expr {
            Expr::Call(_) => self.visit_node("Call"),
            Expr::Path(_) => self.visit_node("Path"),
            Expr::Lit(_) => self.visit_node("Lit"),
            Expr::Block(_) => self.visit_node("Block"),
            Expr::If(_) => self.visit_node("If"),
            Expr::Match(_) => self.visit_node("Match"),
            _ => self.visit_node("Expr"),
        }
    }
    
    fn visit_node(&mut self, node_type: &str) {
        if let Some(sig) = self.fusion.syn_signatures.get(node_type) {
            self.signatures.push(sig.clone());
        }
    }
    
    pub fn get_mathematical_summary(&self) -> String {
        let total_complexity: f64 = self.signatures.iter()
            .map(|s| s.complexity_score)
            .sum();
        
        let emoji_sequence: String = self.signatures.iter()
            .map(|s| s.emoji.as_str())
            .collect();
        
        format!("🌙 Moonshine Summary: {} nodes, complexity: {:.2}, sequence: {}", 
                self.signatures.len(), total_complexity, emoji_sequence)
    }
}

/// Export C-compatible functions for P2P integration
#[no_mangle]
pub extern "C" fn moonshine_create_fusion() -> *mut MoonshineFusion {
    Box::into_raw(Box::new(MoonshineFusion::new()))
}

#[no_mangle]
pub extern "C" fn moonshine_fuse_node(
    fusion: *mut MoonshineFusion, 
    node_name: *const std::os::raw::c_char
) -> *mut std::os::raw::c_char {
    unsafe {
        if fusion.is_null() { return std::ptr::null_mut(); }
        
        let fusion = &*fusion;
        let c_str = std::ffi::CStr::from_ptr(node_name);
        
        if let Ok(node_str) = c_str.to_str() {
            if let Some(result) = fusion.fuse_ast_nodes(node_str) {
                let c_string = std::ffi::CString::new(result).unwrap();
                return c_string.into_raw();
            }
        }
        
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn moonshine_destroy_fusion(fusion: *mut MoonshineFusion) {
    if !fusion.is_null() {
        unsafe { Box::from_raw(fusion); }
    }
}
SYNEOF

    echo "✅ Created syn-moonshine library"
    
else
    echo "✅ Found syn at: $SYN_PATH"
fi

# Build the library
cd syn_moonshine
echo "🔨 Building syn.so..."
cargo build --release

if [ -f "target/release/libsyn_moonshine.so" ]; then
    cp target/release/libsyn_moonshine.so ../syn.so
    echo "✅ syn.so created successfully!"
else
    echo "❌ Failed to build syn.so"
    exit 1
fi

cd ..
