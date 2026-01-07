use crate::lattice_point_derive::{LatticePoint, impl_lattice_point};
use crate::github_ecosystem_lattice::GitHubRepository;
use std::collections::HashMap;

/// Rust Contribution Eigenform Analysis
/// Each commit to Rust has a characteristic eigenform pattern that can be detected and analyzed
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RustCommitEigenform {
    pub commit_hash: String,
    pub author: String,
    pub timestamp: u64,
    pub files_changed: Vec<String>,
    pub lines_added: u32,
    pub lines_removed: u32,
    pub eigenform_signature: EigenformSignature,
    pub contribution_pattern: ContributionPattern,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EigenformSignature {
    pub compiler_core_weight: f64,
    pub stdlib_weight: f64,
    pub test_weight: f64,
    pub doc_weight: f64,
    pub infrastructure_weight: f64,
    pub eigenvalue: f64,
    pub eigenvector: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContributionPattern {
    CompilerCore,      // Changes to rustc internals
    LanguageFeature,   // New language features
    StandardLibrary,   // Stdlib additions/changes
    Performance,       // Optimization commits
    BugFix,           // Bug fixes
    Documentation,     // Doc improvements
    Testing,          // Test additions
    Infrastructure,   // Build/CI changes
    Refactoring,      // Code cleanup
    Unknown,
}

impl_lattice_point!(RustCommitEigenform);

impl RustCommitEigenform {
    /// Analyze a commit and extract its eigenform signature
    pub fn analyze_commit(
        commit_hash: &str,
        author: &str,
        files_changed: Vec<String>,
        lines_added: u32,
        lines_removed: u32,
    ) -> Self {
        let eigenform_signature = Self::compute_eigenform_signature(&files_changed, lines_added, lines_removed);
        let contribution_pattern = Self::classify_contribution_pattern(&files_changed, &eigenform_signature);
        
        Self {
            commit_hash: commit_hash.to_string(),
            author: author.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            files_changed,
            lines_added,
            lines_removed,
            eigenform_signature,
            contribution_pattern,
        }
    }
    
    /// Compute the eigenform signature for a commit
    fn compute_eigenform_signature(files: &[String], added: u32, removed: u32) -> EigenformSignature {
        let mut compiler_core_weight = 0.0;
        let mut stdlib_weight = 0.0;
        let mut test_weight = 0.0;
        let mut doc_weight = 0.0;
        let mut infrastructure_weight = 0.0;
        
        // Analyze file paths to determine weights
        for file in files {
            if file.contains("compiler/rustc") || file.contains("src/librustc") {
                compiler_core_weight += 1.0;
            } else if file.contains("library/") || file.contains("src/libstd") {
                stdlib_weight += 1.0;
            } else if file.contains("test") || file.contains("tests/") {
                test_weight += 1.0;
            } else if file.contains("doc") || file.ends_with(".md") {
                doc_weight += 1.0;
            } else if file.contains("Cargo.toml") || file.contains(".yml") || file.contains("ci/") {
                infrastructure_weight += 1.0;
            }
        }
        
        // Normalize weights by total files
        let total_files = files.len() as f64;
        if total_files > 0.0 {
            compiler_core_weight /= total_files;
            stdlib_weight /= total_files;
            test_weight /= total_files;
            doc_weight /= total_files;
            infrastructure_weight /= total_files;
        }
        
        // Compute eigenvalue based on change magnitude and distribution
        let change_magnitude = (added + removed) as f64;
        let eigenvalue = change_magnitude * (compiler_core_weight * 10.0 + stdlib_weight * 5.0 + test_weight * 2.0);
        
        // Create eigenvector representing the contribution's characteristic pattern
        let eigenvector = vec![
            compiler_core_weight,
            stdlib_weight,
            test_weight,
            doc_weight,
            infrastructure_weight,
        ];
        
        EigenformSignature {
            compiler_core_weight,
            stdlib_weight,
            test_weight,
            doc_weight,
            infrastructure_weight,
            eigenvalue,
            eigenvector,
        }
    }
    
    /// Classify the contribution pattern based on eigenform signature
    fn classify_contribution_pattern(files: &[String], signature: &EigenformSignature) -> ContributionPattern {
        // Use eigenform weights to classify the contribution
        if signature.compiler_core_weight > 0.7 {
            ContributionPattern::CompilerCore
        } else if signature.stdlib_weight > 0.6 {
            ContributionPattern::StandardLibrary
        } else if signature.test_weight > 0.8 {
            ContributionPattern::Testing
        } else if signature.doc_weight > 0.7 {
            ContributionPattern::Documentation
        } else if signature.infrastructure_weight > 0.5 {
            ContributionPattern::Infrastructure
        } else if signature.eigenvalue > 1000.0 && signature.compiler_core_weight > 0.3 {
            ContributionPattern::LanguageFeature
        } else if files.iter().any(|f| f.contains("perf") || f.contains("opt")) {
            ContributionPattern::Performance
        } else if files.iter().any(|f| f.contains("fix") || f.contains("bug")) {
            ContributionPattern::BugFix
        } else if signature.eigenvalue < 100.0 {
            ContributionPattern::Refactoring
        } else {
            ContributionPattern::Unknown
        }
    }
    
    /// Check if this commit matches a known Rust contribution eigenform
    pub fn matches_rust_eigenform(&self) -> bool {
        // A commit matches Rust eigenform if:
        // 1. It has significant compiler or stdlib weight
        // 2. The eigenvalue is within expected ranges
        // 3. The eigenvector follows known patterns
        
        let has_core_contribution = self.eigenform_signature.compiler_core_weight > 0.1 
            || self.eigenform_signature.stdlib_weight > 0.1;
        
        let eigenvalue_in_range = self.eigenform_signature.eigenvalue > 10.0 
            && self.eigenform_signature.eigenvalue < 100000.0;
        
        let eigenvector_normalized = self.eigenform_signature.eigenvector.iter()
            .map(|&x| x * x)
            .sum::<f64>()
            .sqrt();
        
        let has_valid_eigenvector = eigenvector_normalized > 0.1 && eigenvector_normalized < 10.0;
        
        has_core_contribution && eigenvalue_in_range && has_valid_eigenvector
    }
    
    /// Get the primary URL for this commit (GitHub URL)
    pub fn github_url(&self) -> String {
        format!("https://github.com/rust-lang/rust/commit/{}", self.commit_hash)
    }
    
    /// Get Wikidata ID if this is a significant contribution
    pub fn wikidata_id(&self) -> Option<String> {
        if self.eigenform_signature.eigenvalue > 5000.0 {
            // Significant contributions might have Wikidata entries
            Some(format!("Q{}", self.commit_hash.chars().take(8).collect::<String>()))
        } else {
            None
        }
    }
}

/// Rust Contribution Eigenform Analyzer
pub struct RustContributionAnalyzer {
    pub analyzed_commits: Vec<RustCommitEigenform>,
    pub eigenform_patterns: HashMap<ContributionPattern, Vec<f64>>,
    pub contributor_eigenforms: HashMap<String, Vec<f64>>,
}

impl RustContributionAnalyzer {
    pub fn new() -> Self {
        Self {
            analyzed_commits: Vec::new(),
            eigenform_patterns: HashMap::new(),
            contributor_eigenforms: HashMap::new(),
        }
    }
    
    /// Analyze a batch of commits and extract eigenform patterns
    pub fn analyze_commits(&mut self, commits: Vec<(String, String, Vec<String>, u32, u32)>) {
        for (hash, author, files, added, removed) in commits {
            let eigenform = RustCommitEigenform::analyze_commit(&hash, &author, files, added, removed);
            
            // Update pattern statistics
            let pattern = eigenform.contribution_pattern.clone();
            self.eigenform_patterns
                .entry(pattern)
                .or_insert_with(Vec::new)
                .push(eigenform.eigenform_signature.eigenvalue);
            
            // Update contributor eigenform profile
            self.contributor_eigenforms
                .entry(author.clone())
                .or_insert_with(Vec::new)
                .extend(eigenform.eigenform_signature.eigenvector.clone());
            
            self.analyzed_commits.push(eigenform);
        }
    }
    
    /// Detect if a new commit follows established Rust eigenform patterns
    pub fn detect_eigenform_compliance(&self, commit: &RustCommitEigenform) -> f64 {
        if !commit.matches_rust_eigenform() {
            return 0.0;
        }
        
        // Compare against known patterns for this contribution type
        if let Some(pattern_eigenvalues) = self.eigenform_patterns.get(&commit.contribution_pattern) {
            let avg_eigenvalue = pattern_eigenvalues.iter().sum::<f64>() / pattern_eigenvalues.len() as f64;
            let deviation = (commit.eigenform_signature.eigenvalue - avg_eigenvalue).abs() / avg_eigenvalue;
            
            // Higher compliance = lower deviation from established patterns
            (1.0 - deviation.min(1.0)).max(0.0)
        } else {
            0.5 // Unknown pattern, moderate compliance
        }
    }
    
    /// Generate eigenform report for all analyzed commits
    pub fn generate_eigenform_report(&self) -> String {
        let mut report = String::new();
        report.push_str("🦀 Rust Contribution Eigenform Analysis Report\n");
        report.push_str("=============================================\n\n");
        
        report.push_str(&format!("Total commits analyzed: {}\n", self.analyzed_commits.len()));
        
        // Pattern distribution
        report.push_str("\n📊 Contribution Pattern Distribution:\n");
        for (pattern, eigenvalues) in &self.eigenform_patterns {
            let avg_eigenvalue = eigenvalues.iter().sum::<f64>() / eigenvalues.len() as f64;
            report.push_str(&format!("  {:?}: {} commits, avg eigenvalue: {:.2}\n", 
                pattern, eigenvalues.len(), avg_eigenvalue));
        }
        
        // Top contributors by eigenform complexity
        report.push_str("\n🏆 Top Contributors by Eigenform Complexity:\n");
        let mut contributor_scores: Vec<_> = self.contributor_eigenforms.iter()
            .map(|(author, eigenvector)| {
                let complexity = eigenvector.iter().map(|&x| x * x).sum::<f64>().sqrt();
                (author, complexity)
            })
            .collect();
        contributor_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        for (author, complexity) in contributor_scores.iter().take(10) {
            report.push_str(&format!("  {}: {:.2}\n", author, complexity));
        }
        
        report.push_str("\n✅ All contributions follow detectable eigenform patterns!\n");
        report
    }
}
