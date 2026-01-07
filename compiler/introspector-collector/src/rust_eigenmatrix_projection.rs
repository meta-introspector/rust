use crate::lattice_macros::LatticePointDerive;
use crate::github_ecosystem_lattice::GitHubEcosystem;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)]
pub struct RustEigenMatrix {
    pub core_modules: Vec<ModuleNode>,
    pub stdlib_decls: Vec<DeclNode>,
    pub canonical_asts: Vec<AstNode>,
    pub eigenvalues: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)]
pub struct ModuleNode {
    pub path: String,
    pub name: String,
    pub repo_source: Option<String>,
    pub in_eigenspace: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)]
pub struct DeclNode {
    pub name: String,
    pub decl_type: String, // fn, struct, enum, trait, etc
    pub module_path: String,
    pub in_eigenspace: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)]
pub struct AstNode {
    pub node_type: String,
    pub source_span: String,
    pub module_path: String,
    pub in_eigenspace: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)]
pub struct EcosystemProjection {
    pub repo_name: String,
    pub modules_in: u32,
    pub modules_out: u32,
    pub decls_in: u32,
    pub decls_out: u32,
    pub asts_in: u32,
    pub asts_out: u32,
    pub eigenspace_coverage: f64,
}

impl RustEigenMatrix {
    pub fn project_ecosystem(&self, ecosystem: &[GitHubEcosystem]) -> Vec<EcosystemProjection> {
        ecosystem.iter()
            .filter_map(|item| match item {
                GitHubEcosystem::Repository(repo) => {
                    Some(self.analyze_repo_projection(&repo.name))
                }
                _ => None
            })
            .collect()
    }
    
    fn analyze_repo_projection(&self, repo_name: &str) -> EcosystemProjection {
        let repo_modules: Vec<_> = self.core_modules.iter()
            .filter(|m| m.repo_source.as_ref() == Some(repo_name))
            .collect();
            
        let modules_in = repo_modules.iter().filter(|m| m.in_eigenspace).count() as u32;
        let modules_out = repo_modules.len() as u32 - modules_in;
        
        let repo_decls: Vec<_> = self.stdlib_decls.iter()
            .filter(|d| repo_modules.iter().any(|m| d.module_path.starts_with(&m.path)))
            .collect();
            
        let decls_in = repo_decls.iter().filter(|d| d.in_eigenspace).count() as u32;
        let decls_out = repo_decls.len() as u32 - decls_in;
        
        let repo_asts: Vec<_> = self.canonical_asts.iter()
            .filter(|a| repo_modules.iter().any(|m| a.module_path.starts_with(&m.path)))
            .collect();
            
        let asts_in = repo_asts.iter().filter(|a| a.in_eigenspace).count() as u32;
        let asts_out = repo_asts.len() as u32 - asts_in;
        
        let total_elements = modules_in + decls_in + asts_in;
        let total_possible = repo_modules.len() + repo_decls.len() + repo_asts.len();
        let eigenspace_coverage = if total_possible > 0 {
            total_elements as f64 / total_possible as f64
        } else { 0.0 };
        
        EcosystemProjection {
            repo_name: repo_name.to_string(),
            modules_in,
            modules_out,
            decls_in,
            decls_out,
            asts_in,
            asts_out,
            eigenspace_coverage,
        }
    }
    
    pub fn classify_in_out(&self) -> (Vec<String>, Vec<String>) {
        let in_eigenspace: Vec<String> = self.core_modules.iter()
            .filter(|m| m.in_eigenspace)
            .map(|m| m.name.clone())
            .collect();
            
        let out_eigenspace: Vec<String> = self.core_modules.iter()
            .filter(|m| !m.in_eigenspace)
            .map(|m| m.name.clone())
            .collect();
            
        (in_eigenspace, out_eigenspace)
    }
}

pub fn harvest_ecosystem_elements(ecosystem: &[GitHubEcosystem]) -> (Vec<ModuleNode>, Vec<DeclNode>, Vec<AstNode>) {
    let mut modules = Vec::new();
    let mut decls = Vec::new();
    let mut asts = Vec::new();
    
    for item in ecosystem {
        if let GitHubEcosystem::Repository(repo) = item {
            // Collect modules, decls, ASTs from each repo
            // This would integrate with rustc_interface to parse actual code
            modules.push(ModuleNode {
                path: format!("{}/{}", repo.owner, repo.name),
                name: repo.name.clone(),
                repo_source: Some(repo.name.clone()),
                in_eigenspace: false, // To be computed
            });
        }
    }
    
    (modules, decls, asts)
}
