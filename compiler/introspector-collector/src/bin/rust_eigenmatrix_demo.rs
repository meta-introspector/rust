use crate::rust_eigenmatrix_projection::*;
use crate::github_ecosystem_lattice::GitHubEcosystem;

fn main() {
    println!("🦀 Rust Eigenmatrix Ecosystem Projection");
    
    // Load canonical Rust eigenmatrix
    let rust_eigen = RustEigenMatrix {
        core_modules: vec![
            ModuleNode {
                path: "std::collections".to_string(),
                name: "collections".to_string(),
                repo_source: Some("rust-lang/rust".to_string()),
                in_eigenspace: true,
            },
            ModuleNode {
                path: "std::io".to_string(),
                name: "io".to_string(),
                repo_source: Some("rust-lang/rust".to_string()),
                in_eigenspace: true,
            },
        ],
        stdlib_decls: vec![
            DeclNode {
                name: "Vec".to_string(),
                decl_type: "struct".to_string(),
                module_path: "std::vec".to_string(),
                in_eigenspace: true,
            },
        ],
        canonical_asts: vec![],
        eigenvalues: vec![1.0, 0.8, 0.6],
    };
    
    // Mock ecosystem data
    let ecosystem = vec![
        GitHubEcosystem::Repository(crate::github_ecosystem_lattice::GitHubRepository {
            name: "serde".to_string(),
            owner: "serde-rs".to_string(),
            url: "https://github.com/serde-rs/serde".to_string(),
            stars: 8500,
            forks: 750,
            last_commit_sha: Some("abc123".to_string()),
        }),
    ];
    
    // Project ecosystem onto eigenmatrix
    let projections = rust_eigen.project_ecosystem(&ecosystem);
    
    println!("\n📊 Ecosystem Projection Results:");
    for proj in projections {
        println!("Repository: {}", proj.repo_name);
        println!("  Modules IN eigenspace: {}", proj.modules_in);
        println!("  Modules OUT eigenspace: {}", proj.modules_out);
        println!("  Decls IN eigenspace: {}", proj.decls_in);
        println!("  Decls OUT eigenspace: {}", proj.decls_out);
        println!("  ASTs IN eigenspace: {}", proj.asts_in);
        println!("  ASTs OUT eigenspace: {}", proj.asts_out);
        println!("  Eigenspace coverage: {:.2}%", proj.eigenspace_coverage * 100.0);
    }
    
    // Show IN/OUT classification
    let (in_eigen, out_eigen) = rust_eigen.classify_in_out();
    println!("\n✅ IN Rust Eigenspace: {:?}", in_eigen);
    println!("❌ OUT Rust Eigenspace: {:?}", out_eigen);
}
