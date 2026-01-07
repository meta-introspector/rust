use introspector_collector::{repos};
use introspector_collector::repository_enumification::*;

fn main() {
    println!("🌐 REPOSITORY ENUMIFICATION SYSTEM");
    println!("📦 Every git repo in rustc closure becomes an enum variant");
    
    // Build complete rustc repository closure
    let rustc_closure = repos!(rustc_closure);
    
    println!("\n🦀 Rustc Repository Closure Analysis:");
    println!("  📊 Total repositories: {}", rustc_closure.repositories.len());
    
    // Show repositories by type
    let rust_lang_repos = rustc_closure.get_by_type("rust-lang");
    println!("  🏛️  rust-lang org repos: {}", rust_lang_repos.len());
    for repo in &rust_lang_repos {
        println!("    {:?}", repo);
    }
    
    let external_repos = rustc_closure.get_by_type("external");
    println!("  🔗 External dependencies: {}", external_repos.len());
    for repo in &external_repos {
        println!("    {:?}", repo);
    }
    
    let build_system_repos = rustc_closure.get_by_type("build-system");
    println!("  🔧 Build system repos: {}", build_system_repos.len());
    for repo in &build_system_repos {
        println!("    {:?}", repo);
    }
    
    // Show all URLs
    println!("\n🌍 Complete Repository URLs:");
    let all_urls = rustc_closure.get_all_urls();
    for (i, url) in all_urls.iter().enumerate() {
        println!("  {}: {}", i + 1, url);
    }
    
    // Show submodules
    println!("\n📂 Submodule Mapping:");
    for (parent, submodules) in &rustc_closure.submodules {
        println!("  {} contains:", parent);
        for submodule in submodules {
            println!("    └── {}", submodule);
        }
    }
    
    // Show dependency graph
    println!("\n🔗 Dependency Graph:");
    for (repo, deps) in &rustc_closure.dependency_graph {
        println!("  {} depends on:", repo);
        for dep in deps {
            println!("    └── {}", dep);
        }
    }
    
    // Show build order
    println!("\n🏗️  Build Order:");
    for (i, repo) in rustc_closure.build_order.iter().enumerate() {
        println!("  {}: {}", i + 1, repo);
    }
    
    // Generate Nix sources
    println!("\n❄️  Nix Sources Generation:");
    let nix_sources = rustc_closure.generate_nix_sources();
    println!("Generated Nix sources (first 20 lines):");
    for line in nix_sources.lines().take(20) {
        println!("  {}", line);
    }
    
    // Macro examples
    println!("\n🔍 Repository Queries:");
    
    let rust_lang_macro = repos!(rust_lang);
    println!("  repos!(rust_lang): {} repositories", rust_lang_macro.len());
    
    let external_deps_macro = repos!(external_deps);
    println!("  repos!(external_deps): {} repositories", external_deps_macro.len());
    
    // Show random server examples
    println!("\n🌐 Random Git Servers:");
    let random_servers = vec![
        GitRepository::RandomServer(RandomServerRepo::Codeberg("rust-lang/rust".to_string())),
        GitRepository::RandomServer(RandomServerRepo::GitLab("rust-lang/rust".to_string())),
        GitRepository::RandomServer(RandomServerRepo::SourceHut("~user/rust".to_string())),
        GitRepository::RandomServer(RandomServerRepo::SelfHosted("git.company.com".to_string(), "rust".to_string())),
    ];
    
    for server in &random_servers {
        println!("  {:?}", server);
    }
    
    // Fork examples
    println!("\n🍴 Fork Repository Examples:");
    let forks = vec![
        GitRepository::Fork(ForkRepo::RustFork("mycompany".to_string(), "custom_patches".to_string())),
        GitRepository::Fork(ForkRepo::LlvmFork("myteam".to_string(), "optimization_patches".to_string())),
        GitRepository::Fork(ForkRepo::Custom("user".to_string(), "special-rust".to_string(), "experimental_features".to_string())),
    ];
    
    for fork in &forks {
        println!("  {:?}", fork);
    }
    
    println!("\n📊 Repository Enumification Summary:");
    println!("  🦀 rust-lang repos: {}", rust_lang_repos.len());
    println!("  🔗 External deps: {}", external_repos.len());
    println!("  🔧 Build system: {}", build_system_repos.len());
    println!("  🌐 Random servers: {}", random_servers.len());
    println!("  🍴 Forks: {}", forks.len());
    
    let total_repos = rust_lang_repos.len() + external_repos.len() + build_system_repos.len() + random_servers.len() + forks.len();
    println!("  📦 Total enumified repos: {}", total_repos);
    
    println!("\n✨ Repository Enumification Complete!");
    println!("🎯 Every git repo needed for rustc is now an enum variant");
    println!("🔍 Query with: repos!(rustc_closure)");
    println!("🏗️  Build order calculated from dependency graph");
    println!("❄️  Nix sources generated for reproducible builds");
    println!("🌐 Supports GitHub, Codeberg, GitLab, and random git servers");
    
    // Save generated files
    std::fs::create_dir_all("src/generated").ok();
    
    std::fs::write("src/generated/rustc_repository_closure.nix", nix_sources)
        .expect("Failed to write Nix sources");
    
    let build_order_json = serde_json::to_string_pretty(&rustc_closure.build_order)
        .expect("Failed to serialize build order");
    std::fs::write("src/generated/rustc_build_order.json", build_order_json)
        .expect("Failed to write build order");
    
    println!("💾 Repository closure saved to src/generated/");
}
