use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌐 MASTER TLD ANALYZER");
    println!("======================");
    
    // Extract all .gitmodules files
    let gitmodules_output = std::process::Command::new("grep")
        .args(&[r"\.gitmodules$", "/mnt/data1/files.txt"])
        .output()?;
    
    let gitmodules_files = String::from_utf8(gitmodules_output.stdout)?;
    let gitmodules_paths: Vec<&str> = gitmodules_files.lines().collect();
    
    // Extract all .git/config files  
    let gitconfig_output = std::process::Command::new("grep")
        .args(&[r"\.git/config$", "/mnt/data1/files.txt"])
        .output()?;
    
    let gitconfig_files = String::from_utf8(gitconfig_output.stdout)?;
    let gitconfig_paths: Vec<&str> = gitconfig_files.lines().collect();
    
    println!("📁 Processing {} .gitmodules files", gitmodules_paths.len());
    println!("📁 Processing {} .git/config files", gitconfig_paths.len());
    
    let mut tld_counts = HashMap::new();
    let mut full_domains = HashMap::new();
    let mut split_decls_repos = Vec::new();
    
    // Process ALL .gitmodules files
    println!("\n🔍 PROCESSING ALL GITMODULES...");
    for (i, gitmodules_path) in gitmodules_paths.iter().enumerate() {
        if i % 200 == 0 {
            println!("   Progress: {}/{}", i, gitmodules_paths.len());
        }
        
        let clean_path = gitmodules_path.strip_prefix("./").unwrap_or(gitmodules_path);
        let full_path = format!("/mnt/data1/{}", clean_path);
        
        if let Ok(content) = fs::read_to_string(&full_path) {
            extract_domains_from_content(&content, &mut tld_counts, &mut full_domains, &mut split_decls_repos);
        }
    }
    
    // Process ALL .git/config files
    println!("\n🔍 PROCESSING ALL GIT CONFIGS...");
    for (i, gitconfig_path) in gitconfig_paths.iter().enumerate() {
        if i % 200 == 0 {
            println!("   Progress: {}/{}", i, gitconfig_paths.len());
        }
        
        let clean_path = gitconfig_path.strip_prefix("./").unwrap_or(gitconfig_path);
        let full_path = format!("/mnt/data1/{}", clean_path);
        
        if let Ok(content) = fs::read_to_string(&full_path) {
            extract_domains_from_content(&content, &mut tld_counts, &mut full_domains, &mut split_decls_repos);
        }
    }
    
    // Report comprehensive TLD analysis
    println!("\n🌐 MASTER TLD ANALYSIS:");
    println!("=======================");
    
    println!("\n🏆 TOP LEVEL DOMAINS:");
    let mut sorted_tlds: Vec<_> = tld_counts.iter().collect();
    sorted_tlds.sort_by(|a, b| b.1.cmp(a.1));
    
    let total_domains: usize = tld_counts.values().sum();
    println!("Total domains found: {}", total_domains);
    
    for (tld, count) in sorted_tlds.iter().take(20) {
        let percentage = (**count as f64 / total_domains as f64) * 100.0;
        println!("   .{}: {} ({:.1}%)", tld, count, percentage);
    }
    
    println!("\n🌍 FULL DOMAIN BREAKDOWN:");
    let mut sorted_domains: Vec<_> = full_domains.iter().collect();
    sorted_domains.sort_by(|a, b| b.1.cmp(a.1));
    
    for (domain, count) in sorted_domains.iter().take(25) {
        println!("   {}: {}", domain, count);
    }
    
    println!("\n✂️  SPLIT-DECLS REPOSITORIES:");
    println!("Found {} split-decls related repositories:", split_decls_repos.len());
    for repo in &split_decls_repos {
        println!("   {}", repo);
    }
    
    // Look for specific patterns
    println!("\n🔍 PATTERN ANALYSIS:");
    let rust_domains: Vec<_> = full_domains.iter()
        .filter(|(domain, _)| domain.contains("rust") || domain.contains("crates"))
        .collect();
    
    let github_count = full_domains.get("github.com").unwrap_or(&0);
    let gitlab_count = full_domains.get("gitlab.com").unwrap_or(&0);
    let bitbucket_count = full_domains.get("bitbucket.org").unwrap_or(&0);
    
    println!("   GitHub dominance: {} repositories", github_count);
    println!("   GitLab presence: {} repositories", gitlab_count);
    println!("   Bitbucket presence: {} repositories", bitbucket_count);
    println!("   Rust-related domains: {}", rust_domains.len());
    
    for (domain, count) in rust_domains {
        println!("     {}: {}", domain, count);
    }
    
    Ok(())
}

fn extract_domains_from_content(
    content: &str,
    tld_counts: &mut HashMap<String, usize>,
    full_domains: &mut HashMap<String, usize>,
    split_decls_repos: &mut Vec<String>,
) {
    let lines: Vec<&str> = content.lines().collect();
    
    for line in lines {
        let line = line.trim();
        
        // Look for URLs in various formats
        if line.contains("url =") || line.contains("://") || line.contains("git@") {
            if let Some(domain) = extract_domain_from_line(line) {
                *full_domains.entry(domain.clone()).or_insert(0) += 1;
                
                // Extract TLD
                if let Some(tld) = extract_tld(&domain) {
                    *tld_counts.entry(tld).or_insert(0) += 1;
                }
                
                // Check for split-decls
                if line.contains("split") && (line.contains("decl") || line.contains("declaration")) {
                    split_decls_repos.push(line.to_string());
                }
            }
        }
    }
}

fn extract_domain_from_line(line: &str) -> Option<String> {
    // Handle various URL formats
    if let Some(url) = line.strip_prefix("url =").map(|s| s.trim().trim_matches('"')) {
        extract_domain_from_url(url)
    } else if line.contains("://") {
        // Direct URL in line
        if let Some(start) = line.find("://") {
            let start_pos = if start >= 10 { start - 10 } else { 0 };
            let url_part = &line[start_pos..].trim_start();
            extract_domain_from_url(url_part)
        } else {
            None
        }
    } else if line.contains("git@") {
        // SSH format: git@domain:user/repo
        if let Some(start) = line.find("git@") {
            let ssh_part = &line[start+4..];
            if let Some(colon_pos) = ssh_part.find(':') {
                Some(ssh_part[..colon_pos].to_string())
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn extract_domain_from_url(url: &str) -> Option<String> {
    if url.starts_with("https://") {
        url.strip_prefix("https://")
            .and_then(|s| s.split('/').next())
            .map(|s| s.to_string())
    } else if url.starts_with("http://") {
        url.strip_prefix("http://")
            .and_then(|s| s.split('/').next())
            .map(|s| s.to_string())
    } else if url.starts_with("git://") {
        url.strip_prefix("git://")
            .and_then(|s| s.split('/').next())
            .map(|s| s.to_string())
    } else {
        None
    }
}

fn extract_tld(domain: &str) -> Option<String> {
    domain.split('.').last().map(|s| s.to_string())
}
