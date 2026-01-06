use std::collections::HashMap;
use std::fs;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let main_manifest = "../../test_usage_data/working_usage_collector_manifest.json";
    let mut merged_report = HashMap::new();
    let mut total_usages = 0;
    let mut crate_stats = HashMap::new();
    
    println!("🔍 Reading comprehensive manifest and all dependencies...");
    
    // Read main manifest
    let manifest_content = fs::read_to_string(main_manifest)?;
    let manifest: Value = serde_json::from_str(&manifest_content)?;
    
    // Process all dependency manifests
    if let Some(deps) = manifest.get("dependency_manifests").and_then(|v| v.as_array()) {
        for dep_path in deps {
            if let Some(path_str) = dep_path.as_str() {
                let full_path = format!("../../test_usage_data/{}", path_str.trim_start_matches("./test_usage_data/"));
                process_manifest(&full_path, &mut merged_report, &mut total_usages, &mut crate_stats)?;
            }
        }
    }
    
    // Also scan for all usage data files in the directory
    scan_usage_files("../../test_usage_data", &mut merged_report, &mut total_usages, &mut crate_stats)?;
    
    println!("\n📊 COMPREHENSIVE USAGE REPORT");
    println!("==============================");
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
    
    for (crate_name, total, module_count) in crate_totals.iter().take(20) {
        println!("  {:30} {:6} usages ({} modules)", crate_name, total, module_count);
    }
    
    println!("\n🔍 DETAILED BREAKDOWN BY MODULE TYPE:");
    let mut module_types = HashMap::new();
    for (_, (_, module, count)) in &merged_report {
        *module_types.entry(module.clone()).or_insert(0) += count;
    }
    
    let mut sorted_modules: Vec<_> = module_types.iter().collect();
    sorted_modules.sort_by(|a, b| b.1.cmp(a.1));
    
    for (module_type, count) in sorted_modules.iter().take(15) {
        println!("  {:20} {:6} total usages", module_type, count);
    }
    
    println!("\n💾 Saving comprehensive report to comprehensive_usage_report.json");
    let detailed_report = serde_json::json!({
        "summary": {
            "total_usages": total_usages,
            "total_crates": crate_stats.len(),
            "total_files": merged_report.len(),
            "manifest_source": main_manifest
        },
        "crate_stats": crate_stats,
        "module_type_stats": module_types,
        "file_details": merged_report
    });
    
    fs::write("comprehensive_usage_report.json", serde_json::to_string_pretty(&detailed_report)?)?;
    println!("✅ Comprehensive report saved successfully!");
    
    Ok(())
}

fn process_manifest(manifest_path: &str, merged_report: &mut HashMap<String, (String, String, usize)>, total_usages: &mut usize, crate_stats: &mut HashMap<String, HashMap<String, usize>>) -> Result<(), Box<dyn std::error::Error>> {
    if !std::path::Path::new(manifest_path).exists() {
        return Ok(());
    }
    
    let content = fs::read_to_string(manifest_path)?;
    let manifest: Value = serde_json::from_str(&content)?;
    
    if let Some(crate_name) = manifest.get("crate_name").and_then(|v| v.as_str()) {
        println!("  📦 Processing crate: {}", crate_name);
        
        // Process usage files mentioned in manifest
        if let Some(usage_files) = manifest.get("usage_files").and_then(|v| v.as_array()) {
            for file_path in usage_files {
                if let Some(path_str) = file_path.as_str() {
                    let full_path = format!("../../test_usage_data/{}", path_str.trim_start_matches("./test_usage_data/"));
                    process_usage_file(&full_path, merged_report, total_usages, crate_stats)?;
                }
            }
        }
    }
    
    Ok(())
}

fn scan_usage_files(dir: &str, merged_report: &mut HashMap<String, (String, String, usize)>, total_usages: &mut usize, crate_stats: &mut HashMap<String, HashMap<String, usize>>) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().map_or(false, |ext| ext == "json") {
            let filename = path.file_name().unwrap().to_string_lossy();
            
            // Skip manifest and complexity files, focus on usage data
            if filename.contains("_manifest.json") || filename.contains("_complexity.json") {
                continue;
            }
            
            process_usage_file(&path.to_string_lossy(), merged_report, total_usages, crate_stats)?;
        }
    }
    Ok(())
}

fn process_usage_file(file_path: &str, merged_report: &mut HashMap<String, (String, String, usize)>, total_usages: &mut usize, crate_stats: &mut HashMap<String, HashMap<String, usize>>) -> Result<(), Box<dyn std::error::Error>> {
    if !std::path::Path::new(file_path).exists() {
        return Ok(());
    }
    
    let content = fs::read_to_string(file_path)?;
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
                
                let filename = std::path::Path::new(file_path)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                merged_report.insert(filename, (crate_str.to_string(), module_str.to_string(), usage_count));
            }
        }
    }
    Ok(())
}
