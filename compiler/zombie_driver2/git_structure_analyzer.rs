use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔗 GIT REPOSITORY STRUCTURE ANALYZER");
    println!("====================================");
    
    // Extract .gitmodules files
    let gitmodules_output = std::process::Command::new("grep")
        .args(&[r"\.gitmodules$", "/mnt/data1/files.txt"])
        .output()?;
    
    let gitmodules_files = String::from_utf8(gitmodules_output.stdout)?;
    let gitmodules_paths: Vec<&str> = gitmodules_files.lines().collect();
    
    // Extract .git/config files  
    let gitconfig_output = std::process::Command::new("grep")
        .args(&[r"\.git/config$", "/mnt/data1/files.txt"])
        .output()?;
    
    let gitconfig_files = String::from_utf8(gitconfig_output.stdout)?;
    let gitconfig_paths: Vec<&str> = gitconfig_files.lines().collect();
    
    println!("📁 Found {} .gitmodules files", gitmodules_paths.len());
    println!("📁 Found {} .git/config files", gitconfig_paths.len());
    
    // Analyze .gitmodules for submodule patterns
    println!("\n🔍 ANALYZING GITMODULES...");
    let mut submodule_urls = HashMap::new();
    let mut submodule_paths = HashMap::new();
    
    for (i, gitmodules_path) in gitmodules_paths.iter().take(100).enumerate() {
        if i % 20 == 0 {
            println!("   Progress: {}/100", i);
        }
        
        let clean_path = gitmodules_path.strip_prefix("./").unwrap_or(gitmodules_path);
        let full_path = format!("/mnt/data1/{}", clean_path);
        
        if let Ok(content) = fs::read_to_string(&full_path) {
            analyze_gitmodules(&content, &mut submodule_urls, &mut submodule_paths);
        }
    }
    
    // Analyze .git/config for remote patterns
    println!("\n🔍 ANALYZING GIT CONFIGS...");
    let mut remote_urls = HashMap::new();
    let mut remote_hosts = HashMap::new();
    
    for (i, gitconfig_path) in gitconfig_paths.iter().take(100).enumerate() {
        if i % 20 == 0 {
            println!("   Progress: {}/100", i);
        }
        
        let clean_path = gitconfig_path.strip_prefix("./").unwrap_or(gitconfig_path);
        let full_path = format!("/mnt/data1/{}", clean_path);
        
        if let Ok(content) = fs::read_to_string(&full_path) {
            analyze_git_config(&content, &mut remote_urls, &mut remote_hosts);
        }
    }
    
    // Report findings
    println!("\n📊 SUBMODULE ANALYSIS:");
    println!("======================");
    
    println!("\n🔗 TOP SUBMODULE URLS:");
    let mut sorted_urls: Vec<_> = submodule_urls.iter().collect();
    sorted_urls.sort_by(|a, b| b.1.cmp(a.1));
    for (url, count) in sorted_urls.iter().take(10) {
        println!("   {} ({})", url, count);
    }
    
    println!("\n📂 TOP SUBMODULE PATHS:");
    let mut sorted_paths: Vec<_> = submodule_paths.iter().collect();
    sorted_paths.sort_by(|a, b| b.1.cmp(a.1));
    for (path, count) in sorted_paths.iter().take(10) {
        println!("   {} ({})", path, count);
    }
    
    println!("\n📊 REMOTE ANALYSIS:");
    println!("===================");
    
    println!("\n🌐 TOP REMOTE HOSTS:");
    let mut sorted_hosts: Vec<_> = remote_hosts.iter().collect();
    sorted_hosts.sort_by(|a, b| b.1.cmp(a.1));
    for (host, count) in sorted_hosts.iter().take(10) {
        println!("   {} ({})", host, count);
    }
    
    println!("\n🔗 TOP REMOTE URLS:");
    let mut sorted_remotes: Vec<_> = remote_urls.iter().collect();
    sorted_remotes.sort_by(|a, b| b.1.cmp(a.1));
    for (url, count) in sorted_remotes.iter().take(10) {
        println!("   {} ({})", url, count);
    }
    
    // Look for split-decls related repositories
    println!("\n✂️  SPLIT-DECLS REPOSITORIES:");
    let split_decls_submodules: Vec<_> = submodule_urls.keys()
        .filter(|url| url.contains("split") && url.contains("decl"))
        .collect();
    
    let split_decls_remotes: Vec<_> = remote_urls.keys()
        .filter(|url| url.contains("split") && url.contains("decl"))
        .collect();
    
    println!("   Submodules: {}", split_decls_submodules.len());
    for url in &split_decls_submodules {
        println!("     {}", url);
    }
    
    println!("   Remotes: {}", split_decls_remotes.len());
    for url in &split_decls_remotes {
        println!("     {}", url);
    }
    
    Ok(())
}

fn analyze_gitmodules(
    content: &str,
    submodule_urls: &mut HashMap<String, usize>,
    submodule_paths: &mut HashMap<String, usize>,
) {
    let lines: Vec<&str> = content.lines().collect();
    
    for line in lines {
        let line = line.trim();
        
        if line.starts_with("url =") {
            if let Some(url) = line.strip_prefix("url =").map(|s| s.trim()) {
                *submodule_urls.entry(url.to_string()).or_insert(0) += 1;
            }
        }
        
        if line.starts_with("path =") {
            if let Some(path) = line.strip_prefix("path =").map(|s| s.trim()) {
                *submodule_paths.entry(path.to_string()).or_insert(0) += 1;
            }
        }
    }
}

fn analyze_git_config(
    content: &str,
    remote_urls: &mut HashMap<String, usize>,
    remote_hosts: &mut HashMap<String, usize>,
) {
    let lines: Vec<&str> = content.lines().collect();
    
    for line in lines {
        let line = line.trim();
        
        if line.starts_with("url =") {
            if let Some(url) = line.strip_prefix("url =").map(|s| s.trim()) {
                *remote_urls.entry(url.to_string()).or_insert(0) += 1;
                
                // Extract host
                if let Some(host) = extract_host_from_url(url) {
                    *remote_hosts.entry(host).or_insert(0) += 1;
                }
            }
        }
    }
}

fn extract_host_from_url(url: &str) -> Option<String> {
    if url.starts_with("https://") {
        url.strip_prefix("https://")
            .and_then(|s| s.split('/').next())
            .map(|s| s.to_string())
    } else if url.starts_with("git@") {
        url.strip_prefix("git@")
            .and_then(|s| s.split(':').next())
            .map(|s| s.to_string())
    } else {
        None
    }
}
