use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌐 META-INTROSPECTOR TLD DIRECTORY ORGANIZER");
    println!("============================================");
    
    let base_path = "/mnt/data1/meta-introspector";
    
    // Create TLD-based directory structure from our analysis
    let tld_data = vec![
        ("com", "Commercial domains - GitHub, Google, enterprise"),
        ("org", "Organizations - Freedesktop, GNU, open source"),
        ("co", "Modern startups - HuggingFace, tech companies"),
        ("fr", "French domains - INRIA, French research"),
        ("ht", "Haiti domains - specialized projects"),
        ("cz", "Czech domains - regional projects"),
        ("net", "Network domains - infrastructure"),
        ("dev", "Developer domains - Google dev projects"),
        ("io", "Input/Output domains - Crates.io, tech"),
        ("me", "Personal domains - individual projects"),
        ("de", "German domains - German tech"),
        ("edu", "Educational domains - universities"),
        ("us", "US government domains"),
    ];
    
    // Create directory structure
    for (tld, description) in &tld_data {
        let tld_path = format!("{}/{}", base_path, tld);
        fs::create_dir_all(&tld_path)?;
        
        // Create README for each TLD
        let readme_content = format!(
            "# .{} Domain Overview\n\n{}\n\n## Statistics from Analysis:\n\n",
            tld, description
        );
        
        let readme_path = format!("{}/README.md", tld_path);
        fs::write(&readme_path, readme_content)?;
        
        println!("📁 Created: {} - {}", tld, description);
    }
    
    // Create specific subdirectories for major hosts
    let major_hosts = vec![
        ("com/github", "GitHub repositories - 55,752 repos"),
        ("com/googlesource", "Google source repositories - Chromium, Android"),
        ("co/huggingface", "HuggingFace AI/ML models - 115 repos"),
        ("org/freedesktop", "Freedesktop.org - Desktop Linux ecosystem"),
        ("org/gitlab", "GitLab repositories - 90 repos"),
        ("org/gnu", "GNU project repositories"),
        ("org/qemu", "QEMU virtualization project"),
        ("org/scheme", "Scheme language projects"),
        ("org/blender", "Blender 3D projects"),
        ("io/crates", "Rust crates ecosystem"),
    ];
    
    for (host_path, description) in &major_hosts {
        let full_path = format!("{}/{}", base_path, host_path);
        fs::create_dir_all(&full_path)?;
        
        let readme_content = format!(
            "# {} Overview\n\n{}\n\n## Repository Analysis\n\nThis directory contains analysis and metadata for repositories from this host.\n\n",
            host_path, description
        );
        
        let readme_path = format!("{}/README.md", full_path);
        fs::write(&readme_path, readme_content)?;
        
        println!("📂 Created: {} - {}", host_path, description);
    }
    
    // Create special directories for our analysis
    let analysis_dirs = vec![
        ("analysis", "Comprehensive analysis results and reports"),
        ("split-decls", "Split declarations canonical form projects"),
        ("rust-ecosystem", "Rust-specific analysis and tools"),
        ("tld-stats", "Top-level domain statistics and breakdowns"),
    ];
    
    for (dir_name, description) in &analysis_dirs {
        let dir_path = format!("{}/{}", base_path, dir_name);
        fs::create_dir_all(&dir_path)?;
        
        let readme_content = format!(
            "# {} Directory\n\n{}\n\n## Contents\n\nThis directory contains analysis results and tools related to {}.\n\n",
            dir_name, description, dir_name
        );
        
        let readme_path = format!("{}/README.md", dir_path);
        fs::write(&readme_path, readme_content)?;
        
        println!("🔧 Created: {} - {}", dir_name, description);
    }
    
    // Update main README with directory structure
    let main_readme = format!("{}/README.md", base_path);
    let updated_readme = format!(
        "# Meta-Introspector Repository Analysis\n\n\
        Comprehensive analysis of 57,106 domains across 33.9M files.\n\n\
        ## Directory Structure\n\n\
        ### Top-Level Domains (TLDs)\n\
        - `com/` - Commercial domains (98.3% - 56,155 repos)\n\
        - `org/` - Organizations (1.4% - 775 repos)\n\
        - `co/` - Modern startups (0.2% - 123 repos)\n\
        - `fr/`, `cz/`, `de/` - Regional domains\n\
        - `io/`, `dev/`, `net/` - Tech-focused domains\n\
        - `edu/`, `us/` - Educational and government\n\n\
        ### Major Repository Hosts\n\
        - `com/github/` - GitHub (55,752 repositories - 97.6%)\n\
        - `com/googlesource/` - Google projects (Chromium, Android)\n\
        - `co/huggingface/` - AI/ML models (115 repositories)\n\
        - `org/freedesktop/` - Desktop Linux (472 repositories)\n\
        - `org/gitlab/` - GitLab projects (90 repositories)\n\n\
        ### Analysis Results\n\
        - `analysis/` - Comprehensive analysis reports\n\
        - `split-decls/` - Split declarations projects (13 found!)\n\
        - `rust-ecosystem/` - Rust-specific analysis (42K Cargo.toml, 1.47M .rs files)\n\
        - `tld-stats/` - Domain statistics and breakdowns\n\n\
        ## Key Findings\n\n\
        - **GitHub Dominance**: 97.6% of repositories hosted on GitHub\n\
        - **Split-Decls Active**: 13 repositories using split-decls-rs\n\
        - **Massive Rust Ecosystem**: 1.47M Rust files, 42K projects\n\
        - **Enterprise Presence**: Google, GNU, Freedesktop integration\n\n\
        ## Tools and Analysis\n\n\
        See `tools/` directory for analysis scripts and utilities.\n"
    );
    
    fs::write(&main_readme, updated_readme)?;
    println!("📝 Updated main README.md with directory structure");
    
    // Create a summary file with our TLD analysis results
    let tld_summary = format!("{}/tld-stats/SUMMARY.md", base_path);
    let summary_content = format!(
        "# TLD Analysis Summary\n\n\
        ## Total Scale\n\
        - **57,106 domains** analyzed\n\
        - **1,713 .gitmodules** files\n\
        - **1,745 .git/config** files\n\n\
        ## Top-Level Domain Distribution\n\
        1. .com: 56,155 (98.3%)\n\
        2. .org: 775 (1.4%)\n\
        3. .co: 123 (0.2%)\n\
        4. Others: <0.1%\n\n\
        ## Major Repository Hosts\n\
        1. github.com: 55,752 (97.6%)\n\
        2. anongit.freedesktop.org: 472\n\
        3. chromium.googlesource.com: 188\n\
        4. huggingface.co: 115\n\
        5. gitlab.com: 90\n\n\
        ## Split-Decls Discovery\n\
        - **13 repositories** using split-decls-rs\n\
        - All pointing to: github.com/meta-introspector/split-decls-rs\n\
        - **Active deployment confirmed**\n\n\
        Generated: {}\n",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    );
    
    fs::write(&tld_summary, summary_content)?;
    println!("📊 Created TLD analysis summary");
    
    println!("\n✅ Meta-introspector directory structure organized!");
    println!("📁 {} TLD directories created", tld_data.len());
    println!("📂 {} host-specific directories created", major_hosts.len());
    println!("🔧 {} analysis directories created", analysis_dirs.len());
    
    Ok(())
}
