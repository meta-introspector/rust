use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let usage_dirs = vec![
        "../../test_usage_data",
        "/home/mdupont/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f"
    ];
    let mut merged_report = HashMap::new();
    let mut total_usages = 0;
    let mut crate_stats = HashMap::new();
    
    println!("🔍 Scanning usage data files from multiple sources...");
    
    for usage_dir in &usage_dirs {
        if !std::path::Path::new(usage_dir).exists() {
            println!("⚠️  Skipping non-existent directory: {}", usage_dir);
            continue;
        }
        
        scan_directory(usage_dir, &mut merged_report, &mut total_usages, &mut crate_stats)?;
    }
    
    println!("\n📊 MERGED USAGE REPORT");
    println!("======================");
    println!("Total usage entries: {}", total_usages);
    println!("Total crates analyzed: {}", crate_stats.len());
    println!("Total files processed: {}", merged_report.len());
    
    println!("\n📈 TOP CRATES BY USAGE COUNT:");
    let mut crate_totals: Vec<_> = crate_stats.iter()
        .map(|(crate_name, modules)| {
            let total: usize = modules.values().sum();
            (crate_name, total, modules.len())
        })
        .collect();
    crate_totals.sort_by(|a, b| b.1.cmp(&a.1));
    
    for (crate_name, total, module_count) in crate_totals.iter().take(15) {
        println!("  {:25} {:6} usages ({} modules)", crate_name, total, module_count);
    }
    
    println!("\n🔍 DETAILED BREAKDOWN BY MODULE TYPE:");
    let mut module_types = HashMap::new();
    for (_, (_, module, count)) in &merged_report {
        *module_types.entry(module.clone()).or_insert(0) += count;
    }
    
    let mut sorted_modules: Vec<_> = module_types.iter().collect();
    sorted_modules.sort_by(|a, b| b.1.cmp(a.1));
    
    for (module_type, count) in sorted_modules {
        println!("  {:15} {:6} total usages", module_type, count);
    }
    
    println!("\n💾 Saving detailed report to merged_usage_report.json");
    let detailed_report = serde_json::json!({
        "summary": {
            "total_usages": total_usages,
            "total_crates": crate_stats.len(),
            "total_files": merged_report.len()
        },
        "crate_stats": crate_stats,
        "module_type_stats": module_types,
        "file_details": merged_report
    });
    
    fs::write("merged_usage_report.json", serde_json::to_string_pretty(&detailed_report)?)?;
    println!("✅ Report saved successfully!");
    
    Ok(())
}

fn scan_directory(
    usage_dir: &str,
    merged_report: &mut HashMap<String, (String, String, usize)>,
    total_usages: &mut usize,
    crate_stats: &mut HashMap<String, HashMap<String, usize>>
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(usage_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().map_or(false, |ext| ext == "json") {
            let filename = path.file_name().unwrap().to_string_lossy();
            
            // Skip manifest and complexity files, focus on usage data
            if filename.contains("_manifest.json") || filename.contains("_complexity.json") {
                continue;
            }
            
            let content = fs::read_to_string(&path)?;
            if let Ok(json) = serde_json::from_str::<Value>(&content) {
                if let Some(obj) = json.as_object() {
                    if let (Some(crate_name), Some(module), Some(usages)) = 
                        (obj.get("crate"), obj.get("module"), obj.get("usages")) {
                        
                        let crate_str = crate_name.as_str().unwrap_or("unknown");
                        let module_str = module.as_str().unwrap_or("unknown");
                        let usage_count = usages.as_array().map_or(0, |arr| arr.len());
                        
                        *total_usages += usage_count;
                        
                        let crate_entry = crate_stats.entry(crate_str.to_string())
                            .or_insert_with(|| HashMap::new());
                        crate_entry.insert(module_str.to_string(), usage_count);
                        
                        merged_report.insert(filename.to_string(), (crate_str.to_string(), module_str.to_string(), usage_count));
                    }
                }
            }
        }
    }
    Ok(())
}
