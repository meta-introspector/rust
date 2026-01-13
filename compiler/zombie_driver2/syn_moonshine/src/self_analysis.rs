use goblin::elf::Elf;
use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;
use syn::{File, visit::Visit};

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfAnalysisReport {
    compile_time_view: CompileTimeAnalysis,
    runtime_view: RuntimeAnalysis,
    joined_analysis: JoinedAnalysis,
    shifted_perspective: ShiftedAnalysis,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompileTimeAnalysis {
    our_ast: Vec<String>,
    syn_ast: Vec<String>,
    mathematical_signatures: Vec<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RuntimeAnalysis {
    our_symbols: Vec<String>,
    syn_symbols: Vec<String>,
    binary_signatures: Vec<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinedAnalysis {
    matched_patterns: Vec<(String, String)>, // (source_pattern, binary_pattern)
    divergences: Vec<String>,
    resonance_frequency: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShiftedAnalysis {
    compile_time_becomes_runtime: Vec<String>,
    runtime_becomes_compile_time: Vec<String>,
    transformation_matrix: Vec<Vec<f64>>,
}

pub struct SelfAnalyzer;

impl SelfAnalyzer {
    pub fn analyze_self() -> Result<SelfAnalysisReport, Box<dyn std::error::Error>> {
        println!("🌙 MOONSHINE SELF-ANALYSIS: COMPILE→RUNTIME→SHIFT");
        println!("================================================");

        // Phase 1: Compile everything
        let compile_artifacts = Self::compile_phase()?;

        // Phase 2: Load and analyze binaries
        let runtime_analysis = Self::runtime_phase(&compile_artifacts)?;

        // Phase 3: Parse source with syn
        let compile_analysis = Self::compile_time_phase()?;

        // Phase 4: Join runtime + compile time
        let joined = Self::join_phase(&compile_analysis, &runtime_analysis)?;

        // Phase 5: Shift perspectives
        let shifted = Self::shift_phase(&joined)?;

        Ok(SelfAnalysisReport {
            compile_time_view: compile_analysis,
            runtime_view: runtime_analysis,
            joined_analysis: joined,
            shifted_perspective: shifted,
        })
    }

    fn compile_phase() -> Result<CompileArtifacts, Box<dyn std::error::Error>> {
        println!("🔨 Phase 1: Compiling self and syn...");

        // Compile our moonshine tool
        let our_build =
            Command::new("cargo").args(&["build", "--lib"]).current_dir(".").output()?;

        if !our_build.status.success() {
            return Err("Failed to compile moonshine".into());
        }

        // Find our compiled SO
        let our_so_path = Self::find_so_file("libsyn_moonshine")?;

        // Find syn's compiled SO (from dependencies)
        let syn_so_path = Self::find_so_file("libsyn")?;

        println!("✅ Compiled artifacts:");
        println!("   Our SO: {}", our_so_path);
        println!("   Syn SO: {}", syn_so_path);

        Ok(CompileArtifacts { our_so: our_so_path, syn_so: syn_so_path })
    }

    fn runtime_phase(
        artifacts: &CompileArtifacts,
    ) -> Result<RuntimeAnalysis, Box<dyn std::error::Error>> {
        println!("🔍 Phase 2: Runtime binary analysis...");

        // Load our SO
        let our_buffer = fs::read(&artifacts.our_so)?;
        let our_elf = Elf::parse(&our_buffer)?;
        let our_symbols = Self::extract_symbols(&our_elf);

        // Load syn SO
        let syn_buffer = fs::read(&artifacts.syn_so)?;
        let syn_elf = Elf::parse(&syn_buffer)?;
        let syn_symbols = Self::extract_symbols(&syn_elf);

        // Generate binary signatures
        let binary_signatures = Self::compute_binary_signatures(&our_symbols, &syn_symbols);

        println!("📊 Runtime analysis complete:");
        println!("   Our symbols: {}", our_symbols.len());
        println!("   Syn symbols: {}", syn_symbols.len());

        Ok(RuntimeAnalysis { our_symbols, syn_symbols, binary_signatures })
    }

    fn compile_time_phase() -> Result<CompileTimeAnalysis, Box<dyn std::error::Error>> {
        println!("🌳 Phase 3: Compile-time AST analysis...");

        // Parse our own source
        let our_source = fs::read_to_string("src/lib.rs")?;
        let our_ast = syn::parse_file(&our_source)?;
        let our_paths = Self::extract_ast_paths(&our_ast);

        // Parse syn's source (from cargo registry or git)
        let syn_paths = Self::parse_syn_source()?;

        // Generate mathematical signatures
        let math_signatures = Self::compute_mathematical_signatures(&our_paths, &syn_paths);

        println!("🧮 Compile-time analysis complete:");
        println!("   Our AST paths: {}", our_paths.len());
        println!("   Syn AST paths: {}", syn_paths.len());

        Ok(CompileTimeAnalysis {
            our_ast: our_paths,
            syn_ast: syn_paths,
            mathematical_signatures: math_signatures,
        })
    }

    fn join_phase(
        compile: &CompileTimeAnalysis,
        runtime: &RuntimeAnalysis,
    ) -> Result<JoinedAnalysis, Box<dyn std::error::Error>> {
        println!("🔗 Phase 4: Joining compile-time ↔ runtime...");

        let mut matched_patterns = Vec::new();
        let mut divergences = Vec::new();

        // Match AST patterns to binary symbols
        for ast_path in &compile.our_ast {
            if let Some(matching_symbol) =
                Self::find_matching_symbol(ast_path, &runtime.our_symbols)
            {
                matched_patterns.push((ast_path.clone(), matching_symbol));
            } else {
                divergences.push(format!("AST pattern '{}' not found in binary", ast_path));
            }
        }

        // Find binary symbols not represented in AST
        for symbol in &runtime.our_symbols {
            if !Self::symbol_has_ast_match(symbol, &compile.our_ast) {
                divergences.push(format!("Binary symbol '{}' not found in AST", symbol));
            }
        }

        // Calculate resonance frequency between compile/runtime
        let resonance_frequency =
            Self::calculate_resonance(&compile.mathematical_signatures, &runtime.binary_signatures);

        println!("🎯 Join analysis complete:");
        println!("   Matched patterns: {}", matched_patterns.len());
        println!("   Divergences: {}", divergences.len());
        println!("   Resonance frequency: {:.3}", resonance_frequency);

        Ok(JoinedAnalysis { matched_patterns, divergences, resonance_frequency })
    }

    fn shift_phase(joined: &JoinedAnalysis) -> Result<ShiftedAnalysis, Box<dyn std::error::Error>> {
        println!("🌀 Phase 5: Shifting perspectives...");

        let mut compile_to_runtime = Vec::new();
        let mut runtime_to_compile = Vec::new();

        // Transform compile-time patterns into runtime perspective
        for (ast_pattern, binary_pattern) in &joined.matched_patterns {
            let transformed = format!("{}→{}", ast_pattern, binary_pattern);
            compile_to_runtime.push(transformed);
        }

        // Transform runtime patterns into compile-time perspective
        for (ast_pattern, binary_pattern) in &joined.matched_patterns {
            let transformed = format!("{}←{}", binary_pattern, ast_pattern);
            runtime_to_compile.push(transformed);
        }

        // Create transformation matrix
        let matrix_size = compile_to_runtime.len().min(10); // Limit size
        let mut transformation_matrix = vec![vec![0.0; matrix_size]; matrix_size];

        for i in 0..matrix_size {
            for j in 0..matrix_size {
                // Simple transformation based on string similarity
                transformation_matrix[i][j] =
                    Self::calculate_similarity(&compile_to_runtime[i], &runtime_to_compile[j]);
            }
        }

        println!("🔄 Perspective shift complete:");
        println!("   Compile→Runtime transformations: {}", compile_to_runtime.len());
        println!("   Runtime→Compile transformations: {}", runtime_to_compile.len());

        Ok(ShiftedAnalysis {
            compile_time_becomes_runtime: compile_to_runtime,
            runtime_becomes_compile_time: runtime_to_compile,
            transformation_matrix,
        })
    }

    // Helper functions
    fn find_so_file(name: &str) -> Result<String, Box<dyn std::error::Error>> {
        let output =
            Command::new("find").args(&["target", "-name", &format!("{}*.so", name)]).output()?;

        let paths = String::from_utf8(output.stdout)?;
        paths
            .lines()
            .next()
            .ok_or_else(|| format!("SO file not found: {}", name).into())
            .map(|s| s.to_string())
    }

    fn extract_symbols(elf: &Elf) -> Vec<String> {
        elf.syms
            .iter()
            .filter_map(|sym| {
                if sym.st_type() == 2 && sym.st_size > 0 {
                    // FUNC type
                    elf.strtab.get_at(sym.st_name).map(|s| s.to_string())
                } else {
                    None
                }
            })
            .collect()
    }

    fn extract_ast_paths(file: &File) -> Vec<String> {
        let mut visitor = PathVisitor::new();
        visitor.visit_file(file);
        visitor.paths
    }

    fn parse_syn_source() -> Result<Vec<String>, Box<dyn std::error::Error>> {
        // Try to find syn source in cargo registry or deps
        let syn_src_path = Self::find_syn_source()?;
        let syn_source = fs::read_to_string(syn_src_path)?;
        let syn_ast = syn::parse_file(&syn_source)?;
        Ok(Self::extract_ast_paths(&syn_ast))
    }

    fn find_syn_source() -> Result<String, Box<dyn std::error::Error>> {
        // Look for syn source in common locations
        let locations =
            ["~/.cargo/registry/src/*/syn-*/src/lib.rs", "target/debug/deps/syn-*/src/lib.rs"];

        for location in &locations {
            let output = Command::new("find").args(&["/", "-path", location]).output()?;

            if let Some(path) = String::from_utf8(output.stdout)?.lines().next() {
                return Ok(path.to_string());
            }
        }

        Err("Syn source not found".into())
    }

    fn compute_binary_signatures(our_symbols: &[String], syn_symbols: &[String]) -> Vec<u64> {
        our_symbols.iter().chain(syn_symbols.iter()).map(|s| Self::hash_string(s)).collect()
    }

    fn compute_mathematical_signatures(our_paths: &[String], syn_paths: &[String]) -> Vec<u64> {
        our_paths.iter().chain(syn_paths.iter()).map(|s| Self::hash_string(s)).collect()
    }

    fn hash_string(s: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    }

    fn find_matching_symbol(ast_path: &str, symbols: &[String]) -> Option<String> {
        symbols.iter().find(|symbol| symbol.contains(&ast_path.replace("::", "_"))).cloned()
    }

    fn symbol_has_ast_match(symbol: &str, ast_paths: &[String]) -> bool {
        ast_paths.iter().any(|path| symbol.contains(&path.replace("::", "_")))
    }

    fn calculate_resonance(math_sigs: &[u64], binary_sigs: &[u64]) -> f64 {
        let common_count = math_sigs.iter().filter(|sig| binary_sigs.contains(sig)).count();

        common_count as f64 / (math_sigs.len() + binary_sigs.len()) as f64
    }

    fn calculate_similarity(a: &str, b: &str) -> f64 {
        let common_chars = a.chars().filter(|c| b.contains(*c)).count();

        common_chars as f64 / (a.len() + b.len()) as f64
    }
}

struct CompileArtifacts {
    our_so: String,
    syn_so: String,
}

struct PathVisitor {
    paths: Vec<String>,
}

impl PathVisitor {
    fn new() -> Self {
        Self { paths: Vec::new() }
    }
}

impl<'ast> Visit<'ast> for PathVisitor {
    fn visit_path(&mut self, path: &'ast syn::Path) {
        let path_str =
            path.segments.iter().map(|seg| seg.ident.to_string()).collect::<Vec<_>>().join("::");
        self.paths.push(path_str);
        syn::visit::visit_path(self, path);
    }
}
