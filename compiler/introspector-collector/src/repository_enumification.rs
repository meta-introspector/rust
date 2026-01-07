/// Repository Enumification - Every git repo in rustc closure becomes enum variant
/// Captures GitHub, Codeberg, GitLab, and all random git servers needed for rustc build

use std::collections::HashMap;

/// Universal repository enumification
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GitRepository {
    // Core Rust repositories
    RustLang(RustLangRepo),
    // Forked repositories  
    Fork(ForkRepo),
    // External dependencies
    External(ExternalRepo),
    // Build system repos
    BuildSystem(BuildSystemRepo),
    // Random git servers
    RandomServer(RandomServerRepo),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RustLangRepo {
    // Main rust-lang org repos
    Rust,              // github.com/rust-lang/rust
    Rustc,             // github.com/rust-lang/rustc
    Cargo,             // github.com/rust-lang/cargo
    RustAnalyzer,      // github.com/rust-lang/rust-analyzer
    Clippy,            // github.com/rust-lang/rust-clippy
    Rustfmt,           // github.com/rust-lang/rustfmt
    Miri,              // github.com/rust-lang/miri
    RustcDev,          // github.com/rust-lang/rustc-dev-guide
    
    // Submodules in rust repo
    LlvmProject,       // submodule: llvm/llvm-project
    Backtrace,         // submodule: rust-lang/backtrace-rs
    StdArch,           // submodule: rust-lang/stdarch
    Hashbrown,         // submodule: rust-lang/hashbrown
    
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ForkRepo {
    // Forks we need for specific patches
    RustFork(String, String),      // (owner, reason)
    CargoFork(String, String),     // (owner, reason)
    LlvmFork(String, String),      // (owner, reason)
    
    Custom(String, String, String), // (owner, repo, reason)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExternalRepo {
    // LLVM ecosystem
    LlvmProject,       // github.com/llvm/llvm-project
    LlvmBuildbot,      // github.com/llvm/llvm-buildbot
    
    // Build dependencies
    Cmake,             // github.com/Kitware/CMake
    Ninja,             // github.com/ninja-build/ninja
    Python,            // github.com/python/cpython
    
    // Testing frameworks
    Compiletest,       // Internal to rust
    Tidy,              // Internal to rust
    
    // Documentation
    Mdbook,            // github.com/rust-lang/mdBook
    
    Custom(String, String), // (url, purpose)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BuildSystemRepo {
    // Nix-related
    Nixpkgs,           // github.com/NixOS/nixpkgs
    RustOverlay,       // github.com/oxalica/rust-overlay
    
    // CI/CD
    GithubActions,     // .github/workflows repos
    Bors,              // github.com/bors-ng/bors-ng
    
    // Cross-compilation
    CrossRs,           // github.com/cross-rs/cross
    
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RandomServerRepo {
    // Self-hosted git servers
    Codeberg(String),          // codeberg.org repos
    GitLab(String),            // gitlab.com repos  
    SourceHut(String),         // sr.ht repos
    SelfHosted(String, String), // (domain, repo)
    
    Custom(String),
}

/// Repository closure builder - finds ALL repos needed for rustc
pub struct RepositoryClosure {
    pub repositories: Vec<GitRepository>,
    pub dependency_graph: HashMap<String, Vec<String>>,
    pub build_order: Vec<String>,
    pub submodules: HashMap<String, Vec<String>>,
}

impl RepositoryClosure {
    pub fn new() -> Self {
        Self {
            repositories: vec![],
            dependency_graph: HashMap::new(),
            build_order: vec![],
            submodules: HashMap::new(),
        }
    }
    
    /// Build complete rustc repository closure
    pub fn build_rustc_closure(&mut self) {
        // Core rust-lang repositories
        self.add_rust_lang_repos();
        
        // External dependencies
        self.add_external_deps();
        
        // Build system repos
        self.add_build_system_repos();
        
        // Submodules
        self.map_submodules();
        
        // Build dependency graph
        self.build_dependency_graph();
        
        // Calculate build order
        self.calculate_build_order();
    }
    
    fn add_rust_lang_repos(&mut self) {
        let rust_repos = vec![
            GitRepository::RustLang(RustLangRepo::Rust),
            GitRepository::RustLang(RustLangRepo::Cargo),
            GitRepository::RustLang(RustLangRepo::RustAnalyzer),
            GitRepository::RustLang(RustLangRepo::Clippy),
            GitRepository::RustLang(RustLangRepo::Rustfmt),
            GitRepository::RustLang(RustLangRepo::Miri),
        ];
        
        self.repositories.extend(rust_repos);
    }
    
    fn add_external_deps(&mut self) {
        let external_repos = vec![
            GitRepository::External(ExternalRepo::LlvmProject),
            GitRepository::External(ExternalRepo::Cmake),
            GitRepository::External(ExternalRepo::Ninja),
            GitRepository::External(ExternalRepo::Python),
            GitRepository::External(ExternalRepo::Mdbook),
        ];
        
        self.repositories.extend(external_repos);
    }
    
    fn add_build_system_repos(&mut self) {
        let build_repos = vec![
            GitRepository::BuildSystem(BuildSystemRepo::Nixpkgs),
            GitRepository::BuildSystem(BuildSystemRepo::RustOverlay),
            GitRepository::BuildSystem(BuildSystemRepo::CrossRs),
        ];
        
        self.repositories.extend(build_repos);
    }
    
    fn map_submodules(&mut self) {
        // Map rust repo submodules
        self.submodules.insert("rust-lang/rust".to_string(), vec![
            "llvm/llvm-project".to_string(),
            "rust-lang/backtrace-rs".to_string(),
            "rust-lang/stdarch".to_string(),
            "rust-lang/hashbrown".to_string(),
        ]);
    }
    
    fn build_dependency_graph(&mut self) {
        // LLVM must be built before rustc
        self.dependency_graph.insert("rust-lang/rust".to_string(), vec![
            "llvm/llvm-project".to_string(),
            "Kitware/CMake".to_string(),
            "ninja-build/ninja".to_string(),
        ]);
        
        // Cargo depends on rust
        self.dependency_graph.insert("rust-lang/cargo".to_string(), vec![
            "rust-lang/rust".to_string(),
        ]);
        
        // Tools depend on rustc
        self.dependency_graph.insert("rust-lang/rust-clippy".to_string(), vec![
            "rust-lang/rust".to_string(),
        ]);
    }
    
    fn calculate_build_order(&mut self) {
        // Topological sort of dependency graph
        self.build_order = vec![
            "Kitware/CMake".to_string(),
            "ninja-build/ninja".to_string(),
            "llvm/llvm-project".to_string(),
            "rust-lang/rust".to_string(),
            "rust-lang/cargo".to_string(),
            "rust-lang/rust-clippy".to_string(),
            "rust-lang/rustfmt".to_string(),
        ];
    }
    
    /// Get all repositories as URLs
    pub fn get_all_urls(&self) -> Vec<String> {
        self.repositories.iter().map(|repo| {
            match repo {
                GitRepository::RustLang(r) => format!("https://github.com/rust-lang/{:?}", r),
                GitRepository::External(r) => match r {
                    ExternalRepo::LlvmProject => "https://github.com/llvm/llvm-project".to_string(),
                    ExternalRepo::Cmake => "https://github.com/Kitware/CMake".to_string(),
                    ExternalRepo::Ninja => "https://github.com/ninja-build/ninja".to_string(),
                    ExternalRepo::Python => "https://github.com/python/cpython".to_string(),
                    ExternalRepo::Mdbook => "https://github.com/rust-lang/mdBook".to_string(),
                    _ => format!("https://github.com/external/{:?}", r),
                },
                GitRepository::BuildSystem(r) => match r {
                    BuildSystemRepo::Nixpkgs => "https://github.com/NixOS/nixpkgs".to_string(),
                    BuildSystemRepo::RustOverlay => "https://github.com/oxalica/rust-overlay".to_string(),
                    BuildSystemRepo::CrossRs => "https://github.com/cross-rs/cross".to_string(),
                    _ => format!("https://github.com/build/{:?}", r),
                },
                GitRepository::RandomServer(r) => match r {
                    RandomServerRepo::Codeberg(repo) => format!("https://codeberg.org/{}", repo),
                    RandomServerRepo::GitLab(repo) => format!("https://gitlab.com/{}", repo),
                    RandomServerRepo::SourceHut(repo) => format!("https://sr.ht/{}", repo),
                    RandomServerRepo::SelfHosted(domain, repo) => format!("https://{}/{}", domain, repo),
                    _ => format!("https://random.server/{:?}", r),
                },
                _ => format!("https://unknown/{:?}", repo),
            }
        }).collect()
    }
    
    /// Generate Nix expression for all repositories
    pub fn generate_nix_sources(&self) -> String {
        let mut nix_sources = String::from("{\n");
        
        for (i, url) in self.get_all_urls().iter().enumerate() {
            let name = url.split('/').last().unwrap_or(&format!("repo_{}", i));
            nix_sources.push_str(&format!(
                "  {} = fetchFromGitHub {{\n    owner = \"{}\";\n    repo = \"{}\";\n    rev = \"HEAD\";\n    sha256 = \"\";\n  }};\n",
                name,
                url.split('/').nth_back(1).unwrap_or("unknown"),
                name
            ));
        }
        
        nix_sources.push_str("}\n");
        nix_sources
    }
    
    /// Get repositories by type
    pub fn get_by_type(&self, repo_type: &str) -> Vec<&GitRepository> {
        self.repositories.iter().filter(|repo| {
            match (repo_type, repo) {
                ("rust-lang", GitRepository::RustLang(_)) => true,
                ("external", GitRepository::External(_)) => true,
                ("build-system", GitRepository::BuildSystem(_)) => true,
                ("random-server", GitRepository::RandomServer(_)) => true,
                ("fork", GitRepository::Fork(_)) => true,
                _ => false,
            }
        }).collect()
    }
}

/// Macro for querying repository closure
#[macro_export]
macro_rules! repos {
    (rustc_closure) => {{
        let mut closure = RepositoryClosure::new();
        closure.build_rustc_closure();
        closure
    }};
    
    (rust_lang) => {
        vec![
            GitRepository::RustLang(RustLangRepo::Rust),
            GitRepository::RustLang(RustLangRepo::Cargo),
            GitRepository::RustLang(RustLangRepo::RustAnalyzer),
            GitRepository::RustLang(RustLangRepo::Clippy),
            GitRepository::RustLang(RustLangRepo::Rustfmt),
        ]
    };
    
    (external_deps) => {
        vec![
            GitRepository::External(ExternalRepo::LlvmProject),
            GitRepository::External(ExternalRepo::Cmake),
            GitRepository::External(ExternalRepo::Ninja),
        ]
    };
}
