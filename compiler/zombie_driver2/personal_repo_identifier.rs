use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 PERSONAL REPOSITORY IDENTIFIER");
    println!("=================================");
    
    // Extract all .git/config files
    let gitconfig_output = std::process::Command::new("grep")
        .args(&[r"\.git/config$", "/mnt/data1/files.txt"])
        .output()?;
    
    let gitconfig_files = String::from_utf8(gitconfig_output.stdout)?;
    let gitconfig_paths: Vec<&str> = gitconfig_files.lines().collect();
    
    println!("📁 Analyzing {} .git/config files for personal repositories", gitconfig_paths.len());
    
    // Look for personal indicators
    let personal_indicators = vec![
        "h4ck3rm1k3",
        "mdupont", 
        "meta-introspector",
        "split-decls",
        "zombie",
        "introspector",
        "du-pont",
        "blickchain",
    ];
    
    let mut personal_repos = Vec::new();
    let mut repo_patterns = HashMap::new();
    
    // Process git configs
    for (i, gitconfig_path) in gitconfig_paths.iter().enumerate() {
        if i % 200 == 0 {
            println!("   Progress: {}/{}", i, gitconfig_paths.len());
        }
        
        let clean_path = gitconfig_path.strip_prefix("./").unwrap_or(gitconfig_path);
        let full_path = format!("/mnt/data1/{}", clean_path);
        
        if let Ok(content) = fs::read_to_string(&full_path) {
            if is_personal_repo(&content, &personal_indicators) {
                let repo_path = extract_repo_path(gitconfig_path);
                personal_repos.push((repo_path.clone(), gitconfig_path.to_string()));
                
                // Analyze patterns
                analyze_repo_patterns(&content, &repo_path, &mut repo_patterns);
            }
        }
    }
    
    // Also check .gitmodules for personal submodules
    let gitmodules_output = std::process::Command::new("grep")
        .args(&[r"\.gitmodules$", "/mnt/data1/files.txt"])
        .output()?;
    
    let gitmodules_files = String::from_utf8(gitmodules_output.stdout)?;
    let gitmodules_paths: Vec<&str> = gitmodules_files.lines().collect();
    
    println!("\n📁 Analyzing {} .gitmodules files for personal submodules", gitmodules_paths.len());
    
    for (i, gitmodules_path) in gitmodules_paths.iter().enumerate() {
        if i % 200 == 0 {
            println!("   Progress: {}/{}", i, gitmodules_paths.len());
        }
        
        let clean_path = gitmodules_path.strip_prefix("./").unwrap_or(gitmodules_path);
        let full_path = format!("/mnt/data1/{}", clean_path);
        
        if let Ok(content) = fs::read_to_string(&full_path) {
            if is_personal_repo(&content, &personal_indicators) {
                let repo_path = extract_repo_path(gitmodules_path);
                if !personal_repos.iter().any(|(path, _)| path == &repo_path) {
                    personal_repos.push((repo_path.clone(), gitmodules_path.to_string()));
                    analyze_repo_patterns(&content, &repo_path, &mut repo_patterns);
                }
            }
        }
    }
    
    // Look for recent activity indicators
    println!("\n🕒 CHECKING FOR RECENT ACTIVITY...");
    let mut recent_repos = Vec::new();
    
    for (repo_path, _) in &personal_repos {
        if has_recent_activity(repo_path) {
            recent_repos.push(repo_path.clone());
        }
    }
    
    // Report findings
    println!("\n📊 PERSONAL REPOSITORY ANALYSIS:");
    println!("================================");
    
    println!("\n🏠 IDENTIFIED PERSONAL REPOSITORIES: {}", personal_repos.len());
    for (repo_path, source) in &personal_repos {
        println!("   {} (from {})", repo_path, source);
    }
    
    println!("\n🕒 RECENTLY ACTIVE REPOSITORIES: {}", recent_repos.len());
    for repo_path in &recent_repos {
        println!("   {}", repo_path);
    }
    
    println!("\n🔍 REPOSITORY PATTERNS:");
    let mut sorted_patterns: Vec<_> = repo_patterns.iter().collect();
    sorted_patterns.sort_by(|a, b| b.1.cmp(a.1));
    
    for (pattern, count) in sorted_patterns.iter().take(10) {
        println!("   {}: {}", pattern, count);
    }
    
    // Look for specific project types
    println!("\n🎯 PROJECT TYPE ANALYSIS:");
    let mut project_types = HashMap::new();
    
    for (repo_path, _) in &personal_repos {
        let project_type = classify_project_type(repo_path);
        *project_types.entry(project_type).or_insert(0) += 1;
    }
    
    for (project_type, count) in &project_types {
        println!("   {}: {}", project_type, count);
    }
    
    // Check for split-decls and zombie projects specifically
    println!("\n✂️  SPLIT-DECLS PROJECTS:");
    let split_decls_repos: Vec<_> = personal_repos.iter()
        .filter(|(path, _)| path.contains("split") && path.contains("decl"))
        .collect();
    
    for (repo_path, _) in &split_decls_repos {
        println!("   {}", repo_path);
    }
    
    println!("\n🧟 ZOMBIE PROJECTS:");
    let zombie_repos: Vec<_> = personal_repos.iter()
        .filter(|(path, _)| path.contains("zombie"))
        .collect();
    
    for (repo_path, _) in &zombie_repos {
        println!("   {}", repo_path);
    }
    
    Ok(())
}

fn is_personal_repo(content: &str, indicators: &[&str]) -> bool {
    let content_lower = content.to_lowercase();
    
    for indicator in indicators {
        if content_lower.contains(&indicator.to_lowercase()) {
            return true;
        }
    }
    
    false
}

fn extract_repo_path(git_file_path: &str) -> String {
    // Extract repository path from .git/config or .gitmodules path
    if let Some(git_pos) = git_file_path.find("/.git/") {
        git_file_path[..git_pos].to_string()
    } else if let Some(gitmodules_pos) = git_file_path.find("/.gitmodules") {
        git_file_path[..gitmodules_pos].to_string()
    } else {
        git_file_path.to_string()
    }
}

fn analyze_repo_patterns(content: &str, repo_path: &str, patterns: &mut HashMap<String, usize>) {
    // Extract URL patterns
    for line in content.lines() {
        if line.contains("url =") {
            if let Some(url) = line.split("url =").nth(1) {
                let url = url.trim().trim_matches('"');
                if url.contains("github.com") {
                    *patterns.entry("GitHub".to_string()).or_insert(0) += 1;
                }
                if url.contains("gitlab") {
                    *patterns.entry("GitLab".to_string()).or_insert(0) += 1;
                }
                if url.contains("h4ck3rm1k3") {
                    *patterns.entry("h4ck3rm1k3 user".to_string()).or_insert(0) += 1;
                }
                if url.contains("meta-introspector") {
                    *patterns.entry("meta-introspector org".to_string()).or_insert(0) += 1;
                }
            }
        }
    }
    
    // Analyze path patterns
    if repo_path.contains("time") {
        *patterns.entry("Time-based organization".to_string()).or_insert(0) += 1;
    }
    if repo_path.contains("2024") || repo_path.contains("2025") {
        *patterns.entry("Recent years".to_string()).or_insert(0) += 1;
    }
    if repo_path.contains("experiments") {
        *patterns.entry("Experimental projects".to_string()).or_insert(0) += 1;
    }
}

fn has_recent_activity(repo_path: &str) -> bool {
    // Check if path indicates recent activity (2024/2025)
    repo_path.contains("2024") || repo_path.contains("2025") || 
    repo_path.contains("time2") || repo_path.contains("recent")
}

fn classify_project_type(repo_path: &str) -> String {
    if repo_path.contains("split") && repo_path.contains("decl") {
        "Split-Decls".to_string()
    } else if repo_path.contains("zombie") {
        "Zombie Driver".to_string()
    } else if repo_path.contains("rust") || repo_path.contains("cargo") {
        "Rust Project".to_string()
    } else if repo_path.contains("experiments") {
        "Experimental".to_string()
    } else if repo_path.contains("time") {
        "Time-organized".to_string()
    } else if repo_path.contains("meta") {
        "Meta-project".to_string()
    } else {
        "General".to_string()
    }
}
