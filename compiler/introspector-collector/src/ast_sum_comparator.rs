use crate::val_type::{Val, EnumOfEnums};
use crate::self_reflective_collector::SelfReflectiveCollector;
use std::collections::HashMap;
use std::process::Command;

/// AST Sum Comparator - Collect all ASTs per compilation, then compare each to the sum
#[derive(Debug, Clone)]
pub struct ASTSumComparator {
    pub compilation_runs: Vec<CompilationRun>,
    pub global_ast_sum: ASTSum,
    pub comparison_results: Vec<ComparisonResult>,
}

#[derive(Debug, Clone)]
pub struct CompilationRun {
    pub run_id: String,
    pub collected_asts: Vec<CollectedAST>,
    pub run_sum: ASTSum,
    pub total_nodes: usize,
}

#[derive(Debug, Clone)]
pub struct CollectedAST {
    pub ast_id: String,
    pub ast_type: String,
    pub node_count: usize,
    pub enum_signature: Vec<Val>,
    pub prime_factorization: u64,
    pub spectral_weight: Val,
    pub compilation_profile: CompilationProfile,
}

#[derive(Debug, Clone)]
pub struct CompilationProfile {
    pub profile_name: String,
    pub dump_flags: Vec<String>,
    pub compile_time_ms: u64,
    pub memory_profile: MemoryProfile,
    pub perf_counters: PerfCounters,
    pub dump_output: String,
    pub character_profile: CharacterProfile,
}

#[derive(Debug, Clone)]
pub struct MemoryProfile {
    pub peak_memory_kb: u64,
    pub allocations: u64,
    pub deallocations: u64,
}

#[derive(Debug, Clone)]
pub struct PerfCounters {
    pub cycles: u64,
    pub instructions: u64,
    pub cache_misses: u64,
    pub page_faults: u64,
}

#[derive(Debug, Clone)]
pub struct CharacterProfile {
    pub total_chars: usize,
    pub total_lines: usize,
    pub avg_line_length: usize,
    pub char_frequencies: HashMap<char, usize>,
    pub entropy: f64,
}

#[derive(Debug, Clone)]
pub struct ASTSum {
    pub total_nodes: usize,
    pub enum_frequencies: HashMap<String, u64>,
    pub prime_product: u64,
    pub spectral_signature: Vec<Val>,
    pub eigenform_hash: u64,
}

#[derive(Debug, Clone)]
pub struct ComparisonResult {
    pub ast_id: String,
    pub similarity_to_sum: f64,
    pub deviation_score: f64,
    pub eigenform_match: bool,
    pub contribution_weight: f64,
}

impl ASTSumComparator {
    pub fn new() -> Self {
        Self {
            compilation_runs: Vec::new(),
            global_ast_sum: ASTSum::empty(),
            comparison_results: Vec::new(),
        }
    }
    
    /// Run compilation and collect all AST data
    pub fn run_compilation_with_collection(&mut self, source_files: &[String]) -> Result<String, Box<dyn std::error::Error>> {
        let run_id = format!("run_{}", self.compilation_runs.len());
        let mut collected_asts = Vec::new();
        
        // Compile each source file and collect AST data
        for (i, source) in source_files.iter().enumerate() {
            let ast_data = self.compile_and_extract_ast(source, &format!("{}_{}", run_id, i))?;
            collected_asts.extend(ast_data);
        }
        
        // Calculate sum for this compilation run
        let run_sum = self.calculate_ast_sum(&collected_asts);
        
        let compilation_run = CompilationRun {
            run_id: run_id.clone(),
            collected_asts,
            run_sum,
            total_nodes: collected_asts.iter().map(|ast| ast.node_count).sum(),
        };
        
        self.compilation_runs.push(compilation_run);
        
        // Update global sum
        self.update_global_sum();
        
        Ok(format!("✅ Compilation run {} completed: {} ASTs collected", run_id, self.compilation_runs.last().unwrap().collected_asts.len()))
    }
    
    /// Compile source and extract AST data with performance profiling
    fn compile_and_extract_ast(&self, source: &str, ast_id: &str) -> Result<Vec<CollectedAST>, Box<dyn std::error::Error>> {
        // Write source to temp file
        let temp_file = format!("/tmp/{}.rs", ast_id);
        std::fs::write(&temp_file, source)?;
        
        let mut asts = Vec::new();
        
        // Profile HIR dump
        let hir_profile = self.profile_compilation(&temp_file, &["dump-hir"], "hir")?;
        let hir_ast = self.create_ast_from_profile("HIR", &hir_profile, &format!("{}_hir", ast_id))?;
        asts.push(hir_ast);
        
        // Profile MIR dump  
        let mir_profile = self.profile_compilation(&temp_file, &["dump-mir"], "mir")?;
        let mir_ast = self.create_ast_from_profile("MIR", &mir_profile, &format!("{}_mir", ast_id))?;
        asts.push(mir_ast);
        
        // Profile type sizes dump
        let type_profile = self.profile_compilation(&temp_file, &["print-type-sizes"], "types")?;
        let type_ast = self.create_ast_from_profile("TYPES", &type_profile, &format!("{}_types", ast_id))?;
        asts.push(type_ast);
        
        // Clean up temp file
        let _ = std::fs::remove_file(&temp_file);
        
        Ok(asts)
    }
    
    /// Profile compilation with specific dump flags
    fn profile_compilation(&self, source_file: &str, dump_flags: &[&str], profile_name: &str) -> Result<CompilationProfile, Box<dyn std::error::Error>> {
        let profile_file = format!("/tmp/profile_{}_{}.json", profile_name, std::process::id());
        
        // Build rustc command with profiling
        let mut cmd = Command::new("rustc");
        cmd.arg(source_file)
           .arg("-Z")
           .arg("self-profile")
           .arg("-Z")
           .arg(format!("self-profile-events=default"))
           .arg("--edition=2021");
        
        // Add dump flags
        for flag in dump_flags {
            cmd.arg("-Z").arg(*flag);
        }
        
        // Run with perf if available
        let start_time = std::time::Instant::now();
        let output = if self.has_perf() {
            Command::new("perf")
                .arg("stat")
                .arg("-e")
                .arg("cycles,instructions,cache-misses,page-faults")
                .arg("--")
                .args(cmd.get_args())
                .arg(cmd.get_program())
                .output()?
        } else {
            cmd.output()?
        };
        let compile_time = start_time.elapsed();
        
        // Extract performance data
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        let profile = CompilationProfile {
            profile_name: profile_name.to_string(),
            dump_flags: dump_flags.iter().map(|s| s.to_string()).collect(),
            compile_time_ms: compile_time.as_millis() as u64,
            memory_profile: self.extract_memory_profile(&stderr),
            perf_counters: self.extract_perf_counters(&stderr),
            dump_output: stderr.to_string(),
            character_profile: self.analyze_character_profile(&stderr),
        };
        
        Ok(profile)
    }
    
    /// Create AST from compilation profile
    fn create_ast_from_profile(&self, ast_type: &str, profile: &CompilationProfile, ast_id: &str) -> Result<CollectedAST, Box<dyn std::error::Error>> {
        // Analyze dump output for AST characteristics
        let node_count = self.count_ast_nodes(&profile.dump_output, ast_type);
        let enum_signature = self.extract_enum_signature_from_dump(&profile.dump_output, ast_type);
        let prime_factorization = self.calculate_prime_factorization(&enum_signature);
        
        // Calculate spectral weight based on performance characteristics
        let spectral_weight = self.calculate_spectral_weight_from_profile(profile);
        
        Ok(CollectedAST {
            ast_id: ast_id.to_string(),
            ast_type: ast_type.to_string(),
            node_count,
            enum_signature,
            prime_factorization,
            spectral_weight,
        })
    }
    
    /// Count AST nodes based on dump type
    fn count_ast_nodes(&self, dump_output: &str, ast_type: &str) -> usize {
        match ast_type {
            "HIR" => {
                dump_output.matches("hir::").count() +
                dump_output.matches("ItemKind::").count() +
                dump_output.matches("ExprKind::").count()
            }
            "MIR" => {
                dump_output.matches("mir::").count() +
                dump_output.matches("StatementKind::").count() +
                dump_output.matches("TerminatorKind::").count()
            }
            "TYPES" => {
                dump_output.matches("type size:").count() +
                dump_output.matches("layout:").count()
            }
            _ => dump_output.lines().count()
        }
    }
    
    /// Extract enum signature from specific dump type
    fn extract_enum_signature_from_dump(&self, dump_output: &str, ast_type: &str) -> Vec<Val> {
        let mut signature = Vec::new();
        
        match ast_type {
            "HIR" => {
                if dump_output.contains("ItemKind::Fn") { signature.push(EnumOfEnums::item_fn()); }
                if dump_output.contains("ItemKind::Struct") { signature.push(EnumOfEnums::item_struct()); }
                if dump_output.contains("ItemKind::Enum") { signature.push(EnumOfEnums::item_enum()); }
                if dump_output.contains("ExprKind::Call") { signature.push(EnumOfEnums::expr_call()); }
                if dump_output.contains("ExprKind::Match") { signature.push(EnumOfEnums::expr_match()); }
            }
            "MIR" => {
                if dump_output.contains("StatementKind::Assign") { signature.push(Val::from_nat(31)); }
                if dump_output.contains("TerminatorKind::Call") { signature.push(Val::from_nat(37)); }
                if dump_output.contains("TerminatorKind::Return") { signature.push(Val::from_nat(41)); }
            }
            "TYPES" => {
                if dump_output.contains("struct") { signature.push(EnumOfEnums::item_struct()); }
                if dump_output.contains("enum") { signature.push(EnumOfEnums::item_enum()); }
                if dump_output.contains("fn") { signature.push(EnumOfEnums::item_fn()); }
            }
            _ => {}
        }
        
        signature
    }
    
    /// Calculate spectral weight from performance profile
    fn calculate_spectral_weight_from_profile(&self, profile: &CompilationProfile) -> Val {
        // Weight based on compilation characteristics
        let time_weight = (profile.compile_time_ms as f64).log10().max(1.0);
        let memory_weight = (profile.memory_profile.peak_memory_kb as f64).log10().max(1.0);
        let char_weight = (profile.character_profile.total_chars as f64).log10().max(1.0);
        
        let combined_weight = time_weight * memory_weight * char_weight;
        Val::new(combined_weight)
    }
    
    /// Extract memory profile from compiler output
    fn extract_memory_profile(&self, stderr: &str) -> MemoryProfile {
        // Parse memory usage from compiler output (simplified)
        let peak_memory = stderr.lines()
            .filter(|line| line.contains("memory") || line.contains("MB") || line.contains("KB"))
            .count() as u64 * 1024; // Simplified estimation
        
        MemoryProfile {
            peak_memory_kb: peak_memory,
            allocations: stderr.matches("alloc").count() as u64,
            deallocations: stderr.matches("dealloc").count() as u64,
        }
    }
    
    /// Extract performance counters from perf output
    fn extract_perf_counters(&self, stderr: &str) -> PerfCounters {
        // Parse perf stat output (simplified)
        PerfCounters {
            cycles: self.extract_perf_value(stderr, "cycles"),
            instructions: self.extract_perf_value(stderr, "instructions"),
            cache_misses: self.extract_perf_value(stderr, "cache-misses"),
            page_faults: self.extract_perf_value(stderr, "page-faults"),
        }
    }
    
    /// Extract specific perf counter value
    fn extract_perf_value(&self, output: &str, counter: &str) -> u64 {
        output.lines()
            .find(|line| line.contains(counter))
            .and_then(|line| line.split_whitespace().next())
            .and_then(|s| s.replace(",", "").parse().ok())
            .unwrap_or(0)
    }
    
    /// Analyze character profile of dump output
    fn analyze_character_profile(&self, dump_output: &str) -> CharacterProfile {
        let total_chars = dump_output.len();
        let lines = dump_output.lines().count();
        let avg_line_length = if lines > 0 { total_chars / lines } else { 0 };
        
        // Character frequency analysis
        let mut char_frequencies = std::collections::HashMap::new();
        for ch in dump_output.chars() {
            *char_frequencies.entry(ch).or_insert(0) += 1;
        }
        
        CharacterProfile {
            total_chars,
            total_lines: lines,
            avg_line_length,
            char_frequencies,
            entropy: self.calculate_entropy(&char_frequencies, total_chars),
        }
    }
    
    /// Calculate entropy of character distribution
    fn calculate_entropy(&self, frequencies: &std::collections::HashMap<char, usize>, total: usize) -> f64 {
        if total == 0 { return 0.0; }
        
        frequencies.values()
            .map(|&freq| {
                let p = freq as f64 / total as f64;
                if p > 0.0 { -p * p.log2() } else { 0.0 }
            })
            .sum()
    }
    
    /// Check if perf is available
    fn has_perf(&self) -> bool {
        Command::new("perf").arg("--version").output().is_ok()
    }
    
    /// Create AST from compilation profile
    fn create_ast_from_profile(&self, ast_type: &str, profile: &CompilationProfile, ast_id: &str) -> Result<CollectedAST, Box<dyn std::error::Error>> {
        // Analyze dump output for AST characteristics
        let node_count = self.count_ast_nodes(&profile.dump_output, ast_type);
        let enum_signature = self.extract_enum_signature_from_dump(&profile.dump_output, ast_type);
        let prime_factorization = self.calculate_prime_factorization(&enum_signature);
        
        // Calculate spectral weight based on performance characteristics
        let spectral_weight = self.calculate_spectral_weight_from_profile(profile);
        
        Ok(CollectedAST {
            ast_id: ast_id.to_string(),
            ast_type: ast_type.to_string(),
            node_count,
            enum_signature,
            prime_factorization,
            spectral_weight,
            compilation_profile: profile.clone(),
        })
    }
    
    /// Extract enum signature from AST line
    fn extract_enum_signature(&self, line: &str) -> Vec<Val> {
        let mut signature = Vec::new();
        
        if line.contains("fn") { signature.push(EnumOfEnums::item_fn()); }
        if line.contains("struct") { signature.push(EnumOfEnums::item_struct()); }
        if line.contains("enum") { signature.push(EnumOfEnums::item_enum()); }
        if line.contains("impl") { signature.push(EnumOfEnums::item_impl()); }
        if line.contains("trait") { signature.push(EnumOfEnums::item_trait()); }
        if line.contains("call") { signature.push(EnumOfEnums::expr_call()); }
        if line.contains("match") { signature.push(EnumOfEnums::expr_match()); }
        
        signature
    }
    
    /// Calculate prime factorization of enum signature
    fn calculate_prime_factorization(&self, signature: &[Val]) -> u64 {
        if signature.is_empty() { return 1; }
        
        signature.iter()
            .map(|v| v.to_nat())
            .product()
    }
    
    /// Calculate AST sum for a collection of ASTs
    fn calculate_ast_sum(&self, asts: &[CollectedAST]) -> ASTSum {
        let mut enum_frequencies = HashMap::new();
        let mut prime_product = 1u64;
        let mut spectral_signature = Vec::new();
        
        for ast in asts {
            // Count enum frequencies
            for enum_val in &ast.enum_signature {
                let enum_key = format!("enum_{}", enum_val.to_nat());
                *enum_frequencies.entry(enum_key).or_insert(0) += 1;
            }
            
            // Multiply prime factorizations
            prime_product = prime_product.saturating_mul(ast.prime_factorization);
            
            // Accumulate spectral signature
            spectral_signature.push(ast.spectral_weight);
        }
        
        let total_nodes = asts.iter().map(|ast| ast.node_count).sum();
        let eigenform_hash = self.calculate_eigenform_hash(&enum_frequencies, prime_product);
        
        ASTSum {
            total_nodes,
            enum_frequencies,
            prime_product,
            spectral_signature,
            eigenform_hash,
        }
    }
    
    /// Calculate eigenform hash from frequencies and prime product
    fn calculate_eigenform_hash(&self, frequencies: &HashMap<String, u64>, prime_product: u64) -> u64 {
        let freq_sum: u64 = frequencies.values().sum();
        freq_sum.wrapping_mul(prime_product) % 1000000
    }
    
    /// Update global sum from all compilation runs
    fn update_global_sum(&mut self) {
        let mut all_asts = Vec::new();
        
        for run in &self.compilation_runs {
            all_asts.extend(run.collected_asts.clone());
        }
        
        self.global_ast_sum = self.calculate_ast_sum(&all_asts);
    }
    
    /// Compare each AST to the global sum
    pub fn compare_asts_to_sum(&mut self) -> String {
        let mut report = String::new();
        report.push_str("🔍 AST Sum Comparison Analysis\n");
        report.push_str("=============================\n\n");
        
        self.comparison_results.clear();
        
        for run in &self.compilation_runs {
            report.push_str(&format!("📊 Run {}: {} ASTs\n", run.run_id, run.collected_asts.len()));
            
            for ast in &run.collected_asts {
                let comparison = self.compare_ast_to_sum(ast);
                self.comparison_results.push(comparison.clone());
                
                report.push_str(&format!("  {} ({}): Similarity {:.3}, Deviation {:.3}, Match {}\n",
                    ast.ast_id,
                    ast.ast_type,
                    comparison.similarity_to_sum,
                    comparison.deviation_score,
                    if comparison.eigenform_match { "✅" } else { "❌" }
                ));
            }
        }
        
        // Summary statistics
        let total_asts = self.comparison_results.len();
        let matching_asts = self.comparison_results.iter().filter(|r| r.eigenform_match).count();
        let avg_similarity: f64 = self.comparison_results.iter().map(|r| r.similarity_to_sum).sum::<f64>() / total_asts as f64;
        
        report.push_str(&format!("\n📈 Summary Statistics:\n"));
        report.push_str(&format!("  Total ASTs analyzed: {}\n", total_asts));
        report.push_str(&format!("  Eigenform matches: {}\n", matching_asts));
        report.push_str(&format!("  Match rate: {:.1}%\n", (matching_asts as f64 / total_asts as f64) * 100.0));
        report.push_str(&format!("  Average similarity: {:.3}\n", avg_similarity));
        report.push_str(&format!("  Global sum nodes: {}\n", self.global_ast_sum.total_nodes));
        report.push_str(&format!("  Global prime product: {}\n", self.global_ast_sum.prime_product));
        report.push_str(&format!("  Global eigenform hash: {}\n", self.global_ast_sum.eigenform_hash));
        
        report
    }
    
    /// Compare individual AST to global sum
    fn compare_ast_to_sum(&self, ast: &CollectedAST) -> ComparisonResult {
        // Calculate similarity based on spectral weight vs sum
        let sum_avg_weight = if self.global_ast_sum.spectral_signature.is_empty() {
            Val::from_nat(1)
        } else {
            let total: f64 = self.global_ast_sum.spectral_signature.iter().map(|v| v.value()).sum();
            Val::new(total / self.global_ast_sum.spectral_signature.len() as f64)
        };
        
        let similarity = 1.0 - (ast.spectral_weight.value() - sum_avg_weight.value()).abs() / sum_avg_weight.value().max(1.0);
        
        // Calculate deviation score
        let expected_contribution = ast.node_count as f64 / self.global_ast_sum.total_nodes as f64;
        let actual_contribution = ast.spectral_weight.value() / self.global_ast_sum.spectral_signature.iter().map(|v| v.value()).sum::<f64>();
        let deviation = (expected_contribution - actual_contribution).abs();
        
        // Check eigenform match
        let eigenform_match = similarity > 0.7 && deviation < 0.3;
        
        ComparisonResult {
            ast_id: ast.ast_id.clone(),
            similarity_to_sum: similarity.max(0.0).min(1.0),
            deviation_score: deviation,
            eigenform_match,
            contribution_weight: actual_contribution,
        }
    }
}

impl ASTSum {
    fn empty() -> Self {
        Self {
            total_nodes: 0,
            enum_frequencies: HashMap::new(),
            prime_product: 1,
            spectral_signature: Vec::new(),
            eigenform_hash: 0,
        }
    }
}
