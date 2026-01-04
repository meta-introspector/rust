use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Clone)]
struct EnumStringMapping {
    enum_value: String,
    string_representations: HashSet<String>,
}

#[derive(Debug)]
struct EquivalenceClass {
    canonical_enum: String,
    equivalent_strings: HashSet<String>,
    evidence_count: usize,
}

fn main() {
    println!("🔍 Constructing Enum→String Equivalence Classes");
    
    let usage_data_dir = "../../usage_data";
    let mut enum_mappings: HashMap<String, HashSet<String>> = HashMap::new();
    let mut total_mappings = 0;
    
    // Collect all enum→string mappings
    if let Ok(entries) = fs::read_dir(usage_data_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "json") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(json) = serde_json::from_str::<Value>(&content) {
                            extract_enum_string_mappings(&json, &mut enum_mappings, &mut total_mappings);
                        }
                    }
                }
            }
        }
    }
    
    println!("📊 Found {} total enum→string mappings", total_mappings);
    println!("📊 Found {} unique enum values", enum_mappings.len());
    
    // Construct equivalence classes
    let equivalence_classes = construct_equivalence_classes(&enum_mappings);
    
    println!("\n🎯 Equivalence Classes (Enum values with multiple string representations):");
    
    let mut sorted_classes: Vec<_> = equivalence_classes.into_iter().collect();
    sorted_classes.sort_by(|a, b| b.evidence_count.cmp(&a.evidence_count));
    
    for class in sorted_classes.iter().take(20) {
        if class.equivalent_strings.len() > 1 {
            println!("   {} ≡ {{{}}} (evidence: {})", 
                class.canonical_enum,
                class.equivalent_strings.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "),
                class.evidence_count
            );
        }
    }
    
    // Test the boolean example: true → "true" and true → "t"
    println!("\n🧪 Testing Boolean Equivalence:");
    test_boolean_equivalence(&enum_mappings);
    
    // Generate inference rules
    println!("\n📜 Inference Rules:");
    generate_inference_rules(&sorted_classes);
    
    // Save results
    let output = serde_json::json!({
        "analysis_type": "enum_string_equivalence_classes",
        "total_mappings": total_mappings,
        "equivalence_classes": sorted_classes.iter().map(|class| {
            serde_json::json!({
                "canonical_enum": class.canonical_enum,
                "equivalent_strings": class.equivalent_strings.iter().collect::<Vec<_>>(),
                "evidence_count": class.evidence_count
            })
        }).collect::<Vec<_>>()
    });
    
    fs::write("enum_string_equivalence.json", serde_json::to_string_pretty(&output).unwrap())
        .expect("Failed to write results");
    
    println!("\n💾 Results saved to enum_string_equivalence.json");
}

fn extract_enum_string_mappings(
    json: &Value, 
    mappings: &mut HashMap<String, HashSet<String>>,
    total_count: &mut usize
) {
    if let Some(usages) = json.get("usages").and_then(|u| u.as_array()) {
        for usage in usages {
            if let (Some(usage_str), Some(used_def_id)) = (
                usage.get("usage").and_then(|u| u.as_str()),
                usage.get("used_def_id").and_then(|u| u.as_str())
            ) {
                // Look for enum values that map to strings
                if let Some((enum_val, string_val)) = extract_enum_string_pair(usage_str, used_def_id) {
                    mappings.entry(enum_val)
                        .or_insert_with(HashSet::new)
                        .insert(string_val);
                    *total_count += 1;
                }
            }
        }
    }
}

fn extract_enum_string_pair(usage: &str, used_def_id: &str) -> Option<(String, String)> {
    // Extract enum→string mappings from usage patterns
    
    // Pattern 1: Boolean values
    if used_def_id.contains("true") || used_def_id.contains("false") {
        if usage.contains("\"true\"") || usage.contains("\"t\"") {
            return Some(("true".to_string(), "true".to_string()));
        }
        if usage.contains("\"false\"") || usage.contains("\"f\"") {
            return Some(("false".to_string(), "false".to_string()));
        }
    }
    
    // Pattern 2: Option variants
    if used_def_id.contains("Option::Some") {
        return Some(("Some".to_string(), "Some".to_string()));
    }
    if used_def_id.contains("Option::None") {
        return Some(("None".to_string(), "None".to_string()));
    }
    
    // Pattern 3: ControlFlow variants
    if used_def_id.contains("ControlFlow::Continue") {
        return Some(("Continue".to_string(), "continue".to_string()));
    }
    if used_def_id.contains("ControlFlow::Break") {
        return Some(("Break".to_string(), "break".to_string()));
    }
    
    // Pattern 4: Log levels
    if used_def_id.contains("Level::DEBUG") {
        return Some(("DEBUG".to_string(), "debug".to_string()));
    }
    if used_def_id.contains("Level::INFO") {
        return Some(("INFO".to_string(), "info".to_string()));
    }
    if used_def_id.contains("Level::WARN") {
        return Some(("WARN".to_string(), "warn".to_string()));
    }
    if used_def_id.contains("Level::ERROR") {
        return Some(("ERROR".to_string(), "error".to_string()));
    }
    
    // Pattern 5: Visibility kinds
    if used_def_id.contains("VisibilityKind::Public") {
        return Some(("Public".to_string(), "pub".to_string()));
    }
    if used_def_id.contains("VisibilityKind::Inherited") {
        return Some(("Inherited".to_string(), "private".to_string()));
    }
    
    None
}

fn construct_equivalence_classes(mappings: &HashMap<String, HashSet<String>>) -> Vec<EquivalenceClass> {
    mappings.iter().map(|(enum_val, string_set)| {
        EquivalenceClass {
            canonical_enum: enum_val.clone(),
            equivalent_strings: string_set.clone(),
            evidence_count: string_set.len(),
        }
    }).collect()
}

fn test_boolean_equivalence(mappings: &HashMap<String, HashSet<String>>) {
    if let Some(true_strings) = mappings.get("true") {
        println!("   true → {{{}}}", true_strings.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "));
        
        // Test inference: if true → "true" and true → "t", then "true" ≡ "t"
        if true_strings.contains("true") && true_strings.contains("t") {
            println!("   ✓ PROVEN: \"true\" ≡ \"t\" (both map from enum true)");
        }
    }
    
    if let Some(false_strings) = mappings.get("false") {
        println!("   false → {{{}}}", false_strings.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "));
        
        if false_strings.contains("false") && false_strings.contains("f") {
            println!("   ✓ PROVEN: \"false\" ≡ \"f\" (both map from enum false)");
        }
    }
}

fn generate_inference_rules(classes: &[EquivalenceClass]) {
    println!("   Rule 1: ∀ enum e, strings s₁,s₂: (e → s₁) ∧ (e → s₂) ⟹ s₁ ≡ s₂");
    println!("   Rule 2: ∀ strings s₁,s₂,s₃: (s₁ ≡ s₂) ∧ (s₂ ≡ s₃) ⟹ s₁ ≡ s₃");
    println!("   Rule 3: String equivalence is reflexive, symmetric, transitive");
    
    let multi_string_classes: Vec<_> = classes.iter()
        .filter(|c| c.equivalent_strings.len() > 1)
        .collect();
    
    if !multi_string_classes.is_empty() {
        println!("\n   Concrete Rules Derived:");
        for class in multi_string_classes.iter().take(5) {
            let strings: Vec<_> = class.equivalent_strings.iter().collect();
            for i in 0..strings.len() {
                for j in i+1..strings.len() {
                    println!("   {} ≡ {} (via enum {})", 
                        strings[i], strings[j], class.canonical_enum);
                }
            }
        }
    }
}
