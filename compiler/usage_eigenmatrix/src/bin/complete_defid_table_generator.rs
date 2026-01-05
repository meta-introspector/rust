use std::collections::HashMap;
use std::fs;
use serde_json::Value;

#[derive(Debug, Default)]
struct DefIdEntry {
    defid: String,
    syn_count: u32,
    hir_count: u32,
    def_count: u32,
    total_count: u32,
}

fn main() {
    println!("📊 Creating Complete DefId Table: Syn + Hir + Def");
    println!("=================================================");
    
    let mut defid_table: HashMap<String, DefIdEntry> = HashMap::new();
    
    // Scan all usage files for DefIds
    scan_all_defids(&mut defid_table);
    
    // Convert to sorted table
    let mut table: Vec<DefIdEntry> = defid_table.into_values().collect();
    table.sort_by(|a, b| b.total_count.cmp(&a.total_count));
    
    // Display results
    display_defid_table(&table);
    
    // Save complete table
    save_defid_table(&table);
}

fn scan_all_defids(table: &mut HashMap<String, DefIdEntry>) {
    println!("🔍 Scanning all 17,728 files for DefIds...");
    
    let usage_dir = "../../usage_data";
    let mut processed = 0;
    
    if let Ok(entries) = fs::read_dir(usage_dir) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".json") {
                    let category = categorize_file(name);
                    extract_defids_by_category(&entry.path().display().to_string(), table, category);
                    
                    processed += 1;
                    if processed % 2000 == 0 {
                        println!("  Processed {} files...", processed);
                    }
                }
            }
        }
    }
    
    println!("✅ Processed {} files total", processed);
}

fn categorize_file(filename: &str) -> &'static str {
    if filename.contains("syn") {
        "syn"
    } else if filename.contains("hir") {
        "hir"
    } else {
        "def"
    }
}

fn extract_defids_by_category(filepath: &str, table: &mut HashMap<String, DefIdEntry>, category: &str) {
    if let Ok(content) = fs::read_to_string(filepath) {
        if let Ok(json) = serde_json::from_str::<Value>(&content) {
            if let Some(usages) = json["usages"].as_array() {
                for usage in usages {
                    if let Some(usage_str) = usage["usage"].as_str() {
                        if let Some(defid) = extract_clean_defid(usage_str) {
                            let entry = table.entry(defid.clone()).or_insert_with(|| DefIdEntry {
                                defid: defid.clone(),
                                ..Default::default()
                            });
                            
                            match category {
                                "syn" => entry.syn_count += 1,
                                "hir" => entry.hir_count += 1,
                                "def" => entry.def_count += 1,
                                _ => {}
                            }
                            entry.total_count += 1;
                        }
                    }
                }
            }
        }
    }
}

fn extract_clean_defid(usage_str: &str) -> Option<String> {
    if let Some(start) = usage_str.find("DefId(") {
        if let Some(end) = usage_str[start..].find(")") {
            let defid_full = &usage_str[start..start+end+1];
            
            // Extract just the crate and item part for cleaner display
            if let Some(tilde_pos) = defid_full.find(" ~ ") {
                if let Some(close_pos) = defid_full[tilde_pos..].find(")") {
                    let clean_part = &defid_full[tilde_pos+3..tilde_pos+close_pos];
                    return Some(clean_part.to_string());
                }
            }
            
            return Some(defid_full.to_string());
        }
    }
    None
}

fn display_defid_table(table: &[DefIdEntry]) {
    println!("\n📊 COMPLETE DEFID TABLE (Top 20):");
    println!("==================================");
    println!("{:<60} {:>6} {:>6} {:>6} {:>8}", "DefId", "Syn", "Hir", "Def", "Total");
    println!("{}", "=".repeat(88));
    
    for (i, entry) in table.iter().take(20).enumerate() {
        let short_defid = if entry.defid.len() > 55 {
            format!("{}...", &entry.defid[..52])
        } else {
            entry.defid.clone()
        };
        
        println!("{:2}. {:<57} {:>6} {:>6} {:>6} {:>8}", 
                i + 1, short_defid, entry.syn_count, entry.hir_count, 
                entry.def_count, entry.total_count);
    }
    
    // Summary statistics
    let total_defids = table.len();
    let syn_defids = table.iter().filter(|e| e.syn_count > 0).count();
    let hir_defids = table.iter().filter(|e| e.hir_count > 0).count();
    let def_defids = table.iter().filter(|e| e.def_count > 0).count();
    let shared_all = table.iter().filter(|e| e.syn_count > 0 && e.hir_count > 0 && e.def_count > 0).count();
    
    println!("\n📈 SUMMARY STATISTICS:");
    println!("======================");
    println!("Total unique DefIds: {}", total_defids);
    println!("DefIds in syn files: {}", syn_defids);
    println!("DefIds in hir files: {}", hir_defids);
    println!("DefIds in def files: {}", def_defids);
    println!("DefIds in all three: {}", shared_all);
    
    let coverage = (shared_all as f64 / total_defids as f64) * 100.0;
    println!("Triple coverage: {:.1}%", coverage);
}

fn save_defid_table(table: &[DefIdEntry]) {
    let mut csv = String::new();
    csv.push_str("DefId,Syn_Count,Hir_Count,Def_Count,Total_Count\n");
    
    for entry in table.iter().take(100) {
        csv.push_str(&format!("{},{},{},{},{}\n", 
                             entry.defid, entry.syn_count, entry.hir_count, 
                             entry.def_count, entry.total_count));
    }
    
    fs::write("complete_defid_table.csv", csv).unwrap();
    
    // Also save markdown version
    let mut md = String::new();
    md.push_str("# Complete DefId Analysis Table\n\n");
    md.push_str("| DefId | Syn | Hir | Def | Total |\n");
    md.push_str("|-------|-----|-----|-----|-------|\n");
    
    for entry in table.iter().take(50) {
        md.push_str(&format!("| {} | {} | {} | {} | {} |\n",
                            entry.defid, entry.syn_count, entry.hir_count,
                            entry.def_count, entry.total_count));
    }
    
    fs::write("complete_defid_table.md", md).unwrap();
    
    println!("\n📁 Tables saved:");
    println!("  - complete_defid_table.csv (top 100)");
    println!("  - complete_defid_table.md (top 50)");
}
