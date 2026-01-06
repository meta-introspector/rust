use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use serde_json::Value;

#[derive(Debug)]
struct HistoricalData {
    usage_patterns: HashMap<String, f64>,
    timing_data: HashMap<String, f64>,
    correlation_map: HashMap<String, Vec<String>>,
}

#[derive(Debug)]
struct Prediction {
    function: String,
    predicted_time: f64,
    confidence: f64,
    related_functions: Vec<String>,
}

fn load_historical_usage(usage_dir: &str) -> HashMap<String, f64> {
    let mut patterns = HashMap::new();
    
    if let Ok(entries) = fs::read_dir(usage_dir) {
        for entry in entries.flatten() {
            if entry.path().extension().map_or(false, |ext| ext == "json") {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    if let Ok(json) = serde_json::from_str::<Value>(&content) {
                        if let Some(obj) = json.as_object() {
                            for (key, value) in obj {
                                if let Some(count) = value.as_f64() {
                                    *patterns.entry(key.clone()).or_insert(0.0) += count;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    patterns
}

fn load_historical_profiles(profile_dir: &str) -> HashMap<String, f64> {
    let mut timing_data = HashMap::new();
    
    if let Ok(entries) = fs::read_dir(profile_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "mm_profdata") {
                let profile_base = path.to_str().unwrap().trim_end_matches(".mm_profdata");
                
                let output = Command::new("summarize")
                    .args(&[profile_base])
                    .output();
                
                if let Ok(result) = output {
                    let summary = String::from_utf8_lossy(&result.stdout);
                    for line in summary.lines() {
                        if line.contains("::") && (line.contains("ms") || line.contains("μs")) {
                            let parts: Vec<&str> = line.split_whitespace().collect();
                            for (i, part) in parts.iter().enumerate() {
                                if part.contains("::") {
                                    let func = part.split("::").last().unwrap_or(part);
                                    if i > 0 {
                                        if let Some(timing_str) = parts.get(i-1) {
                                            let clean_timing = timing_str.replace("ms", "").replace("μs", "");
                                            if let Ok(time) = clean_timing.parse::<f64>() {
                                                *timing_data.entry(func.to_string()).or_insert(0.0) += time;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    timing_data
}

fn build_correlations(usage: &HashMap<String, f64>, timing: &HashMap<String, f64>) -> HashMap<String, Vec<String>> {
    let mut correlations = HashMap::new();
    
    for (func, _) in usage {
        let mut related = Vec::new();
        for (other_func, _) in usage {
            if func != other_func && func.contains(&other_func[..other_func.len().min(5)]) {
                related.push(other_func.clone());
            }
        }
        if !related.is_empty() {
            correlations.insert(func.clone(), related);
        }
    }
    correlations
}

fn predict_current_run(current_usage: &HashMap<String, f64>, historical: &HistoricalData) -> Vec<Prediction> {
    let mut predictions = Vec::new();
    
    for (func, current_count) in current_usage {
        let historical_usage = historical.usage_patterns.get(func).unwrap_or(&0.0);
        let historical_timing = historical.timing_data.get(func).unwrap_or(&0.0);
        
        if *historical_usage > 0.0 {
            let usage_ratio = current_count / historical_usage;
            let predicted_time = historical_timing * usage_ratio;
            let confidence = (historical_usage / 100.0).min(1.0);
            
            let related = historical.correlation_map.get(func).cloned().unwrap_or_default();
            
            predictions.push(Prediction {
                function: func.clone(),
                predicted_time,
                confidence,
                related_functions: related,
            });
        }
    }
    
    predictions.sort_by(|a, b| b.predicted_time.partial_cmp(&a.predicted_time).unwrap());
    predictions
}

fn main() {
    println!("🔮 Predictive Compiler Analysis");
    
    // Load historical data
    let usage_patterns = load_historical_usage("../usage_data");
    let timing_data = load_historical_profiles("../compilation_trace");
    let correlations = build_correlations(&usage_patterns, &timing_data);
    
    let historical = HistoricalData {
        usage_patterns,
        timing_data,
        correlation_map: correlations,
    };
    
    // Load current run data
    let current_usage = load_historical_usage("./test_usage_data");
    
    // Generate predictions
    let predictions = predict_current_run(&current_usage, &historical);
    
    println!("\n📊 Predictions for Current Build:");
    for (i, pred) in predictions.iter().take(10).enumerate() {
        println!("{}. {} - {:.2}ms (confidence: {:.1}%)", 
                 i+1, pred.function, pred.predicted_time, pred.confidence * 100.0);
        if !pred.related_functions.is_empty() {
            println!("   Related: {}", pred.related_functions.join(", "));
        }
    }
    
    let total_predicted: f64 = predictions.iter().map(|p| p.predicted_time).sum();
    println!("\n⏱️  Total predicted build time: {:.2}ms", total_predicted);
}
