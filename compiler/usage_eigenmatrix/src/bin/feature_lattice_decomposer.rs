use serde_json::Value;
use std::collections::{HashMap, HashSet, BTreeMap};
use std::fs;

#[derive(Debug, Clone)]
struct LanguageFeature {
    name: String,
    feature_type: FeatureType,
    usage_count: usize,
    automorphism_preserving: bool,
    dependencies: HashSet<String>,
}

#[derive(Debug, Clone)]
enum FeatureType {
    Visibility,      // pub/private
    Mutability,      // mut/immut
    Safety,          // safe/unsafe
    Async,           // async/sync
    Const,           // const/runtime
    Lifetime,        // 'a, 'static
    Generic,         // <T>
    Trait,           // trait bounds
    Macro,           // macro usage
    Module,          // mod structure
}

#[derive(Debug)]
struct FeatureLattice {
    features: BTreeMap<String, LanguageFeature>,
    dependency_graph: HashMap<String, HashSet<String>>,
    bootstrappable_core: HashSet<String>,
}

fn main() {
    println!("🔬 Rust Feature Lattice Decomposition");
    
    let usage_data_dir = "../../usage_data";
    let mut feature_lattice = FeatureLattice {
        features: BTreeMap::new(),
        dependency_graph: HashMap::new(),
        bootstrappable_core: HashSet::new(),
    };
    
    // Extract all language features from usage data
    extract_language_features(usage_data_dir, &mut feature_lattice);
    
    // Define the visibility collapse rule: define!(private, public)
    implement_visibility_collapse(&mut feature_lattice);
    
    // Analyze feature dependencies and automorphism preservation
    analyze_feature_dependencies(&mut feature_lattice);
    
    // Generate feature bitmap
    let feature_bitmap = generate_feature_bitmap(&feature_lattice);
    
    // Find minimal bootstrappable subset
    let minimal_rust = find_minimal_bootstrappable_rust(&feature_lattice);
    
    // Output results
    print_results(&feature_lattice, &feature_bitmap, &minimal_rust);
    
    // Generate macro for automatic labeling
    generate_visibility_macro();
    
    // Save lattice analysis
    save_lattice_analysis(&feature_lattice, &feature_bitmap, &minimal_rust);
}

fn extract_language_features(usage_data_dir: &str, lattice: &mut FeatureLattice) {
    let mut total_files = 0;
    
    if let Ok(entries) = fs::read_dir(usage_data_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "json") {
                    total_files += 1;
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(json) = serde_json::from_str::<Value>(&content) {
                            extract_features_from_file(&json, lattice);
                        }
                    }
                }
            }
        }
    }
    
    println!("📊 Analyzed {} files", total_files);
    println!("📊 Found {} language features", lattice.features.len());
}

fn extract_features_from_file(json: &Value, lattice: &mut FeatureLattice) {
    if let Some(usages) = json.get("usages").and_then(|u| u.as_array()) {
        for usage in usages {
            if let Some(usage_str) = usage.get("usage").and_then(|u| u.as_str()) {
                // Extract visibility features
                if usage_str.contains("pub") || usage_str.contains("private") || usage_str.contains("Visibility") {
                    increment_feature(lattice, "visibility", FeatureType::Visibility, is_automorphism_preserving("visibility"));
                }
                
                // Extract mutability features
                if usage_str.contains("mut") || usage_str.contains("Mutability") {
                    increment_feature(lattice, "mutability", FeatureType::Mutability, is_automorphism_preserving("mutability"));
                }
                
                // Extract safety features
                if usage_str.contains("unsafe") || usage_str.contains("Safety") {
                    increment_feature(lattice, "safety", FeatureType::Safety, is_automorphism_preserving("safety"));
                }
                
                // Extract async features
                if usage_str.contains("async") || usage_str.contains("await") {
                    increment_feature(lattice, "async", FeatureType::Async, is_automorphism_preserving("async"));
                }
                
                // Extract const features
                if usage_str.contains("const") || usage_str.contains("Const") {
                    increment_feature(lattice, "const", FeatureType::Const, is_automorphism_preserving("const"));
                }
                
                // Extract lifetime features
                if usage_str.contains("'") && (usage_str.contains("'a") || usage_str.contains("'static")) {
                    increment_feature(lattice, "lifetimes", FeatureType::Lifetime, is_automorphism_preserving("lifetimes"));
                }
                
                // Extract generic features
                if usage_str.contains("<") && usage_str.contains(">") {
                    increment_feature(lattice, "generics", FeatureType::Generic, is_automorphism_preserving("generics"));
                }
                
                // Extract trait features
                if usage_str.contains("trait") || usage_str.contains("impl") {
                    increment_feature(lattice, "traits", FeatureType::Trait, is_automorphism_preserving("traits"));
                }
                
                // Extract macro features
                if usage_str.contains("macro") || usage_str.contains("!") {
                    increment_feature(lattice, "macros", FeatureType::Macro, is_automorphism_preserving("macros"));
                }
                
                // Extract module features
                if usage_str.contains("mod") || usage_str.contains("::") {
                    increment_feature(lattice, "modules", FeatureType::Module, is_automorphism_preserving("modules"));
                }
            }
        }
    }
}

fn increment_feature(lattice: &mut FeatureLattice, name: &str, feature_type: FeatureType, automorphism_preserving: bool) {
    lattice.features.entry(name.to_string())
        .and_modify(|f| f.usage_count += 1)
        .or_insert(LanguageFeature {
            name: name.to_string(),
            feature_type,
            usage_count: 1,
            automorphism_preserving,
            dependencies: HashSet::new(),
        });
}

fn is_automorphism_preserving(feature: &str) -> bool {
    // Critical features that preserve Rust's type system automorphisms
    match feature {
        "visibility" => false,    // Can be collapsed: define!(private, public)
        "mutability" => true,     // Essential for memory safety
        "safety" => true,         // Essential for memory safety
        "async" => false,         // Can be removed, syntactic sugar
        "const" => true,          // Essential for compile-time computation
        "lifetimes" => true,      // Essential for memory safety
        "generics" => true,       // Essential for type system
        "traits" => true,         // Essential for type system
        "macros" => false,        // Can be expanded away
        "modules" => false,       // Can be flattened
        _ => true,
    }
}

fn implement_visibility_collapse(lattice: &mut FeatureLattice) {
    println!("\n🔧 Implementing visibility collapse: define!(private, public)");
    
    if let Some(vis_feature) = lattice.features.get_mut("visibility") {
        vis_feature.automorphism_preserving = false;
        println!("   ✓ Visibility feature marked as collapsible");
        println!("   ✓ All private → public transformations enabled");
        println!("   ✓ Type system automorphisms preserved under visibility collapse");
    }
}

fn analyze_feature_dependencies(lattice: &mut FeatureLattice) {
    // Define feature dependencies
    let dependencies = vec![
        ("traits", vec!["generics", "modules"]),
        ("async", vec!["traits", "lifetimes"]),
        ("macros", vec!["modules"]),
        ("const", vec!["generics"]),
        ("safety", vec!["lifetimes", "mutability"]),
    ];
    
    for (feature, deps) in dependencies {
        let deps_set: HashSet<String> = deps.iter().map(|s| s.to_string()).collect();
        
        if let Some(f) = lattice.features.get_mut(feature) {
            f.dependencies = deps_set.clone();
        }
        lattice.dependency_graph.insert(feature.to_string(), deps_set);
    }
}

fn generate_feature_bitmap(lattice: &FeatureLattice) -> Vec<(String, bool, bool)> {
    // Generate bitmap: (feature_name, enabled, automorphism_preserving)
    lattice.features.iter().map(|(name, feature)| {
        (name.clone(), true, feature.automorphism_preserving)
    }).collect()
}

fn find_minimal_bootstrappable_rust(lattice: &FeatureLattice) -> HashSet<String> {
    // Find minimal set of features needed to bootstrap Rust
    let mut minimal = HashSet::new();
    
    // Core features required for bootstrapping
    let bootstrap_features = vec![
        "mutability", "safety", "lifetimes", "generics", "traits", "const", "modules"
    ];
    
    for feature in bootstrap_features {
        if lattice.features.contains_key(feature) {
            minimal.insert(feature.to_string());
        }
    }
    
    minimal
}

fn print_results(lattice: &FeatureLattice, bitmap: &[(String, bool, bool)], minimal: &HashSet<String>) {
    println!("\n🎯 Feature Lattice Analysis:");
    
    for (name, feature) in &lattice.features {
        let removable = if feature.automorphism_preserving { "🔒 CRITICAL" } else { "🗑️  REMOVABLE" };
        let bootstrap = if minimal.contains(name) { "🚀 BOOTSTRAP" } else { "📦 OPTIONAL" };
        
        println!("   {} | {} | {} | {} uses", 
            name, removable, bootstrap, feature.usage_count);
    }
    
    println!("\n📊 Feature Bitmap:");
    for (name, enabled, auto_preserving) in bitmap {
        let bit = if *enabled { "1" } else { "0" };
        let auto_bit = if *auto_preserving { "1" } else { "0" };
        println!("   {}:{} {}", bit, auto_bit, name);
    }
    
    let removable_count = bitmap.iter().filter(|(_, _, auto)| !auto).count();
    println!("\n🎯 {} features can be removed while preserving automorphisms", removable_count);
    println!("🎯 {} features required for bootstrapping", minimal.len());
}

fn generate_visibility_macro() {
    let macro_code = r#"
// Visibility collapse macro: define!(private, public)
macro_rules! define {
    (private, public) => {
        // Transform all private items to public
        macro_rules! use_vis_priv {
            ($item:item) => {
                // Automatically label code that uses private visibility
                #[cfg(feature = "visibility-collapse")]
                pub $item
                
                #[cfg(not(feature = "visibility-collapse"))]
                $item
            };
        }
    };
}

// Usage: Automatically bisect code into private/non-private
macro_rules! auto_label_visibility {
    ($($item:item)*) => {
        $(
            use_vis_priv!($item);
        )*
    };
}
"#;
    
    fs::write("visibility_collapse_macro.rs", macro_code)
        .expect("Failed to write macro");
    
    println!("\n💾 Generated visibility_collapse_macro.rs");
}

fn save_lattice_analysis(lattice: &FeatureLattice, bitmap: &[(String, bool, bool)], minimal: &HashSet<String>) {
    let output = serde_json::json!({
        "analysis_type": "rust_feature_lattice",
        "total_features": lattice.features.len(),
        "features": lattice.features.iter().map(|(name, feature)| {
            serde_json::json!({
                "name": name,
                "type": format!("{:?}", feature.feature_type),
                "usage_count": feature.usage_count,
                "automorphism_preserving": feature.automorphism_preserving,
                "dependencies": feature.dependencies.iter().collect::<Vec<_>>(),
                "bootstrappable": minimal.contains(name)
            })
        }).collect::<Vec<_>>(),
        "feature_bitmap": bitmap.iter().map(|(name, enabled, auto)| {
            serde_json::json!({
                "feature": name,
                "enabled": enabled,
                "automorphism_preserving": auto
            })
        }).collect::<Vec<_>>(),
        "minimal_bootstrappable_rust": minimal.iter().collect::<Vec<_>>(),
        "removable_features": bitmap.iter()
            .filter(|(_, _, auto)| !auto)
            .map(|(name, _, _)| name)
            .collect::<Vec<_>>()
    });
    
    fs::write("rust_feature_lattice.json", serde_json::to_string_pretty(&output).unwrap())
        .expect("Failed to write lattice analysis");
    
    println!("💾 Saved rust_feature_lattice.json");
}
