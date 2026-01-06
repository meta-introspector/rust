use std::collections::{HashMap, HashSet};
use std::fs;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 working_usage_collector.rs AST/HIR Coverage Analysis");
    println!("======================================================");
    
    // Read the working_usage_collector.rs source
    let collector_source = fs::read_to_string("src/working_usage_collector.rs")?;
    
    // Extract what AST/HIR types are currently imported/used
    let mut current_imports = HashSet::new();
    let mut current_types = HashSet::new();
    
    for line in collector_source.lines() {
        if line.contains("rustc_") {
            current_imports.insert(line.trim().to_string());
        }
        if line.contains("::") && (line.contains("Expr") || line.contains("Item") || line.contains("Ty") || line.contains("Pat")) {
            current_types.insert(line.trim().to_string());
        }
    }
    
    // Analyze what's being collected from usage data
    let mut collected_usage_types = HashSet::new();
    let mut collected_node_types = HashSet::new();
    let mut collected_symbols = HashSet::new();
    
    if let Ok(entries) = fs::read_dir("../../test_usage_data") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "json") 
                && !path.file_name().unwrap().to_str().unwrap().contains("manifest") {
                
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(data) = serde_json::from_str::<Value>(&content) {
                        if let Some(usages) = data.get("usages").and_then(|v| v.as_array()) {
                            for usage in usages {
                                if let Some(usage_type) = usage.get("usage_type").and_then(|t| t.as_str()) {
                                    collected_usage_types.insert(usage_type.to_string());
                                }
                                if let Some(node_type) = usage.get("node_type").and_then(|t| t.as_str()) {
                                    collected_node_types.insert(node_type.to_string());
                                }
                                if let Some(symbol) = usage.get("symbol").and_then(|s| s.as_str()) {
                                    collected_symbols.insert(symbol.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    println!("📊 Current State:");
    println!("   Rustc imports: {}", current_imports.len());
    println!("   Usage types collected: {}", collected_usage_types.len());
    println!("   Node types collected: {}", collected_node_types.len());
    println!("   Total symbols: {}", collected_symbols.len());
    
    println!("\n🔧 CURRENT RUSTC IMPORTS:");
    for import in &current_imports {
        println!("  {}", import);
    }
    
    println!("\n📋 USAGE TYPES BEING COLLECTED:");
    let mut usage_types_sorted: Vec<_> = collected_usage_types.iter().collect();
    usage_types_sorted.sort();
    for usage_type in usage_types_sorted {
        println!("  - {}", usage_type);
    }
    
    println!("\n📋 NODE TYPES BEING COLLECTED:");
    let mut node_types_sorted: Vec<_> = collected_node_types.iter().collect();
    node_types_sorted.sort();
    for node_type in node_types_sorted {
        println!("  - {}", node_type);
    }
    
    // Analyze gaps and expansion opportunities
    println!("\n💡 EXPANSION OPPORTUNITIES:");
    println!("============================");
    
    println!("\n🌳 Missing AST/HIR Imports (add to working_usage_collector.rs):");
    let missing_imports = vec![
        "extern crate rustc_span;",
        "extern crate rustc_session;", 
        "extern crate rustc_errors;",
        "extern crate rustc_data_structures;",
        "extern crate rustc_target;",
        "use rustc_hir::*;",
        "use rustc_ast::*;",
        "use rustc_middle::mir::*;",
        "use rustc_middle::ty::*;",
    ];
    
    for import in missing_imports {
        println!("  + {}", import);
    }
    
    println!("\n🔍 Missing Node Types to Collect:");
    let missing_node_types = vec![
        "Pattern", "Type", "Lifetime", "GenericParam", "WhereClause",
        "Block", "Stmt", "Local", "Arm", "Field", "Variant",
        "Path", "PathSegment", "QPath", "Visibility"
    ];
    
    for node_type in missing_node_types {
        println!("  + {}", node_type);
    }
    
    println!("\n⚙️  Missing Usage Types to Track:");
    let missing_usage_types = vec![
        "PatternMatch", "TypeAnnotation", "LifetimeUsage", "GenericUsage",
        "TraitBound", "ImplBlock", "MacroCall", "AttributeUsage",
        "ModuleImport", "FunctionCall", "FieldAccess", "ArrayAccess",
        "TupleAccess", "ClosureUsage", "AsyncUsage", "UnsafeUsage"
    ];
    
    for usage_type in missing_usage_types {
        println!("  + {}", usage_type);
    }
    
    println!("\n🎯 HIGH-VALUE EXPANSIONS:");
    println!("=========================");
    
    println!("\n1. 🔧 Type System Analysis:");
    println!("   - Collect generic parameter usage");
    println!("   - Track trait bound complexity");
    println!("   - Analyze lifetime annotations");
    
    println!("\n2. 🌳 Pattern Matching:");
    println!("   - Match arm complexity");
    println!("   - Pattern destructuring depth");
    println!("   - Guard expression usage");
    
    println!("\n3. 📊 Macro Usage:");
    println!("   - Macro invocation patterns");
    println!("   - Derive macro usage");
    println!("   - Procedural macro complexity");
    
    println!("\n4. 🏗️  Control Flow:");
    println!("   - Loop complexity");
    println!("   - Async/await patterns");
    println!("   - Error handling patterns");
    
    println!("\n5. 🔍 Memory Safety:");
    println!("   - Unsafe block usage");
    println!("   - Raw pointer operations");
    println!("   - Lifetime parameter complexity");
    
    Ok(())
}
