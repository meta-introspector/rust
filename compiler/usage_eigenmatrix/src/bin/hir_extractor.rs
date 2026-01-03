use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<()> {
    println!("🔍 Extracting tcx.hir patterns from eigenmatrix...");
    
    let mut hir_patterns = HashMap::new();
    let mut tcx_patterns = HashMap::new();
    let usage_dir = "../../usage_data";
    
    for entry in fs::read_dir(usage_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path)?;
            
            // Search entire JSON content for hir patterns
            if content.contains("hir") {
                let data: Value = serde_json::from_str(&content)?;
                
                if let Some(usages) = data["usages"].as_array() {
                    for usage_obj in usages {
                        let usage_text = usage_obj["usage"].as_str().unwrap_or("");
                        let used_def_id = usage_obj["used_def_id"].as_str().unwrap_or("");
                        let user_def_id = usage_obj["user_def_id"].as_str().unwrap_or("");
                        
                        // Collect any HIR-related patterns
                        if usage_text.contains("hir") || used_def_id.contains("hir") || user_def_id.contains("hir") {
                            *hir_patterns.entry(used_def_id.to_string()).or_insert(0) += 1;
                        }
                        
                        // Collect TyCtxt patterns
                        if usage_text.contains("tcx") || used_def_id.contains("tcx") || user_def_id.contains("tcx") {
                            *tcx_patterns.entry(used_def_id.to_string()).or_insert(0) += 1;
                        }
                    }
                }
            }
        }
    }
    
    println!("\n🎯 Most common HIR patterns:");
    let mut hir_sorted: Vec<_> = hir_patterns.iter().collect();
    hir_sorted.sort_by(|a, b| b.1.cmp(a.1));
    
    for (pattern, count) in hir_sorted.iter().take(10) {
        println!("  {} - {} uses", pattern, count);
    }
    
    println!("\n🎯 Most common TyCtxt patterns:");
    let mut tcx_sorted: Vec<_> = tcx_patterns.iter().collect();
    tcx_sorted.sort_by(|a, b| b.1.cmp(a.1));
    
    for (pattern, count) in tcx_sorted.iter().take(10) {
        println!("  {} - {} uses", pattern, count);
    }
    
    // Generate code suggestions based on actual patterns
    println!("\n💡 Suggested next lines based on eigenmatrix HIR patterns:");
    
    if !hir_sorted.is_empty() {
        let top_hir = &hir_sorted[0].0;
        println!("1. let hir = tcx.hir();");
        println!("2. // Based on top pattern: {}", top_hir);
        
        if top_hir.contains("visit") {
            println!("3. hir.visit_all_item_likes(&mut visitor);");
        }
        if top_hir.contains("def_path") {
            println!("3. let def_path = tcx.def_path_str(def_id);");
        }
        if top_hir.contains("item") {
            println!("3. for item_id in hir.items() {{");
        }
    }
    
    Ok(())
}
