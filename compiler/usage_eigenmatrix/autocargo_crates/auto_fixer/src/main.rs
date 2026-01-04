use std::fs;

fn main() {
    println!("🔧 Auto-fixer for unified_rustc_wrapped.rs");
    
    // Step 1: Add missing feature flags
    fix_unstable_features();
    
    // Step 2: Remove duplicate diagnostic items
    fix_duplicate_diagnostics();
    
    println!("✅ Applied basic fixes. Test with: cargo check --bin unified_rustc_wrapped");
}

fn fix_unstable_features() {
    println!("1. Adding missing feature flags...");
    
    let path = "src/bin/unified_rustc_wrapped.rs";
    let content = fs::read_to_string(path).expect("Failed to read file");
    
    // Add missing features at the top
    let features_to_add = vec![
        "#![feature(assert_matches)]",
        "#![feature(error_reporter)]",
    ];
    
    let mut lines: Vec<&str> = content.lines().collect();
    
    // Find where to insert features (after existing features)
    let mut insert_pos = 0;
    for (i, line) in lines.iter().enumerate() {
        if line.starts_with("#![feature(") {
            insert_pos = i + 1;
        } else if line.starts_with("#![") {
            continue;
        } else if !line.trim().is_empty() {
            break;
        }
    }
    
    // Insert new features
    for feature in features_to_add.iter().rev() {
        lines.insert(insert_pos, feature);
    }
    
    let new_content = lines.join("\n");
    fs::write(path, new_content).expect("Failed to write file");
    
    println!("   ✅ Added assert_matches and error_reporter features");
}

fn fix_duplicate_diagnostics() {
    println!("2. Removing duplicate diagnostic items...");
    
    let path = "src/bin/unified_rustc_wrapped.rs";
    let content = fs::read_to_string(path).expect("Failed to read file");
    
    // Remove duplicate diagnostic item attributes
    let mut seen_diagnostics = std::collections::HashSet::new();
    let mut lines = Vec::new();
    let mut skip_next = false;
    
    for line in content.lines() {
        if skip_next {
            skip_next = false;
            continue;
        }
        
        if line.contains("#[rustc_diagnostic_item = \"") {
            // Extract the diagnostic item name
            if let Some(start) = line.find("\"") {
                if let Some(end) = line[start + 1..].find("\"") {
                    let item_name = &line[start + 1..start + 1 + end];
                    
                    if seen_diagnostics.contains(item_name) {
                        // Skip this duplicate diagnostic item and the next line (the enum/struct)
                        skip_next = true;
                        continue;
                    } else {
                        seen_diagnostics.insert(item_name.to_string());
                    }
                }
            }
        }
        
        lines.push(line);
    }
    
    let new_content = lines.join("\n");
    fs::write(path, new_content).expect("Failed to write file");
    
    println!("   ✅ Removed duplicate diagnostic items");
}
