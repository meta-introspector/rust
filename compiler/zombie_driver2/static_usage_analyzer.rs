use goblin::elf::Elf;
use std::collections::HashMap;
use std::fs;

macro_rules! analyze_usage {
    ($binary:expr, addresses: $addresses:expr) => {{
        println!("🔍 ANALYZING STATIC DATA USAGE IN: {}", $binary);
        UsageAnalyzer::new($binary.to_string(), $addresses)
    }};
}

macro_rules! generate_usage_enum {
    ($analyzer:expr, name: $enum_name:ident) => {{
        println!("🔧 GENERATING USAGE ENUM: {}", stringify!($enum_name));
        $analyzer.generate_usage_enum(stringify!($enum_name))
    }};
}

#[derive(Debug, Clone)]
struct FunctionUsage {
    user_address: u64,
    user_name: String,
    used_addresses: Vec<u64>,
    used_names: Vec<String>,
    usage_type: UsageType,
}

#[derive(Debug, Clone)]
enum UsageType {
    DirectCall,    // Function calls another function
    DataReference, // Function references another as data
    VTableEntry,   // Function appears in vtable
    StaticData,    // Function address stored as static data
    JumpTable,     // Function in jump table
    Unknown,
}

struct UsageAnalyzer {
    binary_path: String,
    learned_addresses: HashMap<u64, String>,
    usage_patterns: Vec<FunctionUsage>,
}

impl UsageAnalyzer {
    fn new(binary_path: String, addresses: HashMap<u64, String>) -> Self {
        Self { binary_path, learned_addresses: addresses, usage_patterns: Vec::new() }
    }

    fn analyze_static_usage(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let binary = fs::read(&self.binary_path)?;
        let elf = Elf::parse(&binary)?;

        println!("🔍 Analyzing static data usage patterns...");

        // Get data sections where function addresses might be stored
        let data_sections = [".data", ".rodata", ".got", ".plt", ".bss"];

        for section_name in &data_sections {
            if let Some(section) = elf
                .section_headers
                .iter()
                .find(|sh| elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("") == *section_name)
            {
                self.analyze_section(&binary, section, section_name)?;
            }
        }

        // Also analyze text section for embedded addresses
        if let Some(text_section) = elf
            .section_headers
            .iter()
            .find(|sh| elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("") == ".text")
        {
            self.analyze_text_section(&binary, text_section)?;
        }

        println!("🎯 Found {} usage patterns", self.usage_patterns.len());
        Ok(())
    }

    fn analyze_section(
        &mut self,
        binary: &[u8],
        section: &goblin::elf::SectionHeader,
        section_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let start = section.sh_offset as usize;
        let size = section.sh_size as usize;

        if start + size > binary.len() || size == 0 {
            return Ok(());
        }

        let section_bytes = &binary[start..start + size];

        // Look for 8-byte aligned addresses (64-bit pointers)
        for (i, chunk) in section_bytes.chunks_exact(8).enumerate() {
            let potential_addr = u64::from_le_bytes([
                chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7],
            ]);

            // Check if this looks like a function address
            if self.learned_addresses.contains_key(&potential_addr) {
                let usage_type = match section_name {
                    ".got" | ".plt" => UsageType::DirectCall,
                    ".rodata" => UsageType::StaticData,
                    ".data" => UsageType::VTableEntry,
                    _ => UsageType::Unknown,
                };

                let usage = FunctionUsage {
                    user_address: section.sh_addr + (i * 8) as u64,
                    user_name: format!("{}+0x{:x}", section_name, i * 8),
                    used_addresses: vec![potential_addr],
                    used_names: vec![self.learned_addresses[&potential_addr].clone()],
                    usage_type,
                };

                self.usage_patterns.push(usage);
            }
        }

        Ok(())
    }

    fn analyze_text_section(
        &mut self,
        binary: &[u8],
        text_section: &goblin::elf::SectionHeader,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let start = text_section.sh_offset as usize;
        let size = text_section.sh_size as usize;

        if start + size > binary.len() {
            return Ok(());
        }

        let text_bytes = &binary[start..start + size];

        // Look for embedded function addresses in code
        for (i, window) in text_bytes.windows(8).enumerate().step_by(4) {
            if window.len() >= 8 {
                let potential_addr = u64::from_le_bytes([
                    window[0], window[1], window[2], window[3], window[4], window[5], window[6],
                    window[7],
                ]);

                if self.learned_addresses.contains_key(&potential_addr) {
                    let usage = FunctionUsage {
                        user_address: text_section.sh_addr + i as u64,
                        user_name: format!("text+0x{:x}", i),
                        used_addresses: vec![potential_addr],
                        used_names: vec![self.learned_addresses[&potential_addr].clone()],
                        usage_type: UsageType::DataReference,
                    };

                    self.usage_patterns.push(usage);
                }
            }
        }

        Ok(())
    }

    fn generate_usage_enum(&self, enum_name: &str) -> String {
        let mut enum_code = String::new();

        // Generate enum header
        enum_code
            .push_str(&format!("// Auto-generated usage patterns from {}\n", self.binary_path));
        enum_code.push_str(&format!("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n"));
        enum_code.push_str(&format!("pub enum {} {{\n", enum_name));

        // Group by usage type
        let mut by_type: HashMap<String, Vec<&FunctionUsage>> = HashMap::new();
        for usage in &self.usage_patterns {
            let type_name = format!("{:?}", usage.usage_type);
            by_type.entry(type_name).or_default().push(usage);
        }

        // Generate enum variants
        for (usage_type, usages) in &by_type {
            enum_code.push_str(&format!("    // {} Patterns\n", usage_type));

            for (i, usage) in usages.iter().take(50).enumerate() {
                // Limit to prevent huge enums
                let variant_name =
                    self.create_usage_variant_name(&usage.user_name, &usage.used_names[0], i);
                enum_code.push_str(&format!(
                    "    {} = 0x{:x}, // {} uses {}\n",
                    variant_name,
                    usage.user_address,
                    usage.user_name,
                    &usage.used_names[0][..30.min(usage.used_names[0].len())]
                ));
            }
            enum_code.push_str("\n");
        }

        enum_code.push_str("}\n\n");

        // Generate implementation
        enum_code.push_str(&format!("impl {} {{\n", enum_name));
        enum_code.push_str("    pub fn user_address(&self) -> u64 {\n");
        enum_code.push_str("        *self as u64\n");
        enum_code.push_str("    }\n\n");

        enum_code.push_str("    pub fn used_function(&self) -> u64 {\n");
        enum_code.push_str("        match self {\n");

        for (usage_type, usages) in &by_type {
            for (i, usage) in usages.iter().take(50).enumerate() {
                let variant_name =
                    self.create_usage_variant_name(&usage.user_name, &usage.used_names[0], i);
                enum_code.push_str(&format!(
                    "            {}::{} => 0x{:x},\n",
                    enum_name, variant_name, usage.used_addresses[0]
                ));
            }
        }

        enum_code.push_str("        }\n");
        enum_code.push_str("    }\n\n");

        enum_code.push_str("    pub fn usage_type(&self) -> &'static str {\n");
        enum_code.push_str("        match self {\n");

        for (usage_type, usages) in &by_type {
            for (i, usage) in usages.iter().take(50).enumerate() {
                let variant_name =
                    self.create_usage_variant_name(&usage.user_name, &usage.used_names[0], i);
                enum_code.push_str(&format!(
                    "            {}::{} => \"{:?}\",\n",
                    enum_name, variant_name, usage.usage_type
                ));
            }
        }

        enum_code.push_str("        }\n");
        enum_code.push_str("    }\n");
        enum_code.push_str("}\n");

        enum_code
    }

    fn create_usage_variant_name(&self, user_name: &str, used_name: &str, index: usize) -> String {
        // Create variant name from user -> used relationship
        let mut variant = String::new();

        // Extract meaningful parts from user name
        let user_part = if user_name.contains("+") {
            user_name.split("+").next().unwrap_or("Unknown")
        } else {
            user_name
        };

        // Extract meaningful parts from used name
        let used_part = if used_name.len() > 20 { &used_name[..20] } else { used_name };

        // Build variant name
        for ch in user_part.chars().take(10) {
            if ch.is_alphanumeric() {
                variant.push(ch.to_uppercase().next().unwrap_or(ch));
            } else {
                variant.push('_');
            }
        }

        variant.push_str("_Uses_");

        for ch in used_part.chars().take(15) {
            if ch.is_alphanumeric() {
                variant.push(ch);
            } else {
                variant.push('_');
            }
        }

        variant.push_str(&format!("_{}", index));

        // Ensure valid identifier
        if variant.chars().next().unwrap_or('_').is_numeric() {
            variant = format!("Usage_{}", variant);
        }

        variant
    }

    fn print_usage_statistics(&self) {
        println!("\n📊 STATIC DATA USAGE STATISTICS:");
        println!("=================================");

        let mut type_counts: HashMap<String, usize> = HashMap::new();
        for usage in &self.usage_patterns {
            let type_name = format!("{:?}", usage.usage_type);
            *type_counts.entry(type_name).or_insert(0) += 1;
        }

        for (usage_type, count) in &type_counts {
            println!("   {}: {} patterns", usage_type, count);
        }

        println!("\n🎯 TOP USAGE PATTERNS:");
        for usage in self.usage_patterns.iter().take(10) {
            println!(
                "   {} -> {} ({:?})",
                usage.user_name,
                if usage.used_names[0].len() > 40 {
                    &usage.used_names[0][..40]
                } else {
                    &usage.used_names[0]
                },
                usage.usage_type
            );
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 STATIC DATA USAGE ANALYZER");
    println!("=============================");

    let binary_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/librustc_driver.so";

    // First, we need to load the learned addresses (simplified version)
    let binary = fs::read(binary_path)?;
    let elf = Elf::parse(&binary)?;

    let mut learned_addresses = HashMap::new();

    // Learn some addresses first
    for sym in elf.syms.iter().take(10000) {
        // Limit for performance
        if let Some(name) = elf.strtab.get_at(sym.st_name) {
            if sym.st_size > 0 && sym.st_value > 0 && !name.is_empty() {
                learned_addresses.insert(sym.st_value, name.to_string());
            }
        }
    }

    println!("📚 Loaded {} function addresses", learned_addresses.len());

    let mut analyzer = analyze_usage!(binary_path, addresses: learned_addresses);

    // Analyze static data usage
    analyzer.analyze_static_usage()?;

    // Generate usage enum
    let usage_enum = generate_usage_enum!(analyzer, name: StaticUsagePatterns);

    // Save to file
    fs::write("static_usage_patterns.rs", &usage_enum)?;
    println!("💾 Saved usage enum to: static_usage_patterns.rs");

    // Print statistics
    analyzer.print_usage_statistics();

    println!("\n✅ STATIC USAGE ANALYSIS COMPLETE");
    println!("   Generated enum mapping who uses who as static data");
    println!("   Each variant shows user -> used relationship");
    println!("   Includes usage type classification");

    // Show sample of generated enum
    println!("\n📝 SAMPLE GENERATED USAGE ENUM:");
    println!("{}", &usage_enum[..800.min(usage_enum.len())]);
    if usage_enum.len() > 800 {
        println!("... (truncated)");
    }

    Ok(())
}
