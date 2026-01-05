use std::collections::HashMap;

/// Hierarchical address space: Repo -> Crate -> Module -> Object
struct EcosystemAddressManager {
    repos: HashMap<String, RepoAddressSpace>,
    repo_bases: HashMap<String, u32>,
    next_repo_base: u32,
}

struct RepoAddressSpace {
    repo_name: String,
    crates: HashMap<String, CrateAddressSpace>,
    crate_bases: HashMap<String, u32>,
    next_crate_base: u32,
    base_address: u32,
}

struct CrateAddressSpace {
    crate_name: String,
    branches: HashMap<String, BranchAddressSpace>,
    branch_bases: HashMap<String, u32>,
    next_branch_base: u32,
    base_address: u32,
}

struct BranchAddressSpace {
    branch_name: String,
    modules: HashMap<String, ModuleAddressSpace>,
    module_bases: HashMap<String, u32>,
    next_module_base: u32,
    base_address: u32,
}

struct ModuleAddressSpace {
    objects: HashMap<u32, String>,
    size_buckets: HashMap<usize, Vec<u32>>,
    base_address: u32,
}

impl EcosystemAddressManager {
    fn new() -> Self {
        Self {
            repos: HashMap::new(),
            repo_bases: HashMap::new(),
            next_repo_base: 0x00000000,
        }
    }
    
    /// Create repository with 256MB address space
    fn create_repo(&mut self, repo_name: String) -> &mut RepoAddressSpace {
        let base = self.next_repo_base;
        self.repo_bases.insert(repo_name.clone(), base);
        self.next_repo_base += 0x10000000; // 256MB per repo
        
        let repo = RepoAddressSpace::new(repo_name.clone(), base);
        self.repos.insert(repo_name.clone(), repo);
        
        println!("Created repo '{}' at 0x{:08X}", repo_name, base);
        self.repos.get_mut(&repo_name).unwrap()
    }
    
    /// Decode full hierarchical address
    fn decode_address(&self, addr: u32) -> Option<(String, String, String, String, usize, u32)> {
        // Returns: (repo, crate, branch, module, size, index)
        
        for (repo_name, repo_base) in &self.repo_bases {
            if addr >= *repo_base && addr < *repo_base + 0x10000000 {
                if let Some(repo) = self.repos.get(repo_name) {
                    return repo.decode_address(addr);
                }
            }
        }
        None
    }
    
    /// Show ecosystem overview
    fn show_ecosystem(&self) {
        println!("\n=== RUST ECOSYSTEM ADDRESS SPACE ===\n");
        
        for (repo_name, repo) in &self.repos {
            let base = self.repo_bases[repo_name];
            println!("Repo '{}' (0x{:08X}):", repo_name, base);
            repo.show_summary();
            println!();
        }
    }
}

impl RepoAddressSpace {
    fn new(repo_name: String, base_address: u32) -> Self {
        Self {
            repo_name,
            crates: HashMap::new(),
            crate_bases: HashMap::new(),
            next_crate_base: base_address,
            base_address,
        }
    }
    
    /// Create crate with 16MB address space
    fn create_crate(&mut self, crate_name: String) -> &mut CrateAddressSpace {
        let base = self.next_crate_base;
        self.crate_bases.insert(crate_name.clone(), base);
        self.next_crate_base += 0x1000000; // 16MB per crate
        
        let crate_space = CrateAddressSpace::new(crate_name.clone(), base);
        self.crates.insert(crate_name.clone(), crate_space);
        
        println!("  Created crate '{}' at 0x{:08X}", crate_name, base);
        self.crates.get_mut(&crate_name).unwrap()
    }
    
    fn decode_address(&self, addr: u32) -> Option<(String, String, String, String, usize, u32)> {
        for (crate_name, crate_base) in &self.crate_bases {
            if addr >= *crate_base && addr < *crate_base + 0x1000000 {
                if let Some(crate_space) = self.crates.get(crate_name) {
                    if let Some((branch, module, size, index)) = crate_space.decode_address(addr) {
                        return Some((self.repo_name.clone(), crate_name.clone(), branch, module, size, index));
                    }
                }
            }
        }
        None
    }
    
    fn show_summary(&self) {
        for (crate_name, crate_space) in &self.crates {
            let base = self.crate_bases[crate_name];
            println!("    Crate '{}' (0x{:08X}): {} branches", 
                     crate_name, base, crate_space.branches.len());
        }
    }
}

impl CrateAddressSpace {
    fn new(crate_name: String, base_address: u32) -> Self {
        Self {
            crate_name,
            branches: HashMap::new(),
            branch_bases: HashMap::new(),
            next_branch_base: base_address,
            base_address,
        }
    }
    
    /// Create branch with 1MB address space
    fn create_branch(&mut self, branch_name: String) -> &mut BranchAddressSpace {
        let base = self.next_branch_base;
        self.branch_bases.insert(branch_name.clone(), base);
        self.next_branch_base += 0x100000; // 1MB per branch
        
        let branch_space = BranchAddressSpace::new(branch_name.clone(), base);
        self.branches.insert(branch_name.clone(), branch_space);
        
        println!("    Created branch '{}' at 0x{:08X}", branch_name, base);
        self.branches.get_mut(&branch_name).unwrap()
    }
    
    fn decode_address(&self, addr: u32) -> Option<(String, String, usize, u32)> {
        for (branch_name, branch_base) in &self.branch_bases {
            if addr >= *branch_base && addr < *branch_base + 0x100000 {
                if let Some(branch_space) = self.branches.get(branch_name) {
                    if let Some((module, size, index)) = branch_space.decode_address(addr) {
                        return Some((branch_name.clone(), module, size, index));
                    }
                }
            }
        }
        None
    }
}

impl BranchAddressSpace {
    fn new(branch_name: String, base_address: u32) -> Self {
        Self {
            branch_name,
            modules: HashMap::new(),
            module_bases: HashMap::new(),
            next_module_base: base_address,
            base_address,
        }
    }
    
    /// Create module with 64KB address space
    fn create_module(&mut self, module_name: String) -> &mut ModuleAddressSpace {
        let base = self.next_module_base;
        self.module_bases.insert(module_name.clone(), base);
        self.next_module_base += 0x10000; // 64KB per module
        
        let module_space = ModuleAddressSpace::new(base);
        self.modules.insert(module_name.clone(), module_space);
        
        println!("      Created module '{}' at 0x{:08X}", module_name, base);
        self.modules.get_mut(&module_name).unwrap()
    }
    
    fn decode_address(&self, addr: u32) -> Option<(String, usize, u32)> {
        for (module_name, module_base) in &self.module_bases {
            if addr >= *module_base && addr < *module_base + 0x10000 {
                if let Some(module_space) = self.modules.get(module_name) {
                    let local_addr = addr - *module_base;
                    let size = (local_addr >> 8) as usize;  // 8 bits for size
                    let index = local_addr & 0xFF;          // 8 bits for index
                    return Some((module_name.clone(), size, index));
                }
            }
        }
        None
    }
}

impl ModuleAddressSpace {
    fn new(base_address: u32) -> Self {
        Self {
            objects: HashMap::new(),
            size_buckets: HashMap::new(),
            base_address,
        }
    }
    
    fn allocate_object(&mut self, content: String) -> u32 {
        let size = content.len().min(255); // Max size 255 for 8-bit encoding
        let objects_in_bucket = self.size_buckets.entry(size).or_insert_with(Vec::new);
        
        let local_addr = ((size as u32) << 8) | (objects_in_bucket.len() as u32);
        let global_addr = self.base_address + local_addr;
        
        objects_in_bucket.push(global_addr);
        self.objects.insert(global_addr, content);
        
        global_addr
    }
}

fn main() {
    println!("=== RUST ECOSYSTEM HIERARCHICAL ADDRESSING ===");
    
    let mut ecosystem = EcosystemAddressManager::new();
    
    // Create rust-lang/rust repository
    let rust_repo = ecosystem.create_repo("rust-lang/rust".to_string());
    
    // Create rustc crate
    let rustc_crate = rust_repo.create_crate("rustc".to_string());
    
    // Create main and feature branches
    let main_branch = rustc_crate.create_branch("main".to_string());
    
    // Create modules in main branch
    let errors_module = main_branch.create_module("rustc_errors".to_string());
    errors_module.allocate_object("fn emit_err()".to_string());
    errors_module.allocate_object("struct DiagnosticBuilder".to_string());
    
    let middle_module = main_branch.create_module("rustc_middle".to_string());
    middle_module.allocate_object("struct DefId".to_string());
    middle_module.allocate_object("enum TyKind".to_string());
    
    // Create feature branch
    let feature_branch = rustc_crate.create_branch("feature/new-parser".to_string());
    
    // Create modules in feature branch
    let parser_module = feature_branch.create_module("rustc_parse".to_string());
    parser_module.allocate_object("fn parse_expr()".to_string());
    parser_module.allocate_object("struct Parser".to_string());
    
    // Create external crate
    let serde_repo = ecosystem.create_repo("serde-rs/serde".to_string());
    let serde_crate = serde_repo.create_crate("serde".to_string());
    let serde_main = serde_crate.create_branch("main".to_string());
    let serde_core = serde_main.create_module("core".to_string());
    serde_core.allocate_object("trait Serialize".to_string());
    
    // Show ecosystem
    ecosystem.show_ecosystem();
    
    // Test address decoding
    println!("=== ADDRESS DECODING EXAMPLES ===\n");
    let test_addresses = [0x00000000, 0x00000C00, 0x00100000, 0x10000000];
    
    for &addr in &test_addresses {
        if let Some((repo, crate_name, branch, module, size, index)) = ecosystem.decode_address(addr) {
            println!("0x{:08X} -> {}/{}/{}:{} [size:{}][{}]", 
                     addr, repo, crate_name, branch, module, size, index);
        } else {
            println!("0x{:08X} -> Unallocated", addr);
        }
    }
    
    println!("\n✓ Hierarchical ecosystem addressing complete");
    println!("✓ Repos, crates, branches, modules all addressable");
    println!("✓ Full semantic decoding from address alone");
}
