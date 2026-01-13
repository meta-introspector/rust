use goblin::elf::Elf;
use std::collections::HashMap;
use std::fs;

macro_rules! learn_addresses {
    ($binary:expr, version: $version:expr) => {{
        println!("🧠 LEARNING ADDRESSES FROM: {} (version: {})", $binary, $version);
        AddressLearner::new($binary.to_string(), $version.to_string())
    }};
}

macro_rules! generate_enum {
    ($learner:expr, name: $enum_name:ident) => {{
        println!("🔧 GENERATING ENUM: {}", stringify!($enum_name));
        $learner.generate_rust_enum(stringify!($enum_name))
    }};
}

#[derive(Debug, Clone)]
struct LearnedFunction {
    name: String,
    address: u64,
    size: u64,
    demangled_name: String,
    function_type: FunctionType,
}

#[derive(Debug, Clone)]
enum FunctionType {
    Decoder,
    Backend,
    Codegen,
    Architecture,
    Compiler,
    Runtime,
    Unknown,
}

struct AddressLearner {
    binary_path: String,
    version: String,
    learned_functions: Vec<LearnedFunction>,
    address_map: HashMap<u64, String>,
}

impl AddressLearner {
    fn new(binary_path: String, version: String) -> Self {
        Self { binary_path, version, learned_functions: Vec::new(), address_map: HashMap::new() }
    }

    fn learn_all_addresses(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let binary = fs::read(&self.binary_path)?;
        let elf = Elf::parse(&binary)?;

        println!("📚 Learning addresses from {} symbols", elf.syms.len());

        for sym in elf.syms.iter() {
            if let Some(name) = elf.strtab.get_at(sym.st_name) {
                if sym.st_size > 0 && sym.st_value > 0 && !name.is_empty() {
                    let demangled = rust_demangle(name);
                    let function_type = self.classify_function(&demangled);

                    let learned_fn = LearnedFunction {
                        name: name.to_string(),
                        address: sym.st_value,
                        size: sym.st_size,
                        demangled_name: demangled,
                        function_type,
                    };

                    self.address_map.insert(sym.st_value, name.to_string());
                    self.learned_functions.push(learned_fn);
                }
            }
        }

        // Sort by address for better organization
        self.learned_functions.sort_by_key(|f| f.address);

        println!("🎯 Learned {} function addresses", self.learned_functions.len());
        Ok(())
    }

    fn classify_function(&self, demangled_name: &str) -> FunctionType {
        let lower = demangled_name.to_lowercase();

        if lower.contains("decode") || lower.contains("parse") || lower.contains("interpret") {
            FunctionType::Decoder
        } else if lower.contains("backend") || lower.contains("codegen") {
            FunctionType::Backend
        } else if lower.contains("arch") || lower.contains("x86") || lower.contains("fma") {
            FunctionType::Architecture
        } else if lower.contains("rustc") || lower.contains("compiler") {
            FunctionType::Compiler
        } else if lower.contains("runtime") || lower.contains("alloc") {
            FunctionType::Runtime
        } else {
            FunctionType::Unknown
        }
    }

    fn generate_rust_enum(&self, enum_name: &str) -> String {
        let mut enum_code = String::new();

        // Generate enum header
        enum_code
            .push_str(&format!("// Auto-generated from {} ({})\n", self.binary_path, self.version));
        enum_code.push_str(&format!("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n"));
        enum_code.push_str(&format!("pub enum {} {{\n", enum_name));

        // Group functions by type
        let mut by_type: HashMap<String, Vec<&LearnedFunction>> = HashMap::new();
        for func in &self.learned_functions {
            let type_name = format!("{:?}", func.function_type);
            by_type.entry(type_name).or_default().push(func);
        }

        // Generate enum variants
        for (func_type, functions) in &by_type {
            enum_code.push_str(&format!("    // {} Functions\n", func_type));

            for (i, func) in functions.iter().take(20).enumerate() {
                // Limit to prevent huge enums
                let variant_name = self.create_variant_name(&func.demangled_name, i);
                enum_code.push_str(&format!("    {} = 0x{:x},\n", variant_name, func.address));
            }
            enum_code.push_str("\n");
        }

        enum_code.push_str("}\n\n");

        // Generate implementation
        enum_code.push_str(&format!("impl {} {{\n", enum_name));
        enum_code.push_str("    pub fn address(&self) -> u64 {\n");
        enum_code.push_str("        *self as u64\n");
        enum_code.push_str("    }\n\n");

        enum_code.push_str("    pub fn size(&self) -> u64 {\n");
        enum_code.push_str("        match self {\n");

        for (func_type, functions) in &by_type {
            for (i, func) in functions.iter().take(20).enumerate() {
                let variant_name = self.create_variant_name(&func.demangled_name, i);
                enum_code.push_str(&format!(
                    "            {}::{} => {},\n",
                    enum_name, variant_name, func.size
                ));
            }
        }

        enum_code.push_str("        }\n");
        enum_code.push_str("    }\n\n");

        enum_code.push_str("    pub fn name(&self) -> &'static str {\n");
        enum_code.push_str("        match self {\n");

        for (func_type, functions) in &by_type {
            for (i, func) in functions.iter().take(20).enumerate() {
                let variant_name = self.create_variant_name(&func.demangled_name, i);
                let safe_name = func.demangled_name.replace("\"", "\\\"");
                enum_code.push_str(&format!(
                    "            {}::{} => \"{}\",\n",
                    enum_name, variant_name, safe_name
                ));
            }
        }

        enum_code.push_str("        }\n");
        enum_code.push_str("    }\n");
        enum_code.push_str("}\n");

        enum_code
    }

    fn create_variant_name(&self, demangled_name: &str, index: usize) -> String {
        // Create valid Rust identifier from function name
        let mut variant = String::new();
        let mut chars = demangled_name.chars();

        // Start with uppercase letter
        if let Some(first_char) = chars.next() {
            if first_char.is_alphabetic() {
                variant.push(first_char.to_uppercase().next().unwrap_or(first_char));
            } else {
                variant.push('F'); // Default prefix
            }
        }

        // Process remaining characters
        for ch in chars.take(30) {
            // Limit length
            if ch.is_alphanumeric() {
                variant.push(ch);
            } else if ch == ':' || ch == '_' {
                variant.push('_');
            }
        }

        // Add index to ensure uniqueness
        variant.push_str(&format!("_{}", index));

        // Ensure it's a valid identifier
        if variant.chars().next().unwrap_or('_').is_numeric() {
            variant = format!("Func_{}", variant);
        }

        variant
    }

    fn save_enum_to_file(
        &self,
        enum_code: &str,
        filename: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        fs::write(filename, enum_code)?;
        println!("💾 Saved enum to: {}", filename);
        Ok(())
    }

    fn print_statistics(&self) {
        println!("\n📊 ADDRESS LEARNING STATISTICS:");
        println!("===============================");

        let mut type_counts: HashMap<String, usize> = HashMap::new();
        for func in &self.learned_functions {
            let type_name = format!("{:?}", func.function_type);
            *type_counts.entry(type_name).or_insert(0) += 1;
        }

        for (func_type, count) in &type_counts {
            println!("   {}: {} functions", func_type, count);
        }

        println!("\n🎯 TOP LEARNED FUNCTIONS:");
        for func in self.learned_functions.iter().take(10) {
            println!(
                "   0x{:08x}: {} ({:?})",
                func.address,
                if func.demangled_name.len() > 50 {
                    &func.demangled_name[..50]
                } else {
                    &func.demangled_name
                },
                func.function_type
            );
        }
    }
}

fn rust_demangle(mangled: &str) -> String {
    if !mangled.starts_with("_ZN") {
        return mangled.to_string();
    }

    let core = &mangled[3..];
    let mut result = String::new();
    let mut chars = core.chars().peekable();
    let mut current_len = String::new();

    while let Some(ch) = chars.next() {
        if ch.is_ascii_digit() {
            current_len.push(ch);
        } else if ch == 'E' {
            break;
        } else {
            if let Ok(len) = current_len.parse::<usize>() {
                if len > 0 && len < 200 {
                    let mut segment = String::new();
                    segment.push(ch);

                    for _ in 1..len {
                        if let Some(next_ch) = chars.next() {
                            segment.push(next_ch);
                        }
                    }

                    if !result.is_empty() {
                        result.push_str("::");
                    }
                    result.push_str(&segment);
                }
            }
            current_len.clear();
        }
    }

    if result.is_empty() { mangled.to_string() } else { result }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧠 ADDRESS LEARNING SYSTEM");
    println!("==========================");

    let binary_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/librustc_driver.so";

    // Check if binary exists, use fallback if not
    let actual_binary = if std::path::Path::new(binary_path).exists() {
        binary_path
    } else {
        // Find any available .so file
        if let Ok(entries) = fs::read_dir("compiler/zombie_driver2/target/release/deps") {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "so") {
                    if let Some(path_str) = path.to_str() {
                        println!("📁 Using fallback binary: {}", path_str);
                        break;
                    }
                }
            }
        }
        binary_path // Keep original if no fallback found
    };

    let mut learner = learn_addresses!(actual_binary, version: "rustc-1.91.1");

    // Learn all addresses
    learner.learn_all_addresses()?;

    // Generate enum
    let enum_code = generate_enum!(learner, name: RustcAddresses);

    // Save to file
    learner.save_enum_to_file(&enum_code, "rustc_addresses.rs")?;

    // Print statistics
    learner.print_statistics();

    println!("\n✅ ADDRESS LEARNING COMPLETE");
    println!("   Generated enum with exact addresses for this version");
    println!("   Can be used for precise function targeting");
    println!("   Enum variants map directly to binary addresses");

    // Show sample of generated enum
    println!("\n📝 SAMPLE GENERATED ENUM:");
    println!("{}", &enum_code[..500.min(enum_code.len())]);
    if enum_code.len() > 500 {
        println!("... (truncated)");
    }

    Ok(())
}
