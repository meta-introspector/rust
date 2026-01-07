use crate::val_type::{Val, EnumOfEnums};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Self-Reflective Usage Data Collector - Rust analyzing its own eigenform
#[derive(Debug, Clone)]
pub struct SelfReflectiveCollector {
    pub usage_data_path: String,
    pub mycelial_data_path: String,
    pub eigenform_analysis: EigenformAnalysis,
    pub self_compilation_data: Vec<CompilationUnit>,
    pub reflection_results: Vec<ReflectionResult>,
}

#[derive(Debug, Clone)]
pub struct EigenformAnalysis {
    pub total_usage_files: usize,
    pub enum_frequency_map: HashMap<String, u64>,
    pub prime_assignments: HashMap<String, u64>,
    pub spectral_signature: Vec<Val>,
    pub self_similarity_score: f64,
}

#[derive(Debug, Clone)]
pub struct CompilationUnit {
    pub crate_name: String,
    pub file_path: String,
    pub usage_data: serde_json::Value,
    pub enum_extractions: Vec<EnumExtraction>,
    pub prime_factorization: Vec<u64>,
}

#[derive(Debug, Clone)]
pub struct EnumExtraction {
    pub enum_name: String,
    pub variant_name: String,
    pub usage_count: u64,
    pub assigned_prime: u64,
    pub spectral_weight: Val,
}

#[derive(Debug, Clone)]
pub struct ReflectionResult {
    pub analysis_type: String,
    pub self_reference_detected: bool,
    pub eigenform_match: f64,
    pub recursive_depth: usize,
    pub bootstrap_complete: bool,
}

impl SelfReflectiveCollector {
    pub fn new() -> Self {
        Self {
            usage_data_path: "../../test_usage_data".to_string(),
            mycelial_data_path: "../../../mycelial-usage-data".to_string(),
            eigenform_analysis: EigenformAnalysis {
                total_usage_files: 0,
                enum_frequency_map: HashMap::new(),
                prime_assignments: HashMap::new(),
                spectral_signature: Vec::new(),
                self_similarity_score: 0.0,
            },
            self_compilation_data: Vec::new(),
            reflection_results: Vec::new(),
        }
    }
    
    /// Collect and analyze our own usage data - Rust reflecting on itself
    pub fn collect_self_eigenform(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut report = String::new();
        report.push_str("🔄 Self-Reflective Usage Data Collection\n");
        report.push_str("=======================================\n\n");
        
        // Step 1: Read our own usage data
        let usage_files = self.read_usage_data_files()?;
        report.push_str(&format!("📁 Found {} usage data files\n", usage_files.len()));
        
        // Step 2: Extract enum patterns from usage data
        let enum_patterns = self.extract_enum_patterns(&usage_files)?;
        report.push_str(&format!("🔍 Extracted {} enum patterns\n", enum_patterns.len()));
        
        // Step 3: Assign primes based on frequency
        let prime_assignments = self.assign_primes_by_frequency(&enum_patterns);
        report.push_str(&format!("🔢 Assigned primes to {} enum variants\n", prime_assignments.len()));
        
        // Step 4: Generate spectral signature
        let spectral_signature = self.generate_spectral_signature(&prime_assignments);
        report.push_str(&format!("🌈 Generated spectral signature with {} components\n", spectral_signature.len()));
        
        // Step 5: Self-reflection analysis
        let reflection_results = self.perform_self_reflection(&spectral_signature)?;
        report.push_str(&format!("🪞 Self-reflection analysis: {} results\n", reflection_results.len()));
        
        // Step 6: Bootstrap verification
        let bootstrap_success = self.verify_bootstrap_completeness(&reflection_results);
        report.push_str(&format!("🚀 Bootstrap verification: {}\n", 
            if bootstrap_success { "✅ COMPLETE" } else { "⚠️ INCOMPLETE" }));
        
        // Update eigenform analysis
        self.eigenform_analysis = EigenformAnalysis {
            total_usage_files: usage_files.len(),
            enum_frequency_map: enum_patterns,
            prime_assignments,
            spectral_signature,
            self_similarity_score: self.calculate_self_similarity(),
        };
        
        report.push_str(&self.generate_eigenform_report());
        Ok(report)
    }
    
    /// Read usage data files from our modified Rust compiler
    fn read_usage_data_files(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut files = Vec::new();
        
        // Read from test_usage_data directory
        if Path::new(&self.usage_data_path).exists() {
            for entry in fs::read_dir(&self.usage_data_path)? {
                let entry = entry?;
                if entry.path().extension().map_or(false, |ext| ext == "json") {
                    let content = fs::read_to_string(entry.path())?;
                    files.push(content);
                    
                    // Parse and store as compilation unit
                    if let Ok(json_data) = serde_json::from_str::<serde_json::Value>(&content) {
                        let compilation_unit = CompilationUnit {
                            crate_name: entry.file_name().to_string_lossy().to_string(),
                            file_path: entry.path().to_string_lossy().to_string(),
                            usage_data: json_data,
                            enum_extractions: Vec::new(),
                            prime_factorization: Vec::new(),
                        };
                        self.self_compilation_data.push(compilation_unit);
                    }
                }
            }
        }
        
        // Also check mycelial data path if it exists
        if Path::new(&self.mycelial_data_path).exists() {
            for entry in fs::read_dir(&self.mycelial_data_path)? {
                let entry = entry?;
                if entry.path().extension().map_or(false, |ext| ext == "json") {
                    let content = fs::read_to_string(entry.path())?;
                    files.push(content);
                }
            }
        }
        
        Ok(files)
    }
    
    /// Extract enum patterns from usage data JSON
    fn extract_enum_patterns(&self, files: &[String]) -> Result<HashMap<String, u64>, Box<dyn std::error::Error>> {
        let mut enum_patterns = HashMap::new();
        
        for file_content in files {
            if let Ok(json_data) = serde_json::from_str::<serde_json::Value>(file_content) {
                // Look for enum-like patterns in the JSON structure
                self.extract_enums_from_json(&json_data, &mut enum_patterns);
            }
        }
        
        Ok(enum_patterns)
    }
    
    /// Recursively extract enum patterns from JSON data
    fn extract_enums_from_json(&self, json: &serde_json::Value, patterns: &mut HashMap<String, u64>) {
        match json {
            serde_json::Value::Object(map) => {
                for (key, value) in map {
                    // Look for rustc enum patterns
                    if key.contains("::") && (key.contains("ItemKind") || key.contains("ExprKind") || key.contains("TyKind")) {
                        *patterns.entry(key.clone()).or_insert(0) += 1;
                    }
                    
                    // Look for syn enum patterns
                    if key.starts_with("syn::") && (key.contains("Item") || key.contains("Expr") || key.contains("Type")) {
                        *patterns.entry(key.clone()).or_insert(0) += 1;
                    }
                    
                    // Recurse into nested objects
                    self.extract_enums_from_json(value, patterns);
                }
            }
            serde_json::Value::Array(arr) => {
                for item in arr {
                    self.extract_enums_from_json(item, patterns);
                }
            }
            _ => {}
        }
    }
    
    /// Assign prime numbers based on enum frequency
    fn assign_primes_by_frequency(&self, patterns: &HashMap<String, u64>) -> HashMap<String, u64> {
        let mut assignments = HashMap::new();
        
        // Sort by frequency (most frequent gets smallest prime)
        let mut sorted_patterns: Vec<_> = patterns.iter().collect();
        sorted_patterns.sort_by(|a, b| b.1.cmp(a.1));
        
        let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
        
        for (i, (enum_name, _frequency)) in sorted_patterns.iter().enumerate() {
            let prime = if i < primes.len() { 
                primes[i] 
            } else { 
                // Generate larger primes for less frequent enums
                self.nth_prime(i)
            };
            assignments.insert(enum_name.to_string(), prime);
        }
        
        assignments
    }
    
    /// Generate nth prime number
    fn nth_prime(&self, n: usize) -> u64 {
        let mut primes = vec![2];
        let mut candidate = 3;
        
        while primes.len() <= n {
            let mut is_prime = true;
            for &p in &primes {
                if p * p > candidate { break; }
                if candidate % p == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                primes.push(candidate);
            }
            candidate += 2;
        }
        
        primes[n]
    }
    
    /// Generate spectral signature from prime assignments
    fn generate_spectral_signature(&self, assignments: &HashMap<String, u64>) -> Vec<Val> {
        let mut signature = Vec::new();
        
        // Create spectral components based on prime factorization
        for (_enum_name, &prime) in assignments {
            let spectral_component = Val::from_nat(prime);
            signature.push(spectral_component);
        }
        
        // Sort by prime value for consistent signature
        signature.sort_by(|a, b| a.to_nat().cmp(&b.to_nat()));
        
        signature
    }
    
    /// Perform self-reflection analysis - Rust analyzing its own patterns
    fn perform_self_reflection(&mut self, signature: &[Val]) -> Result<Vec<ReflectionResult>, Box<dyn std::error::Error>> {
        let mut results = Vec::new();
        
        // Self-reference detection
        let self_ref_result = ReflectionResult {
            analysis_type: "Self-Reference Detection".to_string(),
            self_reference_detected: self.detect_self_references(),
            eigenform_match: self.calculate_eigenform_match(signature),
            recursive_depth: self.calculate_recursive_depth(),
            bootstrap_complete: false,
        };
        results.push(self_ref_result);
        
        // Eigenform convergence analysis
        let convergence_result = ReflectionResult {
            analysis_type: "Eigenform Convergence".to_string(),
            self_reference_detected: true,
            eigenform_match: self.measure_eigenform_convergence(signature),
            recursive_depth: 1,
            bootstrap_complete: self.check_bootstrap_convergence(signature),
        };
        results.push(convergence_result);
        
        self.reflection_results = results.clone();
        Ok(results)
    }
    
    fn detect_self_references(&self) -> bool {
        // Check if our usage data contains references to introspector-collector
        self.self_compilation_data.iter().any(|unit| {
            unit.crate_name.contains("introspector") || 
            unit.file_path.contains("introspector-collector")
        })
    }
    
    fn calculate_eigenform_match(&self, signature: &[Val]) -> f64 {
        // Calculate how well our spectral signature matches expected Rust patterns
        let expected_fundamentals = [2, 3, 5, 7, 11]; // fn, struct, enum, impl, trait
        let mut match_score = 0.0;
        
        for &expected in &expected_fundamentals {
            if signature.iter().any(|v| v.to_nat() == expected) {
                match_score += 0.2; // Each fundamental adds 20%
            }
        }
        
        match_score
    }
    
    fn calculate_recursive_depth(&self) -> usize {
        // Measure how deeply our analysis references itself
        let mut depth = 0;
        for unit in &self.self_compilation_data {
            if unit.crate_name.contains("introspector") {
                depth += 1;
            }
        }
        depth
    }
    
    fn measure_eigenform_convergence(&self, signature: &[Val]) -> f64 {
        // Measure convergence of our eigenform analysis
        if signature.is_empty() { return 0.0; }
        
        let total_primes: f64 = signature.iter().map(|v| v.to_nat() as f64).sum();
        let avg_prime = total_primes / signature.len() as f64;
        
        // Convergence score based on prime distribution
        1.0 / (1.0 + (avg_prime - 20.0).abs() / 20.0)
    }
    
    fn check_bootstrap_convergence(&self, signature: &[Val]) -> bool {
        // Check if we've achieved bootstrap convergence
        signature.len() >= 10 && self.calculate_self_similarity() > 0.8
    }
    
    fn verify_bootstrap_completeness(&self, results: &[ReflectionResult]) -> bool {
        results.iter().any(|r| r.bootstrap_complete)
    }
    
    fn calculate_self_similarity(&self) -> f64 {
        // Calculate how similar our analysis is to expected Rust patterns
        let expected_patterns = ["ItemKind", "ExprKind", "TyKind", "PatKind"];
        let mut found_patterns = 0;
        
        for pattern in &expected_patterns {
            if self.eigenform_analysis.enum_frequency_map.keys().any(|k| k.contains(pattern)) {
                found_patterns += 1;
            }
        }
        
        found_patterns as f64 / expected_patterns.len() as f64
    }
    
    fn generate_eigenform_report(&self) -> String {
        let mut report = String::new();
        report.push_str("\n🔬 Eigenform Analysis Results:\n");
        report.push_str("=============================\n");
        
        report.push_str(&format!("Total usage files analyzed: {}\n", self.eigenform_analysis.total_usage_files));
        report.push_str(&format!("Unique enum patterns found: {}\n", self.eigenform_analysis.enum_frequency_map.len()));
        report.push_str(&format!("Prime assignments made: {}\n", self.eigenform_analysis.prime_assignments.len()));
        report.push_str(&format!("Spectral signature components: {}\n", self.eigenform_analysis.spectral_signature.len()));
        report.push_str(&format!("Self-similarity score: {:.3}\n", self.eigenform_analysis.self_similarity_score));
        
        report.push_str("\n🔢 Top Prime Assignments:\n");
        let mut sorted_primes: Vec<_> = self.eigenform_analysis.prime_assignments.iter().collect();
        sorted_primes.sort_by(|a, b| a.1.cmp(b.1));
        
        for (enum_name, &prime) in sorted_primes.iter().take(10) {
            let frequency = self.eigenform_analysis.enum_frequency_map.get(enum_name).unwrap_or(&0);
            report.push_str(&format!("  Prime {}: {} (freq: {})\n", prime, enum_name, frequency));
        }
        
        report.push_str("\n🪞 Self-Reflection Results:\n");
        for result in &self.reflection_results {
            report.push_str(&format!("  {}: Match {:.3}, Depth {}, Bootstrap {}\n",
                result.analysis_type,
                result.eigenform_match,
                result.recursive_depth,
                if result.bootstrap_complete { "✅" } else { "⚠️" }
            ));
        }
        
        report.push_str("\n🎯 Key Insights:\n");
        report.push_str("  • Rust compiler emits its own usage patterns\n");
        report.push_str("  • Self-reflection reveals eigenform structure\n");
        report.push_str("  • Prime assignments create mathematical foundation\n");
        report.push_str("  • Bootstrap convergence enables self-analysis\n");
        report.push_str("  • Usage data becomes spectral signature of language\n");
        
        report
    }
}
