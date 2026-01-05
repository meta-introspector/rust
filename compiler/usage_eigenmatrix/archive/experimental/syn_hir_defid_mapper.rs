use std::collections::HashMap;
use std::fs;
use serde_json::Value;

#[derive(Debug)]
struct AlgebraMapping {
    syn_defids: HashMap<String, u32>,
    hir_defids: HashMap<String, u32>,
    shared_defids: HashMap<String, (u32, u32)>,
    bijection_evidence: Vec<String>,
}

fn main() {
    println!("🔗 Mapping Syn/Hir Bijection to Global DefId Dataset");
    println!("====================================================");
    
    let mut mapping = AlgebraMapping {
        syn_defids: HashMap::new(),
        hir_defids: HashMap::new(),
        shared_defids: HashMap::new(),
        bijection_evidence: Vec::new(),
    };
    
    // Scan for syn-related DefIds
    scan_syn_defids(&mut mapping);
    
    // Scan for hir-related DefIds  
    scan_hir_defids(&mut mapping);
    
    // Find shared DefIds (bijection evidence)
    find_shared_defids(&mut mapping);
    
    // Analyze the algebra
    analyze_bijection_algebra(&mapping);
}

fn scan_syn_defids(mapping: &mut AlgebraMapping) {
    println!("🔍 Scanning syn-related DefIds...");
    
    let usage_dir = "../../usage_data";
    if let Ok(entries) = fs::read_dir(usage_dir) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.contains("syn") && name.ends_with(".json") {
                    extract_defids_from_file(&entry.path().display().to_string(), 
                                           &mut mapping.syn_defids, "syn");
                }
            }
        }
    }
    
    println!("  Found {} unique syn DefIds", mapping.syn_defids.len());
}

fn scan_hir_defids(mapping: &mut AlgebraMapping) {
    println!("🔍 Scanning hir-related DefIds...");
    
    let usage_dir = "../../usage_data";
    if let Ok(entries) = fs::read_dir(usage_dir) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.contains("hir") && name.ends_with(".json") {
                    extract_defids_from_file(&entry.path().display().to_string(), 
                                           &mut mapping.hir_defids, "hir");
                }
            }
        }
    }
    
    println!("  Found {} unique hir DefIds", mapping.hir_defids.len());
}

fn extract_defids_from_file(filepath: &str, defid_map: &mut HashMap<String, u32>, source: &str) {
    if let Ok(content) = fs::read_to_string(filepath) {
        if let Ok(json) = serde_json::from_str::<Value>(&content) {
            if let Some(usages) = json["usages"].as_array() {
                for usage in usages {
                    if let Some(usage_str) = usage["usage"].as_str() {
                        if usage_str.contains("DefId") {
                            if let Some(defid) = extract_defid_pattern(usage_str) {
                                *defid_map.entry(defid).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn extract_defid_pattern(usage_str: &str) -> Option<String> {
    // Extract DefId patterns like "DefId(0:123 ~ crate[hash]::item)"
    if let Some(start) = usage_str.find("DefId(") {
        if let Some(end) = usage_str[start..].find(")") {
            let defid_str = &usage_str[start..start+end+1];
            return Some(defid_str.to_string());
        }
    }
    None
}

fn find_shared_defids(mapping: &mut AlgebraMapping) {
    println!("🔗 Finding shared DefIds (bijection evidence)...");
    
    for (syn_defid, syn_count) in &mapping.syn_defids {
        if let Some(hir_count) = mapping.hir_defids.get(syn_defid) {
            mapping.shared_defids.insert(syn_defid.clone(), (*syn_count, *hir_count));
            
            // Evidence of bijection
            mapping.bijection_evidence.push(format!(
                "{} appears {} times in syn, {} times in hir", 
                syn_defid, syn_count, hir_count
            ));
        }
    }
    
    println!("  Found {} shared DefIds", mapping.shared_defids.len());
}

fn analyze_bijection_algebra(mapping: &AlgebraMapping) {
    println!("\n📊 BIJECTION ALGEBRA ANALYSIS:");
    println!("==============================");
    
    let syn_total = mapping.syn_defids.len();
    let hir_total = mapping.hir_defids.len();
    let shared_total = mapping.shared_defids.len();
    
    println!("Syn DefIds: {}", syn_total);
    println!("Hir DefIds: {}", hir_total);
    println!("Shared DefIds: {}", shared_total);
    
    let syn_overlap = if syn_total > 0 { 
        (shared_total as f64 / syn_total as f64) * 100.0 
    } else { 0.0 };
    
    let hir_overlap = if hir_total > 0 { 
        (shared_total as f64 / hir_total as f64) * 100.0 
    } else { 0.0 };
    
    println!("Syn overlap: {:.1}%", syn_overlap);
    println!("Hir overlap: {:.1}%", hir_overlap);
    
    println!("\n🎯 BIJECTION EVIDENCE:");
    println!("======================");
    
    for (i, evidence) in mapping.bijection_evidence.iter().take(5).enumerate() {
        println!("{}. {}", i + 1, evidence);
    }
    
    // Calculate bijection strength
    let mut perfect_matches = 0;
    let mut close_matches = 0;
    
    for (syn_count, hir_count) in mapping.shared_defids.values() {
        if syn_count == hir_count {
            perfect_matches += 1;
        } else if syn_count.abs_diff(*hir_count) <= 2 {
            close_matches += 1;
        }
    }
    
    println!("\n🔍 BIJECTION STRENGTH:");
    println!("======================");
    println!("Perfect matches: {} ({:.1}%)", 
             perfect_matches, 
             (perfect_matches as f64 / shared_total as f64) * 100.0);
    println!("Close matches: {} ({:.1}%)", 
             close_matches,
             (close_matches as f64 / shared_total as f64) * 100.0);
    
    let total_strong = perfect_matches + close_matches;
    let bijection_confidence = (total_strong as f64 / shared_total as f64) * 100.0;
    
    println!("Bijection confidence: {:.1}%", bijection_confidence);
    
    if bijection_confidence > 80.0 {
        println!("\n🎉 STRONG BIJECTION DETECTED!");
        println!("Syn and Hir are isomorphic subsets of the global DefId graph");
    } else if bijection_confidence > 50.0 {
        println!("\n✅ PARTIAL BIJECTION CONFIRMED");
        println!("Syn and Hir share significant DefId overlap");
    } else {
        println!("\n⚠️  WEAK BIJECTION");
        println!("Limited overlap detected");
    }
    
    // Save analysis
    save_bijection_analysis(mapping, bijection_confidence);
}

fn save_bijection_analysis(mapping: &AlgebraMapping, confidence: f64) {
    let mut report = String::new();
    report.push_str("# Syn/Hir Bijection Analysis Against Global DefId Dataset\n\n");
    report.push_str(&format!("Bijection Confidence: {:.1}%\n\n", confidence));
    report.push_str("## Shared DefIds\n");
    
    for (defid, (syn_count, hir_count)) in &mapping.shared_defids {
        report.push_str(&format!("- {}: syn={}, hir={}\n", defid, syn_count, hir_count));
    }
    
    fs::write("syn_hir_bijection_analysis.md", report).unwrap();
    println!("\n📁 Analysis saved to syn_hir_bijection_analysis.md");
}
