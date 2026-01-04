use std::collections::{HashMap, BTreeSet};
use std::fs;

#[derive(Clone, Debug)]
struct Terminal {
    enum_name: String,
    variants: Vec<String>,
    string_constants: Vec<String>,
    usage_count: usize,
}

#[derive(Clone, Debug)]
struct NonTerminal {
    name: String,
    production_rules: Vec<String>,
    depends_on: Vec<String>,
}

#[derive(Clone, Debug)]
struct FeatureSymmetry {
    name: String,
    multiplies_with: Vec<String>,
    symmetry_group: String, // e.g., "S2", "C4", "D3"
}

#[derive(Debug)]
struct LanguageSpec {
    terminals: Vec<Terminal>,
    non_terminals: Vec<NonTerminal>,
    feature_symmetries: Vec<FeatureSymmetry>,
    base_features: BTreeSet<String>,
}

fn main() {
    println!("🔬 Formal mkrust Language Generator");
    
    // Create example terminals for demonstration
    let terminals = create_example_terminals();
    
    // Define non-terminals (language constructs)
    let non_terminals = define_non_terminals();
    
    // Define feature symmetries
    let feature_symmetries = define_feature_symmetries();
    
    // Generate language specification
    let lang_spec = LanguageSpec {
        terminals: terminals.clone(),
        non_terminals,
        feature_symmetries,
        base_features: extract_base_features(&terminals),
    };
    
    // Generate mkrust macro system
    generate_mkrust_system(&lang_spec);
    
    println!("📊 Generated formal language with {} terminals", terminals.len());
}

fn create_example_terminals() -> Vec<Terminal> {
    vec![
        Terminal {
            enum_name: "bool".to_string(),
            variants: vec!["true".to_string(), "false".to_string()],
            string_constants: vec!["true".to_string(), "false".to_string()],
            usage_count: 1500,
        },
        Terminal {
            enum_name: "Option".to_string(),
            variants: vec!["Some".to_string(), "None".to_string()],
            string_constants: vec!["some".to_string(), "none".to_string()],
            usage_count: 2300,
        },
        Terminal {
            enum_name: "Result".to_string(),
            variants: vec!["Ok".to_string(), "Err".to_string()],
            string_constants: vec!["ok".to_string(), "error".to_string()],
            usage_count: 1800,
        },
        Terminal {
            enum_name: "ControlFlow".to_string(),
            variants: vec!["Continue".to_string(), "Break".to_string()],
            string_constants: vec!["continue".to_string(), "break".to_string()],
            usage_count: 800,
        },
    ]
}

fn load_enum_data(usage_data_dir: &str) -> HashMap<String, Terminal> {
    let mut terminals = HashMap::new();
    
    if let Ok(entries) = fs::read_dir(usage_data_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.file_name().unwrap().to_str().unwrap().ends_with("_enums_classified.json") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(data) = serde_json::from_str::<serde_json::Value>(&content) {
                            extract_terminals_from_file(&data, &mut terminals);
                        }
                    }
                }
            }
        }
    }
    
    terminals
}

fn extract_terminals_from_file(data: &serde_json::Value, terminals: &mut HashMap<String, Terminal>) {
    if let Some(enums) = data.get("enums").and_then(|e| e.as_object()) {
        for (enum_name, enum_data) in enums {
            if let Some(variants) = enum_data.get("variants").and_then(|v| v.as_array()) {
                let mut variant_names = Vec::new();
                let mut string_constants = Vec::new();
                let mut total_usage = 0;
                
                for variant in variants {
                    if let Some(variant_name) = variant.get("variant_name").and_then(|v| v.as_str()) {
                        variant_names.push(variant_name.to_string());
                        
                        // Extract string constants from usage classes
                        if let Some(usage_classes) = variant.get("usage_classes") {
                            if let Some(string_conv) = usage_classes.get("string_conversion").and_then(|s| s.as_u64()) {
                                total_usage += string_conv as usize;
                                if string_conv > 0 {
                                    string_constants.push(variant_name.to_lowercase());
                                }
                            }
                        }
                    }
                }
                
                if !string_constants.is_empty() {
                    terminals.insert(enum_name.clone(), Terminal {
                        enum_name: enum_name.clone(),
                        variants: variant_names,
                        string_constants,
                        usage_count: total_usage,
                    });
                }
            }
        }
    }
}

fn extract_terminals(enum_data: &HashMap<String, Terminal>) -> Vec<Terminal> {
    let mut terminals: Vec<Terminal> = enum_data.values().cloned().collect();
    terminals.sort_by(|a, b| b.usage_count.cmp(&a.usage_count));
    terminals
}

fn define_non_terminals() -> Vec<NonTerminal> {
    vec![
        NonTerminal {
            name: "Expression".to_string(),
            production_rules: vec![
                "Terminal".to_string(),
                "Expression BinaryOp Expression".to_string(),
                "UnaryOp Expression".to_string(),
            ],
            depends_on: vec!["Terminal".to_string(), "BinaryOp".to_string(), "UnaryOp".to_string()],
        },
        NonTerminal {
            name: "Statement".to_string(),
            production_rules: vec![
                "Expression ;".to_string(),
                "let Pattern = Expression ;".to_string(),
                "match Expression { MatchArms }".to_string(),
            ],
            depends_on: vec!["Expression".to_string(), "Pattern".to_string()],
        },
        NonTerminal {
            name: "Pattern".to_string(),
            production_rules: vec![
                "Terminal".to_string(),
                "Terminal ( PatternList )".to_string(),
            ],
            depends_on: vec!["Terminal".to_string()],
        },
    ]
}

fn define_feature_symmetries() -> Vec<FeatureSymmetry> {
    vec![
        FeatureSymmetry {
            name: "boolean_ops".to_string(),
            multiplies_with: vec!["comparison".to_string(), "logic".to_string()],
            symmetry_group: "S2".to_string(), // true/false symmetry
        },
        FeatureSymmetry {
            name: "option_ops".to_string(),
            multiplies_with: vec!["error_handling".to_string(), "pattern_matching".to_string()],
            symmetry_group: "S2".to_string(), // Some/None symmetry
        },
        FeatureSymmetry {
            name: "result_ops".to_string(),
            multiplies_with: vec!["error_handling".to_string(), "control_flow".to_string()],
            symmetry_group: "S2".to_string(), // Ok/Err symmetry
        },
        FeatureSymmetry {
            name: "visibility".to_string(),
            multiplies_with: vec!["privacy".to_string(), "modules".to_string()],
            symmetry_group: "C2".to_string(), // pub/private cyclic
        },
    ]
}

fn extract_base_features(terminals: &[Terminal]) -> BTreeSet<String> {
    let mut features = BTreeSet::new();
    
    for terminal in terminals {
        // Classify terminals into base features
        match terminal.enum_name.as_str() {
            name if name.contains("bool") || name == "true" || name == "false" => {
                features.insert("boolean".to_string());
            },
            name if name.contains("Option") => {
                features.insert("option".to_string());
            },
            name if name.contains("Result") => {
                features.insert("result".to_string());
            },
            name if name.contains("Visibility") => {
                features.insert("visibility".to_string());
            },
            name if name.contains("Kind") => {
                features.insert("kinds".to_string());
            },
            _ => {
                features.insert("core".to_string());
            }
        }
    }
    
    features
}

fn generate_mkrust_system(lang_spec: &LanguageSpec) {
    println!("\n🎯 Formal mkrust Language Specification:");
    
    // Generate terminal definitions
    println!("\n// TERMINALS (Enums with String Constants)");
    for terminal in &lang_spec.terminals {
        println!("mkrust!(");
        println!("    terminals!({});", terminal.enum_name);
        println!("    variants = {:?};", terminal.variants);
        println!("    strings = {:?};", terminal.string_constants);
        println!("    usage = {};", terminal.usage_count);
        println!(");");
    }
    
    // Generate non-terminal definitions
    println!("\n// NON-TERMINALS (Language Constructs)");
    for nt in &lang_spec.non_terminals {
        println!("mkrust!(");
        println!("    nonterminals!({});", nt.name);
        println!("    productions = {:?};", nt.production_rules);
        println!("    depends = {:?};", nt.depends_on);
        println!(");");
    }
    
    // Generate feature symmetries
    println!("\n// FEATURE SYMMETRIES");
    for symmetry in &lang_spec.feature_symmetries {
        println!("mkrust!(");
        println!("    symmetry!({});", symmetry.name);
        println!("    multiplies = {:?};", symmetry.multiplies_with);
        println!("    group = \"{}\";", symmetry.symmetry_group);
        println!(");");
    }
    
    // Generate complete language macro
    println!("\n// COMPLETE LANGUAGE GENERATOR");
    println!("macro_rules! mklang {{");
    println!("    (terminals!($($t:ident),*), nonterminals!($($nt:ident),*)) => {{");
    println!("        // Generate language with specified terminals and non-terminals");
    println!("        // Each feature multiplies with others via symmetry groups");
    println!("        $(mkrust!(terminal!($t));)*");
    println!("        $(mkrust!(nonterminal!($nt));)*");
    println!("    }};");
    println!("}}");
    
    // Save formal specification as text
    let mut spec_text = String::new();
    spec_text.push_str("# mkrust Formal Language Specification\n\n");
    spec_text.push_str(&format!("## Terminals: {}\n", lang_spec.terminals.len()));
    for terminal in &lang_spec.terminals {
        spec_text.push_str(&format!("- {}: {:?} -> {:?} ({})\n", 
            terminal.enum_name, terminal.variants, terminal.string_constants, terminal.usage_count));
    }
    spec_text.push_str(&format!("\n## Base Features: {:?}\n", lang_spec.base_features));
    
    fs::write("mkrust_formal_spec.txt", spec_text).unwrap();
    println!("\n💾 Saved formal specification to mkrust_formal_spec.txt");
}
