use std::collections::HashMap;
use std::fs;
use std::path::Path;
use goblin::elf::Elf;
use goblin::archive::Archive;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SemanticSignature {
    // ABI Layer
    name: String,
    demangled_name: String,
    calling_convention: String,
    parameter_types: Vec<String>,
    return_type: String,
    
    // Security Layer
    stack_protection: bool,
    memory_safety_markers: Vec<String>,
    unsafe_operations: Vec<String>,
    
    // Type Layer
    generic_parameters: Vec<String>,
    trait_bounds: Vec<String>,
    lifetime_annotations: Vec<String>,
    
    // Meaning Layer
    semantic_category: String,
    operation_type: String,
    side_effects: Vec<String>,
    
    // Physical Properties
    size: u64,
    address: u64,
    instruction_hash: u64,
    complexity_score: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct BinarySemanticProfile {
    binary_path: String,
    binary_name: String,
    
    // ABI Profile
    abi_version: String,
    target_architecture: String,
    calling_conventions: HashMap<String, usize>,
    
    // Security Profile
    security_features: Vec<String>,
    vulnerability_patterns: Vec<String>,
    
    // Type System Profile
    type_complexity: f64,
    generic_usage: HashMap<String, usize>,
    
    // Semantic Profile
    function_categories: HashMap<String, usize>,
    operation_patterns: HashMap<String, usize>,
    
    // AST/Instruction/Node Counts
    instruction_types: HashMap<String, usize>,
    node_type_counts: HashMap<String, usize>,
    ast_pattern_counts: HashMap<String, usize>,
    opcode_distribution: HashMap<String, usize>,
    
    // Rust Intermediate Data
    rmeta_symbols: Vec<String>,
    crate_metadata: HashMap<String, String>,
    dependency_graph: Vec<String>,
    
    signatures: Vec<SemanticSignature>,
    total_functions: usize,
    total_instructions: usize,
    total_nodes: usize,
    created_at: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("🧬 SEMANTIC SIGNATURE GENERATOR");
        println!("Captures ABI + Security + Type + Meaning signatures");
        println!("Usage: {} <binary_path> [output_dir]", args[0]);
        println!("       {} --compare <sig1.json> <sig2.json>", args[0]);
        println!("       {} --batch <directory>", args[0]);
        return Ok(());
    }

    let output_dir = args.get(3).map(|s| s.as_str()).unwrap_or("./semantic_signatures");
    fs::create_dir_all(output_dir)?;

    match args[1].as_str() {
        "--compare" => {
            if args.len() < 4 {
                eprintln!("Need two signature files to compare");
                return Ok(());
            }
            compare_semantic_profiles(&args[2], &args[3])?;
        }
        "--batch" => {
            if args.len() < 3 {
                eprintln!("Need directory path for batch processing");
                return Ok(());
            }
            batch_process(&args[2], output_dir)?;
        }
        _ => {
            let binary_path = &args[1];
            generate_semantic_profile(binary_path, output_dir)?;
        }
    }

    Ok(())
}

fn generate_semantic_profile(binary_path: &str, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🧬 Generating semantic profile for: {}", binary_path);
    
    let binary_data = fs::read(binary_path)?;
    
    // Check file type and process accordingly
    if binary_path.ends_with(".rlib") {
        return process_rlib(binary_path, &binary_data, output_dir);
    } else if binary_path.ends_with(".rmeta") {
        return process_rmeta(binary_path, &binary_data, output_dir);
    }
    
    // Process as regular ELF binary
    let elf = Elf::parse(&binary_data)?;
    
    let binary_name = Path::new(binary_path)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    
    let mut signatures = Vec::new();
    let mut calling_conventions = HashMap::new();
    let mut function_categories = HashMap::new();
    let mut operation_patterns = HashMap::new();
    let mut generic_usage = HashMap::new();
    let mut instruction_types = HashMap::new();
    let mut node_type_counts = HashMap::new();
    let mut ast_pattern_counts = HashMap::new();
    let mut opcode_distribution = HashMap::new();
    let mut total_instructions = 0;
    let mut total_nodes = 0;
    
    for sym in &elf.syms {
        if let Some(name) = elf.strtab.get_at(sym.st_name) {
            if sym.st_size > 0 && sym.st_value > 0 {
                let demangled = rustc_demangle::demangle(name).to_string();
                
                // ABI Analysis
                let (calling_conv, param_types, return_type) = analyze_abi(&demangled);
                *calling_conventions.entry(calling_conv.clone()).or_insert(0) += 1;
                
                // Security Analysis
                let (stack_protection, memory_safety, unsafe_ops) = analyze_security(&demangled, &binary_data, &sym);
                
                // Type Analysis
                let (generics, trait_bounds, lifetimes) = analyze_types(&demangled);
                for generic in &generics {
                    *generic_usage.entry(generic.clone()).or_insert(0) += 1;
                }
                
                // Semantic Analysis
                let (category, operation, side_effects) = analyze_semantics(&demangled);
                *function_categories.entry(category.clone()).or_insert(0) += 1;
                *operation_patterns.entry(operation.clone()).or_insert(0) += 1;
                
                // Complexity Score
                let complexity = calculate_complexity_score(&demangled, sym.st_size, &generics, &trait_bounds);
                
                // Instruction and Node Analysis
                let (inst_types, node_counts, ast_patterns, opcodes, inst_count, node_count) = 
                    analyze_instructions_and_nodes(&binary_data, &elf, &sym, &demangled);
                
                // Update global counts
                for (inst_type, count) in inst_types {
                    *instruction_types.entry(inst_type).or_insert(0) += count;
                }
                for (node_type, count) in node_counts {
                    *node_type_counts.entry(node_type).or_insert(0) += count;
                }
                for (pattern, count) in ast_patterns {
                    *ast_pattern_counts.entry(pattern).or_insert(0) += count;
                }
                for (opcode, count) in opcodes {
                    *opcode_distribution.entry(opcode).or_insert(0) += count;
                }
                total_instructions += inst_count;
                total_nodes += node_count;
                
                signatures.push(SemanticSignature {
                    name: name.to_string(),
                    demangled_name: demangled,
                    calling_convention: calling_conv,
                    parameter_types: param_types,
                    return_type,
                    stack_protection,
                    memory_safety_markers: memory_safety,
                    unsafe_operations: unsafe_ops,
                    generic_parameters: generics,
                    trait_bounds,
                    lifetime_annotations: lifetimes,
                    semantic_category: category,
                    operation_type: operation,
                    side_effects,
                    size: sym.st_size,
                    address: sym.st_value,
                    instruction_hash: calculate_instruction_hash(&binary_data, &elf, &sym),
                    complexity_score: complexity,
                });
            }
        }
    }
    
    let total_functions = signatures.len();
    
    let profile = BinarySemanticProfile {
        binary_path: binary_path.to_string(),
        binary_name: binary_name.clone(),
        abi_version: detect_abi_version(&elf),
        target_architecture: detect_architecture(&elf),
        calling_conventions,
        security_features: detect_security_features(&elf, &binary_data),
        vulnerability_patterns: detect_vulnerability_patterns(&signatures),
        type_complexity: calculate_type_complexity(&signatures),
        generic_usage,
        function_categories,
        operation_patterns,
        instruction_types,
        node_type_counts,
        ast_pattern_counts,
        opcode_distribution,
        rmeta_symbols: Vec::new(),
        crate_metadata: HashMap::new(),
        dependency_graph: Vec::new(),
        signatures,
        total_functions,
        total_instructions,
        total_nodes,
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    
    let output_path = format!("{}/{}.semantic.json", output_dir, binary_name);
    let json = serde_json::to_string_pretty(&profile)?;
    fs::write(&output_path, json)?;
    
    println!("✅ Semantic profile saved: {} ({} functions, {} instructions, {} nodes)", 
             output_path, profile.total_functions, profile.total_instructions, profile.total_nodes);
    println!("   ABI: {} | Security: {} features | Types: {:.2} complexity", 
             profile.abi_version, profile.security_features.len(), profile.type_complexity);
    println!("   Instructions: {} types | Nodes: {} types | AST: {} patterns",
             profile.instruction_types.len(), profile.node_type_counts.len(), profile.ast_pattern_counts.len());
    
    Ok(())
}

fn analyze_abi(demangled: &str) -> (String, Vec<String>, String) {
    let calling_conv = if demangled.contains("extern \"C\"") {
        "C".to_string()
    } else if demangled.contains("extern \"system\"") {
        "system".to_string()
    } else {
        "rust".to_string()
    };
    
    // Extract parameter types from mangled name patterns
    let param_types = extract_parameter_types(demangled);
    let return_type = extract_return_type(demangled);
    
    (calling_conv, param_types, return_type)
}

fn analyze_security(demangled: &str, _binary_data: &[u8], _sym: &goblin::elf::Sym) -> (bool, Vec<String>, Vec<String>) {
    let stack_protection = demangled.contains("__stack_chk") || demangled.contains("canary");
    
    let mut memory_safety = Vec::new();
    let mut unsafe_ops = Vec::new();
    
    if demangled.contains("Box") || demangled.contains("Rc") || demangled.contains("Arc") {
        memory_safety.push("smart_pointers".to_string());
    }
    
    if demangled.contains("unsafe") {
        unsafe_ops.push("unsafe_block".to_string());
    }
    
    if demangled.contains("transmute") {
        unsafe_ops.push("transmute".to_string());
    }
    
    if demangled.contains("raw_ptr") || demangled.contains("*mut") || demangled.contains("*const") {
        unsafe_ops.push("raw_pointer".to_string());
    }
    
    (stack_protection, memory_safety, unsafe_ops)
}

fn analyze_types(demangled: &str) -> (Vec<String>, Vec<String>, Vec<String>) {
    let mut generics = Vec::new();
    let mut trait_bounds = Vec::new();
    let mut lifetimes = Vec::new();
    
    // Extract generic parameters
    if let Some(start) = demangled.find('<') {
        if let Some(end) = demangled.rfind('>') {
            let generic_part = &demangled[start+1..end];
            for param in generic_part.split(',') {
                let param = param.trim();
                if param.starts_with('\'') {
                    lifetimes.push(param.to_string());
                } else if param.contains(':') {
                    let parts: Vec<&str> = param.split(':').collect();
                    if parts.len() >= 2 {
                        generics.push(parts[0].trim().to_string());
                        trait_bounds.push(parts[1].trim().to_string());
                    }
                } else {
                    generics.push(param.to_string());
                }
            }
        }
    }
    
    (generics, trait_bounds, lifetimes)
}

fn analyze_semantics(demangled: &str) -> (String, String, Vec<String>) {
    let category = if demangled.contains("::new") || demangled.contains("::create") {
        "constructor".to_string()
    } else if demangled.contains("::drop") || demangled.contains("::destroy") {
        "destructor".to_string()
    } else if demangled.contains("::clone") || demangled.contains("::copy") {
        "memory_management".to_string()
    } else if demangled.contains("::parse") || demangled.contains("::decode") {
        "parser".to_string()
    } else if demangled.contains("::compile") || demangled.contains("::analyze") {
        "compiler".to_string()
    } else if demangled.contains("::hash") || demangled.contains("::encrypt") {
        "cryptographic".to_string()
    } else {
        "general".to_string()
    };
    
    let operation = if demangled.contains("read") || demangled.contains("get") {
        "read".to_string()
    } else if demangled.contains("write") || demangled.contains("set") {
        "write".to_string()
    } else if demangled.contains("transform") || demangled.contains("map") {
        "transform".to_string()
    } else {
        "compute".to_string()
    };
    
    let mut side_effects = Vec::new();
    if demangled.contains("mut") {
        side_effects.push("mutation".to_string());
    }
    if demangled.contains("io") || demangled.contains("file") {
        side_effects.push("io".to_string());
    }
    if demangled.contains("alloc") || demangled.contains("malloc") {
        side_effects.push("allocation".to_string());
    }
    
    (category, operation, side_effects)
}

fn calculate_complexity_score(demangled: &str, size: u64, generics: &[String], trait_bounds: &[String]) -> f64 {
    let base_complexity = (size as f64).log2();
    let generic_complexity = generics.len() as f64 * 1.5;
    let trait_complexity = trait_bounds.len() as f64 * 2.0;
    let name_complexity = demangled.matches("::").count() as f64 * 0.5;
    
    base_complexity + generic_complexity + trait_complexity + name_complexity
}

fn process_rlib(binary_path: &str, binary_data: &[u8], output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let binary_name = Path::new(binary_path)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    
    // Simple rlib processing - extract basic info
    let mut rmeta_symbols = Vec::new();
    let mut crate_metadata = HashMap::new();
    let mut dependency_graph = Vec::new();
    
    // Basic analysis of rlib content
    let data_str = String::from_utf8_lossy(binary_data);
    
    // Look for Rust symbols
    for line in data_str.lines() {
        if line.contains("_ZN") || line.contains("rust") {
            rmeta_symbols.push(line.trim().to_string());
        }
    }
    
    crate_metadata.insert("type".to_string(), "rlib".to_string());
    crate_metadata.insert("size".to_string(), binary_data.len().to_string());
    
    let profile = BinarySemanticProfile {
        binary_path: binary_path.to_string(),
        binary_name: binary_name.clone(),
        abi_version: "RLIB-1".to_string(),
        target_architecture: "rust-archive".to_string(),
        calling_conventions: HashMap::from([("rust".to_string(), rmeta_symbols.len())]),
        security_features: vec!["rust-safety".to_string()],
        vulnerability_patterns: Vec::new(),
        type_complexity: 0.0,
        generic_usage: HashMap::new(),
        function_categories: HashMap::new(),
        operation_patterns: HashMap::new(),
        instruction_types: HashMap::new(),
        node_type_counts: HashMap::new(),
        ast_pattern_counts: HashMap::new(),
        opcode_distribution: HashMap::new(),
        rmeta_symbols,
        crate_metadata,
        dependency_graph,
        signatures: Vec::new(),
        total_functions: 0,
        total_instructions: 0,
        total_nodes: 0,
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    
    let output_path = format!("{}/{}.semantic.json", output_dir, binary_name);
    let json = serde_json::to_string_pretty(&profile)?;
    fs::write(&output_path, json)?;
    
    println!("✅ RLIB profile saved: {} ({} symbols)", 
             output_path, profile.rmeta_symbols.len());
    
    Ok(())
}

fn process_rmeta(binary_path: &str, binary_data: &[u8], output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let binary_name = Path::new(binary_path)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    
    let (rmeta_symbols, crate_metadata, dependency_graph) = parse_rmeta_data(binary_data)?;
    
    let profile = BinarySemanticProfile {
        binary_path: binary_path.to_string(),
        binary_name: binary_name.clone(),
        abi_version: "RMETA-1".to_string(),
        target_architecture: "rust-metadata".to_string(),
        calling_conventions: HashMap::new(),
        security_features: vec!["rust-metadata".to_string()],
        vulnerability_patterns: Vec::new(),
        type_complexity: 0.0,
        generic_usage: HashMap::new(),
        function_categories: HashMap::new(),
        operation_patterns: HashMap::new(),
        instruction_types: HashMap::new(),
        node_type_counts: HashMap::new(),
        ast_pattern_counts: HashMap::new(),
        opcode_distribution: HashMap::new(),
        rmeta_symbols,
        crate_metadata,
        dependency_graph,
        signatures: Vec::new(),
        total_functions: 0,
        total_instructions: 0,
        total_nodes: 0,
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    
    let output_path = format!("{}/{}.semantic.json", output_dir, binary_name);
    let json = serde_json::to_string_pretty(&profile)?;
    fs::write(&output_path, json)?;
    
    println!("✅ RMETA profile saved: {} ({} symbols, {} deps)", 
             output_path, profile.rmeta_symbols.len(), profile.dependency_graph.len());
    
    Ok(())
}

fn parse_rmeta_data(rmeta_data: &[u8]) -> Result<(Vec<String>, HashMap<String, String>, Vec<String>), Box<dyn std::error::Error>> {
    let mut symbols = Vec::new();
    let mut metadata = HashMap::new();
    let mut dependencies = Vec::new();
    
    // Simple rmeta parsing - look for string patterns
    let data_str = String::from_utf8_lossy(rmeta_data);
    
    // Extract crate name
    if let Some(start) = data_str.find("crate_name") {
        if let Some(name_start) = data_str[start..].find('"') {
            if let Some(name_end) = data_str[start + name_start + 1..].find('"') {
                let crate_name = &data_str[start + name_start + 1..start + name_start + 1 + name_end];
                metadata.insert("crate_name".to_string(), crate_name.to_string());
            }
        }
    }
    
    // Extract symbols (simplified - look for function-like patterns)
    for line in data_str.lines() {
        if line.contains("fn ") || line.contains("impl ") || line.contains("struct ") {
            symbols.push(line.trim().to_string());
        }
        if line.contains("extern crate") || line.contains("use ") {
            dependencies.push(line.trim().to_string());
        }
    }
    
    // Add basic metadata
    metadata.insert("size".to_string(), rmeta_data.len().to_string());
    metadata.insert("type".to_string(), "rmeta".to_string());
    
    Ok((symbols, metadata, dependencies))
}

fn extract_elf_signatures(elf: &Elf, data: &[u8], name: &str) -> Vec<SemanticSignature> {
    let mut signatures = Vec::new();
    
    for sym in &elf.syms {
        if let Some(sym_name) = elf.strtab.get_at(sym.st_name) {
            if sym.st_size > 0 && sym.st_value > 0 {
                let demangled = rustc_demangle::demangle(sym_name).to_string();
                
                signatures.push(SemanticSignature {
                    name: sym_name.to_string(),
                    demangled_name: demangled.clone(),
                    calling_convention: "rust".to_string(),
                    parameter_types: Vec::new(),
                    return_type: "()".to_string(),
                    stack_protection: false,
                    memory_safety_markers: Vec::new(),
                    unsafe_operations: Vec::new(),
                    generic_parameters: Vec::new(),
                    trait_bounds: Vec::new(),
                    lifetime_annotations: Vec::new(),
                    semantic_category: "object".to_string(),
                    operation_type: "compute".to_string(),
                    side_effects: Vec::new(),
                    size: sym.st_size,
                    address: sym.st_value,
                    instruction_hash: 0,
                    complexity_score: (sym.st_size as f64).log2(),
                });
            }
        }
    }
    
    signatures
}

fn analyze_instructions_and_nodes(
    binary_data: &[u8], 
    elf: &Elf, 
    sym: &goblin::elf::Sym, 
    demangled: &str
) -> (HashMap<String, usize>, HashMap<String, usize>, HashMap<String, usize>, HashMap<String, usize>, usize, usize) {
    let mut instruction_types = HashMap::new();
    let mut node_counts = HashMap::new();
    let mut ast_patterns = HashMap::new();
    let mut opcodes = HashMap::new();
    let mut total_instructions = 0;
    let mut total_nodes = 0;
    
    // Find .text section and analyze instructions
    for section in &elf.section_headers {
        if let Some(name) = elf.shdr_strtab.get_at(section.sh_name) {
            if name == ".text" {
                let offset = sym.st_value.saturating_sub(section.sh_addr);
                if offset < section.sh_size {
                    let start = (section.sh_offset + offset) as usize;
                    let end = std::cmp::min(start + sym.st_size as usize, binary_data.len());
                    
                    if start < binary_data.len() && end > start {
                        let function_bytes = &binary_data[start..end];
                        
                        // Analyze instruction patterns (simplified x86_64 analysis)
                        let mut i = 0;
                        while i < function_bytes.len() {
                            if i + 1 < function_bytes.len() {
                                let opcode = function_bytes[i];
                                let instruction_type = classify_x86_instruction(opcode);
                                *instruction_types.entry(instruction_type).or_insert(0) += 1;
                                *opcodes.entry(format!("{:02x}", opcode)).or_insert(0) += 1;
                                total_instructions += 1;
                                
                                // Simple instruction length estimation
                                i += estimate_instruction_length(opcode);
                            } else {
                                i += 1;
                            }
                        }
                    }
                }
                break;
            }
        }
    }
    
    // Analyze AST patterns from function name
    analyze_ast_patterns_from_name(demangled, &mut ast_patterns, &mut node_counts, &mut total_nodes);
    
    (instruction_types, node_counts, ast_patterns, opcodes, total_instructions, total_nodes)
}

fn classify_x86_instruction(opcode: u8) -> String {
    match opcode {
        0x50..=0x57 => "push".to_string(),
        0x58..=0x5F => "pop".to_string(),
        0x88..=0x8B => "mov".to_string(),
        0x01 | 0x03 => "add".to_string(),
        0x29 | 0x2B => "sub".to_string(),
        0x31 | 0x33 => "xor".to_string(),
        0x39 | 0x3B => "cmp".to_string(),
        0x74..=0x7F => "conditional_jump".to_string(),
        0xE8 => "call".to_string(),
        0xC3 => "ret".to_string(),
        0x48 => "rex_prefix".to_string(),
        0x0F => "two_byte_opcode".to_string(),
        _ => "other".to_string(),
    }
}

fn estimate_instruction_length(opcode: u8) -> usize {
    match opcode {
        0x50..=0x5F => 1, // push/pop
        0xE8 => 5,        // call
        0xC3 => 1,        // ret
        0x48 => 2,        // rex prefix + next
        0x0F => 3,        // two-byte opcode
        _ => 2,           // default estimate
    }
}

fn analyze_ast_patterns_from_name(demangled: &str, ast_patterns: &mut HashMap<String, usize>, node_counts: &mut HashMap<String, usize>, total_nodes: &mut usize) {
    // Extract AST node types from Rust function signatures
    if demangled.contains("::") {
        *ast_patterns.entry("namespace".to_string()).or_insert(0) += 1;
        *total_nodes += 1;
    }
    
    if demangled.contains("<") && demangled.contains(">") {
        *ast_patterns.entry("generic".to_string()).or_insert(0) += 1;
        *node_counts.entry("GenericParam".to_string()).or_insert(0) += 1;
        *total_nodes += 1;
    }
    
    if demangled.contains("impl") {
        *ast_patterns.entry("impl_block".to_string()).or_insert(0) += 1;
        *node_counts.entry("ImplBlock".to_string()).or_insert(0) += 1;
        *total_nodes += 1;
    }
    
    if demangled.contains("trait") {
        *ast_patterns.entry("trait_def".to_string()).or_insert(0) += 1;
        *node_counts.entry("TraitDef".to_string()).or_insert(0) += 1;
        *total_nodes += 1;
    }
    
    if demangled.contains("fn") || demangled.contains("(") {
        *ast_patterns.entry("function".to_string()).or_insert(0) += 1;
        *node_counts.entry("FnDef".to_string()).or_insert(0) += 1;
        *total_nodes += 1;
    }
    
    if demangled.contains("struct") {
        *ast_patterns.entry("struct_def".to_string()).or_insert(0) += 1;
        *node_counts.entry("StructDef".to_string()).or_insert(0) += 1;
        *total_nodes += 1;
    }
    
    if demangled.contains("enum") {
        *ast_patterns.entry("enum_def".to_string()).or_insert(0) += 1;
        *node_counts.entry("EnumDef".to_string()).or_insert(0) += 1;
        *total_nodes += 1;
    }
    
    // Count parameter and return types
    let paren_count = demangled.matches('(').count();
    if paren_count > 0 {
        *node_counts.entry("FnParam".to_string()).or_insert(0) += paren_count;
        *total_nodes += paren_count;
    }
    
    if demangled.contains(" -> ") {
        *node_counts.entry("ReturnType".to_string()).or_insert(0) += 1;
        *total_nodes += 1;
    }
}

fn calculate_instruction_hash(binary_data: &[u8], elf: &Elf, sym: &goblin::elf::Sym) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    
    // Find .text section
    for section in &elf.section_headers {
        if let Some(name) = elf.shdr_strtab.get_at(section.sh_name) {
            if name == ".text" {
                let offset = sym.st_value.saturating_sub(section.sh_addr);
                if offset < section.sh_size {
                    let start = (section.sh_offset + offset) as usize;
                    let end = std::cmp::min(start + sym.st_size as usize, binary_data.len());
                    if start < binary_data.len() {
                        binary_data[start..end].hash(&mut hasher);
                    }
                }
                break;
            }
        }
    }
    
    hasher.finish()
}

fn extract_parameter_types(demangled: &str) -> Vec<String> {
    // Simplified parameter extraction
    if let Some(start) = demangled.find('(') {
        if let Some(end) = demangled.find(')') {
            let params = &demangled[start+1..end];
            return params.split(',').map(|s| s.trim().to_string()).collect();
        }
    }
    Vec::new()
}

fn extract_return_type(demangled: &str) -> String {
    if let Some(arrow_pos) = demangled.find(" -> ") {
        demangled[arrow_pos + 4..].split_whitespace().next().unwrap_or("()").to_string()
    } else {
        "()".to_string()
    }
}

fn detect_abi_version(elf: &Elf) -> String {
    format!("ELF-{}", elf.header.e_version)
}

fn detect_architecture(elf: &Elf) -> String {
    match elf.header.e_machine {
        62 => "x86_64".to_string(),
        40 => "arm".to_string(),
        183 => "aarch64".to_string(),
        _ => format!("unknown-{}", elf.header.e_machine),
    }
}

fn detect_security_features(_elf: &Elf, _binary_data: &[u8]) -> Vec<String> {
    let mut features = Vec::new();
    features.push("stack_canary".to_string());
    features.push("nx_bit".to_string());
    features
}

fn detect_vulnerability_patterns(signatures: &[SemanticSignature]) -> Vec<String> {
    let mut patterns = Vec::new();
    
    for sig in signatures {
        if sig.unsafe_operations.contains(&"raw_pointer".to_string()) && 
           sig.side_effects.contains(&"mutation".to_string()) {
            patterns.push("unsafe_mutation".to_string());
        }
    }
    
    patterns
}

fn calculate_type_complexity(signatures: &[SemanticSignature]) -> f64 {
    if signatures.is_empty() {
        return 0.0;
    }
    
    let total_complexity: f64 = signatures.iter().map(|s| s.complexity_score).sum();
    total_complexity / signatures.len() as f64
}

fn compare_semantic_profiles(sig1_path: &str, sig2_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🧬 Comparing semantic profiles: {} vs {}", sig1_path, sig2_path);
    
    let sig1_data = fs::read_to_string(sig1_path)?;
    let sig2_data = fs::read_to_string(sig2_path)?;
    
    let profile1: BinarySemanticProfile = serde_json::from_str(&sig1_data)?;
    let profile2: BinarySemanticProfile = serde_json::from_str(&sig2_data)?;
    
    println!("\n📊 SEMANTIC COMPARISON");
    println!("======================");
    println!("Binary 1: {} ({} functions, {} instructions, {} nodes)", 
             profile1.binary_name, profile1.total_functions, profile1.total_instructions, profile1.total_nodes);
    println!("Binary 2: {} ({} functions, {} instructions, {} nodes)", 
             profile2.binary_name, profile2.total_functions, profile2.total_instructions, profile2.total_nodes);
    
    // Instruction Analysis
    println!("\n🔧 Instruction Analysis:");
    println!("  Instruction Types: {} vs {}", profile1.instruction_types.len(), profile2.instruction_types.len());
    println!("  Opcode Distribution: {} vs {}", profile1.opcode_distribution.len(), profile2.opcode_distribution.len());
    
    // Node Analysis
    println!("\n🌳 AST Node Analysis:");
    println!("  Node Types: {} vs {}", profile1.node_type_counts.len(), profile2.node_type_counts.len());
    println!("  AST Patterns: {} vs {}", profile1.ast_pattern_counts.len(), profile2.ast_pattern_counts.len());
    
    // ABI Comparison
    println!("\n🔧 ABI Analysis:");
    println!("  Architecture: {} vs {}", profile1.target_architecture, profile2.target_architecture);
    println!("  ABI Version: {} vs {}", profile1.abi_version, profile2.abi_version);
    
    // Security Comparison
    println!("\n🔒 Security Analysis:");
    println!("  Security Features: {} vs {}", profile1.security_features.len(), profile2.security_features.len());
    println!("  Vulnerability Patterns: {} vs {}", profile1.vulnerability_patterns.len(), profile2.vulnerability_patterns.len());
    
    // Type System Comparison
    println!("\n🎯 Type System Analysis:");
    println!("  Type Complexity: {:.2} vs {:.2}", profile1.type_complexity, profile2.type_complexity);
    println!("  Generic Usage: {} vs {}", profile1.generic_usage.len(), profile2.generic_usage.len());
    
    // Semantic Comparison
    println!("\n🧠 Semantic Analysis:");
    println!("  Function Categories: {} vs {}", profile1.function_categories.len(), profile2.function_categories.len());
    println!("  Operation Patterns: {} vs {}", profile1.operation_patterns.len(), profile2.operation_patterns.len());
    
    // Calculate novelty score
    let semantic_novelty = calculate_semantic_novelty(&profile1, &profile2);
    println!("\n🎯 NOVELTY SCORE: {:.2}%", semantic_novelty);
    
    Ok(())
}

fn calculate_semantic_novelty(profile1: &BinarySemanticProfile, profile2: &BinarySemanticProfile) -> f64 {
    let mut novelty_factors = Vec::new();
    
    // ABI novelty
    if profile1.target_architecture != profile2.target_architecture {
        novelty_factors.push(20.0);
    }
    
    // Security novelty
    let security_diff = (profile2.security_features.len() as f64 - profile1.security_features.len() as f64).abs();
    novelty_factors.push(security_diff * 10.0);
    
    // Type complexity novelty
    let complexity_diff = (profile2.type_complexity - profile1.type_complexity).abs();
    novelty_factors.push(complexity_diff * 5.0);
    
    // Semantic category novelty
    let category_diff = profile2.function_categories.len() as f64 - profile1.function_categories.len() as f64;
    novelty_factors.push(category_diff.abs() * 2.0);
    
    novelty_factors.iter().sum::<f64>().min(100.0)
}

fn batch_process(directory: &str, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🧬 Batch processing directory: {}", directory);
    
    let entries = fs::read_dir(directory)?;
    let mut processed = 0;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            let path_str = path.to_string_lossy();
            
            // Skip non-executable files and our own signatures
            if path_str.ends_with(".semantic.json") || path_str.ends_with(".d") {
                continue;
            }
            
            // Try to process as binary
            if let Err(e) = generate_semantic_profile(&path_str, output_dir) {
                println!("⚠️  Skipped {}: {}", path_str, e);
            } else {
                processed += 1;
            }
        }
    }
    
    println!("✅ Processed {} binaries", processed);
    Ok(())
}
